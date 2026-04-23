<template>
  <div class="member-recharge-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('memberRechargeRecords') }}</h1>
      <div class="header-actions">
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          range-separator="~"
          :placeholder="t('selectDateRange')"
          value-format="YYYY-MM-DD"
          @change="loadRecharges"
        />
        <el-button :icon="Refresh" @click="loadRecharges">{{ t('refresh') }}</el-button>
      </div>
    </div>

    <div class="stats-row">
      <div class="stat-card">
        <div class="stat-value">{{ stats.totalCount }}</div>
        <div class="stat-label">{{ t('totalRecharges') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">¥{{ stats.totalAmount.toFixed(2) }}</div>
        <div class="stat-label">{{ t('totalAmount') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">¥{{ stats.avgAmount.toFixed(2) }}</div>
        <div class="stat-label">{{ t('avgAmount') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ stats.totalBonus }}</div>
        <div class="stat-label">{{ t('totalBonus') }}</div>
      </div>
    </div>

    <div class="recharges-table">
      <el-table :data="recharges" stripe max-height="600">
        <el-table-column :label="t('time')" width="180">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('member')" min-width="140">
          <template #default="{ row }">
            <div class="member-cell">
              <el-avatar :size="32" :style="{ background: getLevelColor(row.level) }">
                {{ row.member_name?.charAt(0) || 'M' }}
              </el-avatar>
              <div class="member-info">
                <span class="member-name">{{ row.member_name || t('unknown') }}</span>
                <span class="member-phone">{{ row.phone || '-' }}</span>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('rechargeAmount')" width="140">
          <template #default="{ row }">
            <span class="amount">¥{{ row.amount.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('bonus')" width="120">
          <template #default="{ row }">
            <span class="bonus">+{{ row.bonus || 0 }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('paymentMethod')" width="120">
          <template #default="{ row }">
            <el-tag size="small" :type="getPaymentType(row.payment_method)">
              {{ getPaymentLabel(row.payment_method) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('balanceBefore')" width="120">
          <template #default="{ row }">
            ¥{{ (row.balance_before || 0).toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('balanceAfter')" width="120">
          <template #default="{ row }">
            ¥{{ (row.balance_after || 0).toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('operator')" width="100">
          <template #default="{ row }">
            {{ row.operator || '-' }}
          </template>
        </el-table-column>
        <el-table-column :label="t('remark')" min-width="120">
          <template #default="{ row }">
            {{ row.remark || '-' }}
          </template>
        </el-table-column>
      </el-table>
      
      <div class="pagination-wrapper">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :total="total"
          :page-sizes="[20, 50, 100, 200]"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="loadRecharges"
          @current-change="loadRecharges"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Refresh } from '@element-plus/icons-vue'
import { t } from '../i18n'
import { getMemberBalanceLogs } from '../api'

const dateRange = ref([])
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)
const recharges = ref([])

const stats = reactive({
  totalCount: 0,
  totalAmount: 0,
  avgAmount: 0,
  totalBonus: 0,
})

const formatTime = (ts) => {
  if (!ts) return '-'
  return new Date(ts).toLocaleString('zh-CN', { 
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const getLevelColor = (level) => {
  const colors = {
    '普通会员': '#909399',
    '白银会员': '#909399',
    '黄金会员': '#e6a23c',
    '白金会员': '#409eff',
    '钻石会员': '#a855f7',
  }
  return colors[level] || '#909399'
}

const getPaymentType = (method) => {
  const types = {
    cash: 'success',
    wechat: 'primary',
    alipay: 'warning',
    card: 'info',
  }
  return types[method] || 'info'
}

const getPaymentLabel = (method) => {
  const labels = {
    cash: t('cash'),
    wechat: t('wechatPay'),
    alipay: t('alipay'),
    card: t('memberCard'),
  }
  return labels[method] || method || t('unknown')
}

const loadRecharges = async () => {
  try {
    const logs = await getMemberBalanceLogs(0)
    const rechargeLogs = (logs || []).filter(log => 
      log.type === 'recharge' || 
      (log.description && log.description.includes(t('recharge')))
    )
    
    recharges.value = rechargeLogs.map(log => ({
      ...log,
      member_name: log.member_name || t('unknown'),
      amount: log.amount || 0,
      bonus: log.bonus || 0,
    }))
    
    stats.totalCount = recharges.value.length
    stats.totalAmount = recharges.value.reduce((sum, r) => sum + (r.amount || 0), 0)
    stats.avgAmount = stats.totalCount > 0 ? stats.totalAmount / stats.totalCount : 0
    stats.totalBonus = recharges.value.reduce((sum, r) => sum + (r.bonus || 0), 0)
    total.value = recharges.value.length
  } catch (e) {
    console.error(e)
    ElMessage.error(t('loadFailed'))
  }
}

onMounted(() => {
  loadRecharges()
})
</script>

<style scoped>
.member-recharge-page {
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
  align-items: center;
}

.stats-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  text-align: center;
  border: 1px solid var(--border-color);
}

.stat-value {
  font-size: 28px;
  font-weight: 600;
  color: var(--accent-primary);
  margin-bottom: 8px;
}

.stat-label {
  font-size: 14px;
  color: var(--text-secondary);
}

.recharges-table {
  background: var(--card-bg);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.member-cell {
  display: flex;
  align-items: center;
  gap: 10px;
}

.member-info {
  display: flex;
  flex-direction: column;
}

.member-name {
  font-weight: 500;
}

.member-phone {
  font-size: 12px;
  color: var(--text-secondary);
}

.amount {
  color: var(--accent-success);
  font-weight: 600;
}

.bonus {
  color: var(--accent-warning);
  font-weight: 500;
}

.pagination-wrapper {
  padding: 16px;
  display: flex;
  justify-content: flex-end;
}
</style>