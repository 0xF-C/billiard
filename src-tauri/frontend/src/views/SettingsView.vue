<template>
  <div class="settings-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('systemSettings') }}</h1>
    </div>

    <div class="settings-section">
      <h3 class="section-title">{{ t('printSettings') }}</h3>
      <div class="settings-grid">
        <div class="setting-card" @click="showPrinterDlg = true; loadPrinters()">
          <div class="setting-icon">
            <el-icon><Printer /></el-icon>
          </div>
          <div class="setting-body">
            <span class="setting-name">{{ t('printerConfig') }}</span>
            <span class="setting-desc">{{ t('printReceipt') }}</span>
          </div>
          <el-icon class="arrow"><ArrowRight /></el-icon>
        </div>
        <div class="setting-card" @click="showPrintTemplateDlg = true">
          <div class="setting-icon">
            <el-icon><Document /></el-icon>
          </div>
          <div class="setting-body">
            <span class="setting-name">{{ t('printTemplate') }}</span>
            <span class="setting-desc">{{ t('printTemplateDesc') }}</span>
          </div>
          <el-icon class="arrow"><ArrowRight /></el-icon>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h3 class="section-title">{{ t('language') }}</h3>
      <div class="lang-option">
        <el-select v-model="curLang" @change="changeLang" class="lang-select">
          <el-option value="zh" label="中文" />
          <el-option value="ug" label="维吾尔语" />
          <el-option value="en" label="English" />
        </el-select>
      </div>
    </div>

    <!-- Printer Config Dialog -->
    <el-dialog v-model="showPrinterDlg" :title="t('printerConfig')" width="800px" class="settings-dialog" top="5vh" append-to-body>
      <div class="dialog-content">
        <div class="printer-header">
          <span class="printer-header-title">{{ t('printerManage') }}</span>
          <el-button type="primary" size="small" @click="showAddPrinter">
            <el-icon><Plus /></el-icon>
            {{ t('addPrinter') }}
          </el-button>
        </div>

        <div v-if="printers.length === 0" class="empty-printers">
          <el-icon :size="48" color="var(--text-tertiary)"><Printer /></el-icon>
          <span>{{ t('noPrinters') }}</span>
        </div>

        <div v-else class="printer-list">
          <div v-for="p in printers" :key="p.id" class="printer-card" :class="{ 'is-default': p.is_default, 'disabled': !p.is_enabled }">
            <div class="printer-card-header">
              <div class="printer-info">
                <span class="printer-name">{{ p.name }}</span>
                <span v-if="p.is_default" class="default-badge">{{ t('defaultPrinter') }}</span>
                <span class="connection-badge">{{ getConnectionLabel(p.connection_type) }}</span>
                <span class="paper-badge">{{ p.paper_width }}mm</span>
              </div>
              <div class="printer-actions">
                <el-switch v-model="p.is_enabled" @change="togglePrinter(p)" size="small" />
                <el-button size="small" @click="testPrinter(p.id)" :loading="testingPrinter === p.id">
                  <el-icon><Monitor /></el-icon>
                  {{ t('testPrint') }}
                </el-button>
                <el-button size="small" @click="editPrinter(p)">
                  <el-icon><Edit /></el-icon>
                </el-button>
                <el-button size="small" type="danger" @click="deletePrinter(p.id)" :disabled="printers.length <= 1">
                  <el-icon><Delete /></el-icon>
                </el-button>
              </div>
            </div>
            <div class="printer-card-body">
              <template v-if="p.connection_type === 'network'">
                <span>{{ p.ip_address }}:{{ p.port }}</span>
              </template>
              <template v-else-if="p.connection_type === 'serial'">
                <span>{{ p.serial_port }} ({{ p.baud_rate }}bps)</span>
              </template>
              <template v-else>
                <span>USB {{ t('device') }}</span>
              </template>
            </div>
            <div v-if="!p.is_default" class="set-default-row">
              <el-button size="small" text type="primary" @click="setDefaultPrinter(p.id)">
                {{ t('setDefault') }}
              </el-button>
            </div>
          </div>
        </div>
      </div>
      <template #footer>
        <el-button @click="showPrinterDlg=false">{{ t('cancel') }}</el-button>
      </template>
    </el-dialog>

    <!-- {{ t('AddEditPrinterDialog') }} -->
    <el-dialog v-model="showPrinterForm" :title="editPrinterTarget ? t('editPrinter') : t('addPrinter')" width="500px" class="settings-dialog" top="5vh" append-to-body>
      <div class="dialog-content">
        <div class="form-item">
          <label>{{ t('printerName') }}</label>
          <el-input v-model="printerForm.name" :placeholder="t('printerName')" />
        </div>
        <div class="form-item">
          <label>{{ t('connectionType') }}</label>
          <el-select v-model="printerForm.connection_type" style="width: 100%">
            <el-option :label="t('network')" value="network" />
            <el-option :label="t('usb')" value="usb" />
            <el-option :label="t('serial')" value="serial" />
          </el-select>
        </div>
        <template v-if="printerForm.connection_type === 'network'">
          <div class="form-item">
            <label>{{ t('printerIp') }}</label>
            <el-input v-model="printerForm.ip_address" placeholder="192.168.1.100" />
          </div>
          <div class="form-item">
            <label>{{ t('port') }}</label>
            <el-input-number v-model="printerForm.port" :min="1" :max="65535" style="width: 100%" />
          </div>
        </template>
        <template v-if="printerForm.connection_type === 'serial'">
          <div class="form-item">
            <label>{{ t('serialPort') }}</label>
            <el-input v-model="printerForm.serial_port" placeholder="/dev/ttyUSB0 或 COM3" />
          </div>
          <div class="form-item">
            <label>{{ t('baudRate') }}</label>
            <el-select v-model="printerForm.baud_rate" style="width: 100%">
              <el-option label="9600" :value="9600" />
              <el-option label="19200" :value="19200" />
              <el-option label="38400" :value="38400" />
              <el-option label="57600" :value="57600" />
              <el-option label="115200" :value="115200" />
            </el-select>
          </div>
        </template>
        <div class="form-item">
          <label>{{ t('paperWidth') }}</label>
          <el-select v-model="printerForm.paper_width" style="width: 100%">
            <el-option label="80mm" :value="80" />
            <el-option label="58mm" :value="58" />
          </el-select>
        </div>
      </div>
      <template #footer>
        <el-button @click="showPrinterForm=false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="savePrinterForm">{{ t('save') }}</el-button>
      </template>
    </el-dialog>

    <!-- Print Template Dialog -->
    <el-dialog v-model="showPrintTemplateDlg" :title="t('printTemplate')" width="600px" class="settings-dialog" top="5vh" append-to-body>
      <div class="dialog-content">
        <div class="setting-group">
          <div class="group-header">{{ t('autoPrint') }}</div>
          <div class="group-body">
            <div class="toggle-row">
              <span>{{ t('autoPrintOnClose') }}</span>
              <el-switch v-model="printTemplate.autoPrintOnClose" />
            </div>
            <div class="toggle-row">
              <span>{{ t('autoPrintOnSale') }}</span>
              <el-switch v-model="printTemplate.autoPrintOnSale" />
            </div>
          </div>
        </div>
        <div class="setting-group">
          <div class="group-header">{{ t('printTemplate') }}</div>
          <div class="group-body">
            <div class="toggle-row">
              <span>{{ t('showShopName') }}</span>
              <el-switch v-model="printTemplate.showShopName" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showDateTime') }}</span>
              <el-switch v-model="printTemplate.showDateTime" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showOrderNo') }}</span>
              <el-switch v-model="printTemplate.showOrderNo" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showTableName') }}</span>
              <el-switch v-model="printTemplate.showTableName" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showMemberName') }}</span>
              <el-switch v-model="printTemplate.showMemberName" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showStartTime') }}</span>
              <el-switch v-model="printTemplate.showStartTime" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showEndTime') }}</span>
              <el-switch v-model="printTemplate.showEndTime" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showDuration') }}</span>
              <el-switch v-model="printTemplate.showDuration" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showDiscount') }}</span>
              <el-switch v-model="printTemplate.showDiscount" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showDeposit') }}</span>
              <el-switch v-model="printTemplate.showDeposit" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showPaymentMethod') }}</span>
              <el-switch v-model="printTemplate.showPaymentMethod" />
            </div>
            <div class="toggle-row">
              <span>{{ t('showFooter') }}</span>
              <el-switch v-model="printTemplate.showFooter" />
            </div>
            <div class="input-row" v-if="printTemplate.showFooter">
              <label>{{ t('footerText') }}</label>
              <el-input v-model="printTemplate.footerText" :placeholder="t('footerText')" />
            </div>
          </div>
        </div>
      </div>
      <template #footer>
        <el-button @click="showPrintTemplateDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="savePrintTemplate">{{ t('save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { PriceTag, Printer, ArrowRight, Document, Plus, Edit, Delete, Monitor } from '@element-plus/icons-vue'
