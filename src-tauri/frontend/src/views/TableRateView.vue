<template>
  <div class="table-rate-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('rateSettings') }}</h1>
      <el-button type="primary" @click="showPackageDlg = true">
        <el-icon><Plus /></el-icon>
        {{ t('addPackage') }}
      </el-button>
    </div>

    <!-- Package List -->
    <div class="section">
      <h3 class="section-title">{{ t('packageSettings') }}</h3>
      <div class="package-grid" v-if="packages.length > 0">
        <div v-for="pkg in packages" :key="pkg.id" class="package-card">
          <div class="package-header">
            <div class="package-icon" :class="pkg.type">
              <el-icon><Ticket /></el-icon>
            </div>
            <div class="package-info">
              <span class="package-name">{{ pkg.name }}</span>
              <el-tag :type="getPackageType(pkg.type)" size="small">{{ getPackageTypeName(pkg.type) }}</el-tag>
            </div>
            <el-button size="small" type="danger" plain @click="deletePackageById(pkg.id)">
              <el-icon><Delete /></el-icon>
            </el-button>
          </div>
          <div class="package-body">
            <div class="package-price">
              <span class="price-value">¥{{ pkg.price }}</span>
              <span class="price-unit">/{{ pkg.hours }}{{ t('hours') }}</span>
            </div>
            <div class="package-time" v-if="pkg.start_time || pkg.end_time">
              <el-icon><Clock /></el-icon>
              {{ pkg.start_time || '00:00' }} - {{ pkg.end_time || '23:59' }}
            </div>
            <div class="package-desc">{{ pkg.description || t('noData') }}</div>
          </div>
          <div class="package-tables">
            <span class="tables-label">{{ t('applicableAreas') }}：</span>
            <el-tag v-for="aid in (pkg.area_ids || '').split(',').filter(Boolean)" :key="'a'+aid" size="small" type="warning" class="table-tag">
              {{ getAreaName(parseInt(aid)) }}
            </el-tag>
            <span v-if="!pkg.area_ids && (!pkg.table_ids || pkg.table_ids === '*')" class="all-tables">{{ t('allTables') }}</span>
          </div>
          <div class="package-tables" v-if="pkg.area_ids || (pkg.table_ids && pkg.table_ids !== '*')">
            <span class="tables-label">{{ t('applicableTables') }}：</span>
            <el-tag v-for="tid in (pkg.table_ids || '').split(',').filter(Boolean)" :key="tid" size="small" class="table-tag">
              {{ getTableName(parseInt(tid)) }}
            </el-tag>
          </div>
        </div>
      </div>
      <div v-else class="empty-packages">
        <el-icon :size="48"><Ticket /></el-icon>
        <span>{{ t('noPackages') }}</span>
      </div>
    </div>

    <!-- Special Time Price -->
    <div class="section">
      <h3 class="section-title">{{ t('specialRates') }}</h3>
      <div class="special-rates">
        <div class="rate-card">
          <div class="rate-header">
            <el-icon><Calendar /></el-icon>
            <span>{{ t('weekendRate') }}</span>
          </div>
          <div class="rate-body">
            <div class="rate-item">
              <label>{{ t('priceMultiplier') }}</label>
              <el-input-number v-model="specialRates.weekend.multiplier" :min="1" :max="5" :step="0.1" :precision="1" />
            </div>
            <div class="rate-item">
              <label>{{ t('effectiveDays') }}</label>
              <el-select v-model="specialRates.weekend.days" multiple :placeholder="t('search')">
                <el-option :label="t('saturday')" :value="6" />
                <el-option :label="t('sunday')" :value="0" />
              </el-select>
            </div>
          </div>
        </div>

        <div class="rate-card">
          <div class="rate-header">
            <el-icon><Present /></el-icon>
            <span>{{ t('holidayRate') }}</span>
          </div>
          <div class="rate-body">
            <div class="rate-item">
              <label>{{ t('priceMultiplier') }}</label>
              <el-input-number v-model="specialRates.holiday.multiplier" :min="1" :max="5" :step="0.1" :precision="1" />
            </div>
            <div class="rate-item">
              <label>{{ t('holidayDates') }}</label>
              <el-input v-model="specialRates.holiday.dates" :placeholder="t('holidayDates')" />
            </div>
          </div>
        </div>

        <div class="rate-card">
          <div class="rate-header">
            <el-icon><Star /></el-icon>
            <span>{{ t('memberDay') }}</span>
          </div>
          <div class="rate-body">
            <div class="rate-item">
              <label>{{ t('enable') }}</label>
              <el-switch v-model="specialRates.memberDay.enabled" />
            </div>
            <div class="rate-item">
              <label>{{ t('memberDayDiscount') }} (%)</label>
              <el-input-number v-model="specialRates.memberDay.discount" :min="0" :max="100" :step="5" :disabled="!specialRates.memberDay.enabled" />
            </div>
            <div class="rate-item">
              <label>{{ t('memberDayDates') }}</label>
              <el-input v-model="specialRates.memberDay.dates" :placeholder="t('memberDayDatesPlaceholder')" :disabled="!specialRates.memberDay.enabled" />
            </div>
          </div>
        </div>

        <div class="rate-card">
          <div class="rate-header">
            <el-icon><Switch /></el-icon>
            <span>{{ t('autoCloseExhausted') }}</span>
          </div>
          <div class="rate-body">
            <div class="rate-item">
              <label>{{ t('autoCloseExhaustedDesc') }}</label>
              <el-switch v-model="specialRates.autoClose.enabled" />
            </div>
          </div>
        </div>
      </div>
      <el-button type="primary" @click="saveSpecialRates" class="save-special">
        <el-icon><Check /></el-icon>
        {{ t('saveSpecialRates') }}
      </el-button>
    </div>

    <div v-if="tables.length === 0" class="empty-card">
      <el-icon :size="48"><Warning /></el-icon>
      <span>{{ t('noData') }}</span>
    </div>

    <!-- Add Package Dialog -->
    <el-dialog v-model="showPackageDlg" :title="t('addPackage')" width="500px" class="package-dialog">
      <div class="package-form">
        <div class="form-item">
          <label>{{ t('packageName') }}</label>
          <el-input v-model="packageForm.name" :placeholder="t('packageName')" />
        </div>
        <div class="form-row">
          <div class="form-item">
            <label>{{ t('price') }}</label>
            <el-input-number v-model="packageForm.price" :min="1" />
          </div>
          <div class="form-item">
            <label>{{ t('duration') }}</label>
            <el-input-number v-model="packageForm.hours" :min="0.5" :step="0.5" :precision="1" />
          </div>
        </div>
        <div class="form-row">
          <div class="form-item">
            <label>{{ t('startTime') }}</label>
            <el-time-select v-model="packageForm.start_time" :placeholder="t('startTime')" start="00:00" step="00:30" end="23:30" />
          </div>
          <div class="form-item">
            <label>{{ t('endTime') }}</label>
            <el-time-select v-model="packageForm.end_time" :placeholder="t('endTime')" start="00:00" step="00:30" end="23:30" />
          </div>
        </div>
        <div class="form-item">
          <label>{{ t('packageType') }}</label>
          <el-select v-model="packageForm.type" style="width: 100%">
            <el-option :label="t('morning')" value="morning" />
            <el-option :label="t('afternoon')" value="afternoon" />
            <el-option :label="t('evening')" value="evening" />
            <el-option :label="t('night')" value="night" />
            <el-option :label="t('holiday')" value="holiday" />
            <el-option :label="t('weekend')" value="weekend" />
            <el-option :label="t('normal')" value="normal" />
          </el-select>
        </div>
        <div class="form-item">
          <label>{{ t('applicableAreas') }}</label>
          <el-select v-model="packageForm.area_ids" multiple :placeholder="t('allAreas')" style="width: 100%" :disabled="packageForm.table_ids.includes('*')">
            <el-option v-for="a in areas" :key="a.id" :label="a.name" :value="a.id" />
          </el-select>
        </div>
        <div class="form-item">
          <label>{{ t('applicableTables') }}</label>
          <el-select v-model="packageForm.table_ids" multiple :placeholder="t('allTables')" style="width: 100%" :disabled="packageForm.area_ids.length > 0">
            <el-option :label="t('allTables')" value="*" />
            <el-option v-for="tb in tables" :key="tb.id" :label="tb.name" :value="tb.id" />
          </el-select>
        </div>
        <div class="form-item">
          <label>{{ t('description') }}</label>
          <el-input v-model="packageForm.description" type="textarea" :rows="2" :placeholder="t('description')" />
        </div>
      </div>
      <template #footer>
        <el-button @click="showPackageDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="savePackage">{{ t('save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Check, Warning, Plus, Ticket, Delete, Clock, Calendar, Present, Star, Switch } from '@element-plus/icons-vue'
