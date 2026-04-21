<template>
  <div class="table-manage-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">{{ t('tableManage') }}</h1>
        <div class="filter-group">
          <el-select v-model="filterArea" :placeholder="t('areaOverview')" clearable size="default" style="width: 140px;">
            <el-option :label="t('allAreas')" value="all" />
            <el-option v-for="area in areas" :key="area.id" :label="area.name" :value="area.id" />
          </el-select>
        </div>
      </div>
      <el-button type="primary" :icon="Plus" @click="showAdd">
        <el-icon><Plus /></el-icon>
        {{ t('add') }}
      </el-button>
    </div>

    <div class="tables-grid">
      <div v-for="item in filteredTables" :key="item.id" class="table-card">
        <div class="card-header">
          <div class="table-info">
            <span class="table-name">{{ item.name }}</span>
            <el-tag :type="item.is_private ? 'warning' : 'success'" size="small">
              {{ getAreaName(item.area_id) }}
            </el-tag>
          </div>
          <el-tag :type="getStatusType(item.status)" size="small" effect="dark">
            {{ getStatusText(item.status) }}
          </el-tag>
        </div>

        <div class="card-body">
          <div class="info-row">
            <el-icon><Location /></el-icon>
            <span>{{ getAreaName(item.area_id) }}</span>
          </div>
          <div class="info-row">
            <el-icon><Money /></el-icon>
            <span class="rate">¥{{ item.rate_per_hour }}/{{ t('hour') }}</span>
          </div>
          <div class="info-row">
            <el-icon><Grid /></el-icon>
            <span>{{ item.table_type }}</span>
          </div>
        </div>

        <div class="card-footer">
          <el-button type="primary" plain size="small" @click="showQR(item)">
            <el-icon><Grid /></el-icon>
            {{ t('qrCode') }}
          </el-button>
          <el-button size="small" @click="doEdit(item)">
            <el-icon><Edit /></el-icon>
            {{ t('edit') }}
          </el-button>
          <el-button size="small" type="danger" @click="doDelete(item)">
            <el-icon><Delete /></el-icon>
            {{ t('remove') }}
          </el-button>
        </div>
      </div>

      <div v-if="filteredTables.length === 0" class="empty-card">
        <el-icon :size="48"><Box /></el-icon>
        <span>{{ t('noTables') }}</span>
      </div>
    </div>

    <!-- Add/Edit Dialog -->
    <el-dialog v-model="formDlg" :title="editTarget ? t('editTable') : t('addTable')" width="520px" :close-on-click-modal="false" class="table-form-dialog">
      <div class="form-body">
        <div class="form-icon-wrap">
          <el-icon class="form-big-icon"><Box /></el-icon>
        </div>

        <el-form :model="form" label-position="top" class="elegant-form">
          <el-form-item :label="t('areaManage')" required>
            <el-select v-model="form.area_id" :placeholder="t('selectArea')" class="full-width" @change="onAreaChange">
              <el-option v-for="a in areas" :key="a.id" :label="a.name + ' ¥' + a.rate_per_hour + '/' + t('hour')" :value="a.id" />
            </el-select>
          </el-form-item>

          <div class="form-row">
            <el-form-item :label="t('tableNum')" required>
              <el-input-number v-model="form.num" :min="1" :step="1" :precision="0" class="full-width" />
            </el-form-item>
            <el-form-item :label="t('tableRate') + ' (¥/' + t('hour') + ')'" required>
              <el-input-number v-model="form.rate_per_hour" :min="1" :step="5" class="full-width" />
            </el-form-item>
          </div>

          <el-form-item :label="t('tableType')" required>
            <el-select v-model="form.table_type" :placeholder="t('selectType')" class="full-width">
              <el-option v-for="cat in categories" :key="cat.id" :label="cat.name" :value="cat.name" />
            </el-select>
          </el-form-item>
        </el-form>
      </div>
      <template #footer>
        <el-button @click="formDlg=false" size="large">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="cfmForm" size="large">
          <el-icon><Check /></el-icon>
          {{ t('save') }}
        </el-button>
      </template>
    </el-dialog>

    <!-- QR Dialog -->
    <el-dialog v-model="qrDlg" :title="qrTable ? qrTable.name + ' - ' + t('scanQR') : ''" width="360px" :close-on-click-modal="false">
      <div class="qr-content" v-if="qrTable">
        <div class="qr-wrapper">
          <canvas ref="qrCanvas"></canvas>
        </div>
        <div class="qr-info">
          <div class="qr-name">{{ qrTable.name }}</div>
          <div class="qr-rate">¥{{ qrTable.rate_per_hour }}/{{ t('hour') }}</div>
        </div>
        <div class="qr-actions">
          <el-button type="primary" @click="downloadQR">
            <el-icon><Download /></el-icon>
            {{ t('downloadQR') }}
          </el-button>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, nextTick } from 'vue'