import { currentLang, setLang, t } from '../i18n'
import { getSettings, saveSettings, getPrinters, createPrinter as apiCreatePrinter, updatePrinter as apiUpdatePrinter, deletePrinter as apiDeletePrinter, setDefaultPrinter as apiSetDefaultPrinter, togglePrinter as apiTogglePrinter, testPrinter as apiTestPrinter } from '../api'

const curLang = ref(currentLang.value)
const showPrinterDlg = ref(false)
const showUserDlg = ref(false)
const showBackupDlg = ref(false)
const showPrintTemplateDlg = ref(false)
const editPrinterTarget = ref(null)
const printers = ref([])
const testingPrinter = ref(null)

const printerForm = reactive({
  name: '',
  connection_type: 'network',
  ip_address: '192.168.1.100',
  port: 9100,
  serial_port: '',
  baud_rate: 9600,
  paper_width: 80,
})

const printTemplate = reactive({
  autoPrintOnClose: true,
  autoPrintOnSale: true,
  showShopName: true,
  showDateTime: true,
  showOrderNo: true,
  showTableName: true,
  showMemberName: true,
  showStartTime: true,
  showEndTime: true,
  showDuration: true,
  showDiscount: true,
  showDeposit: true,
  showPaymentMethod: true,
  showFooter: true,
  footerText: t('thankYou'),
})

