<template>
  <div class="loss-records-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('lossRecords') }}</h1>
      <div class="header-actions">
        <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('addLoss') }}</el-button>
        <el-date-picker v-model="dateRange" type="daterange" range-separator="~" value-format="YYYY-MM-DD" />
      </div>
    </div>

    <div class="summary-row">
      <div class="summary-card">
        <span class="label">{{ t('totalLoss') }}</span>
        <span class="value">¥{{ summary.total.toLocaleString() }}</span>
      </div>
      <div class="summary-card">
        <span class="label">{{ t('totalRecords') }}</span>
        <span class="value">{{ summary.count }}</span>
      </div>
      <div class="summary-card">
        <span class="label">{{ t('thisMonth') }}</span>
        <span class="value">¥{{ summary.month.toLocaleString() }}</span>
      </div>
    </div>

    <div class="records-section">
      <div class="section-header">
        <h3>{{ t('lossList') }}</h3>
        <el-select v-model="filterReason" :placeholder="t('reason')" clearable>
          <el-option :label="t('all')" value="" />
          <el-option :label="t('damage')" value="damage" />
          <el-option :label="t('expired')" value="expired" />
          <el-option :label="t('theft')" value="theft" />
          <el-option :label="t('other')" value="other" />
        </el-select>
      </div>
      <el-table :data="filteredRecords" stripe>
        <el-table-column :label="t('date')" width="120">
          <template #default="{ row }">{{ formatDate(row.date) }}</template>
        </el-table-column>
        <el-table-column :label="t('product')" min-width="140">
          <template #default="{ row }">
            <div class="product-cell">
              <span class="name">{{ row.productName }}</span>
              <span class="sku">{{ row.sku }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('quantity')" width="80">
          <template #default="{ row }">{{ row.quantity }} {{ row.unit }}</template>
        </el-table-column>
        <el-table-column :label="t('amount')" width="100">
          <template #default="{ row }">
            <span class="loss-amount">-¥{{ row.amount.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('reason')" width="100">
          <template #default="{ row }">
            <el-tag :type="getReasonTag(row.reason)" size="small">{{ t(row.reason) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('description')">
          <template #default="{ row }">{{ row.description || '-' }}</template>
        </el-table-column>
        <el-table-column :label="t('operator')" width="100">
          <template #default="{ row }">{{ row.operator }}</template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="100" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="editRecord(row)">{{ t('edit') }}</el-button>
            <el-button link type="danger" @click="deleteRecord(row)">{{ t('delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="showAdd" :title="t('addLoss')" width="450px" top="5vh" append-to-body>
      <el-form :model="form" label-width="80px">
        <el-form-item :label="t('date')">
          <el-date-picker v-model="form.date" type="date" value-format="YYYY-MM-DD" />
        </el-form-item>
        <el-form-item :label="t('product')">
          <el-input v-model="form.productName" />
        </el-form-item>
        <el-form-item :label="t('quantity')">
          <el-input-number v-model="form.quantity" :min="1" />
        </el-form-item>
        <el-form-item :label="t('amount')">
          <el-input-number v-model="form.amount" :min="0" :precision="2" />
        </el-form-item>
        <el-form-item :label="t('reason')">
          <el-select v-model="form.reason">
            <el-option :label="t('damage')" value="damage" />
            <el-option :label="t('expired')" value="expired" />
            <el-option :label="t('theft')" value="theft" />
            <el-option :label="t('other')" value="other" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('description')">
          <el-input v-model="form.description" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAdd = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submit">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { t } from '../i18n'


const dateRange = ref([])
const filterReason = ref('')
const showAdd = ref(false)

const form = ref({
  date: new Date().toISOString().split('T')[0],
  productName: '',
  quantity: 1,
  amount: 0,
  reason: '',
  description: '',
})

const summary = ref({ total: 4280, count: 32, month: 1560 })

const records = ref([
  { id: 1, date: '2026-04-18', productName: '可口可乐', sku: 'COLA-330', quantity: 10, unit: '瓶', amount: 30, reason: 'damage', description: '搬运时破损', operator: '张三' },
  { id: 2, date: '2026-04-15', productName: '矿泉水', sku: 'WATER-500', quantity: 20, unit: '瓶', amount: 40, reason: 'expired', description: '过期处理', operator: '李四' },
  { id: 3, date: '2026-04-10', productName: '巧粉', sku: 'CHALK-01', quantity: 5, unit: '个', amount: 50, reason: 'damage', description: '球杆碰撞损坏', operator: '店长' },
])

const filteredRecords = computed(() => {
  if (!filterReason.value) return records.value
  return records.value.filter(r => r.reason === filterReason.value)
})

const getReasonTag = (reason) => {
  const types = { damage: 'warning', expired: 'danger', theft: 'danger', other: 'info' }
  return types[reason] || 'info'
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
}

const submit = () => {
  if (!form.value.productName) { ElMessage.warning(t('pleaseComplete')); return }
  records.value.unshift({ id: Date.now(), ...form.value, operator: '当前用户' })
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
}

const editRecord = (r) => ElMessage.info(`${t('edit')}: ${r.productName}`)
const deleteRecord = async (r) => {
  await ElMessageBox.confirm(t('confirmDelete'), t('confirm'), { type: 'warning' })
  const idx = records.value.findIndex(x => x.id === r.id)
  if (idx !== -1) { records.value.splice(idx, 1); ElMessage.success(t('deleteSuccess')) }
}
</script>

<style scoped>
.loss-records-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.header-actions { display: flex; gap: 12px; }

.summary-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.summary-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  border-left: 4px solid var(--accent-danger);
  text-align: center;
}

.summary-card .label { display: block; font-size: 13px; color: var(--text-secondary); margin-bottom: 8px; }
.summary-card .value { font-size: 28px; font-weight: 600; color: var(--accent-danger); }

.records-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h3 { margin: 0; font-size: 16px; font-weight: 600; }

.product-cell { display: flex; flex-direction: column; }
.product-cell .name { font-weight: 500; }
.product-cell .sku { font-size: 12px; color: var(--text-secondary); }

.loss-amount { color: var(--accent-danger); font-weight: 500; }
</style>
