<template>
  <div class="daily-report-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('dailyReport') }}</h1>
      <div class="header-actions">
        <el-date-picker
          v-model="selectedDate"
          type="date"
          :placeholder="t('selectDate')"
          value-format="YYYY-MM-DD"
          @change="loadReport"
        />
        <el-button :icon="Download" @click="exportReport">{{ t('export') }}</el-button>
      </div>
    </div>

    <div class="report-date">
      <el-icon><Calendar /></el-icon>
      {{ formatDisplayDate(selectedDate) }}
    </div>

    <!-- {{ t('CoreMetrics') }} -->
    <div class="metrics-grid">
      <div class="metric-card highlight">
        <div class="metric-label">{{ t('totalRevenue') }}</div>
        <div class="metric-value">¥{{ report.totalRevenue.toFixed(2) }}</div>
        <div class="metric-compare">
          <span :class="report.revenueChange >= 0 ? 'up' : 'down'">
            {{ report.revenueChange >= 0 ? '↑' : '↓' }} {{ Math.abs(report.revenueChange).toFixed(1) }}%
          </span>
          {{ t('vsYesterday') }}
        </div>
      </div>
      <div class="metric-card">
        <div class="metric-label">{{ t('tableRevenue') }}</div>
        <div class="metric-value">¥{{ report.tableRevenue.toFixed(2) }}</div>
        <div class="metric-percent">{{ percent(report.tableRevenue, report.totalRevenue) }}%</div>
      </div>
      <div class="metric-card">
        <div class="metric-label">{{ t('productRevenue') }}</div>
        <div class="metric-value">¥{{ report.productRevenue.toFixed(2) }}</div>
        <div class="metric-percent">{{ percent(report.productRevenue, report.totalRevenue) }}%</div>
      </div>
      <div class="metric-card">
        <div class="metric-label">{{ t('rechargeAmount') }}</div>
        <div class="metric-value">¥{{ report.rechargeAmount.toFixed(2) }}</div>
        <div class="metric-count">{{ report.rechargeCount }}{{ t('times') }}</div>
      </div>
      <div class="metric-card">
        <div class="metric-label">{{ t('orderCount') }}</div>
        <div class="metric-value">{{ report.orderCount }}</div>
        <div class="metric-compare">
          <span :class="report.orderChange >= 0 ? 'up' : 'down'">
            {{ report.orderChange >= 0 ? '↑' : '↓' }} {{ Math.abs(report.orderChange).toFixed(1) }}%
          </span>
        </div>
      </div>
      <div class="metric-card">
        <div class="metric-label">{{ t('avgOrderValue') }}</div>
        <div class="metric-value">¥{{ report.avgOrderValue.toFixed(0) }}</div>
      </div>
      <div class="metric-card">
        <div class="metric-label">{{ t('tableUsageRate') }}</div>
        <div class="metric-value">{{ report.tableUsageRate.toFixed(1) }}%</div>
      </div>
      <div class="metric-card">
        <div class="metric-label">{{ t('memberConsumption') }}</div>
        <div class="metric-value">¥{{ report.memberRevenue.toFixed(2) }}</div>
        <div class="metric-percent">{{ percent(report.memberRevenue, report.totalRevenue) }}%</div>
      </div>
    </div>

    <!-- {{ t('DetailedData') }} -->
    <div class="detail-grid">
      <!-- {{ t('PaymentMethods') }} -->
      <div class="detail-card">
        <div class="card-header">
          <h3>{{ t('paymentMethods') }}</h3>
        </div>
        <div class="payment-list">
          <div v-for="item in report.paymentBreakdown" :key="item.method" class="payment-item">
            <div class="payment-info">
              <span class="payment-name">{{ t(item.method) }}</span>
              <span class="payment-count">{{ item.count }}{{ t('orders') }}</span>
            </div>
            <div class="payment-amount">¥{{ item.amount.toFixed(2) }}</div>
          </div>
        </div>
      </div>

      <!-- {{ t('TimeDistribution') }} -->
      <div class="detail-card">
        <div class="card-header">
          <h3>{{ t('hourlyDistribution') }}</h3>
        </div>
        <div class="hourly-chart">
          <div v-for="(item, idx) in report.hourlyData" :key="idx" class="hour-bar">
            <div class="bar-wrapper">
              <div class="bar" :style="{ height: barHeight(item.revenue) + '%' }"></div>
            </div>
            <span class="hour-label">{{ item.hour }}</span>
            <span class="hour-value">¥{{ item.revenue }}</span>
          </div>
        </div>
      </div>

      <!-- {{ t('TableUsage') }} -->
      <div class="detail-card">
        <div class="card-header">
          <h3>{{ t('tableUsage') }}</h3>
        </div>
        <el-table :data="report.tableUsage" size="small" max-height="200">
          <el-table-column prop="name" :label="t('table')" width="80" />
          <el-table-column :label="t('usageTime')" width="100">
            <template #default="{ row }">
              {{ row.minutes }}{{ t('minutes') }}
            </template>
          </el-table-column>
          <el-table-column :label="t('revenue')">
            <template #default="{ row }">
              ¥{{ row.revenue.toFixed(2) }}
            </template>
          </el-table-column>
        </el-table>
      </div>

      <!-- {{ t('ExpenseDetails') }} -->
      <div class="detail-card">
        <div class="card-header">
          <h3>{{ t('expenses') }}</h3>
        </div>
        <div class="expense-summary">
          <div class="expense-total">
            <span>{{ t('totalExpense') }}</span>
            <span class="expense-amount">¥{{ report.totalExpense.toFixed(2) }}</span>
          </div>
        </div>
        <el-table :data="report.expenses" size="small" max-height="160">
          <el-table-column prop="category" :label="t('category')" width="100" />
          <el-table-column :label="t('amount')">
            <template #default="{ row }">
              ¥{{ row.amount.toFixed(2) }}
            </template>
          </el-table-column>
          <el-table-column prop="remark" :label="t('remark')" />
        </el-table>
      </div>
    </div>

    <!-- {{ t('OrderDetails') }} -->
    <div class="orders-section">
      <div class="section-header">
        <h3>{{ t('orderDetails') }}</h3>
        <el-radio-group v-model="orderFilter" size="small">
          <el-radio-button value="all">{{ t('all') }}</el-radio-button>
          <el-radio-button value="table">{{ t('tableOrders') }}</el-radio-button>
          <el-radio-button value="product">{{ t('productOrders') }}</el-radio-button>
        </el-radio-group>
      </div>
      <el-table :data="filteredOrders" stripe max-height="400">
        <el-table-column prop="id" label="ID" width="60" />
        <el-table-column :label="t('type')" width="80">
          <template #default="{ row }">
            <el-tag size="small" :type="row.type === 'table' ? 'primary' : 'success'">
              {{ row.type === 'table' ? t('table') : t('product') }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('table')" width="80">
          <template #default="{ row }">
            {{ row.tableName || '-' }}
          </template>
        </el-table-column>
        <el-table-column :label="t('member')" width="100">
          <template #default="{ row }">
            {{ row.memberName || '-' }}
          </template>
        </el-table-column>
        <el-table-column :label="t('duration')" width="100">
          <template #default="{ row }">
            {{ row.minutes ? `${row.minutes}${t('minutes')}` : '-' }}
          </template>
        </el-table-column>
        <el-table-column :label="t('amount')">
          <template #default="{ row }">
            ¥{{ row.amount.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('paymentMethod')" width="80">
          <template #default="{ row }">
            {{ t(row.paymentMethod) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('time')" width="100">
          <template #default="{ row }">
            {{ formatTime(row.closedAt) }}
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Calendar, Download } from '@element-plus/icons-vue'
import { getOrders, getDashboard } from '../api'
import { t } from '../i18n'


const selectedDate = ref(new Date().toISOString().split('T')[0])
const orderFilter = ref('all')

const report = ref({
  totalRevenue: 0,
  tableRevenue: 0,
  productRevenue: 0,
  rechargeAmount: 0,
  rechargeCount: 0,
  orderCount: 0,
  avgOrderValue: 0,
  tableUsageRate: 0,
  memberRevenue: 0,
  revenueChange: 0,
  orderChange: 0,
  paymentBreakdown: [],
  hourlyData: [],
  tableUsage: [],
  expenses: [],
  totalExpense: 0,
  orders: [],
})

const filteredOrders = computed(() => {
  if (orderFilter.value === 'all') return report.value.orders
  return report.value.orders.filter(o => o.type === orderFilter.value)
})

const formatDisplayDate = (d) => {
  if (!d) return '-'
  const date = new Date(d)
  return date.toLocaleDateString('zh-CN', { year: 'numeric', month: 'long', day: 'numeric', weekday: 'long' })
}

const formatTime = (ts) => {
  if (!ts) return '-'
  return new Date(ts).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
}

const percent = (part, total) => {
  if (!total) return 0
  return ((part / total) * 100).toFixed(1)
}

const barHeight = (revenue) => {
  const max = Math.max(...report.value.hourlyData.map(h => h.revenue))
  if (!max) return 0
  return (revenue / max) * 100
}

const loadReport = async () => {
  // 模拟数据，实际应调用后端API
  const dash = await getDashboard().catch(() => null)

  report.value = {
    totalRevenue: 3580.50,
    tableRevenue: 2840.00,
    productRevenue: 740.50,
    rechargeAmount: 2000.00,
    rechargeCount: 5,
    orderCount: 28,
    avgOrderValue: 127.88,
    tableUsageRate: 68.5,
    memberRevenue: 1820.00,
    revenueChange: 12.3,
    orderChange: 5.2,
    paymentBreakdown: [
      { method: 'wechat', amount: 2180.50, count: 15 },
      { method: 'alipay', amount: 900.00, count: 8 },
      { method: 'cash', amount: 500.00, count: 5 },
    ],
    hourlyData: [
      { hour: '10', revenue: 180 },
      { hour: '11', revenue: 320 },
      { hour: '12', revenue: 480 },
      { hour: '13', revenue: 520 },
      { hour: '14', revenue: 450 },
      { hour: '15', revenue: 380 },
      { hour: '16', revenue: 290 },
      { hour: '17', revenue: 420 },
      { hour: '18', revenue: 580 },
      { hour: '19', revenue: 620 },
      { hour: '20', revenue: 480 },
      { hour: '21', revenue: 260 },
    ],
    tableUsage: [
      { name: '1号桌', minutes: 420, revenue: 350.00 },
      { name: '2号桌', minutes: 380, revenue: 280.00 },
      { name: 'VIP-1', minutes: 240, revenue: 480.00 },
      { name: '3号桌', minutes: 180, revenue: 150.00 },
    ],
    expenses: [
      { category: '水电费', amount: 150.00, remark: '日结' },
      { category: '耗材', amount: 80.00, remark: '巧粉、手套' },
    ],
    totalExpense: 230.00,
    orders: [
      { id: 101, type: 'table', tableName: '1号桌', memberName: '张三', minutes: 120, amount: 180.00, paymentMethod: 'wechat', closedAt: new Date() },
      { id: 102, type: 'product', tableName: null, memberName: null, minutes: null, amount: 35.00, paymentMethod: 'cash', closedAt: new Date() },
    ],
  }
}

const exportReport = () => {
  ElMessage.success(t('exportSuccess'))
}

onMounted(() => {
  loadReport()
})
</script>

<style scoped>
.daily-report-page {
  padding: 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
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

.report-date {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  color: var(--text-secondary);
  margin-bottom: 24px;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.metric-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.metric-card.highlight {
  background: linear-gradient(135deg, var(--primary-color), #6366f1);
  color: #fff;
  grid-column: span 1;
}

.metric-label {
  font-size: 13px;
  opacity: 0.8;
  margin-bottom: 8px;
}

.metric-value {
  font-size: 28px;
  font-weight: 600;
  margin-bottom: 4px;
}

.metric-compare {
  font-size: 12px;
  opacity: 0.9;
}

.metric-compare .up { color: #4ade80; }
.metric-compare .down { color: #f87171; }

.metric-percent {
  font-size: 12px;
  color: var(--accent-primary);
}

.metric-count {
  font-size: 12px;
  color: var(--text-secondary);
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 24px;
  margin-bottom: 24px;
}

.detail-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.card-header {
  margin-bottom: 16px;
}

.card-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.payment-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.payment-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--hover-bg);
  border-radius: 8px;
}

.payment-info {
  display: flex;
  flex-direction: column;
}

.payment-name {
  font-weight: 500;
}

.payment-count {
  font-size: 12px;
  color: var(--text-secondary);
}

.payment-amount {
  font-weight: 600;
  color: var(--accent-success);
}

.hourly-chart {
  display: flex;
  align-items: flex-end;
  gap: 4px;
  height: 140px;
  padding-top: 20px;
}

.hour-bar {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.bar-wrapper {
  width: 100%;
  height: 100px;
  display: flex;
  align-items: flex-end;
}

.bar {
  width: 100%;
  background: linear-gradient(to top, var(--primary-color), #6366f1);
  border-radius: 4px 4px 0 0;
  min-height: 4px;
}

.hour-label {
  font-size: 11px;
  color: var(--text-secondary);
  margin-top: 4px;
}

.hour-value {
  font-size: 10px;
  color: var(--text-secondary);
}

.expense-summary {
  margin-bottom: 12px;
}

.expense-total {
  display: flex;
  justify-content: space-between;
  padding: 12px;
  background: rgba(239, 68, 68, 0.1);
  border-radius: 8px;
}

.expense-amount {
  font-weight: 600;
  color: var(--accent-danger);
}

.orders-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
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
</style>
