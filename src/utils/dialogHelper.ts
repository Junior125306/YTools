import { createDiscreteApi } from 'naive-ui'

// 创建独立的 message 和 dialog API（无需 Provider 包裹）
const { message, dialog } = createDiscreteApi(['message', 'dialog'])

/**
 * 显示信息提示框
 * @param text 提示文本
 * @param title 标题（可选）
 */
export async function showInfo(text: string, title = '提示'): Promise<void> {
  message.info(text, { duration: 2500 })
}

/**
 * 显示警告提示框
 * @param text 警告文本
 * @param title 标题（可选）
 */
export async function showWarning(text: string, title = '警告'): Promise<void> {
  message.warning(text, { duration: 3000 })
}

/**
 * 显示错误提示框
 * @param text 错误文本
 * @param title 标题（可选）
 */
export async function showError(text: string, title = '错误'): Promise<void> {
  message.error(text, { duration: 3000 })
}

/**
 * 显示确认对话框（是/否）
 * @param text 确认文本
 * @param title 标题（可选）
 * @returns true 表示确认，false 表示取消
 */
export async function showConfirm(text: string, title = '确认'): Promise<boolean> {
  return new Promise((resolve) => {
    dialog.warning({
      title,
      content: text,
      positiveText: '确定',
      negativeText: '取消',
      onPositiveClick: () => resolve(true),
      onNegativeClick: () => resolve(false),
      onClose: () => resolve(false)
    })
  })
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
  return new Promise((resolve) => {
    dialog.info({
      title,
      content: text,
      positiveText: '是',
      negativeText: '否',
      onPositiveClick: () => resolve(true),
      onNegativeClick: () => resolve(false),
      onClose: () => resolve(null)
    })
  })
}

/**
 * 显示成功消息
 * @param text 成功文本
 * @param title 标题（可选）
 */
export async function showSuccess(text: string, title = '成功'): Promise<void> {
  message.success(text, { duration: 2500 })
}

