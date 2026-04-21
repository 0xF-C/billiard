<template>
  <div class="finance-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('finance') }}</h1>
      <div class="header-right">
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          range-separator="~"
          :start-placeholder="t('startTime')"
          :end-placeholder="t('endTime')"
          value-format="YYYY-MM-DD"
          @change="loadStats"
        />
        <el-button :icon="Refresh" circle @click="loadStats" />
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card highlight">
        <div class="stat-icon">
          <el-icon><Money /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('todayRevenue') }}</span>
          <span class="stat-value">¥{{ stats.todayRevenue?.toFixed(2) || '0.00' }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon blue">
          <el-icon><Coin /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('weekRevenue') }}</span>
          <span class="stat-value">¥{{ stats.weekRevenue?.toFixed(2) || '0.00' }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon purple">
          <el-icon><Wallet /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('monthRevenue') }}</span>
          <span class="stat-value">¥{{ stats.monthRevenue?.toFixed(2) || '0.00' }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon green">
          <el-icon><Ticket /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('totalOrders') }}</span>
          <span class="stat-value">{{ stats.todayOrders || 0 }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon orange">
          <el-icon><Timer /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('activeOrders') }}</span>
          <span class="stat-value">{{ activeOrders.length }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon red">
          <el-icon><User /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('memberConsumption') }}</span>
          <span class="stat-value">¥{{ stats.memberRevenue?.toFixed(2) || '0.00' }}</span>
        </div>
      </div>
    </div>

    <div class="section-header">
      <h2 class="section-title">
        <el-icon><Tickets /></el-icon>
        {{ t('orderRecords') }}
      </h2>
      <div class="section-actions">
        <el-radio-group v-model="curTab" size="default" @change="loadOrders">
          <el-radio-button value="active">
            <el-icon><Timer /></el-icon> {{ t('activeOrders') }} ({{ activeOrders.length }})
          </el-radio-button>
          <el-radio-button value="history">
            <el-icon><Clock /></el-icon> {{ t('historyOrders') }}
          </el-radio-button>
        </el-radio-group>
      </div>
    </div>

    <!-- {{ t('ActiveOrders') }} -->
    <div v-if="curTab === 'active'" class="active-orders-grid">
      <div v-for="order in activeOrders" :key="order.id" class="order-card active">
        <div class="order-header">
          <div class="order-id">#{{ order.id }}</div>
          <el-tag type="warning" size="small" effect="dark">
            <el-icon class="pulse"><Timer /></el-icon>
            {{ t('inProgress') }}
          </el-tag>
        </div>
        
        <div class="order-body">
          <div class="info-row">
            <el-icon><Grid /></el-icon>
            <span class="table-name">{{ order.table_name }}</span>
          </div>
          
          <div class="info-row">
            <el-icon><User /></el-icon>
            <span :class="order.member_name && order.member_name !== t('walkInCustomer') ? 'member-name' : ''">
              {{ order.member_name || t('walkInCustomer') }}
            </span>
          </div>
          
          <div class="info-row">
            <el-icon><Clock /></el-icon>
            <span class="start-time">{{ formatTime(order.start_time) }}</span>
          </div>
        </div>
        
        <div class="order-timer">
          <div class="timer-label">{{ t('consumptionDuration') }}</div>
          <div class="timer-value">{{ getDur(order.start_time) }}</div>
        </div>
        
        <div class="order-footer">
          <div class="cost-preview">
            <div class="cost-row">
              <span>{{ t('amount') }}</span>
              <span class="cost-value">¥{{ calcEstimatedAmount(order) }}</span>
            </div>
            <div v-if="order.deposit && order.deposit > 0" class="cost-row deposit">
              <span>{{ t('depositPaid') }}</span>
              <span>¥{{ order.deposit.toFixed(2) }}</span>
            </div>
          </div>
          
          <el-button type="danger" class="close-btn" @click="doClose(order)">
            <el-icon><Money /></el-icon>
            {{ t('closeTable') }}
          </el-button>
        </div>
      </div>
      
      <div v-if="activeOrders.length === 0" class="empty-state">
        <el-icon :size="64"><Finished /></el-icon>
        <h3>{{ t('noData') }}</h3>
        <p>{{ t('tableFree') }}</p>
      </div>
    </div>

    <!-- {{ t('HistoryOrders') }} -->
    <div v-else class="card">
      <el-table :data="orders" stripe style="width: 100%" v-loading="loading" :default-sort="{ prop: 'end_time', order: 'descending' }">
        <el-table-column :label="t('orderNo')" prop="id" width="80" sortable>
          <template #default="{ row }">#{{ row.id }}</template>
        </el-table-column>
        <el-table-column :label="t('tables')" prop="table_name" min-width="100" />
        <el-table-column :label="t('memberName')" min-width="120">
          <template #default="{ row }">
            <span :class="row.member_name && row.member_name !== t('walkInCustomer') ? 'member-name' : ''">
              {{ row.member_name || t('walkInCustomer') }}
            </span>
          </template>
        </el-table-column>
        <el-table-column :label="t('startTime')" prop="start_time" min-width="160" sortable>
          <template #default="{ row }">{{ formatTime(row.start_time) }}</template>
        </el-table-column>
        <el-table-column :label="t('endTime')" prop="end_time" min-width="160" sortable>
          <template #default="{ row }">{{ formatTime(row.end_time) }}</template>
        </el-table-column>
        <el-table-column :label="t('duration')" min-width="120" sortable :sort-method="(a,b) => a.duration_minutes - b.duration_minutes">
          <template #default="{ row }">{{ getDurFromMins(row.duration_minutes) }}</template>
        </el-table-column>
        <el-table-column :label="t('totalAmount')" prop="total_amount" min-width="100" sortable>
          <template #default="{ row }">
            <span class="amount">¥{{ row.total_amount?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('discount')" prop="discount_amount" min-width="90">
          <template #default="{ row }">
            <span v-if="row.discount_amount > 0" class="discount">-¥{{ row.discount_amount?.toFixed(2) }}</span>
            <span v-else class="no-data">-</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('deposit')" prop="deposit" min-width="90">
          <template #default="{ row }">
            <span v-if="row.deposit > 0" class="deposit">¥{{ row.deposit?.toFixed(2) }}</span>
            <span v-else class="no-data">-</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('finalAmount')" prop="final_amount" min-width="110" sortable>
          <template #default="{ row }">
            <span class="final-amount">¥{{ row.final_amount?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('change')" prop="change_amount" min-width="90">
          <template #default="{ row }">
            <span v-if="row.change_amount > 0" class="change">¥{{ row.change_amount?.toFixed(2) }}</span>
            <span v-else class="no-data">-</span>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- {{ t('CheckoutConfirmDialog') }} -->
    <el-dialog v-model="closeDlg" :title="t('closeTable')" width="480px" :close-on-click-modal="false">
      <div class="checkout-content" v-if="target">
        <div class="checkout-header">
          <div class="table-info">
            <el-icon><Grid /></el-icon>
            <span>{{ target.table_name }}</span>
          </div>
          <el-tag type="warning" size="small" effect="dark">
            <el-icon class="pulse"><Timer /></el-icon>
            {{ t('activeOrders') }}
          </el-tag>
        </div>
        
        <div class="checkout-details">
          <div class="detail-row">
            <span class="detail-label">
              <el-icon><User /></el-icon>
              {{ t('memberName') }}
            </span>
            <span class="detail-value">{{ target.member_name || t('walkIn') }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">
              <el-icon><Clock /></el-icon>
              {{ t('startTime') }}
            </span>
            <span class="detail-value">{{ formatTime(target.start_time) }}</span>
          </div>
          <div class="detail-row highlight">
            <span class="detail-label">
              <el-icon><Timer /></el-icon>
              {{ t('consumptionDuration') }}
            </span>
            <span class="detail-value time">{{ getDur(target.start_time) }}</span>
          </div>
        </div>
        
        <el-divider />
        
        <div class="amount-section">
          <div class="amount-row">
            <span>{{ t('tableRate') }}</span>
            <span>¥{{ target.total_amount?.toFixed(2) }}</span>
          </div>
          <div v-if="target.discount_amount > 0" class="amount-row discount">
            <span>{{ t('memberDiscount') }}</span>
            <span class="discount-value">-¥{{ target.discount_amount?.toFixed(2) }}</span>
          </div>
          <div v-if="target.deposit > 0" class="amount-row deposit">
            <span>{{ t('depositPaid') }}</span>
            <span class="deposit-value">-¥{{ target.deposit?.toFixed(2) }}</span>
          </div>
          <div class="amount-row total">
            <span>{{ t('actualPayment') }}</span>
            <span class="total-value">¥{{ target.final_amount?.toFixed(2) }}</span>
          </div>
          <div v-if="target.change_amount > 0" class="amount-row change">
            <span>{{ t('change') }}</span>
            <span class="change-value">¥{{ target.change_amount?.toFixed(2) }}</span>
          </div>
        </div>
      </div>
      <template #footer>
        <div style="display:flex;gap:12px;justify-content:flex-end;">
          <el-button @click="closeDlg=false">{{ t('cancel') }}</el-button>
          <el-button type="danger" @click="cfmClose">
            <el-icon><Money /></el-icon>
            {{ t('confirmClose') }} ¥{{ target?.final_amount?.toFixed(2) }}
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { ElMessage, ElButton, ElTable, ElTableColumn, ElDialog, ElTag, ElIcon, ElDivider, ElRadioGroup, ElRadioButton, ElDatePicker } from 'element-plus'
import { Money, Coin, Wallet, Ticket, Timer, User, Refresh, Grid, Clock, Tickets, Finished } from '@element-plus/icons-vue'
import { getActiveOrders, getOrders, closeTable, getRevenueTrend, getMemberRevenue, getDashboard } from '../api'
import { t } from '../i18n'

const dateRange = ref([])
const stats = ref({
  todayRevenue: 0,
  weekRevenue: 0,
  monthRevenue: 0,
  todayOrders: 0,
  memberRevenue: 0
})

const activeOrders = ref([])
const orders = ref([])
const loading = ref(false)
const curTab = ref('active')
const closeDlg = ref(false)
const target = ref(null)
let timer = null

const formatTime = (time) => {
  if (!time) return '-'
  return time.replace('T', ' ').slice(0, 16)
}

const getDur = (time) => {
  if (!time) return `0${t('minute')}`
  const d = Math.floor((Date.now() - new Date(time.replace(' ', 'T'))) / 60000)
  const hours = Math.floor(d / 60)
  const mins = d % 60
  return hours > 0 ? `${hours}${t('hour')} ${mins}${t('minute')}` : `${d}${t('minute')}`
}

const getDurFromMins = (mins) => {
  if (!mins) return `0${t('minute')}`
  const hours = Math.floor(mins / 60)
  const m = mins % 60
  return hours > 0 ? `${hours}${t('hour')} ${m}${t('minute')}` : `${m}${t('minute')}`
}

// 计算实时预估金额
const calcEstimatedAmount = (order) => {
  if (!order || !order.start_time) return '0.00'

  const start = new Date(order.start_time.replace(' ', 'T'))
  const now = Date.now()
  const durationMins = Math.max(Math.floor((now - start) / 60000), 1)

  // 计费规则：不足5分钟免费，5-30分钟按30分钟算，30-60分钟按60分钟算，之后按小时进位
  let billMins = 0
  if (durationMins < 5) billMins = 0
  else if (durationMins < 30) billMins = 30
  else if (durationMins < 60) billMins = 60
  else {
    const hours = Math.floor(durationMins / 60)
    const remaining = durationMins % 60
    if (remaining < 5) billMins = hours * 60
    else if (remaining < 30) billMins = hours * 60 + 30
    else billMins = hours * 60 + 60
  }

  // 获取费率 - 使用订单保存的费率或默认30
  const rate = order.hourly_rate || 30
  let total = (billMins / 60) * rate

  // 如果有套餐价格且未超时，使用套餐价格
  if (order.package_price && order.package_hours) {
    const pkgMins = order.package_hours * 60
    if (durationMins <= pkgMins) {
      total = order.package_price
    } else {
      // 超出套餐时间
      const extraMins = durationMins - pkgMins
      const extraBillMins = Math.ceil(extraMins / 30) * 30
      total = order.package_price + (extraBillMins / 60) * rate
    }
  }

  // 应用会员折扣
  if (order.member_id && order.discount && order.discount < 1) {
    total = total * order.discount
  }

  return total.toFixed(2)
}

const loadStats = async () => {
  try {
    const [trend, memberData, dashboard] = await Promise.all([
      getRevenueTrend(30),
      getMemberRevenue(30),
      getDashboard()
    ])
    const today = new Date().toISOString().slice(0, 10)
    const weekStart = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString().slice(0, 10)
    const monthStart = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().slice(0, 10)

    let todayRev = 0, weekRev = 0, monthRev = 0, todayOrd = 0

    trend.forEach(item => {
      const rev = item.revenue || 0
      if (item.date === today) {
        todayRev = rev
        todayOrd = item.order_count || 0
      }
      if (item.date >= weekStart) weekRev += rev
      if (item.date >= monthStart) monthRev += rev
    })

    stats.value = {
      todayRevenue: todayRev,
      weekRevenue: weekRev,
      monthRevenue: monthRev,
      todayOrders: todayOrd,
      memberRevenue: dashboard.member_revenue || 0
    }
  } catch (e) {
    console.error(e)
  }
}

const loadOrders = async () => {
  loading.value = true
  try {
    if (curTab.value === 'active') {
      activeOrders.value = await getActiveOrders()
    } else {
      orders.value = await getOrders({ status: '已结账' })
    }
  } catch (e) {
    ElMessage.error(t('loadFailed'))
  }
  loading.value = false
}

const loadAll = async () => {
  await Promise.all([loadStats(), loadOrders()])
}

const doClose = (o) => { target.value = o; closeDlg.value = true }

const cfmClose = async () => {
  try {
    await closeTable(target.value.id, {})
    ElMessage.success({ message: `${t('closeSuccess')}！¥${target.value.final_amount?.toFixed(2)}`, grouping: true })
    closeDlg.value = false
    await loadAll()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

onMounted(() => {
  loadAll()
  timer = setInterval(loadAll, 30000)
})
onUnmounted(() => clearInterval(timer))
</script>

<style scoped>
.finance-page { display: flex; flex-direction: column; gap: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 16px; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.header-right { display: flex; gap: 12px; align-items: center; }

.stats-grid { display: grid; grid-template-columns: repeat(6, 1fr); gap: 16px; }
.stat-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.2s;
}
.stat-card:hover {
  border-color: var(--accent-primary);
  transform: translateY(-2px);
}
.stat-card.highlight { border-color: rgba(63,185,80,0.3); background: rgba(63,185,80,0.05); }
.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-md);
  background: rgba(63,185,80,0.15);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  color: var(--accent-success);
}
.stat-icon.blue { background: rgba(88,166,255,0.15); color: var(--accent-primary); }
.stat-icon.purple { background: rgba(163,113,247,0.15); color: var(--accent-purple); }
.stat-icon.green { background: rgba(63,185,80,0.15); color: var(--accent-success); }
.stat-icon.orange { background: rgba(230,162,60,0.15); color: var(--accent-warning); }
.stat-icon.red { background: rgba(245,108,108,0.15); color: var(--accent-danger); }
.stat-body { display: flex; flex-direction: column; gap: 4px; }
.stat-label { font-size: 13px; color: var(--text-secondary); }
.stat-value { font-size: 20px; font-weight: 700; color: var(--text-primary); }

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px;
}
.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}
.section-actions { display: flex; gap: 12px; align-items: center; }

.card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
}

/* Active Orders Grid */
.active-orders-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 16px;
}

.order-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  transition: all 0.2s;
}