const changeLang = () => {
  setLang(curLang.value)
}

const loadPrinters = async () => {
  try {
    printers.value = await getPrinters()
  } catch (e) {
    printers.value = []
  }
}

const loadSettings = async () => {
  try {
    const res = await getSettings()
    if (res) {
      Object.assign(printerForm, res.printer || {})
      if (res.printer?.template) {
        Object.assign(printerForm.template, res.printer.template)
      }
      if (res.printer?.protocols) {
        Object.assign(printerForm.protocols, res.printer.protocols)
      }
      if (res.printer?.escpos) {
        Object.assign(printerForm.escpos, res.printer.escpos)
      }
      if (res.printer?.tspl) {
        Object.assign(printerForm.tspl, res.printer.tspl)
      }
      if (res.printer?.zpl) {
        Object.assign(printerForm.zpl, res.printer.zpl)
      }
      if (res.printer?.cpcl) {
        Object.assign(printerForm.cpcl, res.printer.cpcl)
      }
      if (res.printer?.dpl) {
        Object.assign(printerForm.dpl, res.printer.dpl)
      }
      if (res.printTemplate) {
        Object.assign(printTemplate, res.printTemplate)
      }
      if (res.autoPrintOnClose !== undefined) {
        printTemplate.autoPrintOnClose = res.autoPrintOnClose
      }
      if (res.autoPrintOnSale !== undefined) {
        printTemplate.autoPrintOnSale = res.autoPrintOnSale
      }
    }
  } catch (e) {
    console.log('Load settings failed')
  }
}

const savePrintTemplate = async () => {
  try {
    const settings = await getSettings()
    settings.printTemplate = {
      showShopName: printTemplate.showShopName,
      showDateTime: printTemplate.showDateTime,
      showOrderNo: printTemplate.showOrderNo,
      showTableName: printTemplate.showTableName,
      showMemberName: printTemplate.showMemberName,
      showStartTime: printTemplate.showStartTime,
      showEndTime: printTemplate.showEndTime,
      showDuration: printTemplate.showDuration,
      showDiscount: printTemplate.showDiscount,
      showDeposit: printTemplate.showDeposit,
      showPaymentMethod: printTemplate.showPaymentMethod,
      showFooter: printTemplate.showFooter,
      footerText: printTemplate.footerText,
    }
    settings.autoPrintOnClose = printTemplate.autoPrintOnClose
    settings.autoPrintOnSale = printTemplate.autoPrintOnSale
    await saveSettings(settings)
    ElMessage.success(t('saveSuccess'))
    showPrintTemplateDlg.value = false
  } catch (e) {
    ElMessage.error(t('operationFailed'))
  }
}

