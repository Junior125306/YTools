/** 全应用强调色（避免紫色） */
export const ACCENT = {
  teal: '#0d9488',
  tealHover: '#0f766e',
  tealPressed: '#115e59',
  tealSoft: '#5eead4',
  tealRgb: '13, 148, 136',
  cyan: '#5ccfe6',
  cyanHover: '#6fdbf0',
  cyanPressed: '#4fb3c9',
  cyanRgb: '92, 207, 230',
  yellow: '#ffcc66',
} as const

export function tealAlpha(alpha: number): string {
  return `rgba(${ACCENT.tealRgb}, ${alpha})`
}

export function cyanAlpha(alpha: number): string {
  return `rgba(${ACCENT.cyanRgb}, ${alpha})`
}
