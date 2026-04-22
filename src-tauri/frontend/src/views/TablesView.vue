<template>
  <div class="tables-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">{{ t('tables') }}</h1>
        <div class="legend-pills">
          <span class="legend-pill green"><span class="dot"></span>{{ t('tableFree') }}</span>
          <span class="legend-pill red"><span class="dot"></span>{{ t('tableInUse') }}</span>
          <span class="legend-pill yellow"><span class="dot"></span>{{ t('tableMaintenance') }}</span>
        </div>
      </div>
      <div class="header-right">
        <el-select v-model="filterArea" :placeholder="t('areaOverview')" clearable class="area-select">
          <el-option v-for="area in areas" :key="area.id" :label="area.name" :value="area.id" />
        </el-select>
        <el-button :icon="Refresh" circle @click="loadTables" />
      </div>
    </div>

    <div class="table-grid">
      <div v-for="item in filteredTables" :key="item.id"
        class="table-card" :class="getCardClass(item.status)">
        <div class="card-header">
          <div class="table-info">
            <span class="table-name">{{ item.name }}</span>
            <el-tag :type="getStatusType(item.status)" size="small" effect="dark">
              {{ getStatusText(item.status) }}
            </el-tag>
            <el-tag :type="item.is_private ? 'warning' : ''" size="small" class="area-tag">
              {{ getAreaName(item.area_id) }}
            </el-tag>
          </div>
          <div class="table-icon">
            <el-icon><component :is="item.is_private ? 'House' : 'Grid'" /></el-icon>
          </div>
        </div>

        <div class="card-body">
          <div class="info-row">
            <el-icon><Money /></el-icon>
            <span>¥{{ item.rate_per_hour }}/{{ t('hour') }}</span>
          </div>
          <div v-if="item.current_order" class="order-info">
            <el-divider />
            <div class="customer-row">
              <el-icon><User /></el-icon>
              <span>{{ item.current_order.member_name || t('walkIn') }}</span>
            </div>
            <div class="timer-row">
              <el-icon class="pulse"><Timer /></el-icon>
              <span class="timer-value">{{ getDur(item.current_order.start_time) }}</span>
            </div>
            <div class="cost-row">
              <span>{{ t('estimated') }}</span>
              <span class="cost-value">¥{{ calcCurrentCost(item) }}</span>
            </div>
          </div>
        </div>

        <div class="card-footer">
          <el-button v-if="item.status === '空闲'" type="success" class="action-btn" @click="doOpen(item)">
            <el-icon><VideoPlay /></el-icon>
            {{ t('openTable') }}
          </el-button>
          <el-button v-else-if="item.status === '使用中'" type="danger" class="action-btn" @click="doClose(item)">
            <el-icon><Coin /></el-icon>
            {{ t('closeTable') }}
          </el-button>
          <el-tag v-else type="warning" effect="plain">{{ t('tableMaintenance') }}</el-tag>
        </div>
      </div>
    </div>

    <!-- Open Table Dialog - Full Size -->
    <el-dialog
      v-model="openDlg"
      :title="openDlgTitle"
      width="480px"
      :close-on-click-modal="false"
      top="5vh"
      append-to-body
      class="open-table-dialog"
    >
      <div class="open-form" v-if="sel">
        <!-- {{ t('TableInfoCard') }} -->
        <div class="table-hero">
          <div class="hero-icon" :class="sel.is_private ? 'private' : 'hall'">
            <el-icon><component :is="sel.is_private ? 'House' : 'Grid'" /></el-icon>
          </div>
          <div class="hero-info">
            <span class="hero-name">{{ sel.name }}</span>
            <span class="hero-rate">¥{{ sel.rate_per_hour }}/{{ t('hour') }}</span>
          </div>
          <el-tag :type="sel.is_private ? 'warning' : 'success'" size="large">
            {{ sel.is_private ? t('privateRoom') : t('hall') }}
          </el-tag>
        </div>

        <!-- {{ t('CustomerTypeSwitch') }} -->
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

        <!-- {{ t('MemberSearchArea') }} -->
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
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
            <template #default="{ item }">
              <div class="member-option">
                <div class="option-left">
                  <el-avatar :size="32" style="background: var(--accent-primary);">
                    {{ item.name?.charAt(0) }}
                  </el-avatar>
                  <div class="option-info">
                    <span class="option-name">{{ item.name }}</span>
                    <span class="option-phone">{{ item.phone }}</span>
                  </div>
                </div>
                <div class="option-right">
                  <el-tag v-if="getMemberDiscount(item) > 0" type="success" size="small">
                    {{ getMemberDiscount(item) }}{{ t('discount') }}
                  </el-tag>
                  <span class="option-balance">¥{{ item.balance?.toFixed(2) }}</span>
                </div>
              </div>
            </template>
          </el-autocomplete>

          <!-- {{ t('SelectedMemberCard') }} -->
          <div v-if="selectedMember" class="selected-member-card">
            <div class="member-info-left">
              <el-avatar :size="48" style="background: var(--accent-success);">
                {{ selectedMember.name?.charAt(0) }}
              </el-avatar>
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
                <el-tag type="success" size="large">
                  {{ getMemberDiscount(selectedMember) }}{{ t('discount') }}
                </el-tag>
              </div>
            </div>
            <el-button text type="danger" :icon="Close" @click="clearMember">{{ t('cancel') }}</el-button>
          </div>
        </div>

        <!-- {{ t('WalkinCustomerArea') }} -->
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

        <!-- {{ t('PackageSelection') }} -->
        <div class="package-section">
          <div class="section-title">{{ t('selectPackage') }}</div>
          <div class="package-options">
            <div :class="['package-btn', { active: !selectedPackage }]" @click="selectedPackage = null">
              <el-icon><Clock /></el-icon>
              <span>{{ t('noPackage') }}</span>
              <span class="pkg-rate">¥{{ sel.rate_per_hour }}/{{ t('hoursShort') }}</span>
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

        <!-- {{ t('CostPreview') }} -->
        <div class="cost-preview">
          <div class="preview-title">{{ t('amount') }}</div>
          <div class="preview-content">
            <div v-if="selectedPackage" class="preview-row highlight package-row">
              <span>{{ t('packageIncluded') }}</span>
              <span class="pkg-info">{{ selectedPackage.name }} ¥{{ selectedPackage.price }}</span>
            </div>
            <div v-else class="preview-row">
              <span>{{ t('tableRate') }}</span>
              <span>¥{{ sel.rate_per_hour }}/{{ t('hour') }}</span>
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
          <el-button size="large" @click="openDlg=false">{{ t('cancel') }}</el-button>
          <el-button type="success" size="large" @click="cfmOpen" :loading="submitting">
            <el-icon><Check /></el-icon>
            {{ t('confirmOpen') }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <!-- {{ t('CheckoutDialog') }} -->
    <el-dialog v-model="closeDlg" :title="t('closeTable')" width="420px" :close-on-click-modal="false" top="5vh" append-to-body>
      <div class="checkout-content" v-if="preview">
        <div class="checkout-header">
          <div class="checkout-table"><el-icon><Grid /></el-icon><span>{{ sel?.name }}</span></div>
        </div>
        <div class="checkout-details">
          <div class="detail-row"><span class="detail-label">{{ t('memberName') }}</span><span class="detail-value">{{ preview.member_name || walkinName || t('walkIn') }}</span></div>
          <div class="detail-row"><span class="detail-label">{{ t('startTime') }}</span><span class="detail-value">{{ formatDateTime(preview.start_time) }}</span></div>
          <div class="detail-row highlight"><span class="detail-label">{{ t('consumptionDuration') }}</span><span class="detail-value time">{{ getDur(preview.start_time) }}</span></div>
          <div class="detail-row" v-if="preview.package_name">
            <span class="detail-label">{{ t('packageName') }}</span>
            <span class="detail-value" style="color: var(--accent-primary);">{{ preview.package_name }}</span>
          </div>
          <div class="detail-row" v-if="preview.package_price">
            <span class="detail-label">{{ t('packagePrice') }}</span>
            <span class="detail-value">¥{{ preview.package_price }}</span>
          </div>
        </div>
        <el-divider />
        <div class="amount-section">
          <div class="amount-row"><span>{{ t('tableRate') }}</span><span>¥{{ pay.total.toFixed(2) }}</span></div>
          <div v-if="pay.deposit > 0" class="amount-row deposit"><span>{{ t('depositPaid') }}</span><span class="deposit-value">-¥{{ pay.deposit.toFixed(2) }}</span></div>
          <div v-if="pay.discount > 0" class="amount-row discount"><span>{{ t('memberDiscount') }}</span><span class="discount-value">-¥{{ pay.discount.toFixed(2) }}</span></div>
          <div v-if="pay.balancePaid > 0" class="amount-row balance-paid"><span>{{ t('balancePaid') }}</span><span class="balance-value">-¥{{ pay.balancePaid.toFixed(2) }}</span></div>
          <div class="amount-row total"><span>{{ t('actualPayment') }}</span><span class="total-value">¥{{ pay.final.toFixed(2) }}</span></div>
          <div v-if="pay.change > 0" class="amount-row change"><span>{{ t('change') }}</span><span class="change-value">¥{{ pay.change.toFixed(2) }}</span></div>
        </div>
        <div class="close-payment-section">
          <div class="payment-label">{{ t('paymentMethod') }}</div>
          <div class="payment-btns">
            <el-button
              v-for="pm in paymentMethods"
              :key="pm.value"
              :type="closePaymentMethod === pm.value ? 'primary' : 'default'"
              @click="closePaymentMethod = pm.value"
            >
              <el-icon><component :is="pm.icon" /></el-icon>
              {{ pm.label }}
            </el-button>
          </div>
        </div>
      </div>
      <template #footer>
        <div class="payment-btns">
          <el-button type="warning" @click="openCancelDlg" v-if="preview">
            <el-icon><CircleClose /></el-icon>
            {{ t('cancelOrder') }}
          </el-button>
          <el-button @click="closeDlg=false">{{ t('cancel') }}</el-button>
          <el-button type="danger" @click="cfmClose">
            <el-icon><Money /></el-icon>
            {{ t('confirmClose') }}
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox, ElButton, ElSelect, ElOption, ElDialog, ElTag, ElIcon, ElDivider, ElAutocomplete, ElAvatar, ElInputNumber, ElSwitch, ElInput } from 'element-plus'
import { Refresh, Money, User, Timer, VideoPlay, Coin, House, Grid, Check, UserFilled, Avatar, Search, Close, Phone, Grid as GridIcon, Clock, Ticket, CircleClose } from '@element-plus/icons-vue'
import { getTables, getMembers, getAreas, getOrderByTable, openTable, closeTable, getSettings } from '../api'
import { t, currentLang } from '../i18n'

