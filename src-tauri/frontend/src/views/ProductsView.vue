<template>
  <div class="retail-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('retail') }}</h1>
      <div class="header-actions">
        <el-button @click="showRecords = true">
          <el-icon><Document /></el-icon>
          {{ t('salesRecords') }}
        </el-button>
        <el-button type="primary" @click="$router.push('/inventory-manage')">
          <el-icon><Box /></el-icon>
          {{ t('goToInventory') }}
        </el-button>
      </div>
    </div>

    <div class="retail-layout">
      <div class="product-section">
        <div class="filter-bar">
          <el-select v-model="filterCategory" :placeholder="t('allCategories')" clearable style="width: 160px;">
            <el-option v-for="cat in categories" :key="cat.id" :label="cat.name" :value="cat.id" />
          </el-select>
          <el-input v-model="searchKeyword" :placeholder="t('search')" clearable style="flex: 1;">
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
          <el-button @click="loadProducts" :icon="Refresh">{{ t('refresh') }}</el-button>
        </div>

        <div class="product-grid" v-if="filteredProducts.length > 0">
          <div
            v-for="prod in filteredProducts"
            :key="prod.id"
            :class="['product-card', { disabled: prod.quantity <= 0 }]"
            @click="addToCart(prod)"
          >
            <div class="product-icon">
              <el-icon><Goods /></el-icon>
            </div>
            <div class="product-category">{{ prod.category_name || '-' }}</div>
            <div class="product-info">
              <span class="product-name">{{ prod.name }}</span>
              <span class="product-price">¥{{ prod.price?.toFixed(2) }}</span>
            </div>
            <div class="product-stock" :class="prod.quantity < 5 ? 'low' : ''">
              {{ prod.quantity > 0 ? `${prod.quantity}${prod.unit}` : t('lowStock') }}
            </div>
          </div>
        </div>
        <div v-else class="empty-products">
          <el-icon :size="48"><Goods /></el-icon>
          <span>{{ t('noProducts') }}</span>
          <el-button type="primary" @click="$router.push('/inventory-manage')">
            {{ t('goToInventory') }}
          </el-button>
        </div>
      </div>

      <div class="cart-section">
        <div class="cart-header">
          <el-icon><ShoppingCart /></el-icon>
          <span>{{ t('cart') }}</span>
          <span class="cart-count">{{ cart.length }}</span>
        </div>

        <div class="cart-items" v-if="cart.length > 0">
          <div v-for="(item, index) in cart" :key="index" class="cart-item">
            <div class="item-info">
              <span class="item-name">{{ item.name }}</span>
              <span class="item-price">¥{{ item.price?.toFixed(2) }}</span>
            </div>
            <div class="item-controls">
              <el-button size="small" @click="decreaseItem(index)" :icon="Minus" circle />
              <span class="item-qty">{{ item.quantity }}</span>
              <el-button size="small" @click="increaseItem(index)" :icon="Plus" circle />
              <el-button size="small" type="danger" @click="removeItem(index)" :icon="Delete" circle />
            </div>
            <span class="item-total">¥{{ (item.price * item.quantity).toFixed(2) }}</span>
          </div>
        </div>
        <div v-else class="cart-empty">
          <el-icon :size="32"><ShoppingCart /></el-icon>
          <span>{{ t('cartEmpty') }}</span>
        </div>

        <div class="cart-footer">
          <div class="cart-total">
            <span>{{ t('totalAmount') }}</span>
            <span class="total-value">¥{{ cartTotal.toFixed(2) }}</span>
          </div>

          <div class="form-item">
            <label>{{ t('paymentMethod') }}</label>
            <el-select v-model="paymentMethod" style="width: 100%;">
              <el-option :label="t('cash')" value="cash" />
              <el-option :label="t('wechat')" value="wechat" />
              <el-option :label="t('alipay')" value="alipay" />
            </el-select>
          </div>

          <el-button type="success" class="checkout-btn" :disabled="cart.length === 0" @click="doCheckout" :loading="submitting">
            <el-icon><Check /></el-icon>
            {{ t('checkout') }} ¥{{ cartTotal.toFixed(2) }}
          </el-button>
        </div>
      </div>
    </div>

    <el-dialog v-model="showRecords" :title="t('salesRecords')" width="700px" top="5vh" append-to-body>
      <el-table :data="salesRecords" stripe>
        <el-table-column :label="t('productName')" prop="product_name" />
        <el-table-column :label="t('quantity')" prop="quantity" width="80" />
        <el-table-column :label="t('productPrice')" width="90">
          <template #default="{ row }">¥{{ row.unit_price?.toFixed(2) }}</template>
        </el-table-column>
        <el-table-column :label="t('totalAmount')" width="100">
          <template #default="{ row }">¥{{ row.total_amount?.toFixed(2) }}</template>
        </el-table-column>
        <el-table-column :label="t('paymentMethod')" width="100">
          <template #default="{ row }">{{ t(row.payment_method) || row.payment_method }}</template>
        </el-table-column>
        <el-table-column :label="t('time')" width="160">
          <template #default="{ row }">{{ formatTime(row.created_at) }}</template>
        </el-table-column>
      </el-table>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Plus, Delete, Search, Goods, ShoppingCart, Check, Minus, Document, Refresh, Box } from '@element-plus/icons-vue'
