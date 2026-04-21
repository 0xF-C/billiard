<template>
  <div class="recharge-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('recharge') }}</h1>
    </div>

    <div class="main-grid">
      <!-- Left: Member Selection -->
      <div class="member-panel">
        <div class="panel-header">
          <h3>{{ t('selectMember') }}</h3>
          <el-input
            v-model="searchKw"
            :placeholder="t('search') + ' ' + t('phone')"
            class="search-input"
            clearable
            @input="searchMembers"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
        </div>

        <div class="member-list">
          <div
            v-for="m in memberList"
            :key="m.id"
            class="member-item"
            :class="{ active: selectedMember?.id === m.id }"
            @click="selectMember(m)"
          >
            <el-avatar :size="40" :style="{ background: getLevelColor(m.level) }">
              {{ m.name?.charAt(0) }}
            </el-avatar>
            <div class="member-info">
              <div class="member-name">
                {{ m.name }}
                <el-tag size="small" :type="getLevelType(m.level)">{{ m.level }}</el-tag>
              </div>
              <div class="member-meta">
                <span>{{ m.phone }}</span>
                <span class="balance">¥{{ m.balance?.toFixed(2) }}</span>
              </div>
            </div>
          </div>
          <el-empty v-if="memberList.length === 0" :description="t('noData')" />
        </div>
      </div>

      <!-- Right: Recharge Panel -->
      <div class="recharge-panel">
        <template v-if="selectedMember">
          <div class="member-card">
            <div class="card-header">
              <el-avatar :size="56" :style="{ background: getLevelColor(selectedMember.level) }">
                {{ selectedMember.name?.charAt(0) }}
              </el-avatar>
              <div class="card-info">
                <div class="card-name">{{ selectedMember.name }}</div>
                <div class="card-level">
                  <el-tag :type="getLevelType(selectedMember.level)">{{ selectedMember.level }}</el-tag>
                  <span class="discount">{{ (selectedMember.discount * 10).toFixed(1) }}折</span>
                </div>
              </div>
            </div>
            <div class="card-stats">
              <div class="stat-item">
                <span class="stat-label">{{ t('currentBalance') }}</span>
                <span class="stat-value">¥{{ selectedMember.balance?.toFixed(2) }}</span>
              </div>
              <div class="stat-item">
                <span class="stat-label">{{ t('totalSpent') }}</span>
                <span class="stat-value">¥{{ selectedMember.total_spent?.toFixed(2) }}</span>
              </div>
            </div>
          </div>

          <div class="amount-section">
            <h3>{{ t('rechargeAmount') }}</h3>
            <div class="quick-amounts">
              <div
                v-for="amt in quickAmounts"
                :key="amt"
                class="amount-btn"
                :class="{ active: amount === amt }"
                @click="amount = amt"
              >
                ¥{{ amt }}
              </div>
            </div>
            <el-input
              v-model="amount"
              type="number"
              :placeholder="t('inputAmount')"
              class="amount-input"
            >
              <template #prefix>¥</template>
            </el-input>
          </div>

          <div class="bonus-section" v-if="bonusRules.length > 0">
            <h3>{{ t('bonusRule') }}</h3>
            <div class="bonus-rules">
              <div
                v-for="rule in bonusRules"
                :key="rule.id"
                class="bonus-rule"
                :class="{ active: selectedBonus?.id === rule.id }"
                @click="selectBonus(rule)"
              >
                <span class="rule-name">{{ rule.name }}</span>
                <span class="rule-desc">充{{ rule.min_amount }}送{{ rule.bonus }}</span>
              </div>
            </div>
          </div>

          <div class="payment-section">
            <h3>{{ t('paymentMethod') }}</h3>
            <el-radio-group v-model="paymentMethod" class="payment-methods">
              <el-radio-button value="cash">{{ t('cash') }}</el-radio-button>
              <el-radio-button value="wechat">{{ t('wechat') }}</el-radio-button>
              <el-radio-button value="alipay">{{ t('alipay') }}</el-radio-button>
              <el-radio-button value="card">{{ t('card') }}</el-radio-button>
            </el-radio-group>
          </div>

          <div class="summary">
            <div class="summary-row">
              <span>{{ t('rechargeAmount') }}</span>
              <span class="amount">¥{{ amount || 0 }}</span>
            </div>
            <div class="summary-row" v-if="selectedBonus">
              <span>{{ t('bonusAmount') }}</span>
              <span class="bonus">+¥{{ selectedBonus.bonus }}</span>
            </div>
            <div class="summary-row total">
              <span>{{ t('totalToAccount') }}</span>
              <span class="total-amount">¥{{ totalAmount }}</span>
            </div>
          </div>

          <el-button
            type="primary"
            size="large"
            class="submit-btn"
            :loading="loading"
            :disabled="!amount || amount <= 0"
            @click="submit"
          >
            {{ t('confirmRecharge') }}
          </el-button>
        </template>

        <el-empty v-else :description="t('selectMemberToRecharge')" />
      </div>
    </div>

    <!-- Recharge Records -->
    <div class="records-section">
      <div class="section-header">
        <h3>{{ t('recentRecords') }}</h3>
      </div>
      <el-table :data="recentRecords" stripe>
        <el-table-column prop="created_at" :label="t('time')" width="180">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column prop="member_name" :label="t('member')" width="120" />
        <el-table-column :label="t('amount')" width="120">
          <template #default="{ row }">
            <span class="recharge-amount">+¥{{ row.amount }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('bonus')" width="100">
          <template #default="{ row }">
            <span v-if="row.bonus > 0" class="bonus-amount">+¥{{ row.bonus }}</span>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column prop="payment_method" :label="t('paymentMethod')" width="100">
          <template #default="{ row }">
            {{ t(row.payment_method) }}
          </template>
        </el-table-column>
        <el-table-column prop="operator" :label="t('operator')" width="100" />
      </el-table>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search } from '@element-plus/icons-vue'