const router = useRouter()

const tables = ref([])
const areas = ref([])
const members = ref([])
const packages = ref([])
const filterArea = ref('')
const openDlg = ref(false)
const closeDlg = ref(false)
const sel = ref(null)
const selMember = ref('')
const billingRules = ref({ freeMinutes: 5, billingInterval: 30, applyRounding: true, memberDay: { enabled: false, dates: '', discount: 0 } })
const specialRates = ref({ weekend: { enabled: false, days: [], discount: 10 }, memberDay: { enabled: false, dates: '', discount: 0 } })
const preview = ref(null)
const pay = ref({ total: 0, discount: 0, final: 0, deposit: 0, change: 0 })
const submitting = ref(false)

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
const closeForm = ref({ payment_method: 'cash' })
const closePaymentMethod = ref('cash')
const paymentMethods = [
  { value: 'cash', label: t('cash'), icon: Money },
  { value: 'wechat', label: t('wechat'), icon: VideoPlay },
  { value: 'alipay', label: t('alipay'), icon: Coin },
  { value: 'deposit', label: t('deposit'), icon: Ticket },
]

const openCancelDlg = () => {
  ElMessage.info(t('featureNotAvailable'))
}

const openDlgTitle = computed(() => {
  currentLang.value
  return sel.value ? `${t('openTable')} - ${sel.value.name}` : t('openTable')
})

