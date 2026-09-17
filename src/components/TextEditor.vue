<template>
  <div class="text-editor-container" :class="{ 'is-dark': isDark }" @wheel="handleWheel">
    <transition name="fade">
      <div v-if="showSaveStatus" class="save-status-indicator">
        <NSpin v-if="saveStatus === 'saving'" :size="18" />
        <NIcon v-else-if="saveStatus === 'saved'" size="20" color="#52c41a">
          <CheckmarkCircleOutline />
        </NIcon>
        <NIcon v-else-if="saveStatus === 'failed'" size="20" color="#f5222d">
          <CloseCircleOutline />
        </NIcon>
      </div>
    </transition>
    <div ref="editorHost" class="cm-host"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted, onUnmounted } from 'vue';
import { NIcon, NSpin } from 'naive-ui';
import { CheckmarkCircleOutline, CloseCircleOutline } from '@vicons/ionicons5';
import { EditorView, keymap, highlightActiveLine, placeholder as cmPlaceholder } from '@codemirror/view';
import { EditorState, Compartment } from '@codemirror/state';
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
import { searchKeymap, highlightSelectionMatches, search, searchPanelOpen, closeSearchPanel } from '@codemirror/search';
import { markdown } from '@codemirror/lang-markdown';
import { HighlightStyle, syntaxHighlighting, defaultHighlightStyle } from '@codemirror/language';
import { tags as t } from '@lezer/highlight';
import { useTheme } from '../composables/useTheme';
import { ACCENT } from '../constants/theme';
import { formatJsonValue, parseStandaloneJson } from '../utils/jsonFormat';

interface Props {
  modelValue: string;
  placeholder?: string;
  height?: string;
  fontSize?: number;
  fontFamily?: string;
  lineHeight?: number;
  saveStatus?: string;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
  (e: 'update:fontSize', value: number): void;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '开始输入你的笔记...',
  height: '100%',
  fontSize: 16,
  fontFamily: "Consolas, 'Courier New', monospace",
  lineHeight: 1.6,
  saveStatus: ''
});

const emit = defineEmits<Emits>();

const { isDark, themeMode } = useTheme();
const isCyberpunk = computed(() => themeMode.value === 'cyberpunk');

const editorHost = ref<HTMLDivElement>();
const fontSize = ref(props.fontSize);
const fontFamily = ref(props.fontFamily);
const lineHeight = ref(props.lineHeight);
const showSaveStatus = ref(false);
let hideTimer: number | null = null;
let view: EditorView | null = null;
const appearance = new Compartment();
const highlighting = new Compartment();

const darkHighlight = HighlightStyle.define([
  { tag: t.content, color: '#e5e7eb' },
  { tag: t.heading, color: ACCENT.cyan, fontWeight: 'bold' },
  { tag: t.strong, color: '#e5e7eb', fontWeight: 'bold' },
  { tag: t.emphasis, fontStyle: 'italic', color: '#d1d5db' },
  { tag: t.link, color: ACCENT.tealSoft },
  { tag: t.url, color: ACCENT.cyan },
  { tag: t.comment, color: '#94a3b8' },
  { tag: t.keyword, color: '#86efac' },
  { tag: t.string, color: '#fcd34d' },
  { tag: t.monospace, color: '#cbd5e1' },
  { tag: t.meta, color: '#94a3b8' },
  { tag: t.processingInstruction, color: '#94a3b8' },
]);

function editorColors() {
  if (isCyberpunk.value) {
    return { text: '#d7dce2', muted: '#8b9cb3', caret: ACCENT.cyan };
  }
  if (isDark.value) {
    return { text: '#e5e7eb', muted: '#9ca3af', caret: ACCENT.tealSoft };
  }
  return { text: '#1c1917', muted: '#78716c', caret: ACCENT.teal };
}

function buildHighlight() {
  return isDark.value
    ? syntaxHighlighting(darkHighlight, { fallback: true })
    : syntaxHighlighting(defaultHighlightStyle, { fallback: true });
}