import { getMembers, rechargeMember, getMemberBalanceLogs } from '../api'
import { t } from '../i18n'


const searchKw = ref('')
const memberList = ref([])
const selectedMember = ref(null)
const amount = ref(100)
const paymentMethod = ref('cash')
const selectedBonus = ref(null)
const loading = ref(false)
const recentRecords = ref([])

const quickAmounts = [50, 100, 200, 300, 500, 1000]

// 模拟赠送规则（实际应从后端获取）
const bonusRules = ref([
  { id: 1, name: '充100送10', min_amount: 100, bonus: 10 },
  { id: 2, name: '充300送50', min_amount: 300, bonus: 50 },
  { id: 3, name: '充500送100', min_amount: 500, bonus: 100 },
  { id: 4, name: '充1000送300', min_amount: 1000, bonus: 300 },
])

const totalAmount = computed(() => {
  const base = parseFloat(amount.value) || 0
  const bonus = selectedBonus.value?.bonus || 0
  return (base + bonus).toFixed(2)
})

let searchTimer = null
const searchMembers = () => {
  clearTimeout(searchTimer)
  searchTimer = setTimeout(async () => {
    const res = await getMembers(searchKw.value)
    memberList.value = res || []
  }, 300)
}

const selectMember = (m) => {
  selectedMember.value = m
}

const selectBonus = (rule) => {
  if (amount.value >= rule.min_amount) {
    selectedBonus.value = rule
  } else {
    ElMessage.warning(`充值金额需满${rule.min_amount}元`)
  }
}

const getLevelColor = (level) => {
  const colors = {
    '普通会员': '#909399',
    '银卡会员': '#909399',
    '金卡会员': '#e6a23c',
    '白金会员': '#409eff',
    '钻石会员': '#a855f7',
  }
  return colors[level] || '#909399'
}

const getLevelType = (level) => {
  const types = {
    '普通会员': 'info',
    '银卡会员': 'info',
    '金卡会员': 'warning',
    '白金会员': 'primary',
    '钻石会员': 'danger',
  }
  return types[level] || 'info'
}

const formatTime = (ts) => {
  if (!ts) return '-'
  const d = new Date(ts)
  return d.toLocaleString('zh-CN')
}