const previewCost = computed(() => {
  if (!sel.value) return '0.00'
  // Ensure rate_per_hour is a valid number
  let rate = 0
  if (sel.value.rate_per_hour !== undefined && sel.value.rate_per_hour !== null) {
    rate = parseFloat(sel.value.rate_per_hour)
  }
  if (isNaN(rate) || rate < 0) rate = 0

  if (customerType.value === 'member' && selectedMember.value) {
    const disc = getMemberDiscount(selectedMember.value)
    if (!disc) return rate.toFixed(2)
    const discNum = parseFloat(disc)
    if (isNaN(discNum) || discNum <= 0) return rate.toFixed(2)
    return (rate * discNum / 10).toFixed(2)
  }
  return rate.toFixed(2)
})

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

const availablePackages = computed(() => {
  if (!packages.value || !sel.value) return []
  return packages.value.filter(pkg => {
    const areaIds = pkg.area_ids
    const tableIds = pkg.table_ids
    
    if (areaIds) {
      const areaList = areaIds.split(',').map(Number)
      if (!areaList.includes(sel.value.area_id)) return false
    } else if (tableIds && tableIds !== '*') {
      const tableList = tableIds.split(',').map(Number)
      if (!tableList.includes(sel.value.id)) return false
    }
    
    return isPackageTimeValid(pkg)
  })
})