function buildAppearance() {
  const colors = editorColors();
  return EditorView.theme(
    {
      '&': {
        height: '100%',
        fontSize: `${fontSize.value}px`,
        backgroundColor: 'transparent',
        color: colors.text,
      },
      '.cm-scroller': {
        fontFamily: `${fontFamily.value}, 'Microsoft YaHei', sans-serif`,
        lineHeight: String(lineHeight.value),
        overflow: 'auto',
      },
      '.cm-content': {
        padding: '16px 12px',
        caretColor: colors.caret,
        color: colors.text,
      },
      '.cm-line': {
        color: colors.text,
      },
      '.cm-cursor, .cm-dropCursor': {
        borderLeftColor: colors.caret,
      },
      '.cm-gutters': {
        backgroundColor: 'transparent',
        border: 'none',
        color: colors.muted,
      },
      '.cm-activeLine': {
        backgroundColor: isDark.value ? 'rgba(13, 148, 136, 0.12)' : 'rgba(13, 148, 136, 0.08)',
      },
      '.cm-activeLineGutter': {
        backgroundColor: 'transparent',
        color: colors.text,
      },
      '&.cm-focused': {
        outline: 'none',
      },
      '.cm-selectionBackground, &.cm-focused .cm-selectionBackground': {
        backgroundColor: isDark.value
          ? 'rgba(13, 148, 136, 0.38) !important'
          : 'rgba(13, 148, 136, 0.28) !important',
      },
      '.cm-placeholder': {
        color: colors.muted,
      },
      '.cm-panel': {
        backgroundColor: isDark.value ? '#1f2937' : '#f5f5f4',
        color: colors.text,
      },
      '.cm-panel input': {
        backgroundColor: isDark.value ? '#111827' : '#ffffff',
        color: colors.text,
        border: `1px solid ${colors.muted}`,
      },
    },
    { dark: isDark.value }
  );
}

function getDoc(): string {
  return view?.state.doc.toString() ?? '';
}

function syncFromProps(text: string) {
  if (!view || getDoc() === text) return;
  view.dispatch({
    changes: {
      from: 0,
      to: view.state.doc.length,
      insert: text,
    },
  });
}

watch(() => props.saveStatus, (newStatus) => {
  if (newStatus) {
    showSaveStatus.value = true;
    if (hideTimer) {
      clearTimeout(hideTimer);
      hideTimer = null;
    }
    if (newStatus === 'saved') {
      hideTimer = window.setTimeout(() => {
        showSaveStatus.value = false;
        hideTimer = null;
      }, 450);
    }
  } else {
    showSaveStatus.value = false;
  }
});

watch(() => props.modelValue, (newValue) => {
  syncFromProps(newValue);
});

watch(() => props.fontSize, (newSize) => {
  if (fontSize.value !== newSize) {
    fontSize.value = newSize;
  }
});

watch(() => props.fontFamily, (newFamily) => {
  if (fontFamily.value !== newFamily) {
    fontFamily.value = newFamily;
  }
});

watch(() => props.lineHeight, (newHeight) => {
  if (lineHeight.value !== newHeight) {
    lineHeight.value = newHeight;
  }
});

function reconfigureEditor() {
  if (!view) return;
  view.dispatch({
    effects: [
      appearance.reconfigure(buildAppearance()),
      highlighting.reconfigure(buildHighlight()),
    ],
  });
}

watch([fontSize, fontFamily, lineHeight, isDark, themeMode], () => {
  reconfigureEditor();
});

const handleWheel = (e: WheelEvent) => {
  if (e.ctrlKey) {
    e.preventDefault();
    const delta = e.deltaY > 0 ? -1 : 1;
    const newSize = Math.max(12, Math.min(32, fontSize.value + delta));
    if (newSize !== fontSize.value) {
      fontSize.value = newSize;
      emit('update:fontSize', newSize);
    }
  }
};

const getValue = () => getDoc();

const setValue = (value: string) => {
  syncFromProps(value);
  emit('update:modelValue', value);
};

const insertValue = (value: string) => {
  if (!view) return;
  const { from, to } = view.state.selection.main;
  view.dispatch({
    changes: { from, to, insert: value },
    selection: { anchor: from + value.length },
  });
  emit('update:modelValue', getDoc());
};

const focus = () => {
  view?.focus();
};