import { getInventory, getInventoryCategories, saleBatch, getSalesRecords, printReceipt as apiPrintReceipt } from '../api'
import { t } from '../i18n'

const products = ref([])
const categories = ref([])
const filterCategory = ref('')
const searchKeyword = ref('')
const cart = ref([])
const paymentMethod = ref('cash')
const submitting = ref(false)

const showRecords = ref(false)
const salesRecords = ref([])

const filteredProducts = computed(() => {
  let result = products.value
  if (filterCategory.value) {
    result = result.filter(p => p.category_id === filterCategory.value)
  }
  if (searchKeyword.value) {
    const kw = searchKeyword.value.toLowerCase()
    result = result.filter(p => p.name.toLowerCase().includes(kw))
  }
  return result
})

const cartTotal = computed(() => {
  return cart.value.reduce((sum, item) => sum + item.price * item.quantity, 0)
})

const loadProducts = async () => {
  try {
    products.value = await getInventory()
  } catch { products.value = [] }
}

const loadCategories = async () => {
  try {
    categories.value = await getInventoryCategories()
  } catch { categories.value = [] }
}

const loadRecords = async () => {
  try {
    salesRecords.value = await getSalesRecords({})
  } catch { salesRecords.value = [] }
}

const addToCart = (prod) => {
  if (prod.quantity <= 0) return ElMessage.warning(t('lowStock'))
  const existing = cart.value.find(c => c.id === prod.id)
  if (existing) {
    if (existing.quantity >= prod.quantity) return ElMessage.warning(t('lowStock'))
    existing.quantity++
  } else {
    cart.value.push({
      id: prod.id,
      name: prod.name,
      price: prod.price,
      quantity: 1,
      stock: prod.quantity
    })
  }
}

const increaseItem = (index) => {
  const item = cart.value[index]
  if (item.quantity >= item.stock) return ElMessage.warning(t('lowStock'))
  item.quantity++
}

const decreaseItem = (index) => {
  if (cart.value[index].quantity > 1) {
    cart.value[index].quantity--
  } else {
    cart.value.splice(index, 1)
  }
}

const removeItem = (index) => {
  cart.value.splice(index, 1)
}

const doCheckout = async () => {
  if (cart.value.length === 0) return
  submitting.value = true
  try {
    const sales = await saleBatch({
      items: cart.value.map(item => ({
        product_id: item.id,
        quantity: item.quantity
      })),
      payment_method: paymentMethod.value
    })
    ElMessage.success(t('saveSuccess'))
    // Auto-print sale receipt (mirrors close-table logic)
    try {
      const total = sales.reduce((sum, s) => sum + (s.total_amount || 0), 0)
      await apiPrintReceipt({
        receipt_type: 'sale',
        total_amount: total,
        final_amount: total,
        payment_method: paymentMethod.value,
        items: sales.map(s => ({ name: s.product_name, quantity: s.quantity, price: s.unit_price }))
      })
    } catch (_) { /* print failure is non-fatal */ }
    cart.value = []
    await loadProducts()
  } catch(e) {
    ElMessage.error(e.response?.data?.error || t('operationFailed'))
  } finally {
    submitting.value = false
  }
}