const filteredTables = computed(() => {
  if (!filterArea.value) return tables.value
  return tables.value.filter(t => t.area_id === filterArea.value)
})

const getCardClass = (status) => {
  if (status === '使用中') return 'in-use'
  if (status === '维护中') return 'maint'
  return 'free'
}

const getStatusType = (status) => {
  if (status === '使用中') return 'danger'
  if (status === '维护中') return 'warning'
  return 'success'
}

const getStatusText = (status) => {
  if (status === '使用中') return t('tableInUse')
  if (status === '维护中') return t('tableMaintenance')
  return t('tableFree')
}

const getAreaName = (areaId) => {
  if (!areaId) return '-'
  const area = areas.value.find(a => a.id === areaId)
  return area ? area.name : '-'
}

const formatDateTime = (time) => {
  if (!time) return ''
  return time.replace('T', ' ').slice(0, 16)
}

const getDur = (time) => {
  if (!time) return `0${t('minute')}`
  const d = Math.floor((Date.now() - new Date(time.replace(' ', 'T'))) / 60000)
  const hours = Math.floor(d / 60)
  const mins = d % 60
  if (hours > 0) return `${hours}${t('hour')} ${mins}${t('minute')}`
  return `${d}${t('minute')}`
}

const rnd = (n, d) => Math.round(n * Math.pow(10, d)) / Math.pow(10, d)

const getMemberDiscount = (member) => {
  if (!member || !member.discount) return 10
  return Math.round(member.discount * 10 * 10) / 10
}

const calcCurrentCost = (table) => {
  if (!table.current_order) return '0.00'
  const order = table.current_order
  const dur = Math.max(Math.floor((Date.now() - new Date(order.start_time.replace(' ', 'T'))) / 60000), 1)
  const total = (dur / 60) * (table.rate_per_hour || 30)
  if (order.member_id) {
    const m = members.value.find(x => x.id === order.member_id)
    if (m) {
      const discount = rnd(total * (1 - m.discount), 2)
      return (total - discount).toFixed(2)
    }
  }
  return total.toFixed(2)
}

const searchMembers = (query, callback) => {
  if (!query) { callback([]); return }
  const kw = query.toLowerCase()
  const results = members.value
    .filter(m => m.phone.includes(kw) || m.name.toLowerCase().includes(kw))
    .slice(0, 10)
    .map(m => ({ ...m, value: `${m.name} - ${m.phone}` }))
  callback(results)
}

const onMemberSelect = (item) => {
  selectedMember.value = item
  memberSearch.value = ''
  customerType.value = 'member'
}

const onMemberSearchChange = (val) => {
  if (!val) {
    selectedMember.value = null
  }
}

const clearMember = () => {
  selectedMember.value = null
  memberSearch.value = ''
}

const doOpen = (item) => {
  sel.value = item
  customerType.value = 'walkin'
  memberSearch.value = ''
  selectedMember.value = null
  selectedPackage.value = null
  walkinForm.value = { name: '', phone: '', useDeposit: false, deposit: 100 }
  openDlg.value = true
}