const submit = async () => {
  if (!selectedMember.value) {
    ElMessage.warning(t('selectMember'))
    return
  }
  if (!amount.value || amount.value <= 0) {
    ElMessage.warning(t('inputValidAmount'))
    return
  }

  try {
    await ElMessageBox.confirm(
      `确认向 ${selectedMember.value.name} 充值 ¥${amount.value}？${selectedBonus.value ? `\n赠送 ¥${selectedBonus.value.bonus}` : ''}`,
      t('confirm'),
      { type: 'info' }
    )

    loading.value = true
    const res = await rechargeMember(selectedMember.value.id, parseFloat(amount.value), paymentMethod.value)
    if (res.success !== false) {
      ElMessage.success(t('rechargeSuccess'))
      // 更新会员余额显示
      selectedMember.value.balance = (selectedMember.value.balance || 0) + parseFloat(amount.value) + (selectedBonus.value?.bonus || 0)
      // 刷新记录
      loadRecentRecords()
      // 重置
      amount.value = 100
      selectedBonus.value = null
    }
  } catch (e) {
    if (e !== 'cancel') console.error(e)
  } finally {
    loading.value = false
  }
}

const loadRecentRecords = async () => {
  // 这里应该有一个专门的获取充值记录的API
  // 暂时用余额日志代替
  recentRecords.value = []
}

onMounted(() => {
  searchMembers()
})
</script>

<style scoped>
.recharge-page {
  padding: 24px;
}

.page-header {
  margin-bottom: 24px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
}

.main-grid {
  display: grid;
  grid-template-columns: 340px 1fr;
  gap: 24px;
  margin-bottom: 24px;
}

.member-panel {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.panel-header h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
}

.search-input {
  margin-bottom: 16px;
}

.member-list {
  max-height: 400px;
  overflow-y: auto;
}

.member-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.member-item:hover {
  background: var(--hover-bg);
}

.member-item.active {
  background: var(--primary-light);
  border: 1px solid var(--primary-color);
}

.member-info {
  flex: 1;
}

.member-name {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 500;
}

.member-meta {
  display: flex;
  justify-content: space-between;
  margin-top: 4px;
  font-size: 13px;
  color: var(--text-secondary);
}

.balance {
  color: var(--accent-success);
  font-weight: 500;
}

.recharge-panel {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 24px;
  border: 1px solid var(--border-color);
}

.member-card {
  background: linear-gradient(135deg, var(--primary-color), #6366f1);
  border-radius: 12px;
  padding: 20px;
  color: #fff;
  margin-bottom: 24px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
}

.card-name {
  font-size: 18px;
  font-weight: 600;
}

.card-level {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
}

.discount {
  font-size: 13px;
  opacity: 0.9;
}

.card-stats {
  display: flex;
  gap: 32px;
}

.stat-item {
  display: flex;
  flex-direction: column;
}

.stat-label {
  font-size: 12px;
  opacity: 0.8;
}

.stat-value {
  font-size: 20px;
  font-weight: 600;
}

.amount-section h3,
.bonus-section h3,
.payment-section h3 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
}

.quick-amounts {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 16px;
}

.amount-btn {
  padding: 10px 20px;
  border-radius: 8px;
  background: var(--hover-bg);
  border: 1px solid var(--border-color);
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
}

.amount-btn:hover {
  border-color: var(--primary-color);
}

.amount-btn.active {
  background: var(--primary-color);
  color: #fff;
  border-color: var(--primary-color);
}

.amount-input {
  margin-bottom: 24px;
}

.bonus-rules {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 24px;
}

.bonus-rule {
  padding: 10px 16px;
  border-radius: 8px;
  background: var(--hover-bg);
  border: 1px solid var(--border-color);
  cursor: pointer;
  transition: all 0.2s;
}

.bonus-rule:hover {
  border-color: var(--accent-warning);
}

.bonus-rule.active {
  background: rgba(229, 165, 48, 0.1);
  border-color: var(--accent-warning);
}

.rule-name {
  font-weight: 500;
  margin-right: 8px;
}

.rule-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

.payment-methods {
  margin-bottom: 24px;
}

.summary {
  background: var(--hover-bg);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 24px;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  font-size: 14px;
}

.summary-row.total {
  border-top: 1px dashed var(--border-color);
  margin-top: 8px;
  padding-top: 16px;
  font-size: 16px;
  font-weight: 600;
}

.total-amount {
  color: var(--accent-success);
  font-size: 20px;
}

.bonus {
  color: var(--accent-warning);
}

.submit-btn {
  width: 100%;
  height: 48px;
  font-size: 16px;
}

.records-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.section-header {
  margin-bottom: 16px;
}

.section-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.recharge-amount {
  color: var(--accent-success);
  font-weight: 500;
}

.bonus-amount {
  color: var(--accent-warning);
}
</style>
