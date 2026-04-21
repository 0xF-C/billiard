<template>
  <div class="export-reports-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('exportReports') }}</h1>
    </div>

    <div class="report-types">
      <h3>{{ t('selectReportType') }}</h3>
      <div class="type-grid">
        <div
          v-for="type in reportTypes"
          :key="type.id"
          class="type-card"
          :class="{ active: selectedTypes.includes(type.id) }"
          @click="toggleType(type.id)"
        >
          <div class="type-icon" :style="{ background: type.color }">
            <el-icon :size="24"><component :is="type.icon" /></el-icon>
          </div>
          <div class="type-info">
            <span class="type-name">{{ t(type.name) }}</span>
            <span class="type-desc">{{ t(type.desc) }}</span>
          </div>
          <el-checkbox :model-value="selectedTypes.includes(type.id)" />
        </div>
      </div>
    </div>

    <div class="export-config">
      <el-form label-width="100px">
        <el-form-item :label="t('dateRange')">
          <el-date-picker
            v-model="dateRange"
            type="daterange"
            range-separator="~"
            :start-placeholder="t('startDate')"
            :end-placeholder="t('endDate')"
            value-format="YYYY-MM-DD"
          />
        </el-form-item>
        <el-form-item :label="t('format')">
          <el-radio-group v-model="exportFormat">
            <el-radio value="xlsx">
              <el-icon><Document /></el-icon> Excel
            </el-radio>
            <el-radio value="csv">
              <el-icon><Document /></el-icon> CSV
            </el-radio>
            <el-radio value="pdf">
              <el-icon><Document /></el-icon> PDF
            </el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item :label="t('includeCharts')">
          <el-switch v-model="includeCharts" />
        </el-form-item>
      </el-form>
    </div>

    <div class="quick-exports">
      <h3>{{ t('quickExport') }}</h3>
      <div class="quick-grid">
        <div class="quick-card" @click="quickExport('daily')">
          <el-icon :size="32"><Calendar /></el-icon>
          <span class="quick-name">{{ t('dailyReport') }}</span>
          <span class="quick-desc">{{ t('todayData') }}</span>
        </div>
        <div class="quick-card" @click="quickExport('weekly')">
          <el-icon :size="32"><Calendar /></el-icon>
          <span class="quick-name">{{ t('weeklyReport') }}</span>
          <span class="quick-desc">{{ t('thisWeekData') }}</span>
        </div>
        <div class="quick-card" @click="quickExport('monthly')">
          <el-icon :size="32"><Calendar /></el-icon>
          <span class="quick-name">{{ t('monthlyReport') }}</span>
          <span class="quick-desc">{{ t('thisMonthData') }}</span>
        </div>
        <div class="quick-card" @click="quickExport('custom')">
          <el-icon :size="32"><Setting /></el-icon>
          <span class="quick-name">{{ t('customReport') }}</span>
          <span class="quick-desc">{{ t('selectRange') }}</span>
        </div>
      </div>
    </div>

    <div class="history-section">
      <div class="section-header">
        <h3>{{ t('exportHistory') }}</h3>
      </div>
      <el-table :data="exportHistory" stripe>
        <el-table-column :label="t('exportTime')" width="180">
          <template #default="{ row }">
            {{ formatTime(row.time) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('reportType')">
          <template #default="{ row }">
            <el-tag v-for="t in row.types" :key="t" size="small" class="type-tag">{{ t }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('dateRange')">
          <template #default="{ row }">
            {{ row.startDate }} ~ {{ row.endDate }}
          </template>
        </el-table-column>
        <el-table-column :label="t('format')" width="80">
          <template #default="{ row }">
            {{ row.format.toUpperCase() }}
          </template>
        </el-table-column>
        <el-table-column :label="t('operator')" width="100">
          <template #default="{ row }">
            {{ row.operator }}
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="120" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="downloadFile(row)">
              <el-icon><Download /></el-icon> {{ t('download') }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <div class="export-actions">
      <el-button size="large" @click="resetForm">{{ t('reset') }}</el-button>
      <el-button type="primary" size="large" :loading="exporting" @click="doExport">
        <el-icon><Download /></el-icon>
        {{ t('export') }}
      </el-button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import {
  Document, Calendar, Setting, Download,
  Money, User, ShoppingCart, Timer, TrendCharts
} from '@element-plus/icons-vue'
import { t } from '../i18n'


const selectedTypes = ref(['revenue'])
const dateRange = ref([])
const exportFormat = ref('xlsx')
const includeCharts = ref(true)
const exporting = ref(false)

const reportTypes = ref([
  { id: 'revenue', name: 'revenueReport', desc: 'revenueReportDesc', icon: Money, color: '#3b82f6' },
  { id: 'orders', name: 'orderReport', desc: 'orderReportDesc', icon: ShoppingCart, color: '#10b981' },
  { id: 'members', name: 'memberReport', desc: 'memberReportDesc', icon: User, color: '#8b5cf6' },
  { id: 'tables', name: 'tableReport', desc: 'tableReportDesc', icon: Timer, color: '#f59e0b' },
  { id: 'inventory', name: 'inventoryReport', desc: 'inventoryReportDesc', icon: Document, color: '#ef4444' },
  { id: 'analysis', name: 'analysisReport', desc: 'analysisReportDesc', icon: TrendCharts, color: '#6366f1' },
])

const exportHistory = ref([
  { time: new Date(Date.now() - 3600000), types: [t('revenueReport'), t('orderReport')], startDate: '2026-04-01', endDate: '2026-04-19', format: 'xlsx', operator: t('admin') },
  { time: new Date(Date.now() - 86400000), types: [t('memberReport')], startDate: '2026-03-01', endDate: '2026-03-31', format: 'pdf', operator: t('manager') },
  { time: new Date(Date.now() - 86400000 * 3), types: [t('tableReport'), t('analysisReport')], startDate: '2026-04-01', endDate: '2026-04-15', format: 'xlsx', operator: t('admin') },
])

const toggleType = (id) => {
  const idx = selectedTypes.value.indexOf(id)
  if (idx === -1) {
    selectedTypes.value.push(id)
  } else {
    selectedTypes.value.splice(idx, 1)
  }
}

const formatTime = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleString(currentLang.value === 'zh' ? 'zh-CN' : (currentLang.value === 'en' ? 'en-US' : 'zh-CN'))
}

const quickExport = (type) => {
  const today = new Date()
  const start = new Date()

  if (type === 'daily') {
    dateRange.value = [today.toISOString().split('T')[0], today.toISOString().split('T')[0]]
  } else if (type === 'weekly') {
    start.setDate(today.getDate() - today.getDay() + 1)
    dateRange.value = [start.toISOString().split('T')[0], today.toISOString().split('T')[0]]
  } else if (type === 'monthly') {
    start.setDate(1)
    dateRange.value = [start.toISOString().split('T')[0], today.toISOString().split('T')[0]]
  }

  selectedTypes.value = ['revenue', 'orders', 'members']
  ElMessage.success(t('configReady'))
}

const resetForm = () => {
  selectedTypes.value = ['revenue']
  dateRange.value = []
  exportFormat.value = 'xlsx'
  includeCharts.value = true
}

const doExport = async () => {
  if (selectedTypes.value.length === 0) {
    ElMessage.warning(t('selectReportType'))
    return
  }
  if (!dateRange.value || dateRange.value.length !== 2) {
    ElMessage.warning(t('selectDateRange'))
    return
  }

  exporting.value = true
  await new Promise(r => setTimeout(r, 1500))
  exporting.value = false

  exportHistory.value.unshift({
    time: new Date(),
    types: selectedTypes.value.map(id => {
      const type = reportTypes.value.find(t => t.id === id)
      return type ? t(type.name) : id
    }),
    startDate: dateRange.value[0],
    endDate: dateRange.value[1],
    format: exportFormat.value,
    operator: t('currentUser'),
  })

  ElMessage.success(t('exportSuccess'))
}

const downloadFile = (row) => {
  ElMessage.success(t('downloadStarted'))
}

onMounted(() => {
  // init
})
</script>

<style scoped>
.export-reports-page {
  padding: 24px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0 0 24px 0;
}

h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
}

.type-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.type-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 16px;
  border: 2px solid var(--border-color);
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.type-card:hover {
  border-color: var(--primary-color);
}

.type-card.active {
  border-color: var(--primary-color);
  background: var(--primary-light);
}

.type-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
}

.type-info {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.type-name {
  font-weight: 600;
  margin-bottom: 4px;
}

.type-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

.export-config {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  margin-bottom: 24px;
}

.quick-exports {
  margin-bottom: 24px;
}

.quick-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}

.quick-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  text-align: center;
  cursor: pointer;
  transition: all 0.2s;
}

.quick-card:hover {
  border-color: var(--primary-color);
  transform: translateY(-2px);
}

.quick-card .el-icon {
  color: var(--primary-color);
  margin-bottom: 8px;
}

.quick-name {
  display: block;
  font-weight: 600;
  margin-bottom: 4px;
}

.quick-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

.history-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  margin-bottom: 24px;
}

.section-header {
  margin-bottom: 16px;
}

.type-tag {
  margin-right: 4px;
}

.export-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>