const getMemberDayDiscount = () => {
  const today = new Date()
  const dow = today.getDay()
  const mm = String(today.getMonth() + 1).padStart(2, '0')
  const dd = String(today.getDate()).padStart(2, '0')
  const todayStr = `${mm}-${dd}`

  if (specialRates.weekend?.enabled && specialRates.weekend.days?.length) {
    const monBased = dow === 0 ? 7 : dow
    if (specialRates.weekend.days.some(d => d === monBased)) {
      return (specialRates.weekend.discount || 10)
    }
  }

  const mday = billingRules.memberDay
  if (mday?.enabled && mday.dates) {
    if (mday.dates.split(',').some(d => d.trim() === todayStr)) {
      return mday.discount || 0
    }
  }
  return 0
}

const calcBillMinutes = (dur) => {
  const free = billingRules.freeMinutes || 5
  const interval = billingRules.billingInterval || 30
  if (!billingRules.applyRounding) return Math.max(0, dur - free)
  if (dur < free) return 0
  const billable = dur - free
  if (billable <= interval) return free + interval
  const fullUnits = Math.floor(billable / interval)
  const rem = billable % interval
  const grace = Math.max(1, Math.floor(interval / 6))
  return free + (rem >= grace ? fullUnits + 1 : fullUnits) * interval
}

const doClose = async (item) => {
  try {
    preview.value = await getOrderByTable(item.id)
    sel.value = item
    const dur = Math.max(Math.floor((Date.now() - new Date(preview.value.start_time.replace(' ', 'T'))) / 60000), 1)
    const rate = item.rate_per_hour || 30
    let total = 0

    if (preview.value.package_id && preview.value.package_price) {
      total = preview.value.package_price
      const pkgDur = Math.floor((preview.value.package_hours || 0) * 60)
      if (dur > pkgDur) {
        const extra = dur - pkgDur
        total += (extra / 60) * rate
      }
    } else {
      const billMin = calcBillMinutes(dur)
      total = (billMin / 60) * rate
    }
    total = rnd(total, 2)

    let disc = 0
    let fin = total
    const deposit = preview.value.deposit || 0
    let change = 0
    let balancePaid = 0
    let memberBalance = 0

    if (preview.value.member_id) {
      const m = members.value.find(x => x.id === preview.value.member_id)
      if (m) {
        memberBalance = m.balance || 0
        const mdd = getMemberDayDiscount()
        const memberDisc = m.discount || 1
        const mdayDisc = 1 - (mdd / 100)
        const finalFactor = memberDisc * mdayDisc
        disc = rnd(total * (1 - finalFactor), 2)
        fin = rnd(total - disc, 2)

        if (memberBalance > 0 && fin > 0) {
          balancePaid = Math.min(memberBalance, fin)
          fin = Math.max(0, fin - balancePaid)
        }
      }
    }

    if (deposit > 0) {
      const originalCash = fin
      fin = Math.max(0, fin - deposit)
      change = deposit > originalCash ? deposit - originalCash : 0
    }

    pay.value = { total, discount: disc, final: fin, deposit, change: Math.max(0, change), balancePaid, memberBalance }
    closeDlg.value = true
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const cfmOpen = async () => {
  submitting.value = true
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
      openDlg.value = false
      const confirmed = await ElMessageBox.confirm(
        t('noBalanceRechargeMsg'),
        t('noBalanceRechargeTitle'),
        {
          confirmButtonText: t('goRecharge'),
          cancelButtonText: t('waitLater'),
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
      table_id: sel.value.id,
      member_id: memberId,
      package_id: selectedPackage.value?.id || null,
      customer_name: walkinForm.value.name || null,
      customer_phone: walkinForm.value.phone || null,
      deposit: deposit
    })

    ElMessage.success({ message: `${sel.value.name} ${t('openSuccess')}！`, grouping: true })
    openDlg.value = false
    await loadTables()
  } catch(e) {
    const errMsg = e.response?.data?.error || ''
    if (errMsg.includes('余额不足')) {
      ElMessage.warning('会员卡余额不足，请先充值')
      openDlg.value = false
      selectedMember.value = null
      customerType.value = 'walkin'
      return
    }
    ElMessage.error(e.response?.data?.error || t('operationFailed'))
  } finally {
    submitting.value = false
  }
}

const cfmClose = async () => {
  try {
    await closeTable(preview.value.id, { payment_method: closePaymentMethod.value })
    ElMessage.success({ message: `${t('closeSuccess')}！¥${pay.value.final.toFixed(2)}`, grouping: true })
    closeDlg.value = false
    closeForm.value = { payment_method: 'cash' }
    closePaymentMethod.value = 'cash'
    await loadTables()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const loadTables = async () => {
  try {
    tables.value = await getTables(true)
    const settings = await getSettings()
    packages.value = settings.packages || []
  } catch {
    ElMessage.error(t('loadFailed'))
  }
}

const loadAreas = async () => {
  try {
    areas.value = await getAreas()
  } catch {
    areas.value = []
  }
}

let timer = null

const loadBillingRules = async () => {
  try {
    const s = await getSettings()
    if (s.billingRules) {
      billingRules.value.freeMinutes = s.billingRules.freeMinutes ?? 5
      billingRules.value.billingInterval = s.billingRules.billingInterval ?? 30
      billingRules.value.applyRounding = s.billingRules.applyRounding ?? true
    }
    if (s.memberDay) {
      billingRules.value.memberDay = {
        enabled: s.memberDay.enabled ?? false,
        dates: s.memberDay.dates ?? '',
        discount: s.memberDay.discount ?? 0,
      }
    }
    if (s.specialRates?.weekendDiscount) {
      const wd = s.specialRates.weekendDiscount
      const days = wd.days ? String(wd.days).split(',').map(Number) : []
      specialRates.value.weekend = {
        enabled: wd.enabled ?? false,
        discount: wd.discount ?? 10,
        days: days,
      }
    }
  } catch {}
}

onMounted(async () => {
  await Promise.all([loadTables(), loadAreas(), loadBillingRules()])
  members.value = await getMembers()
  timer = setInterval(loadTables, 30000)
})

onUnmounted(() => clearInterval(timer))
</script>

<style scoped>
.tables-page { display: flex; flex-direction: column; gap: 24px; }

.page-header { display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 16px; }
.header-left { display: flex; flex-direction: column; gap: 12px; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }

.legend-pills { display: flex; gap: 12px; }
.legend-pill { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--text-secondary); }
.legend-pill .dot { width: 8px; height: 8px; border-radius: 50%; }
.legend-pill.green .dot { background: var(--accent-success); }
.legend-pill.red .dot { background: var(--accent-danger); }
.legend-pill.yellow .dot { background: var(--accent-warning); }

.header-right { display: flex; gap: 12px; align-items: center; }
.area-select { width: 160px; }

.table-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }

.table-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  transition: all 0.2s;
  display: flex;
  flex-direction: column;
}
.table-card:hover { border-color: var(--accent-primary); transform: translateY(-2px); box-shadow: var(--shadow-md); }
.table-card.in-use { border-color: rgba(248,81,73,0.3); }
.table-card.maint { border-color: rgba(210,153,34,0.3); opacity: 0.7; }

