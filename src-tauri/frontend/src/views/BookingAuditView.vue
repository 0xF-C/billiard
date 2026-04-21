<template>
  <div class="booking-audit-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('bookingAudit') }}</h1>
      <div class="header-actions">
        <el-radio-group v-model="filterStatus" size="default">
          <el-radio-button value="">{{ t('all') }}</el-radio-button>
          <el-radio-button value="pending">{{ t('pending') }} ({{ pendingCount }})</el-radio-button>
          <el-radio-button value="approved">{{ t('approved') }}</el-radio-button>
          <el-radio-button value="rejected">{{ t('rejected') }}</el-radio-button>
        </el-radio-group>
      </div>
    </div>

    <div class="summary-row">
      <div class="summary-card pending">
        <span class="value">{{ pendingCount }}</span>
        <span class="label">{{ t('pendingReview') }}</span>
      </div>
      <div class="summary-card approved">
        <span class="value">{{ approvedCount }}</span>
        <span class="label">{{ t('approvedToday') }}</span>
      </div>
      <div class="summary-card rejected">
        <span class="value">{{ rejectedCount }}</span>
        <span class="label">{{ t('rejectedToday') }}</span>
      </div>
    </div>

    <div class="audit-table">
      <el-table :data="filteredList" stripe>
        <el-table-column :label="t('bookingTime')" width="160">
          <template #default="{ row }">{{ formatTime(row.createdAt) }}</template>
        </el-table-column>
        <el-table-column :label="t('customer')" min-width="140">
          <template #default="{ row }">
            <div class="customer-cell">
              <span class="name">{{ row.customerName }}</span>
              <span class="phone">{{ row.customerPhone }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('table')" width="100">
          <template #default="{ row }">{{ row.tableName || '-' }}</template>
        </el-table-column>
        <el-table-column :label="t('bookingSlot')" width="160">
          <template #default="{ row }">
            {{ row.date }} {{ row.startTime }}~{{ row.endTime }}
          </template>
        </el-table-column>
        <el-table-column :label="t('deposit')" width="80">
          <template #default="{ row }">¥{{ row.deposit }}</template>
        </el-table-column>
        <el-table-column :label="t('status')" width="90">
          <template #default="{ row }">
            <el-tag :type="getStatusTag(row.status)" size="small">{{ t(row.status) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('auditNote')" min-width="120">
          <template #default="{ row }">
            <span class="note-text">{{ row.auditNote || '-' }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="180" fixed="right">
          <template #default="{ row }">
            <template v-if="row.status === 'pending'">
              <el-button type="primary" size="small" @click="approve(row)">{{ t('approve') }}</el-button>
              <el-button type="danger" size="small" plain @click="reject(row)">{{ t('reject') }}</el-button>
            </template>
            <template v-else>
              <span class="audited-info">{{ row.auditedBy }} {{ formatTime(row.auditedAt) }}</span>
            </template>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="showReject" :title="t('rejectBooking')" width="400px">
      <el-form label-width="80px">
        <el-form-item :label="t('rejectReason')">
          <el-input v-model="rejectReason" type="textarea" :rows="3" :placeholder="t('inputRejectReason')" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showReject = false">{{ t('cancel') }}</el-button>
        <el-button type="danger" @click="confirmReject">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { t } from '../i18n'


const filterStatus = ref('pending')
const showReject = ref(false)
const rejectReason = ref('')
const selectedItem = ref(null)

const list = ref([
  { id: 1, customerName: t('zhangsan'), customerPhone: '13800138001', tableName: '1号桌', date: '2026-04-20', startTime: '14:00', endTime: '16:00', deposit: 100, status: 'pending', auditNote: '', createdAt: new Date() },
  { id: 2, customerName: t('lisi'), customerPhone: '13800138002', tableName: 'VIP-1', date: '2026-04-20', startTime: '18:00', endTime: '21:00', deposit: 200, status: 'pending', auditNote: '', createdAt: new Date(Date.now() - 1800000) },
  { id: 3, customerName: t('wangwu'), customerPhone: '13800138003', tableName: '2号桌', date: '2026-04-19', startTime: '10:00', endTime: '12:00', deposit: 50, status: 'approved', auditNote: '', auditedBy: '店长', auditedAt: new Date(Date.now() - 3600000) },
  { id: 4, customerName: t('zhaoliu'), customerPhone: '13800138004', tableName: null, date: '2026-04-19', startTime: '16:00', endTime: '18:00', deposit: 0, status: 'rejected', auditNote: t('slotFull'), auditedBy: '店长', auditedAt: new Date(Date.now() - 7200000) },
])

const filteredList = computed(() => {
  if (!filterStatus.value) return list.value
  return list.value.filter(o => o.status === filterStatus.value)
})

const pendingCount = computed(() => list.value.filter(o => o.status === 'pending').length)
const approvedCount = computed(() => list.value.filter(o => o.status === 'approved').length)
const rejectedCount = computed(() => list.value.filter(o => o.status === 'rejected').length)

const getStatusTag = (status) => {
  const types = { pending: 'warning', approved: 'success', rejected: 'danger' }
  return types[status] || 'info'
}

const formatTime = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' })
}

const approve = (item) => {
  item.status = 'approved'
  item.auditedBy = '当前用户'
  item.auditedAt = new Date()
  ElMessage.success(t('approveSuccess'))
}

const reject = (item) => {
  selectedItem.value = item
  rejectReason.value = ''
  showReject.value = true
}

const confirmReject = () => {
  if (!rejectReason.value.trim()) {
    ElMessage.warning(t('inputRejectReason'))
    return
  }
  selectedItem.value.status = 'rejected'
  selectedItem.value.auditNote = rejectReason.value
  selectedItem.value.auditedBy = '当前用户'
  selectedItem.value.auditedAt = new Date()
  showReject.value = false
  ElMessage.success(t('rejectSuccess'))
}
</script>

<style scoped>
.booking-audit-page { padding: 24px; }

.page-title { font-size: 24px; font-weight: 600; margin: 0 0 24px 0; }

.header-actions { display: flex; gap: 12px; }

.summary-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.summary-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  text-align: center;
  border: 1px solid var(--border-color);
  border-left: 4px solid var(--accent-primary);
}

.summary-card.pending { border-left-color: var(--accent-warning); }
.summary-card.approved { border-left-color: var(--accent-success); }
.summary-card.rejected { border-left-color: var(--accent-danger); }

.summary-card .value { display: block; font-size: 28px; font-weight: 600; }
.summary-card .label { font-size: 13px; color: var(--text-secondary); }

.audit-table {
  background: var(--card-bg);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.customer-cell { display: flex; flex-direction: column; }
.customer-cell .name { font-weight: 500; }
.customer-cell .phone { font-size: 12px; color: var(--text-secondary); }

.note-text { font-size: 13px; color: var(--text-secondary); }

.audited-info {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>
