//! 系统集成模块
//! 
//! 提供系统托盘、窗口管理、目录打开等系统级功能

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// 打开文件夹（使用系统文件管理器）
#[tauri::command]
pub fn open_directory(path: String) -> Result<(), String> {
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
                if let Some(window) = app.get_webview_window("main") {
                    // 使用与快捷键相同的逻辑
                    let is_visible = window.is_visible().unwrap_or(false);
                    let is_minimized = window.is_minimized().unwrap_or(false);

                    if is_visible && !is_minimized {
                        let _ = window.hide();
                    } else {
                        if is_minimized {
                            let _ = window.unminimize();
                        }
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
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
                if let Some(window) = app.get_webview_window("main") {
                    // 使用与快捷键相同的逻辑
                    let is_visible = window.is_visible().unwrap_or(false);
                    let is_minimized = window.is_minimized().unwrap_or(false);

                    if is_visible && !is_minimized {
                        let _ = window.hide();
                    } else {
                        if is_minimized {
                            let _ = window.unminimize();
                        }
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
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
    // 跟踪主窗口焦点状态（不再在失焦时自动隐藏）
    let main_focused = Arc::new(AtomicBool::new(false));
    if let Some(main_window) = app.get_webview_window("main") {
        let focused_flag = main_focused.clone();
        main_window.on_window_event(move |event| {
            if let WindowEvent::Focused(focused) = event {
                focused_flag.store(*focused, Ordering::SeqCst);
            }
        });
    }

    // 预创建搜索窗口（隐藏），提升首次打开速度
    use tauri::WebviewWindowBuilder;
    let search_window_result = if cfg!(debug_assertions) {
        // 开发模式
        WebviewWindowBuilder::new(app, "search", tauri::WebviewUrl::App("/#/search".into()))
    } else {
        // 生产模式
        WebviewWindowBuilder::new(app, "search", tauri::WebviewUrl::App("/".into()))
            .initialization_script("window.location.hash = '#/search';")
    };

    if let Ok(search_window) = search_window_result
        .title("搜索工作区")
        .inner_size(650.0, 550.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .shadow(false) // 移除窗口阴影和边框
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false) // 默认隐藏
        .center()
        .build()
    {
        // 为搜索窗口添加失焦自动隐藏功能
        let search_window_clone = search_window.clone();
        search_window.on_window_event(move |event| {
            if let WindowEvent::Focused(false) = event {
                // 窗口失去焦点时自动隐藏
                let _ = search_window_clone.hide();
            }
        });
    }

    // 设置全局快捷键
    let alt_space = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    let ctrl_space = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);

    let main_focused_for_shortcut = main_focused.clone();
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app_handle, received_shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    // Alt+Space: 显示/隐藏主窗口
                    if received_shortcut == &alt_space {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            // 更可靠的状态检查：同时检查可见性和是否最小化
                            let is_visible = window.is_visible().unwrap_or(false);
                            let is_minimized = window.is_minimized().unwrap_or(false);
                            let is_focused =
                                main_focused_for_shortcut.load(Ordering::SeqCst);

                            if is_visible && !is_minimized {
                                // 如果已获得焦点，则隐藏；否则将其聚焦到前台
                                if is_focused {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            } else {
                                // 窗口不可见或最小化，显示并聚焦
                                if is_minimized {
                                    let _ = window.unminimize();
                                }
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    // Ctrl+Space: 切换搜索窗口显示/隐藏（窗口已预创建）
                    else if received_shortcut == &ctrl_space {
                        if let Some(search_window) = app_handle.get_webview_window("search")
                        {
                            // 切换显示状态
                            if search_window.is_visible().unwrap_or(false) {
                                let _ = search_window.hide();
                            } else {
                                let _ = search_window.show();
                                let _ = search_window.set_focus();
                            }
                        }
                    }
                }
            })
            .build(),
    )?;

    // 注册 Alt+Space 快捷键
    let _ = app.global_shortcut().register(alt_space);

    // 注册 Ctrl+Space 快捷键
    let _ = app.global_shortcut().register(ctrl_space);

    Ok(main_focused)
}

