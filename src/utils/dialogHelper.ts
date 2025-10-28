import { message, ask, confirm } from '@tauri-apps/plugin-dialog';

/**
 * 显示信息提示框
 * @param text 提示文本
 * @param title 标题（可选）
 */
export async function showInfo(text: string, title = '提示'): Promise<void> {
  await message(text, {
    title,
    kind: 'info'
  });
}

/**
 * 显示警告提示框
 * @param text 警告文本
 * @param title 标题（可选）
 */
export async function showWarning(text: string, title = '警告'): Promise<void> {
  await message(text, {
    title,
    kind: 'warning'
  });
}

/**
 * 显示错误提示框
 * @param text 错误文本
 * @param title 标题（可选）
 */
export async function showError(text: string, title = '错误'): Promise<void> {
  await message(text, {
    title,
    kind: 'error'
  });
}

/**
 * 显示确认对话框（是/否）
 * @param text 确认文本
 * @param title 标题（可选）
 * @returns true 表示确认，false 表示取消
 */
export async function showConfirm(text: string, title = '确认'): Promise<boolean> {
  return await confirm(text, {
    title,
    kind: 'warning'
  });
}

/**
 * 显示询问对话框（是/否/取消）
 * @param text 询问文本
 * @param title 标题（可选）
 * @returns true 表示"是"，false 表示"否"，null 表示取消
 */
export async function showAsk(
  text: string, 
  title = '询问'
): Promise<boolean | null> {
  return await ask(text, { title });
}

/**
 * 显示成功消息
 * @param text 成功文本
 * @param title 标题（可选）
 */
export async function showSuccess(text: string, title = '成功'): Promise<void> {
  await message(text, {
    title,
    kind: 'info'
  });
}

