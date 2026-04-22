<template>
  <div class="dashboard">
    <div class="quick-sell-section">
      <div class="quick-sell-header">
        <span class="section-title">
          <el-icon><Goods /></el-icon>
          {{ t('quickSell') }}
        </span>
        <el-button size="small" @click="showQuickSell = true" type="primary">
          <el-icon><ShoppingCart /></el-icon>
          {{ t('openPos') }}
        </el-button>
      </div>
      <div class="quick-products-grid">
        <div
          v-for="prod in displayProducts.slice(0, 12)"
          :key="prod.id"
          :class="['quick-product-card', { disabled: prod.stock <= 0, 'in-cart': getCartItem(prod.id) }]"
          @click="addToQuickCart(prod)"
        >
          <div class="qp-header">
            <span class="qp-name">{{ prod.name }}</span>
            <span class="qp-badge" v-if="getCartItem(prod.id)">x{{ getCartItem(prod.id).quantity }}</span>
          </div>
          <div class="qp-price">¥{{ prod.price?.toFixed(1) }}</div>
          <div class="qp-footer">
            <span class="qp-stock" :class="prod.stock < 5 ? 'low' : ''">{{ prod.stock }}{{ prod.unit }}</span>
            <span class="qp-category-tag" v-if="prod.category_name">{{ prod.category_name }}</span>
          </div>
        </div>
        <div class="quick-cart-summary" @click="showQuickSell = true" v-if="quickCart.length > 0">
          <div class="cart-items-count">{{ quickCart.length }} {{ t('items') }}</div>
          <div class="cart-total">{{ t('total') }}: ¥{{ quickCartTotal.toFixed(1) }}</div>
          <el-button type="primary" size="small">{{ t('checkoutNow') }}</el-button>
        </div>
      </div>
    </div>

    <div class="main-grid">
      <div class="card tables-overview">
        <div class="card-header">
          <span class="card-title">{{ t('tables') }} {{ t('areaOverview') }}</span>
          <router-link to="/tables" class="card-link">{{ t('viewAll') }}</router-link>
        </div>
        <div class="area-groups">
          <div v-for="group in groupedTables" :key="group.areaId" class="area-group">
            <div class="area-header">
              <span class="area-name">
                <el-icon><Location /></el-icon>
                {{ group.areaName }}
                <span class="area-rate-badge">¥{{ group.areaRate }}/{{ t('hour') }}</span>
              </span>
              <div class="area-stats">
                <span class="stat-pill green">{{ group.freeCount }} {{ t('tableFree') }}</span>
                <span class="stat-pill red">{{ group.useCount }} {{ t('tableInUse') }}</span>
              </div>
            </div>
            <div class="table-grid">
              <div
                v-for="item in group.tables"
                :key="item.id"
                class="table-mini"
                :class="getTableStatusClass(item.status)"
                @click="onTableClick(item)"
              >
                <span class="table-num">{{ item.name }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <el-dialog v-model="openDlg" :title="`${t('openTable')} - ${sel?.name}`" width="720px" :close-on-click-modal="false" class="open-table-dialog">
      <div class="open-form" v-if="sel">
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
              <el-tag type="success" size="large">{{ getMemberDiscount(selectedMember) }}{{ t('discount') }}</el-tag>
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
                  <el-button v-for="amt in [50, 100, 200, 500]" :key="amt"
                    :class="['amount-btn', { active: walkinForm.deposit === amt }]" @click="walkinForm.deposit = amt">
                    ¥{{ amt }}
                  </el-button>
                </div>
                <div class="custom-deposit">
                  <el-input-number v-model="walkinForm.deposit" :min="0" :step="10" controls-position="right" />
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
        <el-button size="large" @click="openDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="success" size="large" @click="cfmOpen" :loading="submitting">
          <el-icon><Check /></el-icon>
          {{ t('confirmOpen') }}
        </el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="closeDlg" :title="t('closeTable')" width="420px" :close-on-click-modal="false">
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

    <el-dialog v-model="cancelDlg" :title="t('cancelOrder')" width="420px">
      <div style="display:flex;flex-direction:column;gap:12px;">
        <div style="padding:12px;background:var(--bg-primary);border-radius:8px;">
          <div style="display:flex;justify-content:space-between;padding:4px 0;">
            <span style="color:var(--text-secondary)">{{ t('tableName') }}</span>
            <span style="font-weight:600;">{{ sel?.name }}</span>
          </div>
          <div style="display:flex;justify-content:space-between;padding:4px 0;">
            <span style="color:var(--text-secondary)">{{ t('consumptionDuration') }}</span>
            <span style="color:var(--accent-primary);font-weight:600;">{{ preview ? getDur(preview.start_time) : '' }}</span>
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
        <el-alert v-if="cancelRefund > 0" type="success" :closable="false">
          {{ t('cancelRefundTip') }}：¥{{ cancelRefund.toFixed(2) }}
        </el-alert>
        <el-alert v-if="cancelCharge > 0" type="warning" :closable="false">
          {{ t('cancelChargeTip') }}：¥{{ cancelCharge.toFixed(2) }}
        </el-alert>
      </div>
      <template #footer>
        <el-button @click="cancelDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="danger" @click="cfmCancel">{{ t('confirmCancel') }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="showQuickSell" :title="t('quickSell')" width="900px" :close-on-click-modal="false" class="pos-dialog">
      <div class="pos-layout">
        <div class="pos-products">
          <div class="pos-search">
            <el-input
              v-model="quickSearchKeyword"
              :placeholder="t('searchProduct')"
              clearable
              size="large"
            >
              <template #prefix><el-icon><Search /></el-icon></template>
            </el-input>
          </div>
          <div class="pos-products-grid">
            <div
              v-for="prod in filteredQuickProducts"
              :key="prod.id"
              :class="['pos-product-card', { disabled: prod.stock <= 0 }]"
              @click="addToQuickCart(prod)"
            >
              <div class="pos-prod-name">{{ prod.name }}</div>
              <div class="pos-prod-price">¥{{ prod.price?.toFixed(1) }}</div>
              <div class="pos-prod-stock" :class="prod.stock < 5 ? 'low' : ''">{{ prod.stock }}{{ prod.unit }}</div>
            </div>
            <div v-if="filteredQuickProducts.length === 0" class="pos-empty">
              <el-icon :size="48"><Goods /></el-icon>
              <span>{{ t('noProducts') }}</span>
            </div>
          </div>
        </div>

        <div class="pos-cart">
          <div class="pos-cart-header">
            <span><el-icon><ShoppingCart /></el-icon> {{ t('cart') }}</span>
            <el-button size="small" text type="danger" @click="clearQuickCart" v-if="quickCart.length > 0">
              {{ t('clearCart') }}
            </el-button>
          </div>

          <div class="pos-cart-items" v-if="quickCart.length > 0">
            <div v-for="(item, index) in quickCart" :key="index" class="pos-cart-item">
              <div class="pci-info">
                <span class="pci-name">{{ item.name }}</span>
                <span class="pci-price">¥{{ item.price?.toFixed(1) }}</span>
              </div>
              <div class="pci-controls">
                <el-button size="small" @click="decreaseQuickItem(index)" :disabled="item.quantity <= 1" circle>
                  <el-icon><Minus /></el-icon>
                </el-button>
                <span class="pci-qty">{{ item.quantity }}</span>
                <el-button size="small" @click="increaseQuickItem(index)" :disabled="item.quantity >= item.stock" circle>
                  <el-icon><Plus /></el-icon>
                </el-button>
                <el-button size="small" type="danger" @click="removeQuickItem(index)" circle>
                  <el-icon><Delete /></el-icon>
                </el-button>
              </div>
              <span class="pci-subtotal">¥{{ (item.price * item.quantity).toFixed(1) }}</span>
            </div>
          </div>

          <div class="pos-cart-empty" v-else>
            <el-icon :size="48"><ShoppingCart /></el-icon>
            <span>{{ t('cartEmpty') }}</span>
          </div>

          <div class="pos-cart-footer">
            <div class="pos-cart-total">
              <span>{{ t('totalAmount') }}</span>
              <span class="pos-total-value">¥{{ quickCartTotal.toFixed(1) }}</span>
            </div>

            <div class="pos-payment">
              <div class="pos-payment-label">{{ t('paymentMethod') }}</div>
              <div class="pos-payment-btns">
                <el-button
                  v-for="pm in paymentMethods"
                  :key="pm.value"
                  :type="quickPaymentMethod === pm.value ? 'primary' : ''"
                  @click="quickPaymentMethod = pm.value"
                >
                  <el-icon><component :is="pm.icon" /></el-icon>
                  {{ pm.label }}
                </el-button>
              </div>
            </div>

            <el-button type="success" size="large" class="pos-checkout-btn" :disabled="quickCart.length === 0" :loading="saleSubmitting" @click="checkoutQuickSale">
              <el-icon><Money /></el-icon>
              {{ t('checkout') }} ¥{{ quickCartTotal.toFixed(1) }}
            </el-button>
          </div>
        </div>
      </div>
    </el-dialog>

    <el-dialog v-model="showReceipt" :title="t('receipt')" width="360px" class="receipt-dialog">
      <div class="receipt-content" ref="receiptRef">
        <div class="title">{{ shopName || t('billiardHall') }}</div>
        <div class="time">{{ currentReceiptTime }}</div>
        <div class="divider"></div>
        <template v-if="receiptData?.type === 'order'">
          <div class="row"><span class="label">{{ t('tableNumber') }}:</span><span class="value">{{ receiptData.table }}</span></div>
          <div class="row"><span class="label">{{ t('memberName') }}:</span><span class="value">{{ receiptData.member || t('walkIn') }}</span></div>
          <div class="row"><span class="label">{{ t('startTime') }}:</span><span class="value">{{ formatTimeShort(receiptData.startTime) }}</span></div>
          <div class="row"><span class="label">{{ t('endTime') }}:</span><span class="value">{{ formatTimeShort(receiptData.endTime) }}</span></div>
          <div class="row"><span class="label">{{ t('consumptionDuration') }}:</span><span class="value">{{ receiptData.duration }} {{ t('minute') }}</span></div>
        </template>
        <template v-else-if="receiptData?.type === 'sale'">
          <div v-for="(item, idx) in receiptData.items" :key="idx" class="row">
            <span class="label">{{ item.name }} x{{ item.qty }}</span>
            <span class="value">¥{{ (item.qty * item.price).toFixed(2) }}</span>
          </div>
        </template>
        <div class="divider"></div>
        <div class="row total"><span class="label">{{ t('total') }}</span><span class="value">¥{{ (receiptData?.total || receiptData?.final || 0).toFixed(2) }}</span></div>
        <div v-if="receiptData?.deposit" class="row"><span class="label">{{ t('deposit') }}</span><span class="value">¥{{ receiptData.deposit.toFixed(2) }}</span></div>
        <div v-if="receiptData?.discount" class="row"><span class="label">{{ t('discount') }}</span><span class="value">¥{{ receiptData.discount.toFixed(2) }}</span></div>
        <div class="row"><span class="label">{{ t('paymentMethod') }}</span><span class="value">{{ getPaymentName(receiptData?.payment) }}</span></div>
        <div v-if="receiptData?.final" class="row total"><span class="label">{{ t('actualPayment') }}</span><span class="value">¥{{ receiptData.final.toFixed(2) }}</span></div>
        <div class="divider"></div>
        <div class="footer">{{ t('thankYou') }}</div>
        <div class="cut-mark">- - - {{ t('cutMark') }} - - -</div>
      </div>
      <template #footer>
        <el-button @click="showReceipt = false">{{ t('close') }}</el-button>
        <el-button type="primary" @click="printReceipt">
          <el-icon><Printer /></el-icon>
          {{ t('printReceipt') }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { ElMessage, ElIcon, ElDialog, ElButton, ElSelect, ElOption, ElDivider, ElAutocomplete, ElAvatar, ElInputNumber, ElSwitch, ElInput } from 'element-plus'
import { Grid, User, Money, Clock, Location, Warning, TrendCharts, DataLine, Timer, UserFilled, Avatar, Search, Close, Phone, Check, House, Ticket, Goods, Plus, More, Printer, Coin, ChatDotRound, Wallet, ShoppingCart, Minus, Delete, CircleClose } from '@element-plus/icons-vue'
import { getRevenueTrend, getTableUsage, getHourlyRevenue, getTables, getAreas, getMembers, getOrderByTable, openTable, closeTable, getSettings, checkExpiredPackages, autoCloseExpired, autoCloseExhausted, getInventory, saleBatch, cancelOrder, realtimeCheck, printReceipt as apiPrintReceipt } from '../api'
import { t, currentLang } from '../i18n'

const trend = ref([])
const usage = ref([])
const hourly = ref([])
const tables = ref([])
const areas = ref([])
const members = ref([])
const packages = ref([])
const shopName = ref('')
let timer = null
let realtimeTimer = null

const openDlg = ref(false)
const closeDlg = ref(false)
const cancelDlg = ref(false)
const cancelReason = ref('')
const sel = ref(null)
const preview = ref(null)
const pay = ref({ total: 0, discount: 0, final: 0, deposit: 0, change: 0 })
const submitting = ref(false)

const customerType = ref('walkin')
const memberSearch = ref('')
const selectedMember = ref(null)
const selectedPackage = ref(null)
const walkinForm = ref({ name: '', phone: '', useDeposit: false, deposit: 100 })

const allProducts = ref([])
const showQuickSell = ref(false)
const quickSearchKeyword = ref('')
const productSearch = ref('')
const quickCart = ref([])
const quickPaymentMethod = ref('cash')
const closePaymentMethod = ref('cash')
const saleSubmitting = ref(false)

const filteredQuickProducts = computed(() => {
  if (!quickSearchKeyword.value) return allProducts.value || []
  const kw = quickSearchKeyword.value.toLowerCase()
  return (allProducts.value || []).filter(p =>
    p.name.toLowerCase().includes(kw) ||
    (p.category_name && p.category_name.toLowerCase().includes(kw))
  )
})

const displayProducts = computed(() => {
  return (allProducts.value || []).slice(0, 12)
})

const quickCartTotal = computed(() => {
  return quickCart.value.reduce((sum, item) => sum + item.price * item.quantity, 0)
})

const getCartItem = (prodId) => {
  return quickCart.value.find(c => c.id === prodId)
}

const addToQuickCart = (prod) => {
  if (prod.stock <= 0) return ElMessage.warning(t('lowStock'))
  const existing = quickCart.value.find(c => c.id === prod.id)
  if (existing) {
    if (existing.quantity >= prod.stock) return ElMessage.warning(t('lowStock'))
    existing.quantity++
  } else {
    quickCart.value.push({
      id: prod.id,
      name: prod.name,
      price: prod.price,
      quantity: 1,
      stock: prod.stock
    })
  }
}

const increaseQuickItem = (index) => {
  const item = quickCart.value[index]
  if (item.quantity >= item.stock) return ElMessage.warning(t('lowStock'))
  item.quantity++
}

const decreaseQuickItem = (index) => {
  if (quickCart.value[index].quantity > 1) {
    quickCart.value[index].quantity--
  }
}

const removeQuickItem = (index) => {
  quickCart.value.splice(index, 1)
}

const clearQuickCart = () => {
  quickCart.value = []
}

const checkoutQuickSale = async () => {
  if (quickCart.value.length === 0) return
  saleSubmitting.value = true
  try {
    const items = quickCart.value.map(item => ({
      product_id: item.id,
      quantity: item.quantity
    }))
    await saleBatch({
      items,
      payment_method: quickPaymentMethod.value
    })
    ElMessage.success(t('saveSuccess'))
    receiptData.value = {
      type: 'sale',
      items: quickCart.value.map(i => ({ name: i.name, qty: i.quantity, price: i.price })),
      total: quickCartTotal.value,
      payment: quickPaymentMethod.value || 'cash',
    }
    currentReceiptTime.value = new Date().toLocaleString()
    showReceipt.value = true
    quickCart.value = []
    showQuickSell.value = false
    await loadProducts()
  } catch(e) {
    ElMessage.error(e.response?.data?.error || t('operationFailed'))
  } finally {
    saleSubmitting.value = false
  }
}

const showReceipt = ref(false)
const receiptData = ref(null)
const currentReceiptTime = ref('')
const receiptRef = ref(null)
const saleItem = ref(null)
const saleQuantity = ref(1)
const salePrice = ref(0)

const paymentMethods = [
  { value: 'cash', label: t('cash'), icon: 'Wallet' },
  { value: 'wechat', label: t('wechat'), icon: 'ChatDotRound' },
  { value: 'alipay', label: t('alipay'), icon: 'Coin' },
]

const getPaymentName = (method) => {
  const pm = paymentMethods.find(p => p.value === method)
  return pm ? pm.label : method
}

const formatTime = (iso) => {
  if (!iso) return ''
  try { return new Date(iso).toLocaleString() } catch { return iso }
}

const formatTimeShort = (iso) => {
  if (!iso) return ''
  try {
    const d = new Date(iso)
    return `${d.getFullYear()}-${String(d.getMonth()+1).padStart(2,'0')}-${String(d.getDate()).padStart(2,'0')} ${String(d.getHours()).padStart(2,'0')}:${String(d.getMinutes()).padStart(2,'0')}`
  } catch { return iso }
}

const printReceipt = async () => {
  try {
    const req = {
      receipt_type: receiptData.value?.type === 'sale' ? 'sale' : 'order',
      shop_name: shopName.value || t('billiardHall'),
      order_no: null,
      table_name: receiptData.value?.table || null,
      member_name: receiptData.value?.member || null,
      start_time: receiptData.value?.startTime || null,
      end_time: receiptData.value?.endTime || null,
      duration_minutes: receiptData.value?.duration || null,
      items: receiptData.value?.type === 'sale' ? receiptData.value.items?.map(i => ({
        name: i.name,
        quantity: i.qty,
        price: i.price
      })) : null,
      total_amount: receiptData.value?.total || receiptData.value?.final || 0,
      discount_amount: receiptData.value?.discount || null,
      deposit: receiptData.value?.deposit || null,
      final_amount: receiptData.value?.final || receiptData.value?.total || 0,
      payment_method: receiptData.value?.payment || null,
    }
    const res = await apiPrintReceipt(req)
    if (res && res.success) {
      ElMessage.success(t('printSuccess') || '打印成功')
    } else {
      ElMessage.warning(res?.message || t('printFailed'))
      fallbackPrint()
    }
  } catch (e) {
    console.warn('Silent print failed, fallback to browser print:', e)
    fallbackPrint()
  }
}

const fallbackPrint = () => {
  const printContent = receiptRef.value
  if (!printContent) return

  const iframe = document.createElement('iframe')
  iframe.style.position = 'fixed'
  iframe.style.right = '0'
  iframe.style.bottom = '0'
  iframe.style.width = '0'
  iframe.style.height = '0'
  iframe.style.border = '0'
  document.body.appendChild(iframe)

  const doc = iframe.contentWindow.document
  doc.open()
  doc.write(`
    <html>
    <head><title>{{ t('receipt') }}</title>
      <style>
        @page { size: 80mm auto; margin: 0; }
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: "Courier New", "SimSun", "Microsoft YaHei", monospace; font-size: 12px; padding: 4px 8px; width: 80mm; max-width: 80mm; overflow-x: hidden; }
        .title { font-size: 16px; font-weight: bold; text-align: center; margin-bottom: 4px; letter-spacing: 2px; }
        .time { text-align: center; font-size: 11px; margin-bottom: 6px; }
        .divider { border-top: 1px dashed #000; margin: 6px 0; }
        .row { display: flex; justify-content: space-between; align-items: baseline; padding: 2px 0; }
        .row .label { flex: 0 0 auto; white-space: nowrap; margin-right: 8px; }
        .row .value { flex: 1 1 auto; text-align: right; word-break: keep-all; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
        .total { font-size: 14px; font-weight: bold; }
        .total .label { font-weight: bold; }
        .total .value { font-weight: bold; }
        .footer { text-align: center; margin-top: 10px; font-size: 12px; letter-spacing: 2px; }
        .cut-mark { border-top: 2px dashed #000; border-bottom: 2px dashed #000; margin: 12px 0; padding: 4px 0; text-align: center; font-size: 10px; color: #666; }
      </style>
    </head>
    <body>${printContent.innerHTML}</body>
    </html>
  `)
  doc.close()

  iframe.contentWindow.focus()
  iframe.contentWindow.print()
  setTimeout(() => { document.body.removeChild(iframe) }, 1000)
}

const loadProducts = async () => {
  try {
    const items = await getInventory({})
    allProducts.value = (Array.isArray(items) ? items : []).map(item => ({
      id: item.id,
      name: item.name,
      category: item.category_name || '',
      category_name: item.category_name || '',
      price: item.price,
      stock: item.quantity,
      unit: item.unit || '个'
    }))
    if (allProducts.value.length === 0) {
      ElMessage.warning(t('noProducts'))
    }
  } catch(e) {
    console.error('[Dashboard] loadInventory error:', e)
    ElMessage.error(t('loadProductsFailed'))
    allProducts.value = []
  }
}

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

  // 跨天判断
  const [sh, sm] = startTime ? startTime.split(':').map(Number) : [0, 0]
  const [eh, em] = endTime ? endTime.split(':').map(Number) : [23, 59]
  const startMinutes = sh * 60 + sm
  const endMinutes = eh * 60 + em

  if (endMinutes > startMinutes) {
    // 不跨天：直接区间判断
    if (startTime && currentTime < startMinutes) return false
    if (endTime && currentTime > endMinutes) return false
  } else {
    // 跨天（如23:00-06:00）：currentTime >= startTime 或 currentTime <= endMinutes
    if (startTime && currentTime < startMinutes && currentTime > endMinutes) return false
    if (endTime && currentTime > endMinutes && currentTime < startMinutes) return false
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

const walkinName = computed(() => {
  if (customerType.value === 'walkin' && walkinForm.value.name) {
    return walkinForm.value.name
  }
  return ''
})

const maxRevenue = computed(() => Math.max(...(trend.value || []).map(x => x.revenue), 1))
const avgHourly = computed(() => (hourly.value && Array.isArray(hourly.value) && hourly.value.length) ? hourly.value.reduce((a,b) => a+b.revenue,0) / hourly.value.length : 0)

const todayTotalRevenue = computed(() => (hourly.value && Array.isArray(hourly.value)) ? hourly.value.reduce((sum, h) => sum + h.revenue, 0) : 0)
const todayOrderCount = computed(() => {
  return (hourly.value && Array.isArray(hourly.value)) ? hourly.value.reduce((sum, h) => sum + (h.order_count || 0), 0) : 0
})
const peakHour = computed(() => {
  if (!hourly.value || !Array.isArray(hourly.value) || !hourly.value.length) return '-'
  const peak = hourly.value.reduce((max, h) => h.revenue > max.revenue ? h : max, hourly.value[0])
  return peak.revenue > 0 ? `${peak.hour}:00` : '-'
})
const avgOrderValue = computed(() => {
  const total = todayTotalRevenue.value
  const count = todayOrderCount.value
  return count > 0 ? total / count : 0
})

const barHeight = (rev) => Math.max(Math.round((rev / maxRevenue.value) * 100), 4)

const getTableStatusClass = (status) => {
  if (status === '使用中') return 'in-use'
  if (status === '维护中') return 'maint'
  return 'free'
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
  if (!member || !member.discount || member.discount >= 1) return ''
  return (member.discount * 10).toFixed(1)  // 0.8 → "8.0"
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

const groupedTables = computed(() => {
  currentLang.value
  const groups = []
  const areaMap = new Map()
  const areaRateMap = new Map()
  areas.value.forEach(a => {
    areaMap.set(a.id, a.name)
    areaRateMap.set(a.id, a.rate_per_hour)
  })

  const grouped = {}
  tables.value.forEach(table => {
    const areaId = table.area_id || 0
    if (!grouped[areaId]) {
      grouped[areaId] = {
        areaId,
        areaName: areaId === 0 || !areaMap.has(areaId) ? t('areaManage') : areaMap.get(areaId),
        areaRate: areaId === 0 || !areaRateMap.has(areaId) ? 30 : areaRateMap.get(areaId),
        tables: [],
        freeCount: 0,
        useCount: 0,
      }
    }
    grouped[areaId].tables.push(table)
    if (table.status === '空闲') grouped[areaId].freeCount++
    if (table.status === '使用中') grouped[areaId].useCount++
  })

  Object.values(grouped).forEach(g => {
    groups.push(g)
  })

  return groups
})

const onTableClick = async (item) => {
  if (item.status === '维护中') return
  
  const areaRate = (() => {
    const area = areas.value.find(a => a.id === item.area_id)
    return area?.rate_per_hour || 30
  })()
  const finalRate = areaRate
  
  sel.value = { ...item, rate_per_hour: finalRate }

  if (item.status === '空闲') {
    customerType.value = 'walkin'
    memberSearch.value = ''
    selectedMember.value = null
    selectedPackage.value = null
    walkinForm.value = { name: '', phone: '', useDeposit: false, deposit: 100 }
    openDlg.value = true
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
      const total = rnd((billMin / 60) * finalRate, 2)
      let disc = 0
      let fin = total
      let deposit = preview.value.deposit || 0
      let change = 0

      if (preview.value.member_id) {
        const m = members.value.find(x => x.id === preview.value.member_id)
        if (m) {
          const memberDayFactor = 1  // 会员日折扣由后端决定，前端只应用会员折扣
          disc = rnd(total * (1 - m.discount), 2)
          fin = rnd(total - disc, 2)
        }
      }
      // 散客不享受任何折扣

      if (deposit > 0) {
        fin = Math.max(0, fin - deposit)
        change = deposit - (total - disc)
      }

      pay.value = { total, discount: disc, final: fin, deposit, change: Math.max(0, change) }
      closeDlg.value = true
    } catch(e) {
      ElMessage.error(e.response?.data?.error || t('operationFailed'))
    }
  }
}


// ===== 取消订单 =====
const cancelRefund = computed(() => {
  if (!preview.value) return 0
  const dep = preview.value.deposit || 0
  const fin = pay.value.total
  if (preview.value.member_id) {
    const m = members.value.find(x => x.id === preview.value.member_id)
    if (m) {
      const balance_used = Math.min(m.balance, fin)
      const remaining = fin - balance_used
      return Math.max(0, dep - remaining)
    }
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
const closeCancelDlg = () => { cancelDlg.value = false }

const cfmCancel = async () => {
  try {
    await cancelOrder(preview.value.id, cancelReason.value || '用户取消')
    ElMessage.success(t('cancelSuccess'))
    cancelDlg.value = false
    closeDlg.value = false
    await load()
  } catch(e) {
    ElMessage.error(e.response?.data?.error || t('operationFailed'))
  }
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

    await openTable({
      table_id: sel.value.id,
      member_id: memberId,
      customer_name: walkinForm.value.name || null,
      customer_phone: walkinForm.value.phone || null,
      deposit: deposit,
      package_id: selectedPackage.value?.id || null
    })

    ElMessage.success({ message: `${sel.value.name} ${t('openSuccess')}！`, grouping: true })
    openDlg.value = false
    await load()
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
  if (!preview.value || !preview.value.id) {
    ElMessage.error(t('orderNotFound'))
    return
  }
  submitting.value = true
  try {
    const order = await closeTable(preview.value.id, { payment_method: closePaymentMethod.value })
    ElMessage.success(t('closeTableSuccess'))
    closeDlg.value = false
    receiptData.value = {
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
    currentReceiptTime.value = new Date().toLocaleString()
    showReceipt.value = true
    await load()
  } catch(e) {
    console.error('Close table error:', e)
    ElMessage.error(e.message || e.response?.data?.error || t('operationFailed'))
  } finally {
    submitting.value = false
  }
}

const load = async () => {
  try {
    trend.value = await getRevenueTrend(7)
    usage.value = await getTableUsage()
    hourly.value = await getHourlyRevenue()
    tables.value = await getTables(true)
    areas.value = await getAreas()
    members.value = await getMembers()
    const settings = await getSettings()
    packages.value = settings.packages || []
    
    await loadProducts()
    
    checkExpiredOrders()
  } catch (e) { console.error(e) }
}

const quickSale = async (prod) => {
  if (prod.stock <= 0) return ElMessage.warning(t('lowStock'))
  try {
    await saleBatch({
      items: [{
        product_id: prod.id,
        quantity: 1
      }],
      payment_method: 'cash'
    })
    ElMessage.success(t('saveSuccess'))
    prod.stock--
  } catch(e) {
    ElMessage.error(e.response?.data?.error || t('operationFailed'))
  }
}

const checkExpiredOrders = async () => {
  try {
    const res = await checkExpiredPackages()
    if (res && res.length > 0) {
      const orderIds = res.map(o => o.order_id)
      const confirmed = await ElMessageBox.confirm(
        `发现 ${orderIds.length} 个套餐已过期，是否自动结账？`,
        '套餐过期提醒',
        { confirmButtonText: '确定', cancelButtonText: '取消', type: 'warning' }
      ).catch(() => false)
      if (confirmed) {
        await autoCloseExpired(orderIds)
        ElMessage.success(`已自动结账 ${orderIds.length} 个订单`)
        load()
      }
    }
  } catch (e) {
    if (e !== 'cancel') console.error('[套餐过期检查失败]', e)
  }
}

onMounted(() => {
    load()
    timer = setInterval(load, 60000)
    realtimeTimer = setInterval(async () => {
      try { await realtimeCheck(30) } catch {}
      try { const closed = await autoCloseExhausted(); if (closed && closed.length > 0) { ElMessage.warning(`余额不足自动关台: ${closed.length} 桌`); load(); } } catch {}
    }, 30 * 60 * 1000)
  })
onUnmounted(() => { clearInterval(timer); clearInterval(realtimeTimer) })
</script>

<style scoped>
.dashboard { display: flex; flex-direction: column; gap: 24px; }

.main-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  box-shadow: var(--shadow-card);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.card-link {
  font-size: 12px;
  color: var(--accent-primary);
  text-decoration: none;
}

.card-link:hover { text-decoration: underline; }

.card-icon { font-size: 18px; color: var(--text-tertiary); }

.tables-overview { grid-column: 1 / -1; }

.quick-sell-section { background: var(--bg-secondary); border: 1px solid var(--border-default); border-radius: var(--radius-lg); padding: 16px 20px; box-shadow: var(--shadow-card); }
.quick-sell-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
.section-title { display: flex; align-items: center; gap: 8px; font-size: 14px; font-weight: 600; color: var(--text-primary); }

.quick-products-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(100px, 1fr)); gap: 10px; }
.quick-product-card {
  background: var(--bg-primary); border: 1px solid var(--border-default); border-radius: var(--radius-md);
  padding: 12px; cursor: pointer; transition: all 0.2s; display: flex; flex-direction: column; gap: 6px;
}
.quick-product-card:hover { border-color: var(--accent-primary); transform: translateY(-2px); }
.quick-product-card.disabled { opacity: 0.5; cursor: not-allowed; }
.quick-product-card.in-cart { border-color: var(--accent-success); background: rgba(63,185,80,0.05); }
.qp-header { display: flex; justify-content: space-between; align-items: center; }
.qp-name { font-size: 13px; font-weight: 500; color: var(--text-primary); flex: 1; }
.qp-badge { background: var(--accent-success); color: #fff; font-size: 10px; padding: 2px 6px; border-radius: 10px; font-weight: 600; }
.qp-price { font-size: 16px; font-weight: 700; color: var(--accent-success); }
.qp-footer { display: flex; justify-content: space-between; align-items: center; }
.qp-stock { font-size: 11px; color: var(--text-tertiary); }
.qp-stock.low { color: var(--accent-danger); }
.qp-category-tag { font-size: 10px; color: var(--text-tertiary); background: var(--bg-tertiary); padding: 2px 6px; border-radius: 8px; }

.quick-cart-summary {
  background: linear-gradient(135deg, var(--accent-success), #2ecc71); border-radius: var(--radius-md);
  padding: 16px; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 4px;
  cursor: pointer; transition: all 0.2s; color: #fff;
}
.quick-cart-summary:hover { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(63,185,80,0.3); }
.cart-items-count { font-size: 12px; opacity: 0.9; }
.cart-total { font-size: 16px; font-weight: 700; }

.pos-layout { display: grid; grid-template-columns: 1fr 320px; gap: 20px; min-height: 500px; }
.pos-products { display: flex; flex-direction: column; gap: 16px; }
.pos-search :deep(.el-input__wrapper) { background: var(--bg-primary); }
.pos-products-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; overflow-y: auto; max-height: 450px; }
.pos-product-card {
  background: var(--bg-primary); border: 1px solid var(--border-default); border-radius: var(--radius-md);
  padding: 16px 12px; cursor: pointer; transition: all 0.2s; display: flex; flex-direction: column; align-items: center; gap: 8px;
}
.pos-product-card:hover { border-color: var(--accent-primary); transform: scale(1.02); }
.pos-product-card.disabled { opacity: 0.5; cursor: not-allowed; }
.pos-prod-name { font-size: 14px; font-weight: 500; color: var(--text-primary); text-align: center; }
.pos-prod-price { font-size: 18px; font-weight: 700; color: var(--accent-success); }
.pos-prod-stock { font-size: 12px; color: var(--text-tertiary); }
.pos-prod-stock.low { color: var(--accent-danger); }
.pos-empty { grid-column: 1 / -1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; padding: 60px; color: var(--text-tertiary); }

.pos-cart { background: var(--bg-primary); border-radius: var(--radius-lg); display: flex; flex-direction: column; overflow: hidden; }
.pos-cart-header { display: flex; justify-content: space-between; align-items: center; padding: 16px; border-bottom: 1px solid var(--border-default); font-weight: 600; }
.pos-cart-header span { display: flex; align-items: center; gap: 8px; }
.pos-cart-items { flex: 1; overflow-y: auto; padding: 12px; display: flex; flex-direction: column; gap: 10px; }
.pos-cart-item { display: flex; align-items: center; gap: 10px; padding: 10px; background: var(--bg-secondary); border-radius: var(--radius-md); }
.pci-info { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.pci-name { font-size: 13px; font-weight: 500; color: var(--text-primary); }
.pci-price { font-size: 12px; color: var(--text-tertiary); }
.pci-controls { display: flex; align-items: center; gap: 4px; }
.pci-qty { min-width: 24px; text-align: center; font-weight: 600; }
.pci-subtotal { font-size: 14px; font-weight: 700; color: var(--accent-success); min-width: 50px; text-align: right; }
.pos-cart-empty { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; color: var(--text-tertiary); }
.pos-cart-footer { padding: 16px; border-top: 1px solid var(--border-default); display: flex; flex-direction: column; gap: 16px; }
.pos-cart-total { display: flex; justify-content: space-between; align-items: center; font-size: 16px; }
.pos-total-value { font-size: 24px; font-weight: 700; color: var(--accent-success); }
.pos-payment { display: flex; flex-direction: column; gap: 8px; }
.pos-payment-label { font-size: 13px; color: var(--text-secondary); font-weight: 500; }
.pos-payment-btns { display: flex; gap: 8px; }
.pos-payment-btns .el-button { flex: 1; }
.pos-payment-btns .el-button--default {
  background: var(--bg-tertiary) !important;
  border: 2px solid var(--border-default) !important;
  color: var(--text-secondary) !important;
  font-weight: 600;
}
.pos-payment-btns .el-button--default:hover {
  border-color: var(--accent-primary) !important;
  color: var(--accent-primary) !important;
  background: var(--bg-active) !important;
}
.pos-payment-btns .el-button--primary {
  border: 2px solid var(--accent-primary) !important;
}
.pos-checkout-btn { height: 48px; font-size: 16px; font-weight: 600; }

.receipt-content { font-family: monospace; }
.receipt-header { text-align: center; margin-bottom: 12px; }
.receipt-title { font-size: 18px; font-weight: bold; }
.receipt-time { font-size: 12px; color: #333; margin-top: 4px; }
.receipt-items { margin: 12px 0; }
.receipt-item { display: flex; justify-content: space-between; padding: 6px 0; }
.ri-name { color: var(--text-primary); }
.ri-price { color: var(--text-primary); font-weight: 500; }
.receipt-summary { margin-top: 8px; }
.rs-row { display: flex; justify-content: space-between; padding: 4px 0; font-weight: 600; }
.receipt-footer { text-align: center; margin-top: 12px; font-size: 13px; color: var(--text-tertiary); }

.area-groups { display: flex; flex-direction: column; gap: 16px; }

.area-group {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  padding: 14px;
}

.area-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.area-name {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
}

.area-rate-badge {
  font-size: 11px;
  font-weight: 500;
  color: var(--accent-primary);
  background: rgba(88, 166, 255, 0.1);
  padding: 2px 8px;
  border-radius: 10px;
  margin-left: 8px;
}

.area-stats { display: flex; gap: 8px; }

.stat-pill {
  font-size: 11px;
  padding: 3px 10px;
  border-radius: 20px;
  font-weight: 500;
}

.stat-pill.green { background: rgba(63,185,80,0.15); color: var(--accent-success); }
.stat-pill.red { background: rgba(248,81,73,0.15); color: var(--accent-danger); }

.today-revenue-summary { display: flex; flex-direction: column; gap: 16px; }
.revenue-total { display: flex; justify-content: space-between; align-items: center; padding: 16px; background: linear-gradient(135deg, var(--accent-primary), #3b82f6); border-radius: var(--radius-md); color: #fff; }
.rt-label { font-size: 14px; opacity: 0.9; }
.rt-value { font-size: 28px; font-weight: 700; }
.revenue-stats { display: flex; gap: 12px; }
.rs-item { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 4px; padding: 12px; background: var(--bg-primary); border-radius: var(--radius-md); }
.rsi-label { font-size: 11px; color: var(--text-tertiary); }
.rsi-value { font-size: 14px; font-weight: 600; color: var(--text-primary); }

.table-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(72px, 1fr));
  gap: 8px;
}

.table-mini {
  aspect-ratio: 1;
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.table-mini:hover {
  transform: scale(1.05);
  border-color: var(--accent-primary);
}

.table-mini.free { border-color: rgba(63,185,80,0.3); }
.table-mini.in-use { border-color: rgba(248,81,73,0.3); background: rgba(248,81,73,0.08); }
.table-mini.maint { border-color: rgba(210,153,34,0.3); background: rgba(210,153,34,0.05); }

.table-num { font-size: 12px; font-weight: 600; color: var(--text-secondary); }
.table-status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent-success);
}

.table-mini.free .table-status-dot { background: var(--accent-success); }
.table-mini.in-use .table-status-dot { background: var(--accent-danger); }
.table-mini.maint .table-status-dot { background: var(--accent-warning); }

.bar-chart {
  display: flex;
  align-items: flex-end;
  gap: 10px;
  height: 160px;
  padding: 10px 4px 0;
}

.bar-col {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  height: 100%;
}

.bar-wrap {
  flex: 1;
  width: 100%;
  display: flex;
  align-items: flex-end;
  justify-content: center;
}

.bar {
  width: 70%;
  background: var(--gradient-blue);
  border-radius: 4px 4px 0 0;
  min-height: 6px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  transition: height 0.5s;
}

.bar-val {
  font-size: 10px;
  color: #fff;
  font-weight: 600;
  padding-top: 4px;
}

.bar-date { font-size: 11px; color: var(--text-tertiary); }

.usage-list { display: flex; flex-direction: column; gap: 12px; }

.usage-row { display: flex; align-items: center; gap: 12px; }

.usage-name { font-size: 13px; color: var(--text-secondary); width: 56px; }

.usage-track {
  flex: 1;
  height: 6px;
  background: var(--bg-primary);
  border-radius: 3px;
  overflow: hidden;
}

.usage-fill {
  height: 100%;
  background: var(--gradient-blue);
  border-radius: 3px;
  transition: width 0.5s;
}

.usage-pct { font-size: 12px; color: var(--accent-primary); width: 36px; text-align: right; }

.hour-grid {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 8px;
}

.hour-cell {
  text-align: center;
  padding: 12px 6px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
  transition: all 0.2s;
}

.hour-cell.peak {
  background: rgba(88,166,255,0.1);
  border: 1px solid rgba(88,166,255,0.2);
}

.hour-lbl { font-size: 11px; color: var(--text-tertiary); }
.hour-val { font-size: 12px; font-weight: 600; color: var(--text-secondary); margin-top: 4px; }
.hour-cell.peak .hour-val { color: var(--accent-primary); }

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  color: var(--text-tertiary);
  gap: 12px;
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

:deep(.el-divider) { margin: 12px 0; border-color: var(--border-muted); }

@media (max-width: 1200px) {
  .stat-grid { grid-template-columns: repeat(2, 1fr); }
  .main-grid { grid-template-columns: 1fr; }
  .tables-overview { grid-column: 1; }
}

@media (max-width: 768px) {
  .stat-grid { grid-template-columns: 1fr; }
  .hour-grid { grid-template-columns: repeat(4, 1fr); }
}
</style>
