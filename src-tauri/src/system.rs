//! 系统集成模块
//! 
//! 提供系统托盘、窗口管理、目录打开等系统级功能

use crate::shortcuts;
use std::path::{Component, Path};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WebviewWindow, WindowEvent,
};
use tauri_plugin_global_shortcut::ShortcutState;

/// 唤起后忽略失焦隐藏的宽限（毫秒）。Windows 在其它应用前台时
/// SetForegroundWindow 常失败，show 后会立刻收到 Focused(false)，否则窗会被马上藏掉。
const OVERLAY_BLUR_GRACE_MS: u64 = 350;

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn overlay_shown_at() -> &'static AtomicU64 {
    static SHOWN_AT: AtomicU64 = AtomicU64::new(0);
    &SHOWN_AT
}

fn arm_overlay_blur_guard() {
    overlay_shown_at().store(now_millis(), Ordering::SeqCst);
}

fn in_overlay_blur_grace() -> bool {
    let shown = overlay_shown_at().load(Ordering::SeqCst);
    now_millis().saturating_sub(shown) < OVERLAY_BLUR_GRACE_MS
}

fn overlay_blur_gen() -> &'static AtomicU64 {
    static GEN: AtomicU64 = AtomicU64::new(0);
    &GEN
}

fn cancel_pending_blur_hide() {
    overlay_blur_gen().fetch_add(1, Ordering::SeqCst);
}

/// 操作系统前台窗口是否仍是本窗口（含子 HWND / WebView2）。
/// 点标题栏拖拽区时 WebView Focused(false)，但 Win32 前台仍是我们。
#[cfg(windows)]
fn window_owns_os_foreground(window: &WebviewWindow) -> bool {
    let Ok(hwnd) = window.hwnd() else {
        return false;
    };
    #[link(name = "user32")]
    extern "system" {
        fn GetForegroundWindow() -> isize;
        fn GetAncestor(hwnd: isize, flags: u32) -> isize;
    }
    const GA_ROOT: u32 = 2;
    let ours = hwnd.0 as isize;
    unsafe {
        let fg = GetForegroundWindow();
        if fg == 0 {
            return false;
        }
        fg == ours || GetAncestor(fg, GA_ROOT) == ours
    }
}

#[cfg(not(windows))]
fn window_owns_os_foreground(window: &WebviewWindow) -> bool {
    window.is_focused().unwrap_or(false)
}

/// 真正切到其它应用才隐藏；编辑器失焦、点 Tab 栏/拖拽头都不隐藏。
fn schedule_hide_if_lost_os_focus(window: WebviewWindow) {
    let gen = overlay_blur_gen().fetch_add(1, Ordering::SeqCst) + 1;
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(80));
        let app = window.app_handle().clone();
        let _ = app.run_on_main_thread(move || {
            if overlay_blur_gen().load(Ordering::SeqCst) != gen {
                return;
            }
            if window.is_focused().unwrap_or(false) {
                return;
            }
            // 以操作系统前台窗口为准：点 Tab 栏/拖拽头时 WebView 会失焦，但前台仍是本窗。
            if window_owns_os_foreground(&window) {
                return;
            }
            hide_window(&window);
        });
    });
}

fn has_parent_dir(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

/// 打开文件夹（使用系统文件管理器）
#[tauri::command]
pub fn open_directory(path: String) -> Result<(), String> {
    let dir = Path::new(&path);
    if has_parent_dir(dir) {
        return Err("不允许的目录路径".to_string());
    }
    if !dir.exists() {
        return Err(format!("目录不存在: {}", path));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    Ok(())
}

/// 把窗口拉到前台。
/// `keep_on_top`：搜索窗保持置顶；主窗口只在抢焦点时短暂置顶，拿到焦点后由事件里取消。
fn reveal_window(window: &WebviewWindow, keep_on_top: bool) {
    arm_overlay_blur_guard();
    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_always_on_top(true);
    let _ = window.set_focus();
    if !keep_on_top {
        // 若已经成功聚焦，立即取消置顶，避免挡住其它应用
        if window.is_focused().unwrap_or(false) {
            let _ = window.set_always_on_top(false);
        }
    }
}

/// 仅当窗口真正在前台时才视为“已打开”。不要只信 is_visible：
/// 用久之后 Windows/WebView 可能报告可见，实际已在其它窗口后面，热键会走 hide 分支导致“按了没反应”。
fn is_effectively_foreground(window: &WebviewWindow, cached_focused: Option<bool>) -> bool {
    if !window.is_visible().unwrap_or(false) || window.is_minimized().unwrap_or(false) {
        return false;
    }
    match cached_focused {
        Some(flag) => flag,
        None => window.is_focused().unwrap_or(false),
    }
}

fn hide_window(window: &WebviewWindow) {
    let _ = window.set_always_on_top(false);
    let _ = window.hide();
}

fn toggle_main_visibility(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if is_effectively_foreground(&window, None) {
            hide_window(&window);
        } else {
            reveal_window(&window, false);
        }
    }
}

fn toggle_main_window(app: &tauri::AppHandle, main_focused: &AtomicBool) {
    if let Some(window) = app.get_webview_window("main") {
        let is_focused = main_focused.load(Ordering::SeqCst);
        if is_effectively_foreground(&window, Some(is_focused)) {
            hide_window(&window);
        } else {
            reveal_window(&window, false);
        }
    }
}

fn attach_search_blur_hide(window: &WebviewWindow) {
    let search_window = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Focused(true) = event {
            cancel_pending_blur_hide();
            return;
        }
        if let WindowEvent::Focused(false) = event {
            if in_overlay_blur_grace() {
                let _ = search_window.set_always_on_top(true);
                let _ = search_window.set_focus();
                return;
            }
            schedule_hide_if_lost_os_focus(search_window.clone());
        }
    });
}

