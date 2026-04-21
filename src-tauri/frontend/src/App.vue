<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script setup>
</script>

<style>
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

.page-title { display: none !important; }

:root {
  --bg-primary: #0b0f16;
  --bg-secondary: #111820;
  --bg-tertiary: #19202c;
  --bg-elevated: #252d3a;
  --bg-hover: rgba(88,166,255,0.06);
  --bg-active: rgba(88,166,255,0.12);

  --border-default: #252d3a;
  --border-muted: #19202c;
  --border-accent: #58a6ff;

  --text-primary: #edf1f7;
  --text-secondary: #9ba7b6;
  --text-tertiary: #6a7685;
  --text-link: #58a6ff;

  --accent-primary: #58a6ff;
  --accent-primary-hover: #79b8ff;
  --accent-success: #3fb950;
  --accent-warning: #e5a530;
  --accent-danger: #f85149;
  --accent-purple: #a371f7;

  --gradient-gold: linear-gradient(135deg, #ffd700, #f0a500);
  --gradient-green: linear-gradient(135deg, #238636, #2ea043);
  --gradient-red: linear-gradient(135deg, #da3633, #f85149);
  --gradient-blue: linear-gradient(135deg, #1f6feb, #388bfd);
  --gradient-card: linear-gradient(160deg, rgba(88,166,255,0.03), transparent 60%);

  --shadow-sm: 0 1px 2px rgba(0,0,0,0.3), 0 0 1px rgba(0,0,0,0.2);
  --shadow-md: 0 4px 16px rgba(0,0,0,0.45), 0 0 1px rgba(0,0,0,0.15);
  --shadow-lg: 0 12px 40px rgba(0,0,0,0.55), 0 0 1px rgba(0,0,0,0.1);
  --shadow-xl: 0 20px 60px rgba(0,0,0,0.6);
  --shadow-glow: 0 0 30px rgba(88,166,255,0.1);
  --shadow-card: 0 2px 8px rgba(0,0,0,0.2), 0 0 1px rgba(88,166,255,0.03);
  --shadow-card-hover: 0 8px 24px rgba(0,0,0,0.35), 0 0 12px rgba(88,166,255,0.06);

  --radius-xs: 4px;
  --radius-sm: 6px;
  --radius-md: 10px;
  --radius-lg: 16px;
  --radius-xl: 20px;
  --radius-2xl: 24px;

  --font-sans: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  --font-mono: ui-monospace, 'SF Mono', Consolas, monospace;

  --transition-fast: 0.15s ease;
  --transition-normal: 0.25s ease;
  --transition-slow: 0.4s ease;
}

body {
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: 14px;
  line-height: 1.6;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#app {
  min-height: 100vh;
  background: var(--bg-primary);
}

/* Scrollbar */
::-webkit-scrollbar { width: 6px; height: 6px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { background: var(--bg-elevated); border-radius: 3px; }
::-webkit-scrollbar-thumb:hover { background: var(--border-accent); }

/* Selection */
::selection { background: rgba(88,166,255,0.25); color: var(--text-primary); }

/* ===== Element Plus Global Overrides ===== */

/* --- Table --- */
.el-table { background: transparent !important; --el-table-border-color: var(--border-muted); }
.el-table::before { display: none !important; }
.el-table th.el-table__cell {
  background: var(--bg-tertiary) !important;
  color: var(--text-secondary) !important;
  border-color: var(--border-muted) !important;
  font-weight: 600;
  font-size: 13px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.el-table td.el-table__cell {
  background: transparent !important;
  color: var(--text-primary) !important;
  border-color: var(--border-muted) !important;
}
.el-table tr { transition: background var(--transition-fast); }
.el-table tr:hover td.el-table__cell { background: var(--bg-hover) !important; }
.el-table--striped .el-table__body tr.el-table__row--striped td.el-table__cell { background: rgba(255,255,255,0.015) !important; }
.el-table__empty-block { background: transparent !important; }
.el-table__empty-text { color: var(--text-tertiary) !important; }

/* --- Dialog --- */
.el-dialog {
  background: var(--bg-secondary) !important;
  border: 1px solid var(--border-default) !important;
  border-radius: var(--radius-xl) !important;
  box-shadow: var(--shadow-xl), 0 0 0 1px rgba(88,166,255,0.04) !important;
  top: 3vh !important;
  margin-top: 30px !important;
  overflow: hidden;
}
.el-dialog__header {
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-default);
  padding: 16px 24px !important;
  margin-right: 0 !important;
}
.el-dialog__title { color: var(--text-primary) !important; font-weight: 600; font-size: 16px; }
.el-dialog__headerbtn .el-dialog__close { color: var(--text-tertiary) !important; }
.el-dialog__headerbtn:hover .el-dialog__close { color: var(--text-primary) !important; }
.el-dialog__body {
  max-height: 75vh !important;
  overflow-y: auto !important;
  padding: 20px 24px !important;
  color: var(--text-primary);
}
.el-dialog__footer {
  position: sticky !important;
  bottom: 0 !important;
  background: var(--bg-secondary) !important;
  border-top: 1px solid var(--border-default) !important;
  padding: 14px 24px !important;
}

/* --- Input --- */
.el-input__wrapper {
  background: var(--bg-tertiary) !important;
  box-shadow: none !important;
  border: 1px solid var(--border-default) !important;
  border-radius: var(--radius-md) !important;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast) !important;
}
.el-input__inner { color: var(--text-primary) !important; }
.el-input__inner::placeholder { color: var(--text-tertiary) !important; }
.el-input__wrapper:hover { border-color: rgba(88,166,255,0.4) !important; }
.el-input__wrapper.is-focus { border-color: var(--accent-primary) !important; box-shadow: 0 0 0 3px rgba(88,166,255,0.1) !important; }
.el-input__prefix .el-icon, .el-input__suffix .el-icon { color: var(--text-tertiary) !important; }

.el-textarea__inner {
  background: var(--bg-tertiary) !important;
  border: 1px solid var(--border-default) !important;
  color: var(--text-primary) !important;
  border-radius: var(--radius-md) !important;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast) !important;
}
.el-textarea__inner::placeholder { color: var(--text-tertiary) !important; }
.el-textarea__inner:focus { border-color: var(--accent-primary) !important; box-shadow: 0 0 0 3px rgba(88,166,255,0.1) !important; }

/* --- Select --- */
.el-select__wrapper {
  background: var(--bg-tertiary) !important;
  box-shadow: none !important;
  border: 1px solid var(--border-default) !important;
  border-radius: var(--radius-md) !important;
  transition: border-color var(--transition-fast) !important;
}
.el-select__wrapper:hover { border-color: rgba(88,166,255,0.4) !important; }
.el-select__wrapper.is-focused { border-color: var(--accent-primary) !important; box-shadow: 0 0 0 3px rgba(88,166,255,0.1) !important; }
.el-select-dropdown {
  background: var(--bg-secondary) !important;
  border: 1px solid var(--border-default) !important;
  border-radius: var(--radius-md) !important;
  box-shadow: var(--shadow-lg) !important;
}
.el-select-dropdown__item { color: var(--text-secondary) !important; transition: all var(--transition-fast); }
.el-select-dropdown__item.is-hovering { background: var(--bg-active) !important; color: var(--text-primary) !important; }
.el-select-dropdown__item.is-selected { color: var(--accent-primary) !important; font-weight: 600; }

/* --- Button --- */
.el-button {
  border-radius: var(--radius-md) !important;
  font-weight: 500;
  transition: all var(--transition-fast);
  letter-spacing: 0.2px;
}
.el-button--primary {
  background: var(--gradient-blue) !important;
  border: none !important;
  color: #fff !important;
  box-shadow: 0 2px 8px rgba(31,111,235,0.25) !important;
}
.el-button--primary:hover { transform: translateY(-1px); box-shadow: 0 4px 16px rgba(31,111,235,0.4) !important; filter: brightness(1.1); }
.el-button--primary:active { transform: translateY(0); }
.el-button--success {
  background: var(--gradient-green) !important;
  border: none !important;
  color: #fff !important;
  box-shadow: 0 2px 8px rgba(35,134,54,0.25) !important;
}
.el-button--success:hover { filter: brightness(1.1); transform: translateY(-1px); }
.el-button--danger {
  background: var(--gradient-red) !important;
  border: none !important;
  color: #fff !important;
  box-shadow: 0 2px 8px rgba(218,54,51,0.25) !important;
}
.el-button--danger:hover { filter: brightness(1.1); transform: translateY(-1px); }
.el-button--warning {
  background: var(--gradient-gold) !important;
  border: none !important;
  color: #0a0a1a !important;
  box-shadow: 0 2px 8px rgba(255,215,0,0.2) !important;
}
.el-button--default {
  background: var(--bg-tertiary) !important;
  border: 1px solid var(--border-default) !important;
  color: var(--text-primary) !important;
}
.el-button--default:hover {
  border-color: var(--accent-primary) !important;
  color: var(--accent-primary) !important;
  background: var(--bg-active) !important;
}
.el-button--default:active { background: var(--bg-elevated) !important; }

/* --- Tag --- */
.el-tag { border-radius: var(--radius-sm) !important; }
.el-tag--success { background: rgba(63,185,80,0.12) !important; border-color: rgba(63,185,80,0.25) !important; color: var(--accent-success) !important; }
.el-tag--danger { background: rgba(248,81,73,0.12) !important; border-color: rgba(248,81,73,0.25) !important; color: var(--accent-danger) !important; }
.el-tag--warning { background: rgba(229,165,48,0.12) !important; border-color: rgba(229,165,48,0.25) !important; color: var(--accent-warning) !important; }
.el-tag--info { background: rgba(88,166,255,0.12) !important; border-color: rgba(88,166,255,0.25) !important; color: var(--accent-primary) !important; }

/* --- Message --- */
.el-message { border-radius: var(--radius-md) !important; box-shadow: var(--shadow-md) !important; }
.el-message--success { background: rgba(63,185,80,0.12) !important; border-color: rgba(63,185,80,0.25) !important; }
.el-message--error { background: rgba(248,81,73,0.12) !important; border-color: rgba(248,81,73,0.25) !important; }
.el-message--warning { background: rgba(229,165,48,0.12) !important; border-color: rgba(229,165,48,0.25) !important; }
.el-message__content { color: var(--text-primary) !important; }

/* --- Overlay --- */
.el-overlay { backdrop-filter: blur(12px); background: rgba(5,8,14,0.65) !important; }

/* --- Form --- */
.el-form-item__label { color: var(--text-secondary) !important; font-weight: 500 !important; }
.el-form-item__error { color: var(--accent-danger) !important; }

/* --- Switch --- */
.el-switch.is-checked .el-switch__core { background-color: var(--accent-primary) !important; border-color: var(--accent-primary) !important; }
.el-switch .el-switch__core { background-color: var(--bg-elevated) !important; border-color: var(--border-default) !important; }

/* --- Radio Button --- */
.el-radio-button__inner {
  background: var(--bg-tertiary) !important;
  border-color: var(--border-default) !important;
  color: var(--text-secondary) !important;
}
.el-radio-button__original-radio:checked + .el-radio-button__inner {
  background: var(--accent-primary) !important;
  border-color: var(--accent-primary) !important;
  color: #fff !important;
  box-shadow: none !important;
}

/* --- Checkbox --- */
.el-checkbox__inner { background: var(--bg-tertiary) !important; border-color: var(--border-default) !important; border-radius: 3px !important; }
.el-checkbox__input.is-checked .el-checkbox__inner { background: var(--accent-primary) !important; border-color: var(--accent-primary) !important; }
.el-checkbox__label { color: var(--text-primary) !important; }

/* --- Divider --- */
.el-divider { border-color: var(--border-default) !important; }

/* --- Pagination --- */
.el-pagination { --el-pagination-bg-color: var(--bg-tertiary); --el-pagination-text-color: var(--text-secondary); }
.el-pager li { background: var(--bg-tertiary) !important; color: var(--text-secondary) !important; border-radius: var(--radius-sm) !important; }
.el-pager li:hover { color: var(--accent-primary) !important; }
.el-pager li.is-active { background: var(--accent-primary) !important; color: #fff !important; }

/* --- Date Picker --- */
.el-date-editor { --el-date-editor-datetimerange-width: 380px; }
.el-date-picker, .el-picker-panel { background: var(--bg-secondary) !important; border-color: var(--border-default) !important; }
.el-picker-panel__body { background: var(--bg-secondary) !important; }

/* --- Input Number --- */
.el-input-number {
  border-radius: var(--radius-md) !important;
  overflow: hidden;
}
.el-input-number .el-input__wrapper {
  background: var(--bg-tertiary) !important;
  box-shadow: none !important;
}
.el-input-number .el-input__inner {
  color: var(--text-primary) !important;
  text-align: center !important;
}
.el-input-number__decrease,
.el-input-number__increase {
  background: var(--bg-elevated) !important;
  color: var(--text-primary) !important;
  border-color: var(--border-default) !important;
  transition: all var(--transition-fast) !important;
}
.el-input-number__decrease:hover,
.el-input-number__increase:hover {
  background: var(--accent-primary) !important;
  color: #fff !important;
}
.el-input-number__decrease:active,
.el-input-number__increase:active {
  background: var(--accent-primary-hover) !important;
}
.el-input-number.is-disabled .el-input-number__decrease,
.el-input-number.is-disabled .el-input-number__increase {
  background: var(--bg-tertiary) !important;
  color: var(--text-tertiary) !important;
}

/* --- Autocomplete --- */
.el-autocomplete-suggestion {
  background: var(--bg-secondary) !important;
  border: 1px solid var(--border-default) !important;
  border-radius: var(--radius-md) !important;
  box-shadow: var(--shadow-lg) !important;
}
.el-autocomplete-suggestion li { color: var(--text-primary) !important; }
.el-autocomplete-suggestion li.hover { background: var(--bg-active) !important; }

/* --- Card --- */
.el-card { background: var(--bg-secondary) !important; border-color: var(--border-default) !important; border-radius: var(--radius-lg) !important; color: var(--text-primary) !important; }

/* --- Alert --- */
.el-alert { border-radius: var(--radius-md) !important; }
.el-alert--success { background: rgba(63,185,80,0.1) !important; border-color: rgba(63,185,80,0.25) !important; color: var(--accent-success) !important; }
.el-alert--warning { background: rgba(229,165,48,0.1) !important; border-color: rgba(229,165,48,0.25) !important; color: var(--accent-warning) !important; }
.el-alert--error { background: rgba(248,81,73,0.1) !important; border-color: rgba(248,81,73,0.25) !important; color: var(--accent-danger) !important; }
.el-alert__title { color: var(--text-primary) !important; }
.el-alert__description { color: var(--text-secondary) !important; }

/* --- Loading --- */
.el-loading-mask { background-color: rgba(11,15,22,0.75) !important; }
.el-loading-spinner .el-loading-text { color: var(--text-secondary) !important; }
.el-loading-spinner .circular .path { stroke: var(--accent-primary) !important; }

/* --- Tooltip --- */
.el-tooltip__popper { background: var(--bg-elevated) !important; border: 1px solid var(--border-default) !important; color: var(--text-primary) !important; border-radius: var(--radius-sm) !important; }
.el-tooltip__popper .el-tooltip__arrow::before { border-color: var(--bg-elevated) !important; }

/* --- Popover --- */
.el-popover { background: var(--bg-secondary) !important; border-color: var(--border-default) !important; color: var(--text-primary) !important; border-radius: var(--radius-lg) !important; box-shadow: var(--shadow-lg) !important; }
.el-popover__title { color: var(--text-primary) !important; }

/* --- Dropdown Menu --- */
.el-dropdown-menu {
  background: var(--bg-secondary) !important;
  border: 1px solid var(--border-default) !important;
  border-radius: var(--radius-md) !important;
  box-shadow: var(--shadow-lg) !important;
  padding: 4px !important;
}
.el-dropdown-menu__item { color: var(--text-secondary) !important; border-radius: var(--radius-sm) !important; }
.el-dropdown-menu__item:hover { background: var(--bg-active) !important; color: var(--text-primary) !important; }

/* --- MessageBox --- */
.el-message-box { background: var(--bg-secondary) !important; border-color: var(--border-default) !important; border-radius: var(--radius-xl) !important; box-shadow: var(--shadow-xl) !important; }
.el-message-box__title { color: var(--text-primary) !important; }
.el-message-box__message { color: var(--text-secondary) !important; }
.el-message-box__header { border-bottom: 1px solid var(--border-default) !important; }

/* --- Tab --- */
.el-tabs__item { color: var(--text-secondary) !important; }
.el-tabs__item.is-active { color: var(--accent-primary) !important; }
.el-tabs__active-bar { background-color: var(--accent-primary) !important; }
.el-tabs__nav-wrap::after { background-color: var(--border-default) !important; }

/* --- Badge --- */
.el-badge__content { background-color: var(--accent-danger) !important; border: none !important; }

/* --- Link --- */
.el-link { color: var(--accent-primary) !important; }
.el-link:hover { color: var(--accent-primary-hover) !important; }
.el-link--primary { color: var(--accent-primary) !important; }
</style>