const showAddPrinter = () => {
  editPrinterTarget.value = null
  printerForm.name = ''
  printerForm.connection_type = 'network'
  printerForm.ip_address = '192.168.1.100'
  printerForm.port = 9100
  printerForm.serial_port = ''
  printerForm.baud_rate = 9600
  printerForm.paper_width = 80
  showPrinterForm.value = true
}

const editPrinter = (p) => {
  editPrinterTarget.value = p
  printerForm.name = p.name
  printerForm.connection_type = p.connection_type
  printerForm.ip_address = p.ip_address || '192.168.1.100'
  printerForm.port = p.port || 9100
  printerForm.serial_port = p.serial_port || ''
  printerForm.baud_rate = p.baud_rate || 9600
  printerForm.paper_width = p.paper_width || 80
  showPrinterForm.value = true
}

const savePrinterForm = async () => {
  if (!printerForm.name) return ElMessage.warning(t('pleaseComplete'))
  try {
    const data = {
      name: printerForm.name,
      connection_type: printerForm.connection_type,
      paper_width: printerForm.paper_width,
    }
    if (printerForm.connection_type === 'network') {
      data.ip_address = printerForm.ip_address
      data.port = printerForm.port
    } else if (printerForm.connection_type === 'serial') {
      data.serial_port = printerForm.serial_port
      data.baud_rate = printerForm.baud_rate
    }
    if (editPrinterTarget.value) {
      await apiUpdatePrinter(editPrinterTarget.value.id, data)
      ElMessage.success(t('updateSuccess'))
    } else {
      await apiCreatePrinter(data)
      ElMessage.success(t('createSuccess'))
    }
    showPrinterForm.value = false
    await loadPrinters()
  } catch (e) {
    ElMessage.error(e.response?.data?.error || t('operationFailed'))
  }
}

const togglePrinter = async (p) => {
  try {
    await apiTogglePrinter(p.id, p.is_enabled)
    ElMessage.success(p.is_enabled ? t('enable') + ' ' + t('success') : t('disable') + ' ' + t('success'))
  } catch (e) {
    p.is_enabled = !p.is_enabled
    ElMessage.error(t('operationFailed'))
  }
}

const setDefaultPrinter = async (id) => {
  try {
    await apiSetDefaultPrinter(id)
    ElMessage.success(t('setDefault') + ' ' + t('success'))
    await loadPrinters()
  } catch (e) {
    ElMessage.error(t('operationFailed'))
  }
}

const testPrinter = async (id) => {
  testingPrinter.value = id
  try {
    const res = await apiTestPrinter(id)
    if (res && res.success) {
      ElMessage.success(t('printerOk'))
    } else {
      ElMessage.warning(res?.message || t('printFailed'))
    }
  } catch (e) {
    ElMessage.error(t('operationFailed'))
  } finally {
    testingPrinter.value = null
  }
}

const deletePrinter = async (id) => {
  try {
    await ElMessageBox.confirm(t('deleteConfirm'), t('confirmDelete'), { type: 'warning' })
    await apiDeletePrinter(id)
    ElMessage.success(t('deleteSuccess'))
    await loadPrinters()
  } catch (e) {
    if (e !== 'cancel') ElMessage.error(t('operationFailed'))
  }
}

const getConnectionLabel = (type) => {
  const map = { network: t('network'), usb: t('usb'), serial: t('serial') }
  return map[type] || type
}

onMounted(() => {
  loadSettings()
  loadPrinters()
})
</script>

<style scoped>
.settings-page { display: flex; flex-direction: column; gap: 24px; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.settings-section { display: flex; flex-direction: column; gap: 16px; }
.section-title { font-size: 14px; font-weight: 600; color: var(--text-secondary); margin: 0; text-transform: uppercase; letter-spacing: 1px; }
.settings-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 12px; }
.setting-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 14px;
  cursor: pointer;
  transition: all 0.2s;
}
.setting-card:hover { border-color: var(--accent-primary); background: var(--bg-tertiary); }
.setting-card:hover .arrow { color: var(--accent-primary); }
.setting-icon {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-sm);
  background: var(--bg-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  color: var(--text-secondary);
}
.setting-body { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.setting-name { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.setting-desc { font-size: 12px; color: var(--text-tertiary); }
.arrow { color: var(--text-tertiary); transition: color 0.2s; }
.lang-option { background: var(--bg-secondary); border: 1px solid var(--border-default); border-radius: var(--radius-md); padding: 16px; }
.lang-select { width: 200px; }

.dialog-content { display: flex; flex-direction: column; gap: 16px; }

.setting-group {
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  overflow: hidden;
}
.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-weight: 600;
  font-size: 13px;
}
.group-body { padding: 16px; display: flex; flex-direction: column; gap: 12px; }

.toggle-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: var(--text-primary);
}