import { getTables, updateTable, getSettings, saveSettings, getAreas, getPackages, createPackage, updatePackage, deletePackage as deletePackageApi } from '../api'
import { t } from '../i18n'

const tables = ref([])
const areas = ref([])
const packages = ref([])
const showPackageDlg = ref(false)
const editData = reactive({})

const packageForm = reactive({
  name: '',
  price: 50,
  hours: 2,
  start_time: '',
  end_time: '',
  type: 'normal',
  area_ids: [],
  table_ids: ['*'],
  description: ''
})

const resetPackageForm = () => {
  packageForm.name = ''
  packageForm.price = 50
  packageForm.hours = 2
  packageForm.start_time = ''
  packageForm.end_time = ''
  packageForm.type = 'normal'
  packageForm.area_ids = []
  packageForm.table_ids = ['*']
  packageForm.description = ''
}

const specialRates = reactive({
  weekend: { multiplier: 1.5, days: [6, 0] },
  holiday: { multiplier: 2.0, dates: '01-01,05-01,10-01' },
  memberDay: { enabled: false, discount: 10, dates: '05-20,09-09' },
  autoClose: { enabled: false }
})

const getTableName = (id) => {
  const tb = tables.value.find(t => t.id === id)
  return tb ? tb.name : id
}

const getAreaName = (id) => {
  const a = areas.value.find(x => x.id === id)
  return a ? a.name : id
}

