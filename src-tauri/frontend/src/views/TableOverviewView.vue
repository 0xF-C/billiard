<template>
  <div class="overview-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('areaOverview') }}</h1>
      <div class="header-right">
        <el-select v-model="selectedArea" :placeholder="t('areaOverview')" clearable class="area-select">
          <el-option :value="null" :label="t('allAreas')" />
          <el-option v-for="area in areas" :key="area.id" :label="area.name" :value="area.id" />
        </el-select>
        <el-button :icon="Refresh" circle @click="loadData" />
      </div>
    </div>

    <div class="stats-row">
      <div class="stat-card">
        <div class="stat-icon green"><el-icon><Grid /></el-icon></div>
        <div class="stat-body">
          <span class="stat-value">{{ stats.total }}</span>
          <span class="stat-label">{{ t('totalTables') }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon success"><el-icon><CircleCheckFilled /></el-icon></div>
        <div class="stat-body">
          <span class="stat-value">{{ stats.free }}</span>
          <span class="stat-label">{{ t('tableFree') }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon danger"><el-icon><Timer /></el-icon></div>
        <div class="stat-body">
          <span class="stat-value">{{ stats.inUse }}</span>
          <span class="stat-label">{{ t('tableInUse') }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon warning"><el-icon><Tools /></el-icon></div>
        <div class="stat-body">
          <span class="stat-value">{{ stats.maint }}</span>
          <span class="stat-label">{{ t('tableMaintenance') }}</span>
        </div>
      </div>
    </div>

    <div class="areas-grid">
      <div v-for="area in filteredAreas" :key="area.id" class="area-section">
        <div class="area-header">
          <div class="area-title">
            <el-icon><Location /></el-icon>
            <span>{{ area.name }}</span>
            <span class="area-rate-badge">¥{{ area.rate_per_hour || 30 }}/{{ t('hour') }}</span>
          </div>
          <div class="area-summary">
            <el-tag type="success" size="small">{{ area.freeCount }} {{ t('tableFree') }}</el-tag>
            <el-tag type="danger" size="small">{{ area.useCount }} {{ t('tableInUse') }}</el-tag>
          </div>
        </div>
        <div class="tables-grid">
          <div
            v-for="table in area.tables"
            :key="table.id"
            class="table-item"
            :class="getTableClass(table.status)"
            @click="onTableClick(table)"
          >
            <div class="table-number">{{ table.name }}</div>
            <div class="table-status">{{ getStatusText(table.status) }}</div>
            <div v-if="table.status === '使用中'" class="table-timer" :class="getTimerClass(table.id)">
              {{ getDur(table.current_order?.start_time) }}
            </div>
            <div v-if="table.status === '使用中'" class="table-cost" :class="getCostClass(table.id)">
              ¥{{ calcCurrentCost(table) }}
            </div>
            <div v-if="table.status === '使用中' && getBillingStatus(table.id)" class="table-remaining"
                 :class="'remaining-' + getBillingStatus(table.id)?.warning_level">
              {{ getBillingStatus(table.id)?.remaining_warning }}
            </div>
            <div v-if="table.current_order?.member_name" class="table-member">
              {{ table.current_order.member_name }}
              <el-tag v-if="table.current_order.member_discount" size="small" type="success">{{ table.current_order.member_discount }}{{ t('discount') }}</el-tag>
            </div>
            <div v-if="table.current_order?.package_name" class="table-package">
              <el-icon><Ticket /></el-icon>{{ table.current_order.package_name }}
            </div>
          </div>
        </div>
      </div>

      <div v-if="filteredAreas.length === 0 && areas.length === 0" class="empty-card">
        <el-icon :size="64"><Warning /></el-icon>
        <h3>{{ t('noAreaData') }}</h3>
        <p>{{ t('pleaseAddAreaFirst') }}</p>
        <router-link to="/area-manage">
          <el-button type="primary">{{ t('areaSettings') }}</el-button>
        </router-link>
      </div>
    </div>

    <!-- Open Table Dialog -->
    <OpenTableDialog
      v-model="openDlg"
      :table="sel"
      :members="members"
      :packages="packages"
      @success="loadData"
    />

    <!-- {{ t('CheckoutDialog') }} -->
    <el-dialog v-model="closeDlg" :title="t('closeTable')" width="420px" :close-on-click-modal="false" top="5vh" append-to-body>
      <div class="checkout-content" v-if="preview">
        <div class="checkout-header">
          <div class="checkout-table">
            <el-icon><Grid /></el-icon>
            <span>{{ sel?.name }}</span>
          </div>
        </div>

        <div class="checkout-details">
          <div class="detail-row">
            <span class="detail-label">{{ t('memberName') }}</span>
            <span class="detail-value">{{ preview.member_name || t('walkIn') }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">{{ t('startTime') }}</span>
            <span class="detail-value">{{ formatDateTime(preview.start_time) }}</span>
          </div>
          <div class="detail-row highlight">
            <span class="detail-label">{{ t('consumptionDuration') }}</span>
            <span class="detail-value time">{{ getDur(preview.start_time) }}</span>
          </div>
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
          <div class="amount-row">
            <span>{{ t('tableRate') }}</span>
            <span>¥{{ pay.total.toFixed(2) }}</span>
          </div>
          <div v-if="pay.deposit > 0" class="amount-row deposit">
            <span>{{ t('depositPaid') }}</span>
            <span class="deposit-value">-¥{{ pay.deposit.toFixed(2) }}</span>
          </div>
          <div v-if="pay.discount > 0" class="amount-row discount">
            <span>{{ t('memberDiscount') }}</span>
            <span class="discount-value">-¥{{ pay.discount.toFixed(2) }}</span>
          </div>
          <div class="amount-row total">
            <span>{{ t('actualPayment') }}</span>
            <span class="total-value">¥{{ pay.final.toFixed(2) }}</span>
          </div>
          <div v-if="pay.change > 0" class="amount-row change">
            <span>{{ t('change') }}</span>
            <span class="change-value">¥{{ pay.change.toFixed(2) }}</span>
          </div>
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
          <el-button type="warning" @click="openCancelDlg" v-if="preview?.status === '进行中'">
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

    <!-- {{ t('CancelOrderDialog') }} -->
    <el-dialog v-model="cancelDlg" :title="t('cancelOrder')" width="420px" top="5vh" append-to-body>
      <div style="display:flex;flex-direction:column;gap:12px;" v-if="preview">
        <div style="padding:12px;background:var(--bg-primary);border-radius:8px;">
          <div style="display:flex;justify-content:space-between;padding:4px 0;">
            <span style="color:var(--text-secondary)">{{ t('tableName') }}</span>
            <span style="font-weight:600;">{{ sel?.name }}</span>
          </div>
          <div style="display:flex;justify-content:space-between;padding:4px 0;">
            <span style="color:var(--text-secondary)">{{ t('consumptionDuration') }}</span>
            <span style="color:var(--accent-primary);font-weight:600;">{{ getDur(preview.start_time) }}</span>
          </div>
          <div style="display:flex;justify-content:space-between;padding:4px 0;">
            <span style="color:var(--text-secondary)">{{ t('actualPayment') }}</span>
            <span style="color:var(--accent-danger);font-weight:700;">¥{{ pay.total.toFixed(2) }}</span>
          </div>
        </div>
        <div>
          <label style="font-size:13px;color:var(--text-secondary);margin-bottom:4px;display:block;">{{ t('cancelReason') }}</label>
          <el-input v-model="cancelReason" type="textarea" :rows="2" :placeholder="t('cancelReasonPlaceholder')" />
        </div>
        <el-alert v-if="cancelRefund > 0" type="success" :closable="false">{{ t('cancelRefundTip') }}：¥{{ cancelRefund.toFixed(2) }}</el-alert>
        <el-alert v-if="cancelCharge > 0" type="warning" :closable="false">{{ t('cancelChargeTip') }}：¥{{ cancelCharge.toFixed(2) }}</el-alert>
      </div>
      <template #footer>
        <el-button @click="cancelDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="danger" @click="cfmCancel">{{ t('confirmCancel') }}</el-button>
      </template>
    </el-dialog>

    <!-- {{ t('ReceiptPreviewDialog') }} -->
    <el-dialog v-model="showReceiptPreview" :title="t('receipt')" width="360px" top="5vh" append-to-body>
      <div class="receipt-content" v-if="receiptPreview">
        <div style="text-align:center;font-weight:bold;font-size:16px;margin-bottom:8px;">{{ t('billiardHall') }}</div>
        <div style="text-align:center;color:var(--text-secondary);font-size:12px;margin-bottom:12px;">{{ new Date().toLocaleString() }}</div>
        <div style="border-top:1px dashed #ccc;margin:8px 0;"></div>
        <div style="display:flex;justify-content:space-between;padding:4px 0;">
          <span>{{ t('table') }}:</span><span>{{ receiptPreview.table }}</span>
        </div>
        <div style="display:flex;justify-content:space-between;padding:4px 0;">
          <span>{{ t('member') }}:</span><span>{{ receiptPreview.member || '-' }}</span>
        </div>
        <div style="display:flex;justify-content:space-between;padding:4px 0;">
          <span>{{ t('duration') }}:</span><span>{{ receiptPreview.duration }}{{ t('minute') }}</span>
        </div>
        <div style="border-top:1px dashed #ccc;margin:8px 0;"></div>
        <div style="display:flex;justify-content:space-between;padding:4px 0;">
          <span>{{ t('total') }}:</span><span>¥{{ receiptPreview.total?.toFixed(2) }}</span>
        </div>
        <div v-if="receiptPreview.discount" style="display:flex;justify-content:space-between;padding:4px 0;">
          <span>{{ t('discount') }}:</span><span>-¥{{ receiptPreview.discount?.toFixed(2) }}</span>
        </div>
        <div v-if="receiptPreview.deposit" style="display:flex;justify-content:space-between;padding:4px 0;">
          <span>{{ t('depositPaid') }}:</span><span>-¥{{ receiptPreview.deposit?.toFixed(2) }}</span>
        </div>
        <div style="display:flex;justify-content:space-between;padding:4px 0;font-weight:bold;">
          <span>{{ t('actualPayment') }}:</span><span>¥{{ receiptPreview.final?.toFixed(2) }}</span>
        </div>
        <div style="display:flex;justify-content:space-between;padding:4px 0;">
          <span>{{ t('paymentMethod') }}:</span><span>{{ receiptPreview.payment }}</span>
        </div>
        <div style="border-top:1px dashed #ccc;margin:8px 0;"></div>
        <div style="text-align:center;font-size:12px;color:var(--text-secondary);">{{ t('thankYou') }}！</div>
      </div>
      <template #footer>
        <el-button @click="showReceiptPreview=false">{{ t('close') }}</el-button>
        <el-button type="primary" @click="doPrintReceipt">
          <el-icon><Printer /></el-icon>{{ t('printReceipt') }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
import { ElMessage, ElMessageBox, ElButton, ElSelect, ElOption, ElDialog, ElTag, ElIcon, ElDivider, ElInput } from 'element-plus'
import { Refresh, Grid, CircleCheckFilled, Timer, Tools, Location, Warning, Check, User, Clock, Money, CircleClose, Ticket, Printer } from '@element-plus/icons-vue'
import { getTables, getAreas, getMembers, getOrderByTable, closeTable, cancelOrder, checkExpiredPackages, autoCloseExpired, printReceipt as apiPrintReceipt, getSettings, getRealtimeBillingStatus, autoCloseExhausted } from '../api'
import { t, currentLang } from '../i18n'
import OpenTableDialog from '../components/OpenTableDialog.vue'

const tables = ref([])
const areas = ref([])
const members = ref([])
const packages = ref([])
const selectedArea = ref(null)
const openDlg = ref(false)
const closeDlg = ref(false)
const sel = ref(null)
const preview = ref(null)
const pay = ref({ total: 0, discount: 0, final: 0, deposit: 0, change: 0 })
const submitting = ref(false)
const billingStatuses = ref([])
const autoCloseInterval = ref(10) // 分钟，默认10分钟
const alertedOrders = ref(new Set()) // 已弹过预警的订单，防止重复弹框

const paymentMethods = [
  { value: 'cash', label: t('cash'), icon: 'Wallet' },
  { value: 'wechat', label: t('wechat'), icon: 'ChatDotRound' },
  { value: 'alipay', label: t('alipay'), icon: 'Coin' },
]
const closePaymentMethod = ref('cash')
const cancelDlg = ref(false)
const cancelReason = ref('')

const billingRules = ref({ freeMinutes: 5, billingInterval: 30, applyRounding: true })

const calcBillMinutes = (duration, rules) => {
  const free = rules.freeMinutes || 0
  const interval = rules.billingInterval || 30
  const applyRounding = rules.applyRounding !== false
  
  if (!applyRounding) {
    return Math.max(duration - free, 0)
  }
  
  if (duration < free) return 0
  
  const billable = duration - free
  if (billable < interval) return free + interval
  
  const grace = Math.max(Math.floor(interval / 6), 1)
  const fullUnits = Math.floor(billable / interval)
  const rem = billable % interval
  const roundedUnits = rem >= grace ? fullUnits + 1 : fullUnits
  
  return free + roundedUnits * interval
}

const rnd = (n, d) => Math.round(n * Math.pow(10, d)) / Math.pow(10, d)

const calcCurrentCost = (table) => {
  const order = table.current_order
  const dur = Math.max(Math.floor((Date.now() - new Date(order.start_time.replace(' ', 'T'))) / 60000), 1)
  
  const billMin = calcBillMinutes(dur, billingRules.value)
  const area = areas.value.find(a => a.id === table.area_id)
  const rate = area?.rate_per_hour || 30
  const total = (billMin / 60) * rate
  
  if (order.member_id) {
    const m = members.value.find(x => x.id === order.member_id)
    if (m) {
      const memberDiscount = m.discount || 1
      return (total * memberDiscount).toFixed(2)
    }
  }
  return total.toFixed(2)
}

const stats = computed(() => ({
  total: tables.value.length,
  free: tables.value.filter(t => t.status === '空闲').length,
  inUse: tables.value.filter(t => t.status === '使用中').length,
  maint: tables.value.filter(t => t.status === '维护中').length,
}))

const groupedByArea = computed(() => {
  currentLang.value
  const map = new Map()
  tables.value.forEach(table => {
    const areaId = table.area_id || 0
    const area = areas.value.find(a => a.id === areaId)
    const areaName = area?.name || t('areaManage')
    const areaRate = area?.rate_per_hour || 30
    if (!map.has(areaId)) {
      map.set(areaId, { id: areaId, name: areaName, rate_per_hour: areaRate, tables: [], freeCount: 0, useCount: 0 })
    }
    const group = map.get(areaId)
    group.tables.push(table)
    if (table.status === '空闲') group.freeCount++
    if (table.status === '使用中') group.useCount++
  })
  return Array.from(map.values())
})

const filteredAreas = computed(() => {
  if (!selectedArea.value) return groupedByArea.value
  return groupedByArea.value.filter(a => a.id === selectedArea.value)
})

const getTableClass = (status) => {
  if (status === '使用中') return 'in-use'
  if (status === '维护中') return 'maint'
  return 'free'
}

const getStatusText = (status) => {
  if (status === '使用中') return t('tableInUse')
  if (status === '维护中') return t('tableMaintenance')
  return t('tableFree')
}

const getDur = (time) => {
  if (!time) return `0${t('minute')}`
  const d = Math.floor((Date.now() - new Date(time.replace(' ', 'T'))) / 60000)
  const h = Math.floor(d / 60)
  const m = d % 60
  return h > 0 ? `${h}${t('hour')} ${m}${t('minute')}` : `${d}${t('minute')}`
}

const formatDateTime = (time) => {
  if (!time) return ''
  return time.replace('T', ' ').slice(0, 16)
}

const loadData = async () => {
  try {
    tables.value = await getTables(true)
    areas.value = await getAreas()
    members.value = await getMembers()
    const settings = await getSettings()
    packages.value = settings.packages || []
    autoCloseInterval.value = settings.autoClose?.intervalMinutes || 10
    if (settings.billingRules) {
      billingRules.value.freeMinutes = settings.billingRules.freeMinutes ?? 5
      billingRules.value.billingInterval = settings.billingRules.billingInterval ?? 30
      billingRules.value.applyRounding = settings.billingRules.applyRounding ?? true
    }
  } catch (e) { console.error(e) }
}

// 获取某个台桌的实时计费状态
const getBillingStatus = (tableId) =>
  billingStatuses.value.find(b => b.table_id === tableId)

const getTimerClass = (tableId) => {
  const s = getBillingStatus(tableId)
  if (!s) return ''
  if (s.warning_level === 'exhausted') return 'timer-exhausted'
  if (s.warning_level === 'critical') return 'timer-critical'
  if (s.warning_level === 'low') return 'timer-low'
  return ''
}

const getCostClass = (tableId) => {
  const s = getBillingStatus(tableId)
  if (!s) return ''
  if (s.warning_level === 'exhausted' || s.warning_level === 'critical') return 'cost-warning'
  if (s.warning_level === 'low') return 'cost-low'
  return ''
}

// 拉取实时计费状态并处理预警/自动关台
const pollBillingStatus = async () => {
  try {
    const statuses = await getRealtimeBillingStatus()
    billingStatuses.value = statuses

    for (const s of statuses) {
      // 预警弹窗：余额不足且未弹过
      if ((s.warning_level === 'low' || s.warning_level === 'critical') && !alertedOrders.value.has(s.order_id)) {
        alertedOrders.value.add(s.order_id)
        ElMessageBox.confirm(
          `${s.table_name} 余额不足，${s.remaining_warning}，是否立即结账？`,
          '⚠️ 余额预警',
          { confirmButtonText: '立即结账', cancelButtonText: '稍后', type: 'warning' }
        ).then(async () => {
          try {
            const results = await autoCloseExhausted()
            if (results && results.length > 0) {
              ElMessage.success(`已自动结账 ${results.length} 个订单`)
              await loadData()
            }
          } catch(e) { ElMessage.error(String(e)) }
        }).catch(() => {})
      }
      // 余额耗尽：直接自动关台
      if (s.warning_level === 'exhausted') {
        try {
          await autoCloseExhausted()
          ElMessage.warning(`${s.table_name} 余额已耗尽，已自动结账`)
          await loadData()
        } catch(e) { console.error('Auto close error:', e) }
      }
    }
  } catch (e) { console.error('Poll billing status error:', e) }
}

const onTableClick = async (item) => {
  if (item.status === '维护中') return
  
  const areaRate = (() => {
    const area = areas.value.find(a => a.id === item.area_id)
    return area?.rate_per_hour || 30
  })()
  
sel.value = { ...item, rate_per_hour: areaRate }

  if (item.status === '空闲') {
    openDlg.value = true
  } else if (item.status === '使用中' && item.current_order?.member_id) {
    return router.push(`/members/${item.current_order.member_id}`)
  } else {
    try {
      preview.value = await getOrderByTable(item.id)
      const dur = Math.max(Math.floor((Date.now() - new Date(preview.value.start_time.replace(' ', 'T'))) / 60000), 1)
      let billMin = 0
      if (dur < 5) billMin = 0
      else if (dur < 30) billMin = 30
      else if (dur < 60) billMin = 60
      else {
        const hours = Math.floor(dur / 60)
        const remaining = dur % 60
        if (remaining < 5) billMin = hours * 60
        else if (remaining < 30) billMin = hours * 60 + 30
        else billMin = hours * 60 + 60
      }
      const total = rnd((billMin / 60) * sel.value.rate_per_hour, 2)
      let disc = 0
      let fin = total
      let deposit = preview.value.deposit || 0
      let change = 0

      if (preview.value.member_id) {
        const m = members.value.find(x => x.id === preview.value.member_id)
        if (m && m.discount) {
          const memberDiscount = parseFloat(m.discount)
          disc = rnd(total * (1 - memberDiscount), 2)
          fin = rnd(total - disc, 2)
        }
      }
      // 散客不享受折扣

      if (deposit > 0) {
        fin = Math.max(0, fin - deposit)
        change = deposit - (total - disc)
      }

      pay.value = { total, discount: disc, final: fin, deposit, change: Math.max(0, change) }
      closeDlg.value = true
    } catch(e) { ElMessage.error(String(e) || t('operationFailed')) }
  }
}

const cfmClose = async () => {
  try {
    console.log('[closeTable] calling with orderId:', preview.value.id, 'data:', { payment_method: closePaymentMethod.value })
    const order = await closeTable(preview.value.id, { payment_method: closePaymentMethod.value })
    ElMessage.success({ message: `${t('closeSuccess')}！¥${pay.value.final.toFixed(2)}`, grouping: true })
    closeDlg.value = false

    // 显示打印小票弹窗
    const receiptData = {
      type: 'order',
      table: preview.value.table_name || preview.value.name || '',
      member: preview.value.member_name || '',
      startTime: preview.value.start_time || '',
      endTime: order?.end_time || new Date().toISOString(),
      duration: order?.duration_minutes || preview.value.duration_minutes || 0,
      total: order?.total_amount || preview.value.total_amount || 0,
      deposit: order?.deposit || preview.value.deposit || 0,
      discount: order?.discount_amount || 0,
      final: order?.final_amount || preview.value.final_amount || 0,
      payment: closePaymentMethod.value || 'cash',
    }

    // 先显示小票预览弹窗，用户点击打印按钮后再发送打印命令
    receiptPreview.value = receiptData
    showReceiptPreview.value = true

    await loadData()
  } catch(e) {
    console.error('[closeTable] error:', e)
    ElMessage.error(String(e) || t('operationFailed'))
  }
}

// ===== 小票预览 =====
const showReceiptPreview = ref(false)
const receiptPreview = ref(null)

const doPrintReceipt = async () => {
  if (!receiptPreview.value) return
  try {
    await apiPrintReceipt({
      receipt_type: 'order',
      shop_name: t('billiardHall'),
      order_no: null,
      table_name: receiptPreview.value.table,
      member_name: receiptPreview.value.member,
      start_time: receiptPreview.value.startTime,
      end_time: receiptPreview.value.endTime,
      duration_minutes: receiptPreview.value.duration,
      total_amount: receiptPreview.value.total,
      discount_amount: receiptPreview.value.discount,
      deposit: receiptPreview.value.deposit,
      final_amount: receiptPreview.value.final,
      payment_method: receiptPreview.value.payment,
    })
    ElMessage.success(t('printSuccess'))
    showReceiptPreview.value = false
  } catch (e) {
    console.error('Print error:', e)
    ElMessage.error(t('printFailed'))
  }
}


// ===== 取消订单 =====
const cancelRefund = computed(() => {
  if (!preview.value) return 0
  const dep = preview.value.deposit || 0
  const fin = pay.value.total
  if (preview.value.member_id) {
    const m = members.value.find(x => x.id === preview.value.member_id)
    if (m) { const used = Math.min(m.balance, fin); return Math.max(0, dep - (fin - used)) }
  }
  return Math.max(0, dep - fin)
})

const cancelCharge = computed(() => {
  if (!preview.value) return 0
  if (preview.value.member_id) {
    const m = members.value.find(x => x.id === preview.value.member_id)
    if (m) return Math.min(m.balance, pay.value.total)
  }
  return 0
})

const openCancelDlg = () => { cancelReason.value = ''; cancelDlg.value = true }

const cfmCancel = async () => {
  try {
    await cancelOrder(preview.value.id, cancelReason.value || t('reasonCancel'))
    ElMessage.success(t('cancelSuccess'))
    cancelDlg.value = false
    closeDlg.value = false
    await loadData()
  } catch(e) { ElMessage.error(String(e) || t('operationFailed')) }
}

// ===== 套餐过期自动结账 =====
const checkExpiredOrders = async () => {
  try {
    const res = await checkExpiredPackages()
    if (res && res.length > 0) {
      const ids = res.map(o => o.order_id)
      const confirmed = await ElMessageBox.confirm(
        `发现 ${ids.length} 个套餐已过期，是否自动结账？`,
        '套餐过期提醒',
        { confirmButtonText: '确定', cancelButtonText: '取消', type: 'warning' }
      ).catch(() => false)
      if (confirmed) {
        await autoCloseExpired(ids)
        ElMessage.success(`已自动结账 ${ids.length} 个订单`)
        await loadData()
      }
    }
  } catch {}
}
let refreshTimer = null
let billingTimer = null

onMounted(async () => {
    await loadData()
    refreshTimer = setInterval(() => { loadData(); checkExpiredOrders() }, 30000)
    // 余额检查/自动关台：用设置中的 autoClose.intervalMinutes 频率执行
    billingTimer = setInterval(async () => {
      await pollBillingStatus()
    }, (autoCloseInterval.value || 10) * 60 * 1000)
  })
  onUnmounted(() => { clearInterval(refreshTimer); clearInterval(billingTimer) })
</script>

<style scoped>
.overview-page { display: flex; flex-direction: column; gap: 20px; }
.page-header { display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.header-right { display: flex; gap: 12px; align-items: center; }
.area-select { width: 160px; }

.stats-row { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
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
  transform: translateY(-2px);
  box-shadow: var(--shadow-card-hover);
}
.stat-card:nth-child(1) { --card-accent: var(--accent-primary); }
.stat-card:nth-child(1):hover { border-color: var(--accent-primary); }
.stat-card:nth-child(2) { --card-accent: var(--accent-success); }
.stat-card:nth-child(2):hover { border-color: var(--accent-success); }
.stat-card:nth-child(3) { --card-accent: var(--accent-danger); }
.stat-card:nth-child(3):hover { border-color: var(--accent-danger); }
.stat-card:nth-child(4) { --card-accent: var(--accent-warning); }
.stat-card:nth-child(4):hover { border-color: var(--accent-warning); }
.stat-icon { width: 44px; height: 44px; border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; font-size: 22px; box-shadow: var(--shadow-sm); }
.stat-icon.green { background: rgba(88,166,255,0.1); color: var(--accent-primary); }
.stat-icon.success { background: rgba(63,185,80,0.1); color: var(--accent-success); }
.stat-icon.danger { background: rgba(248,81,73,0.1); color: var(--accent-danger); }
.stat-icon.warning { background: rgba(210,153,34,0.1); color: var(--accent-warning); }
.stat-body { display: flex; flex-direction: column; gap: 4px; flex: 1; }
.stat-value { font-size: 26px; font-weight: 700; color: var(--text-primary); line-height: 1.2; }
.stat-label { font-size: 13px; color: var(--text-secondary); }

.areas-grid { display: flex; flex-direction: column; gap: 20px; }
.area-section {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  box-shadow: var(--shadow-card);
  transition: all var(--transition-normal);
}
.area-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-muted);
}
.area-title { display: flex; align-items: center; gap: 8px; font-size: 16px; font-weight: 600; color: var(--text-primary); }
.area-title .el-icon { font-size: 18px; color: var(--accent-primary); }
.area-title .area-rate-badge {
  font-size: 12px;
  font-weight: 500;
  color: var(--accent-primary);
  background: rgba(88,166,255,0.12);
  padding: 2px 10px;
  border-radius: 10px;
  margin-left: 4px;
  border: 1px solid rgba(88,166,255,0.2);
}
.area-summary { display: flex; gap: 8px; }

.tables-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 10px; }
.table-item {
  aspect-ratio: 1;
  background: var(--bg-primary);
  border: 2px solid var(--border-default);
  border-radius: var(--radius-md);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 3px;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
}
.table-item:hover { transform: scale(1.05); box-shadow: var(--shadow-md); }
.table-item.free { border-color: rgba(63,185,80,0.5); background: rgba(63,185,80,0.06); }
.table-item.free:hover { border-color: var(--accent-success); background: rgba(63,185,80,0.12); }
.table-item.in-use { border-color: rgba(248,81,73,0.5); background: rgba(248,81,73,0.06); }
.table-item.in-use:hover { border-color: var(--accent-danger); background: rgba(248,81,73,0.12); }
.table-item.maint { border-color: rgba(210,153,34,0.4); background: rgba(210,153,34,0.04); opacity: 0.65; }
.table-item.maint:hover { opacity: 0.85; }
.table-number { font-size: 16px; font-weight: 700; color: var(--text-primary); }
.table-status { font-size: 10px; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.5px; font-weight: 500; }
.table-item.free .table-status { color: var(--accent-success); }
.table-item.in-use .table-status { color: var(--accent-danger); }
.table-item.maint .table-status { color: var(--accent-warning); }
.table-timer { font-size: 11px; color: var(--accent-danger); font-weight: 700; font-family: var(--font-mono); }
.table-remaining {
  font-size: 9px;
  font-weight: 600;
  padding: 1px 4px;
  border-radius: 3px;
  margin-top: 2px;
}
.remaining-normal { color: var(--accent-success); background: rgba(63,185,80,0.1); }
.remaining-low { color: var(--accent-warning); background: rgba(210,153,34,0.12); }
.remaining-critical { color: #ff5722; background: rgba(255,87,34,0.15); }
.remaining-exhausted { color: #fff; background: var(--accent-danger); }

.timer-critical { color: #ff5722; }
.timer-low { color: var(--accent-warning); }
.timer-exhausted { color: var(--accent-danger); animation: pulse 1s infinite; }
.cost-warning { color: #ff5722; }
.cost-low { color: var(--accent-warning); }

@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
.table-cost { font-size: 10px; color: var(--accent-success); font-weight: 600; }

.empty-card {
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
  box-shadow: var(--shadow-card);
}
.empty-card h3 { color: var(--text-primary); margin: 0; }
.empty-card p { margin: 0 0 16px; }

/* Dialog */
.dialog-content { display: flex; flex-direction: column; gap: 16px; }
.table-info { background: var(--bg-primary); border-radius: var(--radius-lg); padding: 20px; text-align: center; }
.table-name { font-size: 24px; font-weight: 700; color: var(--text-primary); margin-bottom: 4px; }
.table-rate { font-size: 16px; color: var(--accent-primary); }
.full-width { width: 100%; }
.member-opt { display: flex; justify-content: space-between; align-items: center; padding: 4px 0; }
.opt-balance { color: var(--accent-success); font-size: 12px; }

.checkout-content { display: flex; flex-direction: column; gap: 12px; }
.checkout-row { display: flex; justify-content: space-between; align-items: center; padding: 10px 14px; background: var(--bg-primary); border-radius: var(--radius-sm); }
.checkout-row span { color: var(--text-secondary); font-size: 14px; }
.checkout-row strong { color: var(--text-primary); }
.checkout-row.highlight { background: rgba(88,166,255,0.1); }
.checkout-row .accent { color: var(--accent-primary); display: flex; align-items: center; gap: 6px; }
.checkout-row.discount { color: var(--accent-success); }
.checkout-row.discount span { color: var(--accent-success); }
.checkout-row.discount strong { color: var(--accent-success); }
.checkout-total {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background: rgba(248,81,73,0.1);
  border: 1px solid rgba(248,81,73,0.2);
  border-radius: var(--radius-md);
  margin-top: 8px;
}
.checkout-total span:first-child { color: var(--text-secondary); font-size: 14px; }
.total-value { font-size: 28px; font-weight: 700; color: var(--accent-danger); }
.pulse { animation: pulse 1.5s infinite; }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }

/* Dialog Forms */
.open-form, .close-form { display: flex; flex-direction: column; gap: 20px; }

.table-hero { display: flex; align-items: center; gap: 16px; padding: 20px; background: var(--bg-primary); border-radius: var(--radius-lg); }
.hero-icon { width: 56px; height: 56px; border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; font-size: 28px; }
.hero-icon.hall { background: rgba(63,185,80,0.15); color: var(--accent-success); }
.hero-icon.private { background: rgba(210,153,34,0.15); color: var(--accent-warning); }
.hero-icon.in-use { background: rgba(248,81,73,0.15); color: var(--accent-danger); }
.hero-info { flex: 1; display: flex; flex-direction: column; gap: 4px; }
.hero-name { font-size: 22px; font-weight: 700; color: var(--text-primary); }
.hero-rate { font-size: 14px; color: var(--accent-primary); }

.customer-section { display: flex; flex-direction: column; gap: 12px; }
.section-label { font-size: 13px; color: var(--text-secondary); font-weight: 500; }
.customer-tabs { display: flex; gap: 8px; }
.tab-btn { flex: 1; display: flex; align-items: center; justify-content: center; gap: 8px; padding: 14px; border: 1px solid var(--border-default); border-radius: var(--radius-md); background: var(--bg-primary); color: var(--text-secondary); cursor: pointer; transition: all 0.2s; }
.tab-btn:hover { border-color: var(--accent-primary); color: var(--accent-primary); }
.tab-btn.active { border-color: var(--accent-primary); background: rgba(88,166,255,0.1); color: var(--accent-primary); }

.walkin-form :deep(.el-input__wrapper) { padding: 4px 16px; }
.member-form { display: flex; flex-direction: column; gap: 12px; }
.member-search { width: 100%; }
:deep(.member-search .el-input__wrapper) { padding: 4px 16px; }

.member-suggestion { padding: 8px 0; }
.suggestion-main { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; }
.suggestion-name { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.suggestion-sub { display: flex; justify-content: space-between; font-size: 12px; color: var(--text-tertiary); }
.suggestion-balance { color: var(--accent-success); }

.member-card { display: flex; align-items: center; gap: 12px; padding: 16px; background: var(--bg-primary); border: 1px solid var(--accent-success); border-radius: var(--radius-md); }
.member-card.selected { border-color: var(--accent-success); }
.member-info { flex: 1; display: flex; flex-direction: column; gap: 4px; }
.member-name-row { display: flex; align-items: center; gap: 8px; }
.member-name { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.member-phone { font-size: 12px; color: var(--text-tertiary); }
.member-balance { display: flex; align-items: center; gap: 8px; margin-top: 4px; }
.balance-label { font-size: 11px; color: var(--text-tertiary); }
.balance-value { font-size: 16px; font-weight: 700; color: var(--accent-success); }

.discount-tip { display: flex; align-items: center; gap: 8px; padding: 12px 16px; background: rgba(63,185,80,0.1); border: 1px solid rgba(63,185,80,0.2); border-radius: var(--radius-md); color: var(--accent-success); font-size: 13px; }

.realtime-cost { background: var(--bg-primary); border-radius: var(--radius-lg); overflow: hidden; }
.cost-header { display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; background: rgba(248,81,73,0.1); border-bottom: 1px solid rgba(248,81,73,0.2); font-size: 13px; font-weight: 600; color: var(--accent-danger); }
.cost-update { font-size: 11px; font-weight: normal; opacity: 0.7; }
.cost-main { padding: 16px; display: flex; flex-direction: column; gap: 12px; }
.cost-item { display: flex; justify-content: space-between; align-items: center; }
.cost-label { font-size: 14px; color: var(--text-secondary); }
.cost-value { font-size: 18px; font-weight: 700; }
.cost-value.time { color: var(--accent-primary); }
.cost-value.money { color: var(--text-primary); }
.cost-item.discount .cost-value { color: var(--accent-success); }
.cost-total { display: flex; justify-content: space-between; align-items: center; padding: 16px; background: rgba(248,81,73,0.15); font-size: 14px; font-weight: 600; color: var(--text-primary); }
.total-value { font-size: 32px; font-weight: 700; color: var(--accent-danger); }

.order-details { display: flex; flex-direction: column; gap: 8px; }
.detail-row { display: flex; justify-content: space-between; align-items: center; padding: 10px 14px; background: var(--bg-primary); border-radius: var(--radius-sm); }
.detail-label { display: flex; align-items: center; gap: 8px; font-size: 13px; color: var(--text-secondary); }
.detail-value { font-size: 13px; color: var(--text-primary); }

@media (max-width: 1024px) {
  .stats-row { grid-template-columns: repeat(2, 1fr); }
}
@media (max-width: 768px) {
  .stats-row { grid-template-columns: 1fr; }
  .tables-grid { grid-template-columns: repeat(auto-fill, minmax(80px, 1fr)); }
}

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
.member-balance-display { display: flex; flex-direction: column; align-items: flex-end; gap: 2px; }
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

/* Checkout Dialog */
.checkout-content { display: flex; flex-direction: column; gap: 12px; }
.checkout-header { padding: 12px; background: var(--bg-primary); border-radius: var(--radius-md); text-align: center; }
.checkout-table { display: flex; align-items: center; justify-content: center; gap: 8px; font-size: 18px; font-weight: 700; color: var(--text-primary); }
.checkout-table .el-icon { font-size: 20px; color: var(--accent-primary); }

.checkout-details { display: flex; flex-direction: column; gap: 6px; }
.detail-row {
  display: flex;
  justify-content: space-between;
  padding: 8px 12px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
}
.detail-label { color: var(--text-secondary); font-size: 13px; }
.detail-value { color: var(--text-primary); font-weight: 500; }
.detail-row.highlight { background: rgba(88,166,255,0.1); }
.detail-row .time { color: var(--accent-primary); font-size: 14px; font-weight: 600; }

.amount-section { display: flex; flex-direction: column; gap: 6px; }
.amount-row { display: flex; justify-content: space-between; font-size: 13px; color: var(--text-secondary); }
.amount-row.total { padding: 10px; background: rgba(248,81,73,0.1); border-radius: var(--radius-sm); font-weight: 600; color: var(--text-primary); }
.amount-row.deposit { color: var(--accent-success); }
.amount-row.change { color: var(--accent-warning); }
.total-value { font-size: 24px; font-weight: 700; color: var(--accent-danger); }
.deposit-value { color: var(--accent-success); }
.change-value { color: var(--accent-warning); }

.close-payment-section { margin-top: 16px; padding-top: 16px; border-top: 1px solid var(--border-muted); }
.close-payment-section .payment-label { font-size: 14px; font-weight: 600; margin-bottom: 12px; color: var(--text-primary); }
.close-payment-section .payment-btns { display: flex; gap: 10px; }
.close-payment-section .payment-btns .el-button { flex: 1; }

.payment-btns { display: flex; gap: 12px; justify-content: flex-end; }
.payment-btns .el-button { flex: 1; }

:deep(.el-divider) { margin: 12px 0; border-color: var(--border-muted); }

.table-member {
  font-size: 10px;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  gap: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}
.table-member .el-tag { transform: scale(0.8); }
.table-package {
  font-size: 10px;
  color: var(--accent-primary);
  display: flex;
  align-items: center;
  gap: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}
</style>
