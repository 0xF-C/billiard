<template>
  <el-dialog
    v-model="dialogVisible"
    :title="dialogTitle"
    width="480px"
    :close-on-click-modal="false"
    top="5vh"
    append-to-body
    destroy-on-close
    class="open-table-dialog"
  >
    <div class="open-form" v-if="tableData">
      <div class="table-hero">
        <div class="hero-icon" :class="tableData.is_private ? 'private' : 'hall'">
          <el-icon><component :is="tableData.is_private ? 'House' : 'Grid'" /></el-icon>
        </div>
        <div class="hero-info">
          <span class="hero-name">{{ tableData.name }}</span>
          <span class="hero-rate">¥{{ tableData.rate_per_hour }}/{{ t('hour') }}</span>
        </div>
        <el-tag :type="tableData.is_private ? 'warning' : 'success'" size="large">
          {{ tableData.is_private ? t('privateRoom') : t('hall') }}
        </el-tag>
      </div>

      <div class="customer-type-section">
        <div class="section-title">{{ t('member') }}/{{ t('walkIn') }}</div>
        <div class="customer-tabs">
          <div :class="['tab-btn', { active: customerType === 'walkin' }]" @click="customerType = 'walkin'">
            <el-icon><UserFilled /></el-icon>
            <span>{{ t('walkIn') }}</span>
          </div>
          <div :class="['tab-btn', { active: customerType === 'member' }]" @click="customerType = 'member'">
            <el-icon><Avatar /></el-icon>
            <span>{{ t('member') }}</span>
          </div>
        </div>
      </div>

      <div v-if="customerType === 'member'" class="member-section">
        <div class="section-title">{{ t('searchMember') }}</div>
        <el-autocomplete
          v-model="memberSearch"
          :fetch-suggestions="searchMembers"
          :trigger-on-focus="false"
          clearable
          :placeholder="t('search') + ' ' + t('name') + '/' + t('phone')"
          size="large"
          class="member-search"
          @select="onMemberSelect"
          @change="onMemberSearchChange"
        >
          <template #prefix><el-icon><Search /></el-icon></template>
          <template #default="{ item }">
            <div class="member-option">
              <div class="option-left">
                <el-avatar :size="32" style="background: var(--accent-primary);">{{ item.name?.charAt(0) }}</el-avatar>
                <div class="option-info">
                  <span class="option-name">{{ item.name }}</span>
                  <span class="option-phone">{{ item.phone }}</span>
                </div>
              </div>
              <div class="option-right">
                <el-tag v-if="getMemberDiscount(item) > 0" type="success" size="small">{{ getMemberDiscount(item) }}{{ t('discount') }}</el-tag>
                <span class="option-balance">¥{{ item.balance?.toFixed(2) }}</span>
              </div>
            </div>
          </template>
        </el-autocomplete>

        <div v-if="selectedMember" class="selected-member-card">
          <div class="member-info-left">
            <el-avatar :size="48" style="background: var(--accent-success);">{{ selectedMember.name?.charAt(0) }}</el-avatar>
            <div class="member-details">
              <span class="member-name">{{ selectedMember.name }}</span>
              <span class="member-phone">{{ selectedMember.phone }}</span>
            </div>
          </div>
          <div class="member-info-right">
            <div class="member-balance-display">
              <span class="balance-label">{{ t('balance') }}</span>
              <span class="balance-amount">¥{{ selectedMember.balance?.toFixed(2) }}</span>
            </div>
            <div class="member-discount-display">
              <el-tag type="success" size="large">{{ getMemberDiscount(selectedMember) }}{{ t('discount') }}</el-tag>
            </div>
          </div>
          <el-button text type="danger" :icon="Close" @click="clearMember">{{ t('cancel') }}</el-button>
        </div>
      </div>

      <div v-else class="walkin-section">
        <div class="section-title">{{ t('walkInInfo') }}</div>
        <div class="walkin-form">
          <div class="form-row">
            <div class="form-item">
              <label>{{ t('name') }}</label>
              <el-input v-model="walkinForm.name" :placeholder="t('optional')" clearable>
                <template #prefix><el-icon><User /></el-icon></template>
              </el-input>
            </div>
            <div class="form-item">
              <label>{{ t('phone') }}</label>
              <el-input v-model="walkinForm.phone" :placeholder="t('optional')" clearable>
                <template #prefix><el-icon><Phone /></el-icon></template>
              </el-input>
            </div>
          </div>

          <div class="deposit-section">
            <div class="deposit-header">
              <span class="deposit-title">{{ t('deposit') }}</span>
              <el-switch v-model="walkinForm.useDeposit" :active-text="t('confirm')" :inactive-text="t('cancel')" />
            </div>
            <div v-if="walkinForm.useDeposit" class="deposit-options">
              <div class="quick-amounts">
                <el-button
                  v-for="amt in [50, 100, 200]"
                  :key="amt"
                  :class="['amount-btn', { active: walkinForm.deposit === amt }]"
                  @click="walkinForm.deposit = amt"
                >
                  ¥{{ amt }}
                </el-button>
              </div>
              <div class="custom-deposit">
                <el-input-number
                  v-model="walkinForm.deposit"
                  :min="0"
                  :step="10"
                  controls-position="right"
                  :placeholder="t('amount')"
                />
              </div>
              <div class="deposit-payment">
                <span class="payment-label">{{ t('paymentMethod') }}</span>
                <div class="payment-btns">
                  <el-button
                    v-for="pm in depositPaymentMethods"
                    :key="pm.value"
                    :type="walkinForm.payment_method === pm.value ? 'primary' : 'default'"
                    @click="walkinForm.payment_method = pm.value"
                  >
                    <el-icon><component :is="pm.icon" /></el-icon>
                    {{ pm.label }}
                  </el-button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="package-section">
        <div class="section-title">{{ t('selectPackage') }}</div>
        <div class="package-options">
          <div :class="['package-btn', { active: !selectedPackage }]" @click="selectedPackage = null">
            <el-icon><Clock /></el-icon>
            <span>{{ t('noPackage') }}</span>
            <span class="pkg-rate">¥{{ tableData.rate_per_hour }}/{{ t('hoursShort') }}</span>
          </div>
          <div
            v-for="pkg in availablePackages"
            :key="pkg.id"
            :class="['package-btn', pkg.type, { active: selectedPackage?.id === pkg.id }]"
            @click="selectedPackage = pkg"
          >
            <el-icon><Ticket /></el-icon>
            <div class="pkg-info-col">
              <span class="pkg-name">{{ pkg.name }}</span>
              <span v-if="pkg.start_time || pkg.end_time" class="pkg-time">
                {{ pkg.start_time || '00:00' }} - {{ pkg.end_time || '23:59' }}
              </span>
            </div>
            <span class="pkg-price">¥{{ pkg.price }}</span>
            <span class="pkg-rate">{{ pkg.hours }}{{ t('hoursShort') }}</span>
          </div>
        </div>
      </div>

      <div class="cost-preview">
        <div class="preview-title">{{ t('amount') }}</div>
        <div class="preview-content">
          <div v-if="selectedPackage" class="preview-row highlight package-row">
            <span>{{ t('packageIncluded') }}</span>
            <span class="pkg-info">{{ selectedPackage.name }} ¥{{ selectedPackage.price }}</span>
          </div>
          <div v-else class="preview-row">
            <span>{{ t('tableRate') }}</span>
            <span>¥{{ tableData.rate_per_hour }}/{{ t('hour') }}</span>
          </div>
          <div v-if="customerType === 'member' && selectedMember" class="preview-row highlight">
            <span>{{ t('memberDiscount') }}</span>
            <span class="discount-text">{{ getMemberDiscount(selectedMember) }}{{ t('discount') }}</span>
          </div>
          <div class="preview-row highlight">
            <span>{{ t('actualPayment') }}</span>
            <span class="pay-amount">{{ selectedPackage ? '¥' + selectedPackage.price : '~ ¥' + previewCost + '/' + t('hours') }}</span>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button size="large" @click="handleCancel">{{ t('cancel') }}</el-button>
        <el-button type="success" size="large" @click="handleConfirm" :loading="loading">
          <el-icon><Check /></el-icon>
          {{ t('confirmOpen') }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { User, UserFilled, Avatar, Search, Close, Phone, House, Grid, Clock, Ticket, Check, Money, VideoPlay, Coin } from '@element-plus/icons-vue'
import { t } from '../i18n'
import { openTable } from '../api'

const router = useRouter()

const props = defineProps({
  modelValue: Boolean,
  table: Object,
  members: Array,
  packages: Array,
})

const emit = defineEmits(['update:modelValue', 'success'])

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

const dialogTitle = computed(() => props.table ? `${t('openTable')} - ${props.table.name}` : t('openTable'))

const loading = ref(false)
const customerType = ref('walkin')
const memberSearch = ref('')
const selectedMember = ref(null)
const selectedPackage = ref(null)
const walkinForm = ref({ name: '', phone: '', useDeposit: false, deposit: 100, payment_method: 'cash' })
const depositPaymentMethods = [
  { value: 'cash', label: t('cash'), icon: Money },
  { value: 'wechat', label: t('wechat'), icon: VideoPlay },
  { value: 'alipay', label: t('alipay'), icon: Coin },
]

const tableData = computed(() => props.table)
const availablePackages = computed(() => {
  if (!props.table || !props.packages) return []
  return props.packages.filter(pkg => {
    const areaIds = (pkg.area_ids || '').split(',').filter(Boolean).map(Number)
    const tableIds = (pkg.table_ids || '').split(',').filter(Boolean).map(Number)
    return (areaIds.length === 0 || areaIds.includes(props.table.area_id))
      && (tableIds.length === 0 || tableIds.includes(props.table.id))
      && isPackageTimeValid(pkg)
  })
})

const previewCost = computed(() => {
  if (!props.table) return '0'
  const rate = props.table.rate_per_hour || 30
  return rate.toFixed(2)
})

const getMemberDiscount = (member) => {
  if (!member || !member.discount) return 0
  return Math.round(member.discount * 10 * 10) / 10
}

const searchMembers = (query, callback) => {
  if (!query) { callback([]); return }
  const kw = query.toLowerCase()
  const results = (props.members || [])
    .filter(m => m.phone.includes(kw) || m.name.toLowerCase().includes(kw))
    .slice(0, 10)
    .map(m => ({ ...m, value: `${m.name} - ${m.phone}` }))
  callback(results)
}

const onMemberSelect = (item) => {
  selectedMember.value = item
  memberSearch.value = ''
}

const onMemberSearchChange = (val) => {
  if (!val) selectedMember.value = null
}

const clearMember = () => {
  selectedMember.value = null
  memberSearch.value = ''
}

const isPackageTimeValid = (pkg) => {
  const now = new Date()
  const currentHour = now.getHours()
  const currentMin = now.getMinutes()
  const currentTime = currentHour * 60 + currentMin
  const startTime = pkg.start_time || ''
  const endTime = pkg.end_time || ''
  if (!startTime && !endTime) return true
  if (startTime) {
    const [sh, sm] = startTime.split(':').map(Number)
    if (currentTime < sh * 60 + sm) return false
  }
  if (endTime) {
    const [eh, em] = endTime.split(':').map(Number)
    if (currentTime > eh * 60 + em) return false
  }
  return true
}

const handleCancel = () => {
  resetForm()
  dialogVisible.value = false
}

const resetForm = () => {
  customerType.value = 'walkin'
  memberSearch.value = ''
  selectedMember.value = null
  selectedPackage.value = null
  walkinForm.value = { name: '', phone: '', useDeposit: false, deposit: 100, payment_method: 'cash' }
}

const handleConfirm = async () => {
  loading.value = true
  try {
    let memberId = null
    let deposit = null

    if (customerType.value === 'member' && selectedMember.value) {
      memberId = selectedMember.value.id
    } else if (customerType.value === 'walkin' && walkinForm.value.useDeposit) {
      deposit = walkinForm.value.deposit
    }

    const isMemberNoBalance = customerType.value === 'member' && selectedMember.value && (selectedMember.value.balance || 0) <= 0
    const packageBalanceNeeded = selectedPackage.value ? (selectedPackage.value.price || 0) : 0
    const memberBalance = selectedMember.value ? (selectedMember.value.balance || 0) : 0
    const needsRecharge = (isMemberNoBalance) || (packageBalanceNeeded > 0 && memberBalance < packageBalanceNeeded)
    if (needsRecharge) {
      dialogVisible.value = false
      const confirmed = await ElMessageBox.confirm(
        t('noBalanceRechargeMsg') || '会员余额不足，是否前往充值？',
        t('noBalanceRechargeTitle') || '余额不足',
        {
          confirmButtonText: t('goRecharge') || '前往充值',
          cancelButtonText: t('waitLater') || '稍后再说',
          type: 'warning',
          distinguishCancelAndClose: true,
        }
      ).catch(() => false)
      if (confirmed) {
        router.push({ path: '/members', query: { rechargeId: String(selectedMember.value.id) } })
      }
      return
    }

    await openTable({
      tableId: props.table.id,
      memberId: memberId,
      packageId: selectedPackage.value?.id || null,
      customerName: walkinForm.value.name || null,
      customerPhone: walkinForm.value.phone || null,
      deposit: deposit
    })

    ElMessage.success(`${props.table.name} ${t('openSuccess')}！`)
    resetForm()
    dialogVisible.value = false
    emit('success')
  } catch(e) {
    const errMsg = e.response?.data?.error || ''
    if (errMsg.includes('余额不足')) {
      dialogVisible.value = false
      const confirmed = await ElMessageBox.confirm(
        '会员卡余额不足，是否前往充值？',
        '余额不足',
        {
          confirmButtonText: '前往充值',
          cancelButtonText: '稍后再说',
          type: 'warning',
          distinguishCancelAndClose: true,
        }
      ).catch(() => false)
      if (confirmed) {
        router.push({ path: '/members', query: { rechargeId: String(selectedMember.value.id) } })
      }
      return
    }
    ElMessage.error(e.response?.data?.error || t('operationFailed'))
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.open-table-dialog { --el-dialog-padding: 12px; }
.open-form { display: flex; flex-direction: column; gap: 12px; }

.table-hero {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px;
  background: var(--bg-primary);
  border-radius: var(--radius-md);
}

.hero-icon {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}
.hero-icon.hall { background: rgba(63,185,80,0.15); color: var(--accent-success); }
.hero-icon.private { background: rgba(210,153,34,0.15); color: var(--accent-warning); }

.hero-info { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.hero-name { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.hero-rate { font-size: 13px; color: var(--accent-primary); }

.customer-type-section { display: flex; flex-direction: column; gap: 6px; }
.section-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.customer-tabs { display: flex; gap: 8px; }
.tab-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 8px;
  border: 2px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 13px;
  font-weight: 500;
}
.tab-btn:hover { border-color: var(--accent-primary); color: var(--accent-primary); }
.tab-btn.active {
  border-color: var(--accent-primary);
  background: rgba(88,166,255,0.1);
  color: var(--accent-primary);
}

.member-section, .walkin-section { display: flex; flex-direction: column; gap: 8px; }
.member-search { width: 100%; }

.member-option { display: flex; justify-content: space-between; align-items: center; padding: 4px 0; }
.option-left { display: flex; align-items: center; gap: 8px; }
.option-info { display: flex; flex-direction: column; gap: 1px; }
.option-name { font-size: 12px; font-weight: 600; color: var(--text-primary); }
.option-phone { font-size: 11px; color: var(--text-tertiary); }
.option-right { display: flex; flex-direction: column; align-items: flex-end; gap: 1px; }
.option-balance { font-size: 12px; font-weight: 600; color: var(--accent-success); }

.selected-member-card {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: rgba(63,185,80,0.08);
  border: 2px solid var(--accent-success);
  border-radius: var(--radius-md);
  margin-top: 6px;
}
.member-info-left { display: flex; align-items: center; gap: 8px; }
.member-details { display: flex; flex-direction: column; gap: 1px; }
.member-name { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.member-phone { font-size: 11px; color: var(--text-tertiary); }
.member-info-right { flex: 1; display: flex; justify-content: flex-end; gap: 8px; }
.member-balance-display { display: flex; flex-direction: column; align-items: flex-end; gap: 1px; }
.balance-label { font-size: 10px; color: var(--text-tertiary); }
.balance-amount { font-size: 14px; font-weight: 600; color: var(--accent-success); }

.walkin-form { display: flex; flex-direction: column; gap: 8px; }
.form-row { display: flex; gap: 8px; }
.form-item { flex: 1; display: flex; flex-direction: column; gap: 3px; }
.form-item label { font-size: 11px; color: var(--text-secondary); }

.deposit-section {
  padding: 8px;
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  margin-top: 6px;
}
.deposit-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px; }
.deposit-title { font-size: 12px; font-weight: 600; color: var(--text-primary); }
.deposit-options { display: flex; flex-direction: column; gap: 6px; }
.quick-amounts { display: flex; gap: 4px; }
.amount-btn {
  flex: 1;
  padding: 6px;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-secondary);
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 12px;
}
.amount-btn:hover { border-color: var(--accent-primary); color: var(--accent-primary); }
.amount-btn.active {
  border-color: var(--accent-success);
  background: rgba(63,185,80,0.1);
  color: var(--accent-success);
}
.custom-deposit { display: flex; align-items: center; gap: 6px; }
.deposit-payment { display: flex; flex-direction: column; gap: 4px; margin-top: 6px; }
.payment-label { font-size: 11px; color: var(--text-secondary); }
.payment-btns { display: flex; gap: 6px; }
.payment-btns .el-button { flex: 1; }

.package-section { display: flex; flex-direction: column; gap: 6px; }
.package-options { display: flex; flex-wrap: wrap; gap: 6px; }
.package-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 10px;
  border: 2px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 12px;
}
.package-btn:hover { border-color: var(--accent-primary); }
.package-btn.active {
  border-color: var(--accent-primary);
  background: rgba(88,166,255,0.1);
}
.package-btn .pkg-info-col { display: flex; flex-direction: column; gap: 1px; flex: 1; }
.package-btn .pkg-name { font-weight: 500; color: var(--text-primary); }
.package-btn .pkg-time { font-size: 10px; color: var(--text-tertiary); }
.package-btn .pkg-price { font-weight: 600; color: var(--accent-success); }
.package-btn .pkg-rate { font-size: 11px; color: var(--text-tertiary); }
.package-btn.morning { border-color: rgba(255,193,7,0.3); }
.package-btn.morning.active { border-color: #ffc107; background: rgba(255,193,7,0.1); }
.package-btn.evening { border-color: rgba(244,67,54,0.3); }
.package-btn.evening.active { border-color: #f44336; background: rgba(244,67,54,0.1); }
.package-btn.night { border-color: rgba(103,58,183,0.3); }
.package-btn.night.active { border-color: #673ab7; background: rgba(103,58,183,0.1); }

.package-row { color: var(--accent-primary); }
.pkg-info { font-weight: 600; color: var(--accent-primary); }

.cost-preview {
  padding: 10px;
  background: rgba(88,166,255,0.08);
  border: 1px solid rgba(88,166,255,0.2);
  border-radius: var(--radius-md);
}
.preview-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--accent-primary);
  margin-bottom: 6px;
}
.preview-content { display: flex; flex-direction: column; gap: 4px; }
.preview-row {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-secondary);
}
.preview-row.highlight { font-weight: 600; color: var(--text-primary); }
.discount-text { color: var(--accent-success); }
.pay-amount { font-size: 14px; color: var(--accent-primary); }

.dialog-footer { display: flex; gap: 8px; justify-content: flex-end; }
</style>