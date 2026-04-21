<template>
  <div class="members-page">
    <div class="page-header">
      <div class="page-title-row">
        <h1 class="page-title">{{ t('members') }}</h1>
        <div class="header-actions">
          <el-input
            v-model="kw"
            :placeholder="t('search') + ' ' + t('name') + '/' + t('phone')"
            class="search-input"
            clearable
            @keyup.enter="search"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
          <el-button type="primary" :icon="Plus" @click="showAdd" class="add-btn">{{ t('addMember') }}</el-button>
        </div>
      </div>
    </div>

    <div class="stat-grid">
      <div class="stat-card" style="--accent: var(--accent-primary);">
        <div class="stat-icon-wrap" style="background: rgba(88,166,255,0.1);">
          <el-icon class="stat-icon" style="color: var(--accent-primary);"><User /></el-icon>
        </div>
        <div class="stat-body">
          <div class="stat-value">{{ members.length }}</div>
          <div class="stat-label">{{ t('memberCount') }}</div>
        </div>
      </div>
      <div class="stat-card" style="--accent: var(--accent-warning);">
        <div class="stat-icon-wrap" style="background: rgba(229,165,48,0.1);">
          <el-icon class="stat-icon" style="color: var(--accent-warning);"><Money /></el-icon>
        </div>
        <div class="stat-body">
          <div class="stat-value">¥{{ totalSpent.toFixed(0) }}</div>
          <div class="stat-label">{{ t('totalConsumption') }}</div>
        </div>
      </div>
      <div class="stat-card" style="--accent: var(--accent-success);">
        <div class="stat-icon-wrap" style="background: rgba(63,185,80,0.1);">
          <el-icon class="stat-icon" style="color: var(--accent-success);"><Wallet /></el-icon>
        </div>
        <div class="stat-body">
          <div class="stat-value">¥{{ totalBalance.toFixed(0) }}</div>
          <div class="stat-label">{{ t('balance') }}</div>
        </div>
      </div>
      <div class="stat-card" style="--accent: var(--accent-purple);">
        <div class="stat-icon-wrap" style="background: rgba(163,113,247,0.1);">
          <el-icon class="stat-icon" style="color: var(--accent-purple);"><Calendar /></el-icon>
        </div>
        <div class="stat-body">
          <div class="stat-value">{{ totalVisits }}</div>
          <div class="stat-label">{{ t('visitCount') }}</div>
        </div>
      </div>
    </div>

    <div class="members-list">
      <el-table :data="members" stripe style="width: 100%" :row-class-name="tableRowClass">
        <el-table-column label="" width="52">
          <template #default="{ row }">
            <el-avatar :size="36" :style="{ background: getAvatarColor(row.level) }">
              {{ row.name?.charAt(0) }}
            </el-avatar>
          </template>
        </el-table-column>
        <el-table-column prop="name" :label="t('name')" min-width="120">
          <template #default="{ row }">
            <div class="member-name-cell">
              <span class="member-real-name">{{ row.name }}</span>
              <el-tag :type="getLevelType(row.level)" size="small" class="level-tag">{{ row.level }}</el-tag>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="phone" :label="t('phone')" min-width="110">
          <template #default="{ row }">
            <span class="phone-cell">{{ row.phone }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('balance')" min-width="90">
          <template #default="{ row }">
            <span class="balance-cell" :class="{ 'low-balance': row.balance < 50 }">¥{{ row.balance?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('discount')" min-width="70">
          <template #default="{ row }">
            <span class="discount-cell">{{ (row.discount * 10).toFixed(1) }}{{ t('discountSuffix') }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('totalSpent')" min-width="90">
          <template #default="{ row }">
            <span class="spent-cell">¥{{ row.total_spent?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="visit_count" :label="t('visitCount')" min-width="58">
          <template #default="{ row }">
            <span class="visit-cell">{{ row.visit_count || 0 }}{{ t('times') }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" min-width="190" fixed="right">
          <template #default="{ row }">
            <div class="action-btns">
              <el-button size="small" type="success" @click="doRecharge(row)" class="action-btn recharge-btn">
                <el-icon><Wallet /></el-icon>
                {{ t('recharge') }}
              </el-button>
              <el-button size="small" @click="showHistory(row)" class="action-btn" title="{{ t('balanceTrace') }}">
                <el-icon><Document /></el-icon>
              </el-button>
              <el-button size="small" @click="doEdit(row)" class="action-btn edit-btn" circle>
                <el-icon><Edit /></el-icon>
              </el-button>
              <el-button size="small" type="danger" plain @click="doDelete(row)" class="action-btn delete-btn" circle>
                <el-icon><Delete /></el-icon>
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>
      <div v-if="members.length === 0 && !loading" class="empty-state">
        <div class="empty-icon">
          <el-icon :size="56"><UserFilled /></el-icon>
        </div>
        <div class="empty-text">{{ t('noMembers') }}</div>
        <div class="empty-hint">{{ t('clickAddFirstMember') }}</div>
      </div>
    </div>

    <!-- Add/Edit Dialog -->
    <el-dialog v-model="formDlg" :title="editTarget ? t('editMember') : t('addMember')" width="560px" :close-on-click-modal="false">
      <div class="form-body">
        <div class="form-row">
          <el-form-item :label="t('name')" required>
            <el-input v-model="form.name" :placeholder="t('name')" />
          </el-form-item>
          <el-form-item :label="t('phone')" required>
            <el-input v-model="form.phone" :placeholder="t('phone')" />
          </el-form-item>
        </div>

        <div class="form-row">
          <el-form-item :label="t('gender')">
            <el-select v-model="form.gender" class="full-width">
              <el-option :label="t('male')" value="男" />
              <el-option :label="t('female')" value="女" />
              <el-option :label="t('unknown')" value="未知" />
            </el-select>
          </el-form-item>
          <el-form-item :label="t('birthday')">
            <el-date-picker v-model="form.birthday" type="date" value-format="YYYY-MM-DD" class="full-width" />
          </el-form-item>
        </div>

        <div class="form-row">
          <el-form-item :label="t('memberLevel')">
            <el-select v-model="form.level" class="full-width" @change="onLevelChange">
              <el-option :label="t('normalMember')" value="普通会员" />
              <el-option :label="t('silverMember')" value="银卡会员" />
              <el-option :label="t('goldMember')" value="金卡会员" />
              <el-option :label="t('vipMember')" value="VIP会员" />
              <el-option :label="t('diamondMember')" value="钻石会员" />
            </el-select>
          </el-form-item>
          <el-form-item :label="t('discount')">
            <div class="discount-display">
              <div class="discount-bar">
                <div class="discount-fill" :class="form.discount >= 0.9 ? 'high' : form.discount >= 0.8 ? 'mid' : 'low'" :style="{ width: (form.discount * 100) + '%' }"></div>
              </div>
              <span class="discount-text">{{ (form.discount * 10).toFixed(1) }}{{ t('discount') }}</span>
            </div>
          </el-form-item>
        </div>

        <el-form-item :label="t('email')">
          <el-input v-model="form.email" :placeholder="t('optional')" />
        </el-form-item>

        <el-form-item :label="t('address')">
          <el-input v-model="form.address" :placeholder="t('optional')" />
        </el-form-item>

        <el-form-item :label="t('remark')">
          <el-input v-model="form.remark" type="textarea" :rows="2" :placeholder="t('optional')" />
        </el-form-item>
      </div>
      <template #footer>
        <el-button @click="formDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="cfmForm">
          <el-icon><Check /></el-icon>
          {{ t('save') }}
        </el-button>
      </template>
    </el-dialog>

    <!-- Recharge Dialog -->
    <el-dialog v-model="rechDlg" :title="t('recharge')" width="420px" :close-on-click-modal="false">
      <div class="recharge-content" v-if="rechTarget">
        <el-card class="member-info-card">
          <div class="member-info-header">
            <el-avatar :size="48" :style="{ background: getAvatarColor(rechTarget.level) }">
              {{ rechTarget.name?.charAt(0) }}
            </el-avatar>
            <div class="member-info-body">
              <div class="member-info-name">{{ rechTarget.name }}</div>
              <div class="member-info-phone">{{ rechTarget.phone }}</div>
            </div>
            <div class="member-info-balance">
              <span class="balance-label">{{ t('balance') }}</span>
              <span class="balance-amount">¥{{ rechTarget.balance?.toFixed(2) }}</span>
            </div>
          </div>
        </el-card>

        <div class="quick-amounts">
          <span class="amount-label">{{ t('recharge') }}</span>
          <div class="amount-btns">
            <el-button
              v-for="v in [50, 100, 200, 500, 1000]"
              :key="v"
              :class="['amount-btn', { active: rechAmt === v }]"
              @click="rechAmt = v"
            >
              ¥{{ v }}
            </el-button>
          </div>
        </div>

        <el-input-number v-model="rechAmt" :min="1" :max="99999" :step="10" class="amount-input" />

        <div v-if="rechAmt > 0" class="recharge-tip">
          {{ t('rechargeAfterBalance') }}：<span class="new-balance">¥{{ ((rechTarget.balance || 0) + rechAmt).toFixed(2) }}</span>
        </div>
      </div>
      <template #footer>
        <el-button @click="rechDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="success" @click="cfmRech">
          <el-icon><Wallet /></el-icon>
          {{ t('confirmRecharge') }} ¥{{ rechAmt }}
        </el-button>
      </template>
    </el-dialog>

    <!-- Balance Trace Dialog -->
    <el-dialog v-model="historyDlg" :title="t('balanceHistory') + ' - ' + (historyMember?.name || '')" width="720px" :close-on-click-modal="false">
      <div class="history-content" v-if="historyMember">
        <div class="history-header-card">
          <div class="history-header-left">
            <el-avatar :size="44" :style="{ background: getAvatarColor(historyMember.level) }">
              {{ historyMember.name?.charAt(0) }}
            </el-avatar>
            <div class="history-member-info">
              <div class="history-member-name">{{ historyMember.name }}</div>
              <div class="history-member-phone">{{ historyMember.phone }}</div>
            </div>
          </div>
          <div class="history-header-right">
            <div class="history-balance-block">
              <span class="history-balance-label">{{ t('balance') }}</span>
              <span class="history-balance-value">¥{{ historyMember.balance?.toFixed(2) }}</span>
            </div>
          </div>
        </div>

        <el-table :data="historyLogs" stripe style="width: 100%" max-height="400">
          <el-table-column :label="t('time')" width="170">
            <template #default="{ row }">
              <span class="history-time">{{ row.created_at }}</span>
            </template>
          </el-table-column>
          <el-table-column :label="t('type')" width="100">
            <template #default="{ row }">
              <span :class="['history-type-badge', row.reason]">{{ reasonText(row.reason) }}</span>
            </template>
          </el-table-column>
          <el-table-column :label="t('change')" width="120">
            <template #default="{ row }">
              <span class="history-amount" :class="row.amount > 0 ? 'positive' : 'negative'">
                {{ row.amount > 0 ? '+' : '' }}¥{{ row.amount?.toFixed(2) }}
              </span>
            </template>
          </el-table-column>
          <el-table-column :label="t('balanceBefore')" width="110">
            <template #default="{ row }">
              <span class="history-balance-cell">¥{{ row.balance_before?.toFixed(2) }}</span>
            </template>
          </el-table-column>
          <el-table-column :label="t('balanceAfter')" width="110">
            <template #default="{ row }">
              <span class="history-balance-cell">¥{{ row.balance_after?.toFixed(2) }}</span>
            </template>
          </el-table-column>
        </el-table>

        <div v-if="historyLogs.length === 0 && !historyLoading" class="history-empty">
          <el-icon :size="40" style="color: var(--text-tertiary);"><Document /></el-icon>
          <span>{{ t('noRecords') }}</span>
        </div>

        <div v-if="historyTotal > historyLogs.length" class="history-load-more">
          <el-button size="small" @click="loadMoreLogs" :loading="historyLoading">{{ t('loadMore') || '加载更多' }}</el-button>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox, ElButton, ElInput, ElDialog, ElForm, ElFormItem, ElAvatar, ElTag, ElCard, ElInputNumber, ElSlider, ElSelect, ElOption, ElDatePicker, ElIcon, ElTable, ElTableColumn } from 'element-plus'
import { Search, Plus, User, Wallet, Edit, Delete, Calendar, Location, UserFilled, Check, Money, Document } from '@element-plus/icons-vue'
import { getMembers, createMember, updateMember, deleteMember, rechargeMember, getMemberBalanceLogs } from '../api'
import { t } from '../i18n'

const members = ref([])
const loading = ref(false)
const kw = ref('')
const formDlg = ref(false)
const rechDlg = ref(false)
const editTarget = ref(null)
const rechTarget = ref(null)
const rechAmt = ref(100)

const historyDlg = ref(false)
const historyMember = ref(null)
const historyLogs = ref([])
const historyTotal = ref(0)
const historyPage = ref(1)
const historyLoading = ref(false)

const form = ref({
  name: '',
  phone: '',
  gender: '未知',
  birthday: '',
  level: '普通会员',
  discount: 1.0,
  email: '',
  address: '',
  remark: ''
})

const totalSpent = computed(() => {
  return members.value.reduce((sum, m) => sum + (m.total_spent || 0), 0)
})

const totalBalance = computed(() => {
  return members.value.reduce((sum, m) => sum + (m.balance || 0), 0)
})

const totalVisits = computed(() => {
  return members.value.reduce((sum, m) => sum + (m.visit_count || 0), 0)
})

const tableRowClass = ({ row }) => {
  return ''
}

const getAvatarColor = (level) => {
  const colors = {
    '普通会员': 'var(--accent-primary)',
    '银卡会员': '#909399',
    '金卡会员': '#e6a23c',
    'VIP会员': '#9c27b0',
    '钻石会员': '#f56c6c'
  }
  return colors[level] || 'var(--accent-primary)'
}

const getLevelType = (level) => {
  const types = {
    '普通会员': 'info',
    '银卡会员': '',
    '金卡会员': 'warning',
    'VIP会员': 'danger',
    '钻石会员': ''
  }
  return types[level] || 'info'
}

const search = async () => {
  loading.value = true
  try {
    members.value = await getMembers(kw.value || '')
  } catch {
    ElMessage.error(t('loadFailed'))
  }
  loading.value = false
}

const doEdit = (m) => {
  editTarget.value = m
  form.value = {
    name: m.name,
    phone: m.phone,
    gender: m.gender || '未知',
    birthday: m.birthday || '',
    level: m.level || '普通会员',
    discount: m.discount || 1.0,
    email: m.email || '',
    address: m.address || '',
    remark: m.remark || ''
  }
  formDlg.value = true
}

const levelDiscounts = {
  '普通会员': 1.0,
  '银卡会员': 0.95,
  '金卡会员': 0.9,
  'VIP会员': 0.85,
  '钻石会员': 0.8
}

const onLevelChange = (level) => {
  if (levelDiscounts[level] !== undefined) {
    form.value.discount = levelDiscounts[level]
  }
}

const showAdd = () => {
  editTarget.value = null
  form.value = {
    name: '',
    phone: '',
    gender: '未知',
    birthday: '',
    level: '普通会员',
    discount: 1.0,
    email: '',
    address: '',
    remark: ''
  }
  formDlg.value = true
}

const cfmForm = async () => {
  if (!form.value.name || !form.value.phone) return ElMessage.warning('请填写姓名和手机号')
  try {
    if (editTarget.value) {
      await updateMember(editTarget.value.id, form.value)
      ElMessage.success(t('updateSuccess'))
    } else {
      await createMember(form.value.name, form.value.phone, form.value.gender, form.value.discount)
      ElMessage.success(t('createSuccess'))
    }
    formDlg.value = false
    await search()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const doDelete = async (m) => {
  try {
    await ElMessageBox.confirm(`${t('deleteMemberConfirm')}「${m.name}」${t('confirm')}？`, t('warning'), { type: 'warning' })
    await deleteMember(m.id)
    ElMessage.success(t('deleteSuccess'))
    await search()
  } catch(e) { if (e !== 'cancel') ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const reasonText = (reason) => {
  const map = { recharge: t('reasonRecharge'), order: t('reasonOrder'), cancel: t('reasonCancel'), realtime_deduction: t('reasonRealtime') }
  return map[reason] || reason
}

const showHistory = async (member) => {
  historyMember.value = member
  historyLogs.value = []
  historyPage.value = 1
  historyTotal.value = 0
  historyDlg.value = true
  historyLoading.value = true
  try {
    const res = await getMemberBalanceLogs(member.id, { page: 1, per_page: 20 })
    historyLogs.value = res.items || []
    historyTotal.value = res.total || 0
    historyPage.value = 1
  } catch { historyLogs.value = [] }
  historyLoading.value = false
}

const loadMoreLogs = async () => {
  historyLoading.value = true
  historyPage.value++
  try {
    const res = await getMemberBalanceLogs(historyMember.value.id, { page: historyPage.value, per_page: 20 })
    historyLogs.value = [...historyLogs.value, ...(res.items || [])]
    historyTotal.value = res.total || 0
  } catch {}
  historyLoading.value = false
}

const doRecharge = (m) => { rechTarget.value = m; rechAmt.value = 100; rechDlg.value = true }

const cfmRech = async () => {
  try {
    await rechargeMember(rechTarget.value.id, rechAmt.value, '余额')
    ElMessage.success(`${t('rechargeSuccess')} ¥${rechAmt.value}`)
    rechDlg.value = false
    await search()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

onMounted(search)
</script>

<style scoped>
.members-page {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.page-header {
  margin-bottom: 4px;
}

.page-title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 16px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}

.search-input { width: 260px; }

.add-btn {
  font-weight: 500;
  border-radius: var(--radius-sm);
}

/* Stat Grid */
.stat-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}

.stat-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  display: flex;
  align-items: flex-start;
  gap: 16px;
  transition: all var(--transition-normal);
  box-shadow: var(--shadow-card);
}

.stat-card:hover {
  border-color: var(--accent);
  box-shadow: var(--shadow-card-hover);
  transform: translateY(-2px);
}

.stat-icon-wrap {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: var(--shadow-sm);
}

.stat-icon { font-size: 22px; }

.stat-body { flex: 1; }

.stat-value {
  font-size: 26px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
  margin-top: 4px;
}

/* Members List */
.members-list {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  overflow: hidden;
  box-shadow: var(--shadow-card);
}

.member-name-cell {
  display: flex;
  align-items: center;
  gap: 8px;
}

.member-real-name {
  font-weight: 600;
  color: var(--text-primary);
}

.level-tag {
  border-radius: 10px;
}

.phone-cell {
  font-family: var(--font-mono);
  font-size: 13px;
  color: var(--text-secondary);
  letter-spacing: 0.5px;
}

.balance-cell {
  font-weight: 600;
  color: var(--accent-success);
}

.balance-cell.low-balance {
  color: var(--accent-danger);
}

.discount-cell {
  font-size: 13px;
  padding: 2px 8px;
  background: rgba(88,166,255,0.08);
  border-radius: 10px;
  color: var(--accent-primary);
  font-weight: 500;
}

.spent-cell {
  color: var(--accent-warning);
  font-weight: 500;
}

.visit-cell {
  color: var(--text-secondary);
  font-size: 13px;
}

.action-btns {
  display: flex;
  align-items: center;
  gap: 4px;
}

.action-btn.recharge-btn {
  border-radius: var(--radius-sm);
  font-weight: 500;
}

/* Empty State */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  gap: 12px;
}

.empty-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-tertiary);
}

.empty-text {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-secondary);
}

.empty-hint {
  font-size: 13px;
  color: var(--text-tertiary);
}

/* Table deep styles */
:deep(.el-table) {
  --el-table-bg-color: transparent;
  --el-table-tr-bg-color: transparent;
  --el-table-header-bg-color: var(--bg-tertiary);
  --el-table-row-hover-bg-color: var(--bg-hover);
  --el-table-header-text-color: var(--text-secondary);
  --el-table-text-color: var(--text-primary);
  --el-table-border-color: var(--border-muted);
  --el-table-header-bg-color: var(--bg-tertiary);
}

:deep(.el-table th.el-table__cell) {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary) !important;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  padding: 14px 0;
  background: var(--bg-tertiary) !important;
}

:deep(.el-table td.el-table__cell) {
  padding: 12px 0;
}

:deep(.el-table--striped .el-table__body tr.el-table__row--striped td.el-table__cell) {
  background: rgba(255,255,255,0.015);
}

:deep(.el-table__body tr:hover > td.el-table__cell) {
  background: var(--bg-hover) !important;
}

:deep(.el-table__fixed-right) {
  background: var(--bg-secondary) !important;
}

:deep(.el-table__fixed-right-patch) {
  background: var(--bg-tertiary) !important;
}

:deep(.el-table__fixed-right .el-table__cell) {
  background: var(--bg-secondary) !important;
}

:deep(.el-table--striped .el-table__fixed-right tr.el-table__row--striped .el-table__cell) {
  background: rgba(255,255,255,0.015) !important;
}

:deep(.el-table__fixed-right tr:hover > .el-table__cell) {
  background: var(--bg-hover) !important;
}

:deep(.el-table__fixed-right .el-table__header-wrapper th.el-table__cell) {
  background: var(--bg-tertiary) !important;
}

:deep(.el-table__fixed-right::before) {
  box-shadow: -4px 0 12px rgba(0,0,0,0.2);
}

:deep(.el-table .el-table__cell.is-right-penultimate) {
  border-right: none;
}

:deep(.el-table__fixed-right th.el-table__cell .cell) {
  text-align: center;
}

:deep(.el-table__fixed-right td.el-table__cell .cell) {
  display: flex;
  justify-content: center;
}

/* Form Dialog */
.form-body {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.full-width { width: 100%; }

.discount-display {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 100%;
}

.discount-bar {
  width: 100%;
  height: 6px;
  background: var(--bg-primary);
  border-radius: 3px;
  overflow: hidden;
}

.discount-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.3s ease;
}

.discount-fill.low { background: var(--accent-danger); }
.discount-fill.mid { background: var(--accent-warning); }
.discount-fill.high { background: var(--accent-success); }

.discount-text {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

:deep(.el-form-item__label) {
  color: var(--text-secondary) !important;
  font-weight: 500 !important;
  padding-bottom: 8px !important;
}

:deep(.el-dialog) {
  background: var(--bg-secondary) !important;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg) !important;
}

:deep(.el-dialog__header) {
  border-bottom: 1px solid var(--border-default);
  padding: 16px 20px;
}

:deep(.el-dialog__title) {
  color: var(--text-primary);
  font-weight: 600;
}

:deep(.el-dialog__body) {
  padding: 20px;
}

/* Recharge Dialog */
.recharge-content { display: flex; flex-direction: column; gap: 20px; }

.member-info-card { 
  background: var(--bg-tertiary) !important; 
  border: 1px solid var(--border-default) !important;
  border-radius: var(--radius-md) !important;
}

:deep(.member-info-card .el-card__body) {
  padding: 16px !important;
}

.member-info-header { display: flex; align-items: center; gap: 14px; }

.member-info-body { flex: 1; }
.member-info-name { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.member-info-phone { font-size: 13px; color: var(--text-secondary); }

.member-info-balance { text-align: right; }
.balance-label { display: block; font-size: 11px; color: var(--text-secondary); }
.balance-amount { font-size: 22px; font-weight: 700; color: var(--accent-success); }

.quick-amounts { display: flex; flex-direction: column; gap: 10px; }

.amount-label { font-size: 13px; color: var(--text-secondary); font-weight: 500; }

.amount-btns { display: flex; gap: 8px; }

.amount-btn {
  flex: 1;
  padding: 12px 8px;
  border: 1px solid var(--border-default) !important;
  background: var(--bg-tertiary) !important;
  color: var(--text-secondary) !important;
  font-weight: 600;
  transition: all var(--transition-fast);
  border-radius: var(--radius-sm) !important;
}

.amount-btn:hover {
  border-color: var(--accent-success) !important;
  color: var(--accent-success) !important;
  background: rgba(63,185,80,0.08) !important;
}

.amount-btn.active {
  background: rgba(63,185,80,0.12) !important;
  border-color: var(--accent-success) !important;
  color: var(--accent-success) !important;
  box-shadow: 0 0 0 1px var(--accent-success);
}

.amount-input { width: 100%; }

.recharge-tip {
  text-align: center;
  font-size: 14px;
  color: var(--text-secondary);
  padding: 14px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-default);
}

.new-balance {
  color: var(--accent-success);
  font-weight: 700;
  font-size: 18px;
}

/* Balance History Dialog */
.history-content { display: flex; flex-direction: column; gap: 16px; }

.history-header-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-default);
}

.history-header-left { display: flex; align-items: center; gap: 12px; }
.history-member-info { display: flex; flex-direction: column; gap: 2px; }
.history-member-name { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.history-member-phone { font-size: 13px; color: var(--text-secondary); }
.history-header-right { display: flex; align-items: center; gap: 16px; }
.history-balance-block { text-align: right; }
.history-balance-label { display: block; font-size: 11px; color: var(--text-tertiary); }
.history-balance-value { font-size: 22px; font-weight: 700; color: var(--accent-success); }

.history-time { font-family: var(--font-mono); font-size: 12px; color: var(--text-secondary); }

.history-type-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 10px;
  font-size: 12px;
  font-weight: 500;
}
.history-type-badge.recharge { background: rgba(63,185,80,0.12); color: var(--accent-success); }
.history-type-badge.order { background: rgba(248,81,73,0.12); color: var(--accent-danger); }
.history-type-badge.cancel { background: rgba(88,166,255,0.12); color: var(--accent-primary); }
.history-type-badge.realtime_deduction { background: rgba(229,165,48,0.12); color: var(--accent-warning); }

.history-amount { font-weight: 600; font-family: var(--font-mono); }
.history-amount.positive { color: var(--accent-success); }
.history-amount.negative { color: var(--accent-danger); }

.history-balance-cell { font-family: var(--font-mono); font-size: 13px; color: var(--text-secondary); }

.history-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 40px 0;
  color: var(--text-tertiary);
  font-size: 14px;
}

.history-load-more { text-align: center; padding-top: 8px; }

:deep(.history-content .el-table) {
  --el-table-bg-color: transparent;
  --el-table-tr-bg-color: transparent;
  --el-table-header-bg-color: var(--bg-tertiary);
  --el-table-row-hover-bg-color: var(--bg-hover);
  --el-table-header-text-color: var(--text-secondary);
  --el-table-text-color: var(--text-primary);
  --el-table-border-color: var(--border-muted);
}

:deep(.history-content .el-table th.el-table__cell) {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary) !important;
  background: var(--bg-tertiary) !important;
  padding: 10px 0;
}

:deep(.history-content .el-table td.el-table__cell) {
  padding: 8px 0;
}

@media (max-width: 1024px) {
  .stat-grid { grid-template-columns: repeat(2, 1fr); }
}

@media (max-width: 768px) {
  .stat-grid { grid-template-columns: 1fr; }
  .form-row { grid-template-columns: 1fr; }
  .amount-btns { flex-wrap: wrap; }
  .page-title-row { flex-direction: column; align-items: flex-start; }
  .search-input { width: 100%; }
}
</style>