.card-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 16px; }
.table-info { display: flex; flex-direction: column; gap: 6px; }
.table-name { font-size: 18px; font-weight: 700; color: var(--text-primary); }
.area-tag { margin-left: 4px; }
.table-icon {
  width: 40px; height: 40px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  display: flex; align-items: center; justify-content: center;
  color: var(--text-secondary); font-size: 20px;
}

.card-body { flex: 1; margin-bottom: 16px; }
.info-row { display: flex; align-items: center; gap: 8px; font-size: 13px; color: var(--text-secondary); margin-bottom: 8px; }
.info-row .el-icon { font-size: 14px; color: var(--text-tertiary); }

.order-info { margin-top: 12px; }
:deep(.el-divider) { margin: 12px 0; border-color: var(--border-muted); }
.customer-row { display: flex; align-items: center; gap: 6px; font-size: 13px; color: var(--text-secondary); margin-bottom: 8px; }
.customer-row .el-icon { color: var(--text-tertiary); }
.timer-row { display: flex; align-items: center; gap: 6px; margin-bottom: 8px; }
.timer-value { font-size: 20px; font-weight: 700; color: var(--accent-primary); font-family: var(--font-mono); }
.pulse { animation: pulse 1.5s infinite; }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
.cost-row { display: flex; justify-content: space-between; align-items: center; padding: 8px 12px; background: var(--bg-primary); border-radius: var(--radius-sm); }
.cost-row span:first-child { font-size: 12px; color: var(--text-tertiary); }
.cost-value { font-size: 14px; font-weight: 600; color: var(--accent-success); }

