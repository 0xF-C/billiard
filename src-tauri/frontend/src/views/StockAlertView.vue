<template>
  <div class="stock-alert-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('stockAlert') }}</h1>
      <div class="header-actions">
        <el-button :icon="Setting" @click="showConfig = true">{{ t('alertConfig') }}</el-button>
        <el-button :icon="Refresh" @click="loadAlerts">{{ t('refresh') }}</el-button>
      </div>
    </div>

    <div class="alert-summary">
      <div class="alert-card critical" v-if="criticalCount > 0">
        <el-icon :size="32"><Warning /></el-icon>
        <div class="alert-info">
          <span class="alert-count">{{ criticalCount }}</span>
          <span class="alert-label">{{ t('criticalAlerts') }}</span>
        </div>
      </div>
      <div class="alert-card warning">
        <el-icon :size="32"><Bell /></el-icon>
        <div class="alert-info">
          <span class="alert-count">{{ warningCount }}</span>
          <span class="alert-label">{{ t('lowStockWarning') }}</span>
        </div>
      </div>
      <div class="alert-card expired" v-if="expiredCount > 0">
        <el-icon :size="32"><Clock /></el-icon>
        <div class="alert-info">
          <span class="alert-count">{{ expiredCount }}</span>
          <span class="alert-label">{{ t('expiringSoon') }}</span>
        </div>
      </div>
    </div>

    <div class="alerts-section">
      <div class="section-header">
        <h3>{{ t('lowStockItems') }}</h3>
        <el-radio-group v-model="alertFilter" size="small">
          <el-radio-button value="all">{{ t('all') }}</el-radio-button>
          <el-radio-button value="critical">{{ t('critical') }}</el-radio-button>
          <el-radio-button value="warning">{{ t('warning') }}</el-radio-button>
        </el-radio-group>
      </div>

      <div class="alert-list">
        <div v-for="item in filteredAlerts" :key="item.id" class="alert-item" :class="item.level">
          <div class="item-icon">
            <el-icon :size="24"><Box /></el-icon>
          </div>
          <div class="item-info">
            <div class="item-name">{{ item.name }}</div>
            <div class="item-sku">SKU: {{ item.sku }}</div>
          </div>
          <div class="item-stock">
            <div class="stock-current">{{ item.stock }} {{ item.unit }}</div>
            <div class="stock-threshold">{{ t('threshold') }}: {{ item.threshold }} {{ item.unit }}</div>
          </div>
          <div class="item-progress">
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: stockPercent(item) + '%' }"></div>
            </div>
            <span class="progress-label">{{ stockPercent(item).toFixed(0) }}%</span>
          </div>
          <div class="item-actions">
            <el-button type="primary" size="small" @click="quickStockIn(item)">
              {{ t('stockIn') }}
            </el-button>
          </div>
        </div>
        <el-empty v-if="filteredAlerts.length === 0" :description="t('noAlerts')" />
      </div>
    </div>

    <div class="expiry-section">
      <div class="section-header">
        <h3>{{ t('expiryWarning') }}</h3>
      </div>
      <el-table :data="expiringItems" stripe>
        <el-table-column :label="t('product')" min-width="140">
          <template #default="{ row }">
            <div class="product-cell">
              <span class="product-name">{{ row.name }}</span>
              <span class="product-sku">{{ row.sku }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('batchNo')" width="120">
          <template #default="{ row }">
            {{ row.batchNo }}
          </template>
        </el-table-column>
        <el-table-column :label="t('quantity')" width="80">
          <template #default="{ row }">
            {{ row.quantity }} {{ row.unit }}
          </template>
        </el-table-column>
        <el-table-column :label="t('expiryDate')" width="120">
          <template #default="{ row }">
            <span :class="{ 'text-danger': isExpiringSoon(row.expiryDate) }">
              {{ formatDate(row.expiryDate) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column :label="t('daysLeft')" width="80">
          <template #default="{ row }">
            <el-tag :type="getDaysTagType(row.expiryDate)" size="small">
              {{ getDaysLeft(row.expiryDate) }} {{ t('days') }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="100">
          <template #default="{ row }">
            <el-button link type="warning" @click="markExpired(row)">{{ t('markExpired') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- {{ t('AlertConfigDialog') }} -->
    <el-dialog v-model="showConfig" :title="t('alertConfig')" width="500px">
      <el-form label-width="120px">
        <el-form-item :label="t('lowStockThreshold')">
          <el-input-number :model-value="10" :min="1" />
        </el-form-item>
        <el-form-item :label="t('criticalThreshold')">
          <el-input-number :model-value="5" :min="1" />
        </el-form-item>
        <el-form-item :label="t('expiryWarningDays')">
          <el-input-number :model-value="30" :min="7" />
        </el-form-item>
        <el-form-item :label="t('enableNotification')">
          <el-switch :model-value="true" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showConfig = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="showConfig = false">{{ t('save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Setting, Refresh, Warning, Bell, Clock, Box } from '@element-plus/icons-vue'
import { getInventory } from '../api'
import { t } from '../i18n'


const alertFilter = ref('all')
const showConfig = ref(false)

const alerts = ref([
  { id: 1, name: '可口可乐', sku: 'COLA-330', stock: 3, threshold: 20, unit: '瓶', level: 'critical' },
  { id: 2, name: '矿泉水', sku: 'WATER-500', stock: 8, threshold: 30, unit: '瓶', level: 'critical' },
  { id: 3, name: '巧粉', sku: 'CHALK-01', stock: 15, threshold: 50, unit: '个', level: 'warning' },
  { id: 4, name: '手套', sku: 'GLOVE-M', stock: 20, threshold: 30, unit: '副', level: 'warning' },
])

const expiringItems = ref([
  { id: 1, name: '红牛', sku: 'RB-250', batchNo: 'BN20260101', quantity: 24, unit: '罐', expiryDate: new Date(Date.now() + 86400000 * 15) },
  { id: 2, name: '奶茶', sku: 'MILK-500', batchNo: 'BN20260315', quantity: 10, unit: '杯', expiryDate: new Date(Date.now() + 86400000 * 3) },
])

const criticalCount = computed(() => alerts.value.filter(a => a.level === 'critical').length)
const warningCount = computed(() => alerts.value.filter(a => a.level === 'warning').length)
const expiredCount = computed(() => expiringItems.value.filter(e => isExpiringSoon(e.expiryDate, 7)).length)

const filteredAlerts = computed(() => {
  if (alertFilter.value === 'all') return alerts.value
  return alerts.value.filter(a => a.level === alertFilter.value)
})

const stockPercent = (item) => {
  if (!item.threshold) return 100
  return (item.stock / item.threshold) * 100
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN')
}

const isExpiringSoon = (d, days = 30) => {
  if (!d) return false
  return (new Date(d) - new Date()) / 86400000 < days
}

const getDaysLeft = (d) => {
  if (!d) return 0
  return Math.max(0, Math.ceil((new Date(d) - new Date()) / 86400000))
}

const getDaysTagType = (d) => {
  const days = getDaysLeft(d)
  if (days <= 7) return 'danger'
  if (days <= 14) return 'warning'
  return 'info'
}

const loadAlerts = async () => {
  await getInventory()
  ElMessage.success(t('refreshSuccess'))
}

const quickStockIn = (item) => {
  ElMessage.success(`${t('stockIn')}: ${item.name}`)
}

const markExpired = (item) => {
  ElMessage.warning(`${t('markExpired')}: ${item.name}`)
}

onMounted(() => {
  loadAlerts()
})
</script>

<style scoped>
.stock-alert-page {
  padding: 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.alert-summary {
  display: flex;
  gap: 16px;
  margin-bottom: 24px;
}

.alert-card {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  border-radius: 12px;
  background: var(--card-bg);
  border: 1px solid var(--border-color);
}

.alert-card.critical { border-left: 4px solid var(--accent-danger); }
.alert-card.critical .el-icon { color: var(--accent-danger); }
.alert-card.warning { border-left: 4px solid var(--accent-warning); }
.alert-card.warning .el-icon { color: var(--accent-warning); }
.alert-card.expired { border-left: 4px solid var(--accent-primary); }
.alert-card.expired .el-icon { color: var(--accent-primary); }

.alert-count {
  display: block;
  font-size: 28px;
  font-weight: 600;
}

.alert-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.alerts-section, .expiry-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.alert-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.alert-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: var(--hover-bg);
  border-radius: 8px;
  border-left: 3px solid var(--accent-warning);
}

.alert-item.critical {
  border-left-color: var(--accent-danger);
}

.item-icon {
  color: var(--text-secondary);
}

.item-info {
  flex: 1;
}

.item-name {
  font-weight: 500;
  margin-bottom: 4px;
}

.item-sku {
  font-size: 12px;
  color: var(--text-secondary);
}

.item-stock {
  text-align: right;
  margin-right: 16px;
}

.stock-current {
  font-size: 18px;
  font-weight: 600;
}

.stock-threshold {
  font-size: 12px;
  color: var(--text-secondary);
}

.item-progress {
  width: 120px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.progress-bar {
  flex: 1;
  height: 8px;
  background: var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--accent-danger);
  border-radius: 4px;
}

.progress-label {
  font-size: 12px;
  color: var(--text-secondary);
  width: 32px;
}

.product-cell {
  display: flex;
  flex-direction: column;
}

.product-name {
  font-weight: 500;
}

.product-sku {
  font-size: 12px;
  color: var(--text-secondary);
}

.text-danger {
  color: var(--accent-danger);
}
</style>
