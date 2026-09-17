import JSON5 from 'json5';

function isJsonObjectOrArray(value: unknown): value is object {
  return value !== null && typeof value === 'object';
}

/** 判断文本是否为单个对象或数组（支持标准 JSON 与 JS 字面量） */
export function parseStandaloneJson(text: string): object | null {
  const trimmed = text.trim();
  if (!trimmed.startsWith('{') && !trimmed.startsWith('[')) {
    return null;
  }
  try {
    const parsed = JSON.parse(trimmed) as unknown;
    if (isJsonObjectOrArray(parsed)) {
      return parsed;
    }
  } catch {
    // 继续尝试宽松解析
  }
  try {
    const parsed = JSON5.parse(trimmed) as unknown;
    if (isJsonObjectOrArray(parsed)) {
      return parsed;
    }
  } catch {
    return null;
  }
  return null;
}

/** 将对象/数组格式化为标准 JSON 缩进文本 */
export function formatJsonValue(value: object): string {
  return JSON.stringify(value, null, 2);
}