const getPackageType = (type) => {
  const map = { morning: 'success', afternoon: 'warning', evening: 'danger', night: 'info', holiday: 'danger', weekend: 'warning', normal: '' }
  return map[type] || ''
}

const getPackageTypeName = (type) => {
  const map = {
    morning: t('morning'),
    afternoon: t('afternoon'),
    evening: t('evening'),
    night: t('night'),
    holiday: t('holiday'),
    weekend: t('weekend'),
    normal: t('normal')
  }
  return map[type] || type
}

const loadTables = async () => {
  try {
    tables.value = await getTables()
    tables.value.forEach(tbl => {
      editData[tbl.id] = {
        rate_per_hour: tbl.rate_per_hour,
        min_hours: tbl.min_hours || 0,
        status: tbl.status,
        is_private: tbl.is_private || false
      }
    })
  } catch { ElMessage.error(t('loadFailed')) }
}

const loadPackages = async () => {
  try {
    packages.value = await getPackages()
  } catch { packages.value = [] }
}

const loadAreas = async () => {
  try {
    areas.value = await getAreas()
  } catch { areas.value = [] }
}

const saveRate = async (id) => {
  try {
    await updateTable(id, editData[id])
    ElMessage.success(t('saveSuccess'))
    await loadTables()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const saveSpecialRates = async () => {
  try {
    const { autoClose, ...rest } = specialRates
    await saveSettings({ specialRates: rest, autoClose })
    ElMessage.success(t('saveSuccess'))
  } catch(e) { ElMessage.error(t('operationFailed')) }
}

const savePackage = async () => {
  if (!packageForm.name) return ElMessage.warning(t('pleaseComplete'))
  try {
    await createPackage({
      name: packageForm.name,
      price: packageForm.price,
      hours: packageForm.hours,
      start_time: packageForm.start_time,
      end_time: packageForm.end_time,
      type: packageForm.type,
      table_ids: packageForm.table_ids.join(','),
      area_ids: packageForm.area_ids.join(',')
    })
    ElMessage.success(t('saveSuccess'))
    showPackageDlg.value = false
    resetPackageForm()
    await loadPackages()
  } catch(e) { ElMessage.error(t('operationFailed')) }
}

const deletePackageById = async (id) => {
  try {
    await deletePackageApi(id)
    ElMessage.success(t('deleteSuccess'))
    await loadPackages()
  } catch(e) { ElMessage.error(t('operationFailed')) }
}

onMounted(() => { loadTables(); loadAreas(); loadPackages() })
</script>

<style scoped>
.table-rate-page { display: flex; flex-direction: column; gap: 32px; }
.page-header { display: flex; justify-content: space-between; align-items: center; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }

.section { display: flex; flex-direction: column; gap: 16px; }
.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-default);
}