.card-footer { margin-top: auto; }
.action-btn { width: 100%; }

/* Open Dialog Styles */
.open-form { display: flex; flex-direction: column; gap: 16px; }

.table-hero {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-primary);
  border-radius: var(--radius-md);
}

.hero-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}
.hero-icon.hall { background: rgba(63,185,80,0.15); color: var(--accent-success); }
.hero-icon.private { background: rgba(210,153,34,0.15); color: var(--accent-warning); }

.hero-info { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.hero-name { font-size: 18px; font-weight: 700; color: var(--text-primary); }
.hero-rate { font-size: 14px; color: var(--accent-primary); }

.section-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.customer-type-section { }

.customer-tabs { display: flex; gap: 8px; }
.tab-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px;
  border: 2px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
  font-weight: 500;
}
.tab-btn:hover { border-color: var(--accent-primary); color: var(--accent-primary); }
.tab-btn.active {
  border-color: var(--accent-primary);
  background: rgba(88,166,255,0.1);
  color: var(--accent-primary);
}

.member-search { width: 100%; }

.member-option { display: flex; justify-content: space-between; align-items: center; padding: 6px 0; }
.option-left { display: flex; align-items: center; gap: 10px; }
.option-info { display: flex; flex-direction: column; gap: 2px; }
.option-name { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.option-phone { font-size: 11px; color: var(--text-tertiary); }
.option-right { display: flex; flex-direction: column; align-items: flex-end; gap: 2px; }
.option-balance { font-size: 13px; font-weight: 600; color: var(--accent-success); }

.selected-member-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px;
  background: rgba(63,185,80,0.08);
  border: 2px solid var(--accent-success);
  border-radius: var(--radius-md);
  margin-top: 8px;
}
.member-info-left { display: flex; align-items: center; gap: 10px; }
.member-details { display: flex; flex-direction: column; gap: 2px; }
.member-name { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.member-phone { font-size: 12px; color: var(--text-tertiary); }
.member-info-right { flex: 1; display: flex; justify-content: flex-end; gap: 12px; }
.member-balance-display, .member-discount-display { display: flex; flex-direction: column; align-items: flex-end; gap: 2px; }
.balance-label { font-size: 10px; color: var(--text-tertiary); }
.balance-amount { font-size: 16px; font-weight: 700; color: var(--accent-success); }

.walkin-form { display: flex; flex-direction: column; gap: 12px; }
.form-row { display: flex; gap: 12px; }
.form-item { flex: 1; display: flex; flex-direction: column; gap: 4px; }
.form-item label { font-size: 12px; color: var(--text-secondary); }

.deposit-section {
  padding: 12px;
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  margin-top: 8px;
}
.deposit-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; }
.deposit-title { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.deposit-options { display: flex; flex-direction: column; gap: 8px; }
.quick-amounts { display: flex; gap: 6px; }
.amount-btn {
  flex: 1;
  padding: 8px;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-secondary);
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 13px;
}
.amount-btn:hover { border-color: var(--accent-primary); color: var(--accent-primary); }
.amount-btn.active {
  border-color: var(--accent-success);
  background: rgba(63,185,80,0.1);
  color: var(--accent-success);
}
.custom-deposit { display: flex; align-items: center; gap: 8px; }
.deposit-payment { display: flex; flex-direction: column; gap: 6px; margin-top: 8px; }
.deposit-payment .payment-label { font-size: 12px; color: var(--text-secondary); }
.deposit-payment .payment-btns { display: flex; gap: 8px; }
.deposit-payment .payment-btns .el-button { flex: 1; }

