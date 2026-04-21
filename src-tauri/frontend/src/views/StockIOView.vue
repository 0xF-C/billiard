<template>
  <div class="stock-io-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('stockIO') }}</h1>
      <div class="header-actions">
        <el-button type="primary" :icon="Plus" @click="showStockIn = true">{{ t('stockIn') }}</el-button>
        <el-button type="warning" :icon="Minus" @click="showStockOut = true">{{ t('stockOut') }}</el-button>
      </div>
    </div>

    <div class="tabs-row">
      <el-radio-group v-model="activeTab" @change="loadRecords">
        <el-radio-button value="all">{{ t('all') }}</el-radio-button>
        <el-radio-button value="in">{{ t('stockIn') }}</el-radio-button>
        <el-radio-button value="out">{{ t('stockOut') }}</el-radio-button>
      </el-radio-group>
      <el-date-picker
        v-model="dateRange"
        type="daterange"
        range-separator="~"
        value-format="YYYY-MM-DD"
        @change="loadRecords"
      />
    </div>

    <div class="records-table">
      <el-table :data="records" stripe>
        <el-table-column :label="t('time')" width="160">
          <template #default="{ row }">
            {{ formatTime(row.time) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('type')" width="80">
          <template #default="{ row }">
            <el-tag :type="row.type === 'in' ? 'success' : 'warning'" size="small">
              {{ row.type === 'in' ? t('stockIn') : t('stockOut') }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('product')" min-width="140">
          <template #default="{ row }">
            <div class="product-cell">
              <span class="product-name">{{ row.productName }}</span>
              <span class="product-sku">{{ row.sku }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('quantity')" width="100">
          <template #default="{ row }">
            <span :class="row.type === 'in' ? 'qty-in' : 'qty-out'">
              {{ row.type === 'in' ? '+' : '-' }}{{ row.quantity }} {{ row.unit }}
            </span>
          </template>
        </el-table-column>
        <el-table-column :label="t('beforeStock')" width="100">
          <template #default="{ row }">
            {{ row.beforeStock }} {{ row.unit }}
          </template>
        </el-table-column>
        <el-table-column :label="t('afterStock')" width="100">
          <template #default="{ row }">
            {{ row.afterStock }} {{ row.unit }}
          </template>
        </el-table-column>
        <el-table-column :label="t('reason')" min-width="120">
          <template #default="{ row }">
            {{ row.reason || '-' }}
          </template>
        </el-table-column>
        <el-table-column :label="t('operator')" width="100">
          <template #default="{ row }">
            {{ row.operator }}
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- {{ t('StockInDialog') }} -->
    <el-dialog v-model="showStockIn" :title="t('stockIn')" width="500px">
      <el-form :model="stockInForm" label-width="80px">
        <el-form-item :label="t('product')">
          <el-select v-model="stockInForm.productId" filterable :placeholder="t('selectProduct')">
            <el-option v-for="p in products" :key="p.id" :label="p.name" :value="p.id" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('quantity')">
          <el-input-number v-model="stockInForm.quantity" :min="1" />
        </el-form-item>
        <el-form-item :label="t('unitPrice')">
          <el-input-number v-model="stockInForm.unitPrice" :min="0" :precision="2" />
        </el-form-item>
        <el-form-item :label="t('supplier')">
          <el-input v-model="stockInForm.supplier" />
        </el-form-item>
        <el-form-item :label="t('remark')">
          <el-input v-model="stockInForm.remark" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showStockIn = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitStockIn">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- {{ t('StockOutDialog') }} -->
    <el-dialog v-model="showStockOut" :title="t('stockOut')" width="500px">
      <el-form :model="stockOutForm" label-width="80px">
        <el-form-item :label="t('product')">
          <el-select v-model="stockOutForm.productId" filterable :placeholder="t('selectProduct')">
            <el-option v-for="p in products" :key="p.id" :label="`${p.name} (库存: ${p.stock})`" :value="p.id" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('quantity')">
          <el-input-number v-model="stockOutForm.quantity" :min="1" />
        </el-form-item>
        <el-form-item :label="t('reason')">
          <el-select v-model="stockOutForm.reason" :placeholder="t('selectReason')">
            <el-option :label="t('sales')" value="sales" />
            <el-option :label="t('damage')" value="damage" />
            <el-option :label="t('expired')" value="expired" />
            <el-option :label="t('other')" value="other" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('remark')">
          <el-input v-model="stockOutForm.remark" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showStockOut = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitStockOut">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Plus, Minus } from '@element-plus/icons-vue'
import { getInventory } from '../api'
import { t } from '../i18n'


const activeTab = ref('all')
const dateRange = ref([])
const showStockIn = ref(false)
const showStockOut = ref(false)

const products = ref([])
const records = ref([
  { id: 1, time: new Date(), type: 'in', productName: t('cocaCola'), sku: 'COLA-330', quantity: 50, unit: t('bottle'), beforeStock: 20, afterStock: 70, reason: t('purchase'), operator: '店长' },
  { id: 2, time: new Date(Date.now() - 3600000), type: 'out', productName: t('mineralWater'), sku: 'WATER-500', quantity: 10, unit: t('bottle'), beforeStock: 60, afterStock: 50, reason: t('sales'), operator: t('waiter') },
  { id: 3, time: new Date(Date.now() - 7200000), type: 'in', productName: t('chalk'), sku: 'CHALK-01', quantity: 100, unit: t('piece_unit'), beforeStock: 30, afterStock: 130, reason: t('purchase'), operator: '店长' },
  { id: 4, time: new Date(Date.now() - 86400000), type: 'out', productName: t('gloves'), sku: 'GLOVE-M', quantity: 5, unit: t('pair'), beforeStock: 20, afterStock: 15, reason: t('sales'), operator: t('waiter') },
])

const stockInForm = reactive({
  productId: null,
  quantity: 1,
  unitPrice: 0,
  supplier: '',
  remark: '',
})

const stockOutForm = reactive({
  productId: null,
  quantity: 1,
  reason: '',
  remark: '',
})

const formatTime = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleString(currentLang.value === 'zh' ? 'zh-CN' : (currentLang.value === 'en' ? 'en-US' : 'zh-CN'))
}

const loadRecords = () => {
  // load records
}

const loadProducts = async () => {
  const res = await getInventory()
  products.value = res || []
}

const submitStockIn = () => {
  const product = products.value.find(p => p.id === stockInForm.productId)
  if (!product) {
    ElMessage.warning(t('selectProduct'))
    return
  }

  records.value.unshift({
    id: Date.now(),
    time: new Date(),
    type: 'in',
    productName: product.name,
    sku: product.sku || '-',
    quantity: stockInForm.quantity,
    unit: product.unit || t('piece_unit'),
    beforeStock: product.stock || 0,
    afterStock: (product.stock || 0) + stockInForm.quantity,
    reason: stockInForm.remark || t('purchase'),
    operator: '当前用户',
  })

  ElMessage.success(t('success'))
  showStockIn.value = false
  stockInForm.productId = null
  stockInForm.quantity = 1
  stockInForm.remark = ''
}

const submitStockOut = () => {
  const product = products.value.find(p => p.id === stockOutForm.productId)
  if (!product) {
    ElMessage.warning(t('selectProduct'))
    return
  }

  if (stockOutForm.quantity > (product.stock || 0)) {
    ElMessage.warning(t('stockNotEnough'))
    return
  }

  records.value.unshift({
    id: Date.now(),
    time: new Date(),
    type: 'out',
    productName: product.name,
    sku: product.sku || '-',
    quantity: stockOutForm.quantity,
    unit: product.unit || t('piece_unit'),
    beforeStock: product.stock || 0,
    afterStock: (product.stock || 0) - stockOutForm.quantity,
    reason: stockOutForm.reason || t('stockOutReason'),
    operator: '当前用户',
  })

  ElMessage.success(t('success'))
  showStockOut.value = false
  stockOutForm.productId = null
  stockOutForm.quantity = 1
  stockOutForm.reason = ''
  stockOutForm.remark = ''
}

onMounted(() => {
  loadProducts()
})
</script>

<style scoped>
.stock-io-page {
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
}

.tabs-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.records-table {
  background: var(--card-bg);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.product-cell {
  display: flex;
  flex-direction: column;
}

.product-name {
  font-weight: 500;
}

.product-sku {
  font-size: 12px;
  color: var(--text-secondary);
}

.qty-in {
  color: var(--accent-success);
  font-weight: 500;
}

.qty-out {
  color: var(--accent-warning);
  font-weight: 500;
}
</style>