import { ElMessage, ElMessageBox, ElButton, ElIcon, ElSelect, ElOption, ElInputNumber, ElTag, ElForm, ElFormItem, ElDialog } from 'element-plus'
import { Plus, Edit, Delete, Download, Check, Location, Money, Grid, Box } from '@element-plus/icons-vue'
import QRCodeLib from 'qrcode'
import { getTables, createTable, updateTable, deleteTable, getAreas, getTableCategories } from '../api'
import { t } from '../i18n'

const tables = ref([])
const areas = ref([])
const categories = ref([])
const filterArea = ref('all')
const formDlg = ref(false)
const editTarget = ref(null)
const form = ref({ area_id: null, num: null, table_type: '', rate_per_hour: 10 })

const qrDlg = ref(false)
const qrTable = ref(null)
const qrCanvas = ref(null)

const filteredTables = computed(() => {
  if (!filterArea.value || filterArea.value === 'all') return tables.value
  return tables.value.filter(item => item.area_id === filterArea.value)
})

const getAreaName = (areaId) => {
  if (!areaId) return '-'
  const area = areas.value.find(a => a.id === areaId)
  return area ? area.name : '-'
}

const onAreaChange = (areaId) => {
  const area = areas.value.find(a => a.id === areaId)
  if (area) {
    form.value.rate_per_hour = area.rate_per_hour
  }
}

const getStatusType = (status) => {
  if (status === '使用中') return 'danger'
  if (status === '维护中') return 'warning'
  return 'success'
}

const getStatusText = (status) => {
  if (status === '使用中') return t('tableInUse')
  if (status === '维护中') return t('tableMaintenance')
  return t('tableFree')
}

const loadTables = async () => {
  try { tables.value = await getTables() }
  catch { ElMessage.error(t('loadFailed')) }
}

const loadAreas = async () => {
  try { areas.value = await getAreas() }
  catch { areas.value = [] }
}

const loadCategories = async () => {
  try {
    categories.value = await getTableCategories()
    if (categories.value.length > 0 && !form.value.table_type) {
      form.value.table_type = categories.value[0].name
    }
  } catch { categories.value = [] }
}

const showAdd = () => {
  editTarget.value = null
  form.value = {
    area_id: null, num: null,
    table_type: categories.value[0]?.name || '',
    rate_per_hour: 10
  }
  formDlg.value = true
}

const doEdit = (item) => {
  editTarget.value = item
  const match = item.name.match(/^(\d+)/)
  form.value = {
    area_id: item.area_id,
    num: match ? parseInt(match[1]) : null,
    table_type: item.table_type || categories.value[0]?.name || '',
    rate_per_hour: item.rate_per_hour
  }
  formDlg.value = true
}

