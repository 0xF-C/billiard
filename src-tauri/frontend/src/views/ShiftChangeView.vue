<template>
  <div class="shift-change-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('sub_shift_change') }}</h1>
      <el-button type="primary" @click="showShiftDialog = true">
        <el-icon><Switch /></el-icon>
        {{ t('startShiftChange') }}
      </el-button>
    </div>

    <!-- 统计概览 -->
    <div class="stats-overview">
      <el-card class="stat-card">
        <div class="stat-label">{{ t('currentShift') }}</div>
        <div class="stat-value">{{ currentShift?.shift_name || t('noShift') }}</div>
        <div class="stat-time" v-if="currentShift">
          {{ t('startTime') }}: {{ formatTime(currentShift.start_time) }}
        </div>
      </el-card>
      
      <el-card class="stat-card highlight">
        <div class="stat-label">{{ t('shiftRevenue') }}</div>
        <div class="stat-value">¥{{ shiftStats.revenue?.toFixed(2) || '0.00' }}</div>
        <div class="stat-compare" v-if="lastShiftRevenue">
          {{ t('lastShift') }}: ¥{{ lastShiftRevenue.toFixed(2) }}
        </div>
      </el-card>
      
      <el-card class="stat-card">
        <div class="stat-label">{{ t('orderCount') }}</div>
        <div class="stat-value">{{ shiftStats.order_count || 0 }}</div>
        <div class="stat-detail">
          {{ t('completed') }}: {{ shiftStats.completed_count || 0 }} | 
          {{ t('pending') }}: {{ shiftStats.pending_count || 0 }}
        </div>
      </el-card>
      
      <el-card class="stat-card">
        <div class="stat-label">{{ t('cashInHand') }}</div>
        <div class="stat-value">¥{{ shiftStats.cash_amount?.toFixed(2) || '0.00' }}</div>
        <div class="stat-detail">
          {{ t('wechat') }}: ¥{{ shiftStats.wechat_amount?.toFixed(2) || '0.00' }} | 
          {{ t('alipay') }}: ¥{{ shiftStats.alipay_amount?.toFixed(2) || '0.00' }}
        </div>
      </el-card>
    </div>

    <!-- 班次记录 -->
    <div class="section">
      <h3 class="section-title">{{ t('shiftRecords') }}</h3>
      <el-table :data="shiftRecords" v-loading="loading">
        <el-table-column :label="t('shiftName')" prop="shift_name" min-width="120" />
        <el-table-column :label="t('operator')" prop="operator_name" width="100" />
        <el-table-column :label="t('startTime')" min-width="160">
          <template #default="{ row }">{{ formatTime(row.start_time) }}</template>
        </el-table-column>
        <el-table-column :label="t('endTime')" min-width="160">
          <template #default="{ row }">{{ formatTime(row.end_time) }}</template>
        </el-table-column>
        <el-table-column :label="t('revenue')" width="120">
          <template #default="{ row }">¥{{ row.revenue?.toFixed(2) }}</template>
        </el-table-column>
        <el-table-column :label="t('orderCount')" prop="order_count" width="100" />
        <el-table-column :label="t('status')" width="100">
          <template #default="{ row }">
            <el-tag :type="row.status === 'completed' ? 'success' : 'warning'">
              {{ t(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="150" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="viewDetail(row)">{{ t('detail') }}</el-button>
            <el-button size="small" type="primary" @click="printReport(row)">{{ t('print') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 交班对话框 -->
    <el-dialog v-model="showShiftDialog" :title="t('shiftChangeConfirm')" width="600px">
      <div class="shift-summary">
        <h4>{{ t('shiftSummary') }}</h4>
        <div class="summary-item">
          <span>{{ t('shiftDuration') }}:</span>
          <span>{{ shiftDuration }}</span>
        </div>
        <div class="summary-item">
          <span>{{ t('totalRevenue') }}:</span>
          <span class="highlight">¥{{ shiftStats.revenue?.toFixed(2) || '0.00' }}</span>
        </div>
        <div class="summary-item">
          <span>{{ t('totalOrders') }}:</span>
          <span>{{ shiftStats.order_count || 0 }}</span>
        </div>
        
        <el-divider />
        
        <h4>{{ t('paymentBreakdown') }}</h4>
        <div class="payment-list">
          <div class="payment-item">
            <span>{{ t('cash') }}:</span>
            <span>¥{{ shiftStats.cash_amount?.toFixed(2) || '0.00' }}</span>
          </div>
          <div class="payment-item">
            <span>{{ t('wechat') }}:</span>
            <span>¥{{ shiftStats.wechat_amount?.toFixed(2) || '0.00' }}</span>
          </div>
          <div class="payment-item">
            <span>{{ t('alipay') }}:</span>
            <span>¥{{ shiftStats.alipay_amount?.toFixed(2) || '0.00' }}</span>
          </div>
          <div class="payment-item">
            <span>{{ t('memberPay') }}:</span>
            <span>¥{{ shiftStats.member_amount?.toFixed(2) || '0.00' }}</span>
          </div>
        </div>
        
        <el-divider />
        
        <el-form :model="shiftForm" label-width="120px">
          <el-form-item :label="t('nextOperator')" required>
            <el-select v-model="shiftForm.next_operator_id" :placeholder="t('pleaseSelect')">
              <el-option v-for="user in users" :key="user.id" :label="user.username" :value="user.id" />
            </el-select>
          </el-form-item>
          <el-form-item :label="t('handoverAmount')" required>
            <el-input-number v-model="shiftForm.handover_amount" :min="0" :precision="2" :controls="false" style="width: 200px;" />
          </el-form-item>
          <el-form-item :label="t('remark')">
            <el-input v-model="shiftForm.remark" type="textarea" :rows="2" :placeholder="t('optional')" />
          </el-form-item>
        </el-form>
      </div>
      <template #footer>
        <el-button @click="showShiftDialog = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="confirmShiftChange">{{ t('confirmShiftChange') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { Switch } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { t } from '../i18n'
import { getCurrentShift, getShiftRecords, getShiftStats, createShiftRecord, getUsers } from '../api'

const loading = ref(false)
const currentShift = ref(null)
const shiftRecords = ref([])
const shiftStats = ref({})
const lastShiftRevenue = ref(0)
const users = ref([])
const showShiftDialog = ref(false)
const shiftForm = ref({
  next_operator_id: null,
  handover_amount: 0,
  remark: ''
})

const shiftDuration = computed(() => {
  if (!currentShift.value?.start_time) return '-'
  const start = new Date(currentShift.value.start_time)
  const now = new Date()
  const hours = Math.floor((now - start) / (1000 * 60 * 60))
  const mins = Math.floor(((now - start) % (1000 * 60 * 60)) / (1000 * 60))
  return `${hours}${t('hour')}${mins}${t('minute')}`
})

const loadData = async () => {
  loading.value = true
  try {
    const [shiftRes, recordsRes, statsRes, usersRes] = await Promise.all([
      getCurrentShift(),
      getShiftRecords(),
      getShiftStats(),
      getUsers()
    ])
    currentShift.value = shiftRes
    shiftRecords.value = recordsRes || []
    shiftStats.value = statsRes || {}
    users.value = usersRes || []
    
    if (shiftRecords.value.length > 1) {
      lastShiftRevenue.value = shiftRecords.value[1].revenue || 0
    }
    
    shiftForm.value.handover_amount = shiftStats.value.cash_amount || 0
  } catch (e) {
    ElMessage.error(t('loadFailed'))
  } finally {
    loading.value = false
  }
}

const confirmShiftChange = async () => {
  if (!shiftForm.value.next_operator_id) {
    ElMessage.warning(t('pleaseSelectOperator'))
    return
  }
  try {
    await createShiftRecord({
      ...shiftForm.value,
      shift_stats: shiftStats.value
    })
    ElMessage.success(t('shiftChangeSuccess'))
    showShiftDialog.value = false
    loadData()
  } catch (e) {
    ElMessage.error(e.message || t('shiftChangeFailed'))
  }
}

const viewDetail = (row) => {
  console.log('View shift detail:', row)
}

const printReport = (row) => {
  console.log('Print shift report:', row)
}

const formatTime = (time) => {
  return time ? time.replace('T', ' ').slice(0, 16) : '-'
}

onMounted(loadData)
</script>

<style scoped>
.shift-change-page {
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
.stats-overview {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}
.stat-card {
  text-align: center;
}
.stat-card.highlight {
  border-color: var(--accent-primary);
  background: rgba(31, 111, 235, 0.05);
}
.stat-label {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-bottom: 8px;
}
.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
}
.stat-time, .stat-compare, .stat-detail {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 4px;
}
.section {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 24px;
}
.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 16px 0;
}
.shift-summary {
  padding: 16px;
}
.shift-summary h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  color: var(--text-secondary);
}
.summary-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-muted);
}
.summary-item .highlight {
  color: var(--accent-primary);
  font-weight: 600;
  font-size: 18px;
}
.payment-list {
  margin-bottom: 16px;
}
.payment-item {
  display: flex;
  justify-content: space-between;
  padding: 6px 0;
}
</style>