fn create_search_window(app: &tauri::AppHandle) -> Option<WebviewWindow> {
    let builder = if cfg!(debug_assertions) {
        tauri::WebviewWindowBuilder::new(app, "search", tauri::WebviewUrl::App("/#/search".into()))
    } else {
        tauri::WebviewWindowBuilder::new(app, "search", tauri::WebviewUrl::App("/".into()))
            .initialization_script("window.location.hash = '#/search';")
    };

    let window = builder
        .title("搜索工作区")
        .inner_size(650.0, 550.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .center()
        .build()
        .ok()?;

    attach_search_blur_hide(&window);
    Some(window)
}

fn ensure_search_window(app: &tauri::AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window("search")
        .or_else(|| create_search_window(app))
}

fn toggle_search_window(app: &tauri::AppHandle) {
    let Some(search_window) = ensure_search_window(app) else {
        return;
    };
    if is_effectively_foreground(&search_window, None) {
        hide_window(&search_window);
        return;
    }
    reveal_window(&search_window, true);
}

/// 初始化系统托盘
pub fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 创建托盘菜单
    let show_hide = MenuItemBuilder::with_id("show_hide", "显示/隐藏窗口").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&show_hide, &settings, &quit])
        .build()?;

    // 创建系统托盘图标
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show_hide" => {
                toggle_main_visibility(app);
            }
            "settings" => {
                // 检查设置窗口是否已存在
                if let Some(settings_window) = app.get_webview_window("settings") {
                    // 如果存在，显示并聚焦
                    let _ = settings_window.show();
                    let _ = settings_window.set_focus();
                } else {
                    // 如果不存在，发送事件给主窗口创建（不显示主窗口）
                    if let Some(main_window) = app.get_webview_window("main") {
                        let _ = main_window.emit("open-settings", ());
                    }
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            // 处理托盘图标点击事件
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                toggle_main_visibility(app);
            }
        })
        .build(app)?;

    Ok(())
}

/// 初始化窗口管理和全局快捷键
/// 返回主窗口焦点状态的共享引用，供其他模块使用
pub fn setup_windows_and_shortcuts(
    app: &tauri::App,
) -> Result<Arc<AtomicBool>, Box<dyn std::error::Error>> {
    // 主窗口：拿到焦点后取消置顶；失焦则隐藏（与搜索窗一致，宽限内忽略误报失焦）
    let main_focused = Arc::new(AtomicBool::new(false));
    if let Some(main_window) = app.get_webview_window("main") {
        let focused_flag = main_focused.clone();
        let main_for_blur = main_window.clone();
        main_window.on_window_event(move |event| {
            if let WindowEvent::Focused(focused) = event {
                focused_flag.store(*focused, Ordering::SeqCst);
                if *focused {
                    cancel_pending_blur_hide();
                    let _ = main_for_blur.set_always_on_top(false);
                    return;
                }
                if in_overlay_blur_grace() {
                    let _ = main_for_blur.set_focus();
                    return;
                }
                schedule_hide_if_lost_os_focus(main_for_blur.clone());
            }
        });
    }

    // 预创建搜索窗口（隐藏），提升首次打开速度；失败时热键路径会再尝试创建
    let _ = create_search_window(app.handle());

    let main_focused_for_shortcut = main_focused.clone();
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app_handle, received_shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    let (main_binding, search_binding) = shortcuts::current_bindings();
                    if received_shortcut == &main_binding {
                        toggle_main_window(app_handle, main_focused_for_shortcut.as_ref());
                    } else if received_shortcut == &search_binding {
                        toggle_search_window(app_handle);
                    }
                }
            })
            .build(),
    )?;

    let (main_str, search_str) = shortcuts::load_shortcut_strings(app.handle());
    let _ = shortcuts::apply_global_shortcuts(app.handle(), &main_str, &search_str);

    Ok(main_focused)
}