.package-section { display: flex; flex-direction: column; gap: 8px; }
.package-options { display: flex; flex-wrap: wrap; gap: 8px; }
.package-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  border: 2px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 13px;
}
.package-btn:hover { border-color: var(--accent-primary); }
.package-btn.active {
  border-color: var(--accent-primary);
  background: rgba(88,166,255,0.1);
}
.package-btn .pkg-info-col { display: flex; flex-direction: column; gap: 2px; flex: 1; }
.package-btn .pkg-name { font-weight: 600; color: var(--text-primary); }
.package-btn .pkg-time { font-size: 11px; color: var(--text-tertiary); }
.package-btn .pkg-price { font-weight: 700; color: var(--accent-success); }
.package-btn .pkg-rate { font-size: 12px; color: var(--text-tertiary); }
.package-btn.morning { border-color: rgba(255,193,7,0.3); }
.package-btn.morning.active { border-color: #ffc107; background: rgba(255,193,7,0.1); }
.package-btn.evening { border-color: rgba(244,67,54,0.3); }
.package-btn.evening.active { border-color: #f44336; background: rgba(244,67,54,0.1); }
.package-btn.night { border-color: rgba(103,58,183,0.3); }
.package-btn.night.active { border-color: #673ab7; background: rgba(103,58,183,0.1); }
.package-row { color: var(--accent-primary); }
.pkg-info { font-weight: 600; color: var(--accent-primary); }

.cost-preview {
  padding: 12px;
  background: rgba(88,166,255,0.08);
  border: 1px solid rgba(88,166,255,0.2);
  border-radius: var(--radius-md);
}
.preview-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--accent-primary);
  margin-bottom: 8px;
}
.preview-content { display: flex; flex-direction: column; gap: 6px; }
.preview-row {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: var(--text-secondary);
}
.preview-row.highlight { font-weight: 600; color: var(--text-primary); }
.discount-text { color: var(--accent-success); }
.pay-amount { font-size: 16px; color: var(--accent-primary); }

.dialog-footer { display: flex; justify-content: flex-end; gap: 12px; }

/* Checkout Dialog */
.checkout-content { display: flex; flex-direction: column; gap: 16px; }
.checkout-header { padding: 16px; background: var(--bg-primary); border-radius: var(--radius-md); text-align: center; }
.checkout-table { display: flex; align-items: center; justify-content: center; gap: 8px; font-size: 20px; font-weight: 700; color: var(--text-primary); }
.checkout-table .el-icon { font-size: 24px; color: var(--accent-primary); }

.checkout-details { display: flex; flex-direction: column; gap: 8px; }
.detail-row {
  display: flex;
  justify-content: space-between;
  padding: 10px 14px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
}
.detail-label { color: var(--text-secondary); font-size: 14px; }
.detail-value { color: var(--text-primary); font-weight: 500; }
.detail-row.highlight { background: rgba(88,166,255,0.1); }
.detail-row .time { color: var(--accent-primary); font-size: 16px; font-weight: 600; }

.amount-section { display: flex; flex-direction: column; gap: 8px; }
.amount-row { display: flex; justify-content: space-between; font-size: 14px; color: var(--text-secondary); }
.amount-row.total { padding: 12px; background: rgba(248,81,73,0.1); border-radius: var(--radius-sm); font-weight: 600; color: var(--text-primary); }
.amount-row.deposit { color: var(--accent-success); }
.amount-row.change { color: var(--accent-warning); }
.total-value { font-size: 24px; font-weight: 700; color: var(--accent-danger); }
.deposit-value { color: var(--accent-success); }
.change-value { color: var(--accent-warning); }

.close-payment-section { margin-top: 16px; padding-top: 16px; border-top: 1px solid var(--border-default); }
.close-payment-section .payment-label { font-size: 14px; font-weight: 600; margin-bottom: 12px; color: var(--text-primary); }
.close-payment-section .payment-btns { display: flex; gap: 10px; }
.close-payment-section .payment-btns .el-button { flex: 1; }
.close-payment-section .payment-btns .el-button--default {
  background: var(--bg-tertiary) !important;
  border: 2px solid var(--border-default) !important;
  color: var(--text-secondary) !important;
  font-weight: 600;
}
.close-payment-section .payment-btns .el-button--default:hover {
  border-color: var(--accent-primary) !important;
  color: var(--accent-primary) !important;
  background: var(--bg-active) !important;
}
.close-payment-section .payment-btns .el-button--primary {
  border: 2px solid var(--accent-primary) !important;
}

.payment-btns { display: flex; gap: 12px; justify-content: flex-end; }
.payment-btns .el-button { flex: 1; }
</style>