const cfmForm = async () => {
  if (!form.value.area_id) return ElMessage.warning(t('selectArea') || '请选择区域')
  if (!form.value.num) return ElMessage.warning(t('pleaseComplete'))
  try {
    const name = form.value.num + '号' + (form.value.table_type || '')
    const data = {
      name, table_type: form.value.table_type,
      rate_per_hour: form.value.rate_per_hour, area_id: form.value.area_id,
      is_private: form.value.table_type?.includes('包房') || false
    }
    if (editTarget.value) { await updateTable(editTarget.value.id, data) }
    else { await createTable(name, data.table_type, data.rate_per_hour, data.is_private, data.area_id) }
    ElMessage.success(t('saveSuccess'))
    formDlg.value = false
    await loadTables()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const doDelete = async (item) => {
  try {
    await ElMessageBox.confirm(`${t('confirmDelete')}「${item.name}」？`, '⚠️', { type: 'warning' })
    await deleteTable(item.id)
    ElMessage.success(t('deleteSuccess'))
    await loadTables()
  } catch(e) { if (e !== 'cancel') ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const getBaseUrl = () => {
  const host = window.location.hostname
  return `http://${host}:5001`
}

const showQR = async (table) => {
  qrTable.value = table
  qrDlg.value = true
  
  await nextTick()
  
  if (qrCanvas.value) {
    const url = `${getBaseUrl()}/open/${table.id}`
    try {
      QRCodeLib.toCanvas(qrCanvas.value, url, {
        width: 200,
        margin: 2,
        color: {
          dark: '#58a6ff',
          light: '#ffffff'
        }
      })
    } catch (e) {
      console.error('QR Code error:', e)
    }
  }
}

const downloadQR = () => {
  if (!qrCanvas.value) return
  
  const url = qrCanvas.value.toDataURL('image/png')
  const link = document.createElement('a')
  link.download = `桌球-${qrTable.value?.name}-二维码.png`
  link.href = url
  link.click()
  ElMessage.success(t('scanQR') + ' ' + t('saveSuccess'))
}

onMounted(() => { loadTables(); loadAreas(); loadCategories() })
</script>

<style scoped>
.table-manage-page { display: flex; flex-direction: column; gap: 24px; }

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 16px;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.page-title { 
  font-size: 24px; 
  font-weight: 600; 
  color: var(--text-primary); 
  margin: 0; 
}

.filter-group { display: flex; gap: 8px; }

.tables-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.table-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  transition: all 0.2s;
}

.table-card:hover {
  border-color: var(--accent-primary);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.table-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.table-name {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.info-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--text-secondary);
}

.info-row .el-icon {
  color: var(--text-tertiary);
}

.rate {
  color: var(--accent-success);
  font-weight: 600;
}

.card-footer {
  display: flex;
  gap: 8px;
  margin-top: auto;
}

.card-footer .el-button {
  flex: 1;
}

.empty-card {
  grid-column: 1 / -1;
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 80px;
  text-align: center;
  color: var(--text-tertiary);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.empty-card span {
  font-size: 14px;
}

/* Form Dialog */
.form-body {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.form-icon-wrap {
  text-align: center;
  padding: 20px;
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
}

.form-big-icon {
  font-size: 48px;
  color: var(--accent-primary);
}

.elegant-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.full-width { width: 100%; }

:deep(.el-form-item__label) {
  color: var(--text-secondary) !important;
  font-weight: 500 !important;
  padding-bottom: 8px !important;
}

:deep(.el-form-item) {
  margin-bottom: 0;
}

/* QR Code */
.qr-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  padding: 20px 0;
}

.qr-wrapper {
  background: var(--bg-tertiary);
  padding: 16px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-default);
  box-shadow: 0 4px 12px rgba(0,0,0,0.3);
}

.qr-wrapper canvas {
  display: block;
}

.qr-info {
  text-align: center;
}

.qr-name {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.qr-rate {
  font-size: 14px;
  color: var(--accent-success);
}

.qr-actions {
  display: flex;
  gap: 12px;
}

@media (max-width: 768px) {
  .tables-grid {
    grid-template-columns: 1fr;
  }
}
</style>