const getSelection = () => {
  if (!view) return '';
  const { from, to } = view.state.selection.main;
  return view.state.sliceDoc(from, to);
};

/** 查找面板是否打开，供主窗 Esc 判断 */
const isSearchOpen = () => {
  if (!view) return false;
  return searchPanelOpen(view.state) || !!view.dom.querySelector('.cm-panel');
};

const closeSearch = () => {
  if (!view) return false;
  return closeSearchPanel(view);
};

function formatPastedJson(editor: EditorView) {
  const text = editor.state.doc.toString();
  const parsed = parseStandaloneJson(text);
  if (!parsed) return;
  const formatted = formatJsonValue(parsed);
  if (formatted === text.trim() && text.trim() === text) return;
  editor.dispatch({
    changes: { from: 0, to: editor.state.doc.length, insert: formatted },
    userEvent: 'input.format.json',
  });
}

defineExpose({
  getValue,
  setValue,
  insertValue,
  focus,
  getSelection,
  isSearchOpen,
  closeSearch,
});

onMounted(() => {
  fontSize.value = props.fontSize;
  if (!editorHost.value) return;

  view = new EditorView({
    parent: editorHost.value,
    state: EditorState.create({
      doc: props.modelValue,
      extensions: [
        highlightActiveLine(),
        highlightSelectionMatches(),
        history(),
        markdown(),
        highlighting.of(buildHighlight()),
        search(),
        EditorView.lineWrapping,
        cmPlaceholder(props.placeholder),
        keymap.of([...defaultKeymap, ...historyKeymap, ...searchKeymap, indentWithTab]),
        appearance.of(buildAppearance()),
        EditorView.domEventHandlers({
          paste(event, editor) {
            const clipboard = event.clipboardData?.getData('text/plain') ?? '';
            if (!clipboard.trim()) return false;
            const { from, to } = editor.state.selection.main;
            const leftover =
              editor.state.sliceDoc(0, from) +
              editor.state.sliceDoc(to, editor.state.doc.length);
            if (leftover.trim() !== '') return false;
            const parsed = parseStandaloneJson(clipboard);
            if (!parsed) return false;
            event.preventDefault();
            editor.dispatch({
              changes: {
                from: 0,
                to: editor.state.doc.length,
                insert: formatJsonValue(parsed),
              },
              userEvent: 'input.paste',
            });
            return true;
          },
        }),
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            const value = update.state.doc.toString();
            emit('update:modelValue', value);
            emit('change', value);
          }
          const isFormat = update.transactions.some((tr) =>
            tr.isUserEvent('input.format.json')
          );
          if (isFormat) return;
          const pasted = update.transactions.some((tr) => tr.isUserEvent('input.paste'));
          let replacedWholeDoc = false;
          update.changes.iterChanges((fromA, toA) => {
            if (fromA === 0 && toA === update.startState.doc.length) {
              replacedWholeDoc = true;
            }
          });
          if (pasted || replacedWholeDoc) {
            queueMicrotask(() => formatPastedJson(update.view));
          }
        }),
      ],
    }),
  });
});

onUnmounted(() => {
  view?.destroy();
  view = null;
  if (hideTimer) {
    clearTimeout(hideTimer);
  }
});
</script>

<style scoped>
.text-editor-container {
  height: 100%;
  width: 100%;
  position: relative;
  display: flex;
  flex-direction: column;
  border: 1px solid var(--n-border-color);
  border-radius: 8px;
  overflow: hidden;
  background: var(--n-color, transparent);
}

.save-status-indicator {
  position: absolute;
  top: 1rem;
  right: 1rem;
  z-index: 10;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.cm-host {
  height: 100%;
  width: 100%;
  min-height: 0;
}

.cm-host :deep(.cm-editor) {
  height: 100%;
}

.text-editor-container.is-dark :deep(.cm-content),
.text-editor-container.is-dark :deep(.cm-line) {
  color: #e5e7eb;
}

.cm-host :deep(.cm-scroller)::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.cm-host :deep(.cm-scroller)::-webkit-scrollbar-track {
  background: transparent;
}

.cm-host :deep(.cm-scroller)::-webkit-scrollbar-thumb {
  background: var(--n-scrollbar-color);
  border-radius: 4px;
}
</style>