.order-card:hover {
  border-color: var(--accent-primary);
  transform: translateY(-2px);
}

.order-card.active {
  border-left: 4px solid var(--accent-warning);
}

.order-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.order-id {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
}

.order-body {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.info-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-secondary);
}

.info-row .el-icon {
  color: var(--text-tertiary);
}

.table-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.member-name { color: var(--accent-primary); font-weight: 600; }
.start-time { color: var(--text-tertiary); }

.order-timer {
  background: rgba(210,153,34,0.1);
  border: 1px solid rgba(210,153,34,0.2);
  border-radius: var(--radius-md);
  padding: 16px;
  text-align: center;
}

.timer-label {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-bottom: 4px;
}

.timer-value {
  font-size: 28px;
  font-weight: 700;
  color: var(--accent-warning);
  font-family: var(--font-mono);
}

.order-footer {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.cost-preview {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  padding: 12px;
}

.cost-row {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: var(--text-secondary);
  padding: 4px 0;
}

.cost-value { font-weight: 600; color: var(--text-primary); }
.cost-row.deposit { color: var(--accent-success); }

.close-btn { width: 100%; }

.empty-state {
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

.empty-state h3 { margin: 0; color: var(--text-primary); }
.empty-state p { margin: 0; }

/* Table Styles */
.member-name { color: var(--accent-primary); font-weight: 600; }
.amount { color: var(--text-primary); font-weight: 500; }
.discount { color: var(--accent-success); font-weight: 500; }
.deposit { color: var(--accent-primary); font-weight: 500; }
.final-amount { color: var(--accent-success); font-weight: 600; }
.change { color: var(--accent-warning); font-weight: 500; }
.no-data { color: var(--text-tertiary); }
.pulse { animation: pulse 1.5s infinite; }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }

/* Checkout Dialog */
.checkout-content { display: flex; flex-direction: column; gap: 16px; }
.checkout-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background: var(--bg-primary);
  border-radius: var(--radius-md);
}
.table-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
}
.table-info .el-icon { font-size: 24px; color: var(--accent-primary); }