.package-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }
.package-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.package-header { display: flex; align-items: center; gap: 12px; }
.package-icon {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}
.package-icon.morning { background: rgba(255,193,7,0.15); color: #ffc107; }
.package-icon.afternoon { background: rgba(255,152,0,0.15); color: #ff9800; }
.package-icon.evening { background: rgba(244,67,54,0.15); color: #f44336; }
.package-icon.night { background: rgba(103,58,183,0.15); color: #673ab7; }
.package-icon.holiday { background: rgba(244,67,54,0.15); color: #f44336; }
.package-icon.weekend { background: rgba(33,150,243,0.15); color: #2196f3; }
.package-icon.normal { background: rgba(63,185,80,0.15); color: var(--accent-success); }

.package-info { flex: 1; display: flex; flex-direction: column; gap: 4px; }
.package-name { font-size: 15px; font-weight: 600; color: var(--text-primary); }

.package-body { display: flex; flex-direction: column; gap: 8px; }
.package-price { display: flex; align-items: baseline; gap: 4px; }
.price-value { font-size: 28px; font-weight: 700; color: var(--accent-success); }
.price-unit { font-size: 13px; color: var(--text-tertiary); }
.package-time { display: flex; align-items: center; gap: 6px; font-size: 13px; color: var(--text-secondary); }
.package-desc { font-size: 12px; color: var(--text-tertiary); }

.package-tables { display: flex; flex-wrap: wrap; align-items: center; gap: 6px; font-size: 12px; }
.tables-label { color: var(--text-tertiary); }
.table-tag { margin: 0; }
.all-tables { color: var(--accent-primary); }

.empty-packages {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 40px;
  background: var(--bg-secondary);
  border: 1px dashed var(--border-default);
  border-radius: var(--radius-lg);
  color: var(--text-tertiary);
}

.special-rates { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }
.rate-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  overflow: hidden;
}
.rate-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 16px;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-weight: 600;
  font-size: 14px;
}
.rate-body { padding: 16px; display: flex; flex-direction: column; gap: 12px; }
.rate-item { display: flex; flex-direction: column; gap: 6px; }
.rate-item label { font-size: 12px; color: var(--text-tertiary); }

.save-special { align-self: flex-start; margin-top: 8px; }

.rate-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }
.base-rate {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  transition: all 0.2s;
}
.base-rate:hover { border-color: var(--accent-primary); }
.base-rate.private { border-color: rgba(255,215,0,0.3); }
.card-header { margin-bottom: 16px; }
.table-info { display: flex; justify-content: space-between; align-items: center; }
.table-name { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.base-rate .rate-body { display: flex; flex-direction: column; gap: 12px; }
.base-rate .rate-item { display: flex; justify-content: space-between; align-items: center; }
.base-rate .rate-label { font-size: 13px; color: var(--text-secondary); }
.status-select { width: 120px; }
.save-btn { width: 100%; margin-top: 12px; }

.empty-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 60px;
  text-align: center;
  color: var(--text-tertiary);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.package-form { display: flex; flex-direction: column; gap: 16px; }
.form-item { display: flex; flex-direction: column; gap: 6px; }
.form-item label { font-size: 13px; color: var(--text-secondary); }
.form-row { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }

:deep(.package-dialog .el-dialog) {
  background: var(--bg-secondary) !important;
}
:deep(.package-dialog .el-dialog__header) {
  border-bottom: 1px solid var(--border-muted);
}
:deep(.package-dialog .el-dialog__title) {
  color: var(--text-primary) !important;
}
:deep(.package-dialog .el-dialog__body) {
  padding: 20px !important;
}
:deep(.package-dialog .el-input__wrapper),
:deep(.package-dialog .el-select__wrapper) {
  background: var(--bg-primary) !important;
  box-shadow: none !important;
  border: 1px solid var(--border-default) !important;
}
</style>
