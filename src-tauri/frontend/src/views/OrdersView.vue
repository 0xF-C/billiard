<template>
  <div class="orders-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('activeOrders') }}</h1>
      <div class="header-right">
        <el-radio-group v-model="curTab" size="default" @change="loadOrders">
          <el-radio-button value="active"><el-icon><Timer /></el-icon> {{ t('activeOrders') }}</el-radio-button>
          <el-radio-button value="history"><el-icon><Clock /></el-icon> {{ t('historyOrders') }}</el-radio-button>
        </el-radio-group>
        <el-button :icon="Refresh" circle @click="loadOrders" />
      </div>
    </div>

    <!-- {{ t('ActiveOrders') }} -->
    <div v-if="curTab === 'active'" class="active-orders-grid">
      <div v-for="order in orders" :key="order.id" class="order-card active">
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
              <span>{{ t('estimatedCost') }}</span>
              <span class="cost-value">¥{{ order.total_amount?.toFixed(2) }}</span>
            </div>
            <div v-if="order.deposit && order.deposit > 0" class="cost-row deposit">
              <span>{{ t('deposit') }}</span>
              <span>¥{{ order.deposit.toFixed(2) }}</span>
            </div>
            <div v-if="order.discount_amount && order.discount_amount > 0" class="cost-row discount">
              <span>{{ t('discount') }}</span>
              <span>-¥{{ order.discount_amount.toFixed(2) }}</span>
            </div>
          </div>
          
          <el-button type="danger" class="close-btn" @click="doClose(order)">
            <el-icon><Money /></el-icon>
            {{ t('checkout') }}
          </el-button>
        </div>
      </div>
      
      <div v-if="orders.length === 0 && !loading" class="empty-state">
        <el-icon :size="64"><Finished /></el-icon>
        <h3>{{ t('noActiveOrders') }}</h3>
        <p>{{ t('allTablesFree') }}</p>
      </div>
    </div>

    <!-- {{ t('HistoryOrders') }} -->
    <div v-else class="card">
      <el-table :data="orders" stripe style="width: 100%" v-loading="loading">
        <el-table-column :label="t('orderNo')" prop="id" width="80">
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
        <el-table-column :label="t('startTime')" prop="start_time" min-width="160">
          <template #default="{ row }">{{ formatTime(row.start_time) }}</template>
        </el-table-column>
        <el-table-column :label="t('endTime')" prop="end_time" min-width="160">
          <template #default="{ row }">{{ formatTime(row.end_time) }}</template>
        </el-table-column>
        <el-table-column :label="t('duration')" min-width="120">
          <template #default="{ row }">{{ getDurFromMins(row.duration_minutes) }}</template>
        </el-table-column>
        <el-table-column :label="t('totalAmount')" prop="total_amount" min-width="100">
          <template #default="{ row }">
            <span class="amount">¥{{ row.total_amount?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('discount')" prop="discount_amount" min-width="90">
          <template #default="{ row }">
            <span v-if="row.discount_amount > 0" class="discount">-¥{{ row.discount_amount?.toFixed(2) }}</span>
            <span v-else class="no-discount">-</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('deposit')" prop="deposit" min-width="90">
          <template #default="{ row }">
            <span v-if="row.deposit > 0" class="deposit">¥{{ row.deposit?.toFixed(2) }}</span>
            <span v-else class="no-deposit">-</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('finalAmount')" prop="final_amount" min-width="110">
          <template #default="{ row }">
            <span class="final-amount">¥{{ row.final_amount?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('change')" prop="change_amount" min-width="90">
          <template #default="{ row }">
            <span v-if="row.change_amount > 0" class="change">¥{{ row.change_amount?.toFixed(2) }}</span>
            <span v-else class="no-change">-</span>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- {{ t('CheckoutConfirmDialog') }} -->
    <el-dialog v-model="closeDlg" :title="t('confirmClose')" width="480px" :close-on-click-modal="false" top="5vh" append-to-body>
      <div class="checkout-content" v-if="target">
        <div class="checkout-header">
          <div class="table-info">
            <el-icon><Grid /></el-icon>
            <span>{{ target.table_name }}</span>
          </div>
          <el-tag type="warning" size="small" effect="dark">
            <el-icon class="pulse"><Timer /></el-icon>
            {{ t('inProgress') }}
          </el-tag>
        </div>
        
        <div class="checkout-details">
          <div class="detail-row">
            <span class="detail-label">
              <el-icon><User /></el-icon>
              {{ t('customerName') }}
            </span>
            <span class="detail-value">{{ target.member_name || t('walkInCustomer') }}</span>
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
            <span>{{ t('tableFee') }}</span>
            <span>¥{{ target.total_amount?.toFixed(2) }}</span>
          </div>
          <div class="amount-row">
            <span>{{ t('paymentMethod') }}</span>
            <el-select v-model="closeForm.payment_method" style="width:120px">
              <el-option :label="t('cash')" value="cash" />
              <el-option :label="t('wechat')" value="wechat" />
              <el-option :label="t('alipay')" value="alipay" />
              <el-option :label="t('balance')" value="balance" />
            </el-select>
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
        <el-button @click="closeDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="danger" @click="cfmClose">
          <el-icon><Money /></el-icon>
          {{ t('confirmClose') }} ¥{{ target?.final_amount?.toFixed(2) }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { ElMessage, ElButton, ElTable, ElTableColumn, ElDialog, ElCard, ElRadioGroup, ElRadioButton, ElTag, ElIcon, ElDivider } from 'element-plus'
import { Refresh, Timer, Money, Grid, User, Clock, Finished } from '@element-plus/icons-vue'
import { getActiveOrders, getOrders, closeTable } from '../api'
import { t } from '../i18n'

const orders = ref([])
const loading = ref(false)
const curTab = ref('active')
const closeDlg = ref(false)
const target = ref(null)
const closeForm = ref({ payment_method: 'cash' })
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

const loadOrders = async () => {
  loading.value = true
  try {
    orders.value = curTab.value === 'active'
      ? await getActiveOrders()
      : await getOrders('已完成')
  } catch { ElMessage.error(t('loadFailed')) }
  loading.value = false
}

const doClose = (o) => { target.value = o; closeDlg.value = true }

const cfmClose = async () => {
  try {
    await closeTable(target.value.id, { payment_method: closeForm.value.payment_method })
    ElMessage.success({ message: `${t('closeSuccess')}！¥${target.value.final_amount?.toFixed(2)}`, grouping: true })
    closeDlg.value = false
    closeForm.value = { payment_method: 'cash' }
    await loadOrders()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

onMounted(() => { loadOrders(); timer = setInterval(loadOrders, 30000) })
onUnmounted(() => clearInterval(timer))
</script>

<style scoped>
.orders-page { display: flex; flex-direction: column; gap: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 16px; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.header-right { display: flex; gap: 12px; align-items: center; }
.card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
}
.member-name { color: var(--accent-primary); font-weight: 600; }
.live { display: flex; align-items: center; gap: 4px; color: var(--accent-warning); font-weight: 600; }
.final-amount { color: var(--accent-success); font-weight: 600; }
.pulse { animation: pulse 1.5s infinite; }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
.checkout-summary { display: flex; flex-direction: column; gap: 16px; }
.summary-card { background: var(--bg-primary); }
.summary-row { display: flex; justify-content: space-between; padding: 10px 0; }
.row-label { color: var(--text-secondary); }
.row-value { color: var(--text-primary); }
.row-value.accent { color: var(--accent-warning); display: flex; align-items: center; gap: 4px; }
.summary-row.highlight { background: var(--bg-secondary); margin: 8px -16px; padding: 12px 16px; border-radius: var(--radius-md); }
.summary-total { display: flex; justify-content: space-between; align-items: center; padding: 16px; background: var(--bg-secondary); border-radius: var(--radius-md); }
.total-label { color: var(--text-secondary); }
.total-value { font-size: 28px; font-weight: 700; color: var(--accent-success); }
:deep(.el-table) { background: transparent !important; }
:deep(.el-table th) { background: var(--bg-primary) !important; }
:deep(.el-table tr) { background: transparent !important; }
:deep(.el-table td) { border-bottom: 1px solid var(--border-muted) !important; }
:deep(.el-divider) { margin: 12px 0; border-color: var(--border-muted); }

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

.start-time {
  color: var(--text-tertiary);
}

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

.cost-value {
  font-weight: 600;
  color: var(--text-primary);
}

.cost-row.deposit { color: var(--accent-success); }
.cost-row.discount { color: var(--accent-success); }

.close-btn {
  width: 100%;
}

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

.empty-state h3 {
  margin: 0;
  color: var(--text-primary);
}

.empty-state p {
  margin: 0;
}

/* History Table */
.amount { color: var(--text-primary); font-weight: 500; }
.discount { color: var(--accent-success); font-weight: 500; }
.no-discount { color: var(--text-tertiary); }
.deposit { color: var(--accent-primary); font-weight: 500; }
.no-deposit { color: var(--text-tertiary); }
.change { color: var(--accent-warning); font-weight: 500; }
.no-change { color: var(--text-tertiary); }

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

.table-info .el-icon {
  font-size: 24px;
  color: var(--accent-primary);
}

.checkout-details { display: flex; flex-direction: column; gap: 8px; }
.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
}
.detail-label {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-secondary);
  font-size: 14px;
}
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
.amount-row.discount { color: var(--accent-success); }
.amount-row.deposit { color: var(--accent-success); }
.amount-row.change { color: var(--accent-warning); }
.total-value { font-size: 24px; font-weight: 700; color: var(--accent-danger); }
.discount-value, .deposit-value { color: var(--accent-success); }
.change-value { color: var(--accent-warning); }

@media (max-width: 768px) {
  .active-orders-grid { grid-template-columns: 1fr; }
}
</style>