const formatTime = (time) => {
  if (!time) return '-'
  return time.replace('T', ' ').slice(0, 19)
}

onMounted(() => { loadProducts(); loadCategories(); loadRecords() })
</script>

<style scoped>
.retail-page { display: flex; flex-direction: column; gap: 24px; height: calc(100vh - 140px); }
.page-header { display: flex; justify-content: space-between; align-items: center; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.header-actions { display: flex; gap: 12px; }

.retail-layout { display: grid; grid-template-columns: 1fr 340px; gap: 24px; flex: 1; min-height: 0; }

.product-section { display: flex; flex-direction: column; gap: 16px; background: var(--bg-secondary); border-radius: var(--radius-lg); padding: 20px; }
.filter-bar { display: flex; gap: 12px; }

.product-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 12px; overflow-y: auto; flex: 1; }
.product-card {
  background: var(--bg-primary); border: 1px solid var(--border-default); border-radius: var(--radius-md);
  padding: 16px; display: flex; flex-direction: column; align-items: center; gap: 8px;
  cursor: pointer; transition: all 0.2s;
}
.product-card:hover { border-color: var(--accent-primary); transform: translateY(-2px); }
.product-card.disabled { opacity: 0.5; cursor: not-allowed; }
.product-icon { width: 48px; height: 48px; border-radius: 50%; background: rgba(88,166,255,0.1); display: flex; align-items: center; justify-content: center; font-size: 24px; color: var(--accent-primary); }
.product-category { font-size: 11px; color: var(--text-tertiary); background: var(--bg-tertiary); padding: 2px 8px; border-radius: 10px; }
.product-info { display: flex; flex-direction: column; align-items: center; gap: 4px; text-align: center; }
.product-name { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.product-price { font-size: 16px; font-weight: 700; color: var(--accent-success); }
.product-stock { font-size: 12px; color: var(--text-tertiary); }
.product-stock.low { color: var(--accent-danger); }
.empty-products { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; padding: 60px; color: var(--text-tertiary); flex: 1; }

.cart-section { background: var(--bg-secondary); border-radius: var(--radius-lg); display: flex; flex-direction: column; }
.cart-header { display: flex; align-items: center; gap: 8px; padding: 16px 20px; border-bottom: 1px solid var(--border-default); font-size: 16px; font-weight: 600; color: var(--text-primary); }
.cart-count { background: var(--accent-primary); color: #fff; border-radius: 50%; width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; font-size: 12px; margin-left: auto; }
.cart-items { flex: 1; overflow-y: auto; padding: 12px; display: flex; flex-direction: column; gap: 8px; }
.cart-item { display: flex; align-items: center; gap: 8px; padding: 12px; background: var(--bg-primary); border-radius: var(--radius-md); }
.item-info { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.item-name { font-size: 13px; font-weight: 500; color: var(--text-primary); }
.item-price { font-size: 12px; color: var(--text-tertiary); }
.item-controls { display: flex; align-items: center; gap: 4px; }
.item-qty { min-width: 24px; text-align: center; font-weight: 600; }
.item-total { font-weight: 700; color: var(--accent-success); font-size: 14px; }
.cart-empty { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; color: var(--text-tertiary); }
.cart-footer { padding: 16px; border-top: 1px solid var(--border-default); display: flex; flex-direction: column; gap: 12px; }
.cart-total { display: flex; justify-content: space-between; align-items: center; font-size: 14px; }
.total-value { font-size: 24px; font-weight: 700; color: var(--accent-success); }
.form-item { display: flex; flex-direction: column; gap: 4px; }
.form-item label { font-size: 12px; color: var(--text-tertiary); }
.checkout-btn { height: 48px; font-size: 16px; font-weight: 600; }

@media (max-width: 900px) {
  .retail-layout { grid-template-columns: 1fr; }
  .cart-section { max-height: 400px; }
}
</style>