.input-row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.input-row label {
  min-width: 80px;
  color: var(--text-secondary);
  font-size: 13px;
}
.input-row :deep(.el-input),
.input-row :deep(.el-input-number) {
  flex: 1;
}

.checkbox-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}
.checkbox-grid :deep(.el-checkbox__label) {
  color: var(--text-primary) !important;
}

.protocol-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.protocol-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  transition: all 0.2s;
}

.protocol-item:hover {
  border-color: var(--accent-primary);
}

.protocol-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.protocol-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.protocol-desc {
  font-size: 12px;
  color: var(--text-tertiary);
}

.protocol-config {
  margin-top: 8px;
  padding: 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.protocol-config .input-row {
  margin-bottom: 0;
}

.protocol-config .unit {
  font-size: 12px;
  color: var(--text-tertiary);
  min-width: 20px;
}

.unit {
  font-size: 12px;
  color: var(--text-tertiary);
  min-width: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
}
.section-header span {
  color: var(--text-primary);
  font-weight: 600;
}

.user-table {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.form-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.form-item label {
  color: var(--text-secondary);
  font-size: 13px;
}

.printer-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
}
.printer-header-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.empty-printers {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 48px 16px;
  color: var(--text-tertiary);
  font-size: 14px;
}

.printer-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.printer-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  padding: 16px;
  transition: all 0.2s;
}
.printer-card.is-default {
  border-color: var(--accent-success);
}
.printer-card.disabled {
  opacity: 0.6;
}

.printer-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.printer-info {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.printer-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.default-badge {
  font-size: 11px;
  padding: 2px 6px;
  background: rgba(63,185,80,0.15);
  color: var(--accent-success);
  border-radius: 4px;
}

.connection-badge, .paper-badge {
  font-size: 11px;
  padding: 2px 6px;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  border-radius: 4px;
}

.printer-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.printer-card-body {
  margin-top: 8px;
  font-size: 12px;
  color: var(--text-tertiary);
}

.set-default-row {
  margin-top: 8px;
  text-align: right;
}

.backup-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s;
}
.backup-card:hover { border-color: var(--accent-success); background: var(--bg-tertiary); }
.backup-card.restore:hover { border-color: var(--accent-primary); }
.backup-card .backup-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-md);
  background: rgba(63,185,80,0.15);
  color: var(--accent-success);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}
.backup-card.restore .backup-icon {
  background: rgba(88,166,255,0.15);
  color: var(--accent-primary);
}
.backup-info { flex: 1; display: flex; flex-direction: column; gap: 4px; }
.backup-title { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.backup-desc { font-size: 12px; color: var(--text-tertiary); }
.backup-card .arrow { color: var(--text-tertiary); }

.divider {
  display: flex;
  align-items: center;
  gap: 16px;
  color: var(--text-tertiary);
  font-size: 12px;
}
.divider::before, .divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border-default);
}

:deep(.settings-dialog .el-dialog) {
  background: var(--bg-secondary) !important;
}
:deep(.settings-dialog .el-dialog__header) {
  border-bottom: 1px solid var(--border-muted);
}
:deep(.settings-dialog .el-dialog__title) {
  color: var(--text-primary) !important;
}
:deep(.settings-dialog .el-dialog__body) {
  padding: 20px !important;
}
:deep(.settings-dialog .el-table) {
  background: transparent !important;
}
:deep(.settings-dialog .el-table th.el-table__cell) {
  background: var(--bg-tertiary) !important;
  color: var(--text-secondary) !important;
}
:deep(.settings-dialog .el-table td.el-table__cell) {
  background: transparent !important;
  color: var(--text-primary) !important;
  border-bottom-color: var(--border-muted) !important;
}
:deep(.settings-dialog .el-input__wrapper) {
  background: var(--bg-primary) !important;
  box-shadow: none !important;
  border: 1px solid var(--border-default) !important;
}
:deep(.settings-dialog .el-input__inner) {
  color: var(--text-primary) !important;
}
:deep(.settings-dialog .el-select__wrapper) {
  background: var(--bg-primary) !important;
  box-shadow: none !important;
  border: 1px solid var(--border-default) !important;
}
</style>
