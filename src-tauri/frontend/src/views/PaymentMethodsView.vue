<template>
  <div class="payment-methods-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('paymentMethods') }}</h1>
      <el-button type="primary" :icon="Setting" @click="showConfig = true">
        {{ t('configPayment') }}
      </el-button>
    </div>

    <div class="payment-cards">
      <div v-for="method in paymentMethods" :key="method.id" class="payment-card" :class="{ disabled: !method.enabled }">
        <div class="card-header">
          <div class="method-icon" :style="{ background: method.color }">
            <el-icon :size="24"><component :is="method.icon" /></el-icon>
          </div>
          <div class="method-info">
            <span class="method-name">{{ t(method.name) }}</span>
            <el-switch v-model="method.enabled" @change="toggleMethod(method)" />
          </div>
        </div>
        <div class="card-stats">
          <div class="stat">
            <span class="stat-value">¥{{ method.todayAmount.toLocaleString() }}</span>
            <span class="stat-label">{{ t('today') }}</span>
          </div>
          <div class="stat">
            <span class="stat-value">{{ method.todayCount }}</span>
            <span class="stat-label">{{ t('orders') }}</span>
          </div>
          <div class="stat">
            <span class="stat-value">{{ method.percent.toFixed(1) }}%</span>
            <span class="stat-label">{{ t('percent') }}</span>
          </div>
        </div>
        <div class="card-footer">
          <span class="fee-rate">{{ t('feeRate') }}: {{ method.feeRate }}%</span>
          <span class="fee-amount">{{ t('todayFee') }}: ¥{{ method.todayFee.toFixed(2) }}</span>
        </div>
      </div>
    </div>

    <div class="stats-section">
      <div class="section-header">
        <h3>{{ t('paymentStats') }}</h3>
        <el-radio-group v-model="statsPeriod" size="small">
          <el-radio-button value="today">{{ t('today') }}</el-radio-button>
          <el-radio-button value="week">{{ t('thisWeek') }}</el-radio-button>
          <el-radio-button value="month">{{ t('thisMonth') }}</el-radio-button>
        </el-radio-group>
      </div>

      <div class="summary-row">
        <div class="summary-item">
          <span class="summary-label">{{ t('totalPayment') }}</span>
          <span class="summary-value">¥{{ summary.total.toLocaleString() }}</span>
        </div>
        <div class="summary-item">
          <span class="summary-label">{{ t('totalOrders') }}</span>
          <span class="summary-value">{{ summary.count }}</span>
        </div>
        <div class="summary-item">
          <span class="summary-label">{{ t('totalFee') }}</span>
          <span class="summary-value fee">¥{{ summary.totalFee.toFixed(2) }}</span>
        </div>
      </div>

      <div class="pie-chart">
        <div class="pie-legend">
          <div v-for="item in pieData" :key="item.name" class="legend-item">
            <span class="legend-color" :style="{ background: item.color }"></span>
            <span class="legend-name">{{ t(item.name) }}</span>
            <span class="legend-value">¥{{ item.value.toLocaleString() }}</span>
          </div>
        </div>
        <div class="pie-visual">
          <div class="pie" :style="{ background: pieGradient }"></div>
          <div class="pie-center">
            <span class="pie-total">{{ summary.count }}</span>
            <span class="pie-label">{{ t('orders') }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="records-section">
      <div class="section-header">
        <h3>{{ t('recentPayments') }}</h3>
      </div>
      <el-table :data="recentPayments" stripe>
        <el-table-column :label="t('time')" width="160">
          <template #default="{ row }">
            {{ formatTime(row.time) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('orderNo')" width="100">
          <template #default="{ row }">
            #{{ row.orderId }}
          </template>
        </el-table-column>
        <el-table-column :label="t('method')" width="100">
          <template #default="{ row }">
            <el-tag size="small" :color="getMethodColor(row.method)" style="border: none;">
              {{ t(row.method) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('amount')">
          <template #default="{ row }">
            ¥{{ row.amount.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('fee')" width="100">
          <template #default="{ row }">
            <span class="fee-text">¥{{ row.fee.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('status')" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 'success' ? 'success' : 'danger'" size="small">
              {{ t(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- {{ t('ConfigDialog') }} -->
    <el-dialog v-model="showConfig" :title="t('configPayment')" width="500px" top="5vh" append-to-body>
      <el-form label-width="120px">
        <el-divider content-position="left">{{ t('wechat') }}</el-divider>
        <el-form-item :label="t('feeRate')">
          <el-input-number :model-value="0.6" :min="0" :max="5" :step="0.1" :precision="2" />
          <span class="form-tip">%</span>
        </el-form-item>
        <el-divider content-position="left">{{ t('alipay') }}</el-divider>
        <el-form-item :label="t('feeRate')">
          <el-input-number :model-value="0.6" :min="0" :max="5" :step="0.1" :precision="2" />
          <span class="form-tip">%</span>
        </el-form-item>
        <el-divider content-position="left">{{ t('card') }}</el-divider>
        <el-form-item :label="t('feeRate')">
          <el-input-number :model-value="0" :min="0" :max="5" :step="0.1" :precision="2" />
          <span class="form-tip">%</span>
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
import { Setting, Wallet, CreditCard, Money, Coin } from '@element-plus/icons-vue'
import { t } from '../i18n'


const statsPeriod = ref('today')
const showConfig = ref(false)

const paymentMethods = ref([
  { id: 'wechat', name: 'wechat', icon: Wallet, color: '#07c160', enabled: true, todayAmount: 3280, todayCount: 18, percent: 45.2, feeRate: 0.6, todayFee: 19.68 },
  { id: 'alipay', name: 'alipay', icon: Money, color: '#1677ff', enabled: true, todayAmount: 2460, todayCount: 12, percent: 33.9, feeRate: 0.6, todayFee: 14.76 },
  { id: 'cash', name: 'cash', icon: Coin, color: '#f59e0b', enabled: true, todayAmount: 1200, todayCount: 8, percent: 16.5, feeRate: 0, todayFee: 0 },
  { id: 'card', name: 'card', icon: CreditCard, color: '#8b5cf6', enabled: true, todayAmount: 320, todayCount: 3, percent: 4.4, feeRate: 0, todayFee: 0 },
])

const summary = computed(() => ({
  total: paymentMethods.value.reduce((sum, m) => sum + m.todayAmount, 0),
  count: paymentMethods.value.reduce((sum, m) => sum + m.todayCount, 0),
  totalFee: paymentMethods.value.reduce((sum, m) => sum + m.todayFee, 0),
}))

const pieData = computed(() => paymentMethods.value.filter(m => m.enabled).map(m => ({
  name: m.name,
  value: m.todayAmount,
  color: m.color,
})))

const pieGradient = computed(() => {
  const total = pieData.value.reduce((sum, d) => sum + d.value, 0)
  let cumulative = 0
  const stops = pieData.value.map(d => {
    const start = (cumulative / total) * 100
    cumulative += d.value
    const end = (cumulative / total) * 100
    return `${d.color} ${start}% ${end}%`
  })
  return `conic-gradient(${stops.join(', ')})`
})

const recentPayments = ref([
  { time: new Date(), orderId: 101, method: 'wechat', amount: 180, fee: 1.08, status: 'success' },
  { time: new Date(Date.now() - 300000), orderId: 102, method: 'alipay', amount: 240, fee: 1.44, status: 'success' },
  { time: new Date(Date.now() - 600000), orderId: 103, method: 'cash', amount: 150, fee: 0, status: 'success' },
  { time: new Date(Date.now() - 900000), orderId: 104, method: 'wechat', amount: 320, fee: 1.92, status: 'success' },
  { time: new Date(Date.now() - 1200000), orderId: 105, method: 'card', amount: 500, fee: 0, status: 'success' },
])

const formatTime = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleTimeString('zh-CN')
}

const getMethodColor = (method) => {
  const m = paymentMethods.value.find(p => p.id === method)
  return m?.color || '#909399'
}

const toggleMethod = (method) => {
  ElMessage.success(`${t(method.name)} ${method.enabled ? t('enabled') : t('disabled')}`)
}

onMounted(() => {
  // load data
})
</script>

<style scoped>
.payment-methods-page {
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

.payment-cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.payment-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.payment-card.disabled {
  opacity: 0.5;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.method-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
}

.method-info {
  flex: 1;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.method-name {
  font-weight: 600;
}

.card-stats {
  display: flex;
  justify-content: space-between;
  margin-bottom: 16px;
}

.stat {
  text-align: center;
}

.stat-value {
  display: block;
  font-weight: 600;
  font-size: 18px;
}

.stat-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.card-footer {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-secondary);
  padding-top: 12px;
  border-top: 1px solid var(--border-color);
}

.stats-section {
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

.summary-row {
  display: flex;
  gap: 32px;
  margin-bottom: 24px;
  padding: 16px;
  background: var(--hover-bg);
  border-radius: 8px;
}

.summary-item {
  display: flex;
  flex-direction: column;
}

.summary-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.summary-value {
  font-size: 24px;
  font-weight: 600;
}

.summary-value.fee {
  color: var(--accent-danger);
}

.pie-chart {
  display: flex;
  gap: 32px;
  align-items: center;
}

.pie-legend {
  flex: 1;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 0;
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 3px;
}

.legend-name {
  flex: 1;
}

.legend-value {
  font-weight: 500;
}

.pie-visual {
  position: relative;
}

.pie {
  width: 120px;
  height: 120px;
  border-radius: 50%;
}

.pie-center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}

.pie-total {
  display: block;
  font-size: 24px;
  font-weight: 600;
}

.pie-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.records-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.fee-text {
  color: var(--accent-danger);
}

.form-tip {
  margin-left: 8px;
  color: var(--text-secondary);
}
</style>
