<template>
  <div class="pending-orders-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('sub_pending_orders') }}</h1>
      <el-button type="primary" @click="showCreateDlg = true">
        <el-icon><Plus /></el-icon>
        {{ t('createPendingOrder') }}
      </el-button>
    </div>

    <!-- {{ t('PendingOrdersList') }} -->
    <div class="orders-list">
      <el-card v-for="order in pendingOrders" :key="order.id" class="order-card" :class="{ 'expired': isExpired(order) }">
        <div class="order-header">
          <div class="order-info">
            <span class="order-no">{{ order.order_no }}</span>
            <el-tag :type="getStatusType(order.status)">{{ t(order.status) }}</el-tag>
          </div>
          <div class="order-time">{{ formatTime(order.created_at) }}</div>
        </div>

        <div class="order-body">
          <div class="customer-info">
            <el-icon><User /></el-icon>
            <span>{{ order.customer_name || t('walkIn') }}</span>
            <span v-if="order.customer_phone" class="phone">{{ order.customer_phone }}</span>
          </div>

          <div class="table-info">
            <el-icon><Grid /></el-icon>
            <span>{{ t('table') }}: {{ order.table_name }}</span>
          </div>

          <div class="amount-info">
            <span class="label">{{ t('estimatedAmount') }}:</span>
            <span class="amount">¥{{ order.estimated_amount?.toFixed(2) || '0.00' }}</span>
          </div>

          <div class="remark" v-if="order.remark">
            <el-icon><ChatDotRound /></el-icon>
            <span>{{ order.remark }}</span>
          </div>
        </div>

        <div class="order-footer">
          <el-button size="small" @click="viewDetail(order)">{{ t('detail') }}</el-button>
          <el-button size="small" type="primary" @click="resumeOrder(order)">{{ t('resumeOrder') }}</el-button>
          <el-button size="small" type="danger" @click="cancelOrder(order)">{{ t('cancel') }}</el-button>
        </div>
      </el-card>

      <el-empty v-if="pendingOrders.length === 0" :description="t('noPendingOrders')" />
    </div>

    <!-- {{ t('CreatePendingOrderDialog') }} -->
    <el-dialog v-model="showCreateDlg" :title="t('createPendingOrder')" width="500px">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="t('customerName')">
          <el-input v-model="form.customer_name" :placeholder="t('optional')" />
        </el-form-item>
        <el-form-item :label="t('customerPhone')">
          <el-input v-model="form.customer_phone" :placeholder="t('optional')" />
        </el-form-item>
        <el-form-item :label="t('table')" required>
          <el-select v-model="form.table_id" :placeholder="t('pleaseSelect')">
            <el-option v-for="table in availableTables" :key="table.id" :label="table.name" :value="table.id" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('estimatedAmount')">
          <el-input-number v-model="form.estimated_amount" :min="0" :precision="2" />
        </el-form-item>
        <el-form-item :label="t('remark')">
          <el-input v-model="form.remark" type="textarea" :rows="2" :placeholder="t('optional')" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showCreateDlg = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="createOrder">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- {{ t('PendingOrderDetailDialog') }} -->
    <el-dialog v-model="detailDlg" :title="t('orderDetails')" width="480px">
      <div v-if="selectedOrder" class="detail-content">
        <div class="detail-row">
          <span class="label">{{ t('orderNo') }}:</span>
          <span class="value">{{ selectedOrder.order_no }}</span>
        </div>
        <div class="detail-row">
          <span class="label">{{ t('table') }}:</span>
          <span class="value">{{ selectedOrder.table_name }}</span>
        </div>
        <div class="detail-row">
          <span class="label">{{ t('customerName') }}:</span>
          <span class="value">{{ selectedOrder.customer_name || t('walkIn') }}</span>
        </div>
        <div class="detail-row" v-if="selectedOrder.customer_phone">
          <span class="label">{{ t('customerPhone') }}:</span>
          <span class="value">{{ selectedOrder.customer_phone }}</span>
        </div>
        <div class="detail-row">
          <span class="label">{{ t('estimatedAmount') }}:</span>
          <span class="value amount">¥{{ selectedOrder.estimated_amount?.toFixed(2) || '0.00' }}</span>
        </div>
        <div class="detail-row" v-if="selectedOrder.remark">
          <span class="label">{{ t('remark') }}:</span>
          <span class="value">{{ selectedOrder.remark }}</span>
        </div>
        <div class="detail-row">
          <span class="label">{{ t('createTime') }}:</span>
          <span class="value">{{ formatTime(selectedOrder.created_at) }}</span>
        </div>
        <div class="detail-row">
          <span class="label">{{ t('status') }}:</span>
          <el-tag :type="getStatusType(selectedOrder.status)">{{ t(selectedOrder.status) }}</el-tag>
        </div>
      </div>
      <template #footer>
        <el-button @click="detailDlg = false">{{ t('close') }}</el-button>
        <el-button type="primary" @click="resumeFromDetail">{{ t('resumeOrder') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { Plus, User, Grid, ChatDotRound } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { t } from '../i18n'
import { getTables, getPendingOrders, createPendingOrder, cancelPendingOrder, resumePendingOrder } from '../api'

const pendingOrders = ref([])
const availableTables = ref([])
const showCreateDlg = ref(false)
const detailDlg = ref(false)
const selectedOrder = ref(null)
const form = ref({
  customer_name: '',
  customer_phone: '',
  table_id: null,
  estimated_amount: 0,
  remark: ''
})

const loadData = async () => {
  try {
    const [ordersRes, tablesRes] = await Promise.all([
      getPendingOrders(),
      getTables()
    ])
    pendingOrders.value = ordersRes || []
    availableTables.value = (tablesRes || []).filter(t => t.status === '空闲')
  } catch (e) {
    ElMessage.error(t('loadFailed'))
  }
}

const createOrder = async () => {
  if (!form.value.table_id) {
    ElMessage.warning(t('pleaseSelectTable'))
    return
  }
  try {
    await createPendingOrder(form.value)
    ElMessage.success(t('createSuccess'))
    showCreateDlg.value = false
    form.value = { customer_name: '', customer_phone: '', table_id: null, estimated_amount: 0, remark: '' }
    loadData()
  } catch (e) {
    ElMessage.error(e.message || t('createFailed'))
  }
}

const resumeOrder = async (order) => {
  try {
    await ElMessageBox.confirm(t('confirmResumeOrder'), t('confirm'), { type: 'warning' })
    await resumePendingOrder(order.id)
    ElMessage.success(t('resumeSuccess'))
    loadData()
  } catch (e) {
    if (e !== 'cancel') ElMessage.error(e.message || t('resumeFailed'))
  }
}

const cancelOrder = async (order) => {
  try {
    await ElMessageBox.confirm(t('confirmCancelOrder'), t('confirm'), { type: 'warning' })
    await cancelPendingOrder(order.id)
    ElMessage.success(t('cancelSuccess'))
    loadData()
  } catch (e) {
    if (e !== 'cancel') ElMessage.error(e.message || t('cancelFailed'))
  }
}

const isExpired = (order) => {
  const created = new Date(order.created_at)
  const now = new Date()
  const hours = (now - created) / (1000 * 60 * 60)
  return hours > 24
}

const getStatusType = (status) => {
  const types = { pending: 'warning', processing: 'primary', completed: 'success', cancelled: 'info' }
  return types[status] || 'info'
}

const formatTime = (time) => {
  return time ? time.replace('T', ' ').slice(0, 16) : '-'
}

const viewDetail = (order) => {
  selectedOrder.value = order
  detailDlg.value = true
}

const resumeFromDetail = async () => {
  if (!selectedOrder.value) return
  detailDlg.value = false
  await resumeOrder(selectedOrder.value)
}

onMounted(loadData)
</script>

<style scoped>
.pending-orders-page {
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
.orders-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  gap: 16px;
}
.order-card {
  transition: all 0.3s;
}
.order-card.expired {
  border-color: #f56c6c;
  background: rgba(245, 108, 108, 0.05);
}
.order-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-default);
}
.order-info {
  display: flex;
  align-items: center;
  gap: 12px;
}
.order-no {
  font-weight: 600;
  font-size: 16px;
}
.order-time {
  font-size: 12px;
  color: var(--text-secondary);
}
.order-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 16px;
}
.customer-info, .table-info, .remark {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-secondary);
}
.phone {
  margin-left: 8px;
  color: var(--text-tertiary);
}
.amount-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--bg-primary);
  border-radius: 8px;
  margin-top: 8px;
}
.amount-info .amount {
  font-size: 20px;
  font-weight: 600;
  color: var(--accent-primary);
}
.order-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

/* 详情对话框样式 */
.detail-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--bg-primary);
  border-radius: 8px;
}
.detail-row .label {
  color: var(--text-secondary);
  font-size: 14px;
}
.detail-row .value {
  color: var(--text-primary);
  font-weight: 500;
}
.detail-row .value.amount {
  color: var(--accent-primary);
  font-size: 18px;
  font-weight: 600;
}
</style>