.checkout-details { display: flex; flex-direction: column; gap: 8px; }
.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
}
.detail-label { display: flex; align-items: center; gap: 6px; color: var(--text-secondary); font-size: 14px; }
.detail-label .el-icon { color: var(--text-tertiary); }
.detail-value { color: var(--text-primary); font-weight: 500; }
.detail-row.highlight { background: rgba(88,166,255,0.1); }
.detail-row .time { color: var(--accent-primary); font-size: 16px; font-weight: 600; }

.amount-section { display: flex; flex-direction: column; gap: 8px; }
.amount-row { display: flex; justify-content: space-between; font-size: 14px; color: var(--text-secondary); }
.amount-row.total {
  padding: 12px;
  background: rgba(248,81,73,0.1);
  border-radius: var(--radius-sm);
  font-weight: 600;
  color: var(--text-primary);
}
.amount-row.discount, .amount-row.deposit { color: var(--accent-success); }
.amount-row.change { color: var(--accent-warning); }
.total-value { font-size: 24px; font-weight: 700; color: var(--accent-danger); }
.discount-value, .deposit-value { color: var(--accent-success); }
.change-value { color: var(--accent-warning); }

:deep(.el-table) { background: transparent !important; }
:deep(.el-table th) { background: var(--bg-primary) !important; }
:deep(.el-table tr) { background: transparent !important; }
:deep(.el-table td) { border-bottom: 1px solid var(--border-muted) !important; }
:deep(.el-divider) { margin: 12px 0; border-color: var(--border-muted); }

@media (max-width: 1400px) {
  .stats-grid { grid-template-columns: repeat(3, 1fr); }
}

@media (max-width: 768px) {
  .stats-grid { grid-template-columns: repeat(2, 1fr); }
  .active-orders-grid { grid-template-columns: 1fr; }
}
</style>