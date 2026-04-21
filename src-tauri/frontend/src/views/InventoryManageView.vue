<template>
  <div class="inventory-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('stockManage') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd">{{ t('add') }}</el-button>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon">
          <el-icon><Box /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('total') }} {{ t('quantity') }}</span>
          <span class="stat-value">{{ totalQty }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon success">
          <el-icon><Money /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('total') }} {{ t('costAmount') }}</span>
          <span class="stat-value">¥{{ totalValue.toFixed(2) }}</span>
        </div>
      </div>
    </div>

    <div class="card">
      <el-table :data="inventory" stripe style="width: 100%">
        <el-table-column :label="t('name')" prop="name" min-width="150" />
        <el-table-column :label="t('category')" prop="category" min-width="100" />
        <el-table-column :label="t('quantity')" prop="quantity" min-width="100">
          <template #default="{ row }">
            <span :class="row.quantity < 10 ? 'low-stock' : ''">{{ row.quantity }} {{ row.unit }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('price')" prop="price" min-width="100">
          <template #default="{ row }">
            <span class="price-value">¥{{ row.price?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('supplier')" prop="supplier" min-width="150">
          <template #default="{ row }">{{ row.supplier || '-' }}</template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="140" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="doEdit(row)" :icon="Edit">{{ t('edit') }}</el-button>
            <el-button size="small" type="danger" plain @click="doDelete(row)" :icon="Delete">{{ t('remove') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="formDlg" :title="editTarget ? t('edit') : t('add')" width="460px" :close-on-click-modal="false">
      <el-form :model="form" label-position="top">
        <el-form-item :label="t('name')">
          <el-input v-model="form.name" :placeholder="t('name')" />
        </el-form-item>
        <el-form-item :label="t('category')" required>
          <el-select v-model="form.category_id" class="full-width" :placeholder="t('selectCategoryFirst')">
            <el-option v-for="cat in categories" :key="cat.id" :label="cat.name" :value="cat.id" />
          </el-select>
          <div class="category-hint">
            <el-link type="primary" @click="$router.push('/category-manage')" :underline="false">
              {{ t('goToCategoryManage') }}
            </el-link>
          </div>
        </el-form-item>
        <el-form-item :label="t('quantity')">
          <el-input-number v-model="form.quantity" :min="0" class="full-width" />
        </el-form-item>
        <el-form-item :label="t('price')">
          <el-input-number v-model="form.price" :min="0" :precision="2" class="full-width" />
        </el-form-item>
        <el-form-item :label="t('unit')">
          <el-select v-model="form.unit" class="full-width">
            <el-option value="个" label="个" />
            <el-option value="盒" label="盒" />
            <el-option value="套" label="套" />
            <el-option value="米" label="米" />
            <el-option value="瓶" label="瓶" />
            <el-option value="罐" label="罐" />
            <el-option value="支" label="支" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('supplier')">
          <el-input v-model="form.supplier" :placeholder="t('supplier')" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="formDlg=false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="cfmForm">{{ t('save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox, ElButton, ElTable, ElTableColumn, ElDialog, ElForm, ElFormItem, ElInput, ElSelect, ElOption, ElInputNumber, ElIcon, ElLink } from 'element-plus'
import { Plus, Edit, Delete, Box, Money } from '@element-plus/icons-vue'
import { getInventory, createInventory, updateInventory, deleteInventory, getInventoryCategories } from '../api'
import { t } from '../i18n'

const inventory = ref([])
const categories = ref([])
const formDlg = ref(false)
const editTarget = ref(null)
const form = ref({ name: '', category_id: null, quantity: 0, price: 0, unit: '个', supplier: '' })

const totalQty = computed(() => inventory.value.reduce((a, b) => a + (b.quantity || 0), 0))
const totalValue = computed(() => inventory.value.reduce((a, b) => a + ((b.quantity || 0) * (b.price || 0)), 0))

const loadInventory = async () => {
  try { inventory.value = await getInventory({}) }
  catch { ElMessage.error(t('loadFailed')) }
}

const loadCategories = async () => {
  try { categories.value = await getInventoryCategories() }
  catch { categories.value = [] }
}

const showAdd = () => {
  editTarget.value = null
  form.value = { name: '', category_id: null, quantity: 0, price: 0, unit: '个', supplier: '' }
  formDlg.value = true
}

const doEdit = (item) => {
  editTarget.value = item
  form.value = { 
    name: item.name, 
    category_id: item.category_id, 
    quantity: item.quantity, 
    price: item.price, 
    unit: item.unit, 
    supplier: item.supplier || '' 
  }
  formDlg.value = true
}

const cfmForm = async () => {
  if (!form.value.name) return ElMessage.warning(t('pleaseComplete'))
  if (!form.value.category_id) return ElMessage.warning(t('selectCategoryFirst'))
  try {
    const data = { ...form.value }
    if (editTarget.value) { await updateInventory(editTarget.value.id, data) }
    else { await createInventory(data) }
    ElMessage.success(t('saveSuccess'))
    formDlg.value = false
    await loadInventory()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const doDelete = async (item) => {
  try {
    await ElMessageBox.confirm(`${t('confirmDelete')}「${item.name}」？`, '⚠️', { type: 'warning' })
    await deleteInventory(item.id)
    ElMessage.success(t('deleteSuccess'))
    await loadInventory()
  } catch(error) { if (error !== 'cancel') ElMessage.error(t('operationFailed')) }
}

onMounted(() => { loadInventory(); loadCategories() })
</script>

<style scoped>
.inventory-page { display: flex; flex-direction: column; gap: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: center; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.stats-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px; }
.stat-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
}
.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22px;
  color: var(--text-secondary);
}
.stat-icon.success { background: rgba(63,185,80,0.15); color: var(--accent-success); }
.stat-body { display: flex; flex-direction: column; gap: 4px; }
.stat-label { font-size: 13px; color: var(--text-secondary); }
.stat-value { font-size: 24px; font-weight: 700; color: var(--text-primary); }
.card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
}
.low-stock { color: var(--accent-danger); font-weight: 600; }
.price-value { color: var(--accent-success); font-weight: 600; }
.full-width { width: 100%; }
.category-hint { margin-top: 8px; font-size: 12px; }
:deep(.el-table) { background: transparent !important; }
:deep(.el-table th) { background: var(--bg-primary) !important; }
:deep(.el-table tr) { background: transparent !important; }
:deep(.el-table td) { border-bottom: 1px solid var(--border-muted) !important; }
@media (max-width: 768px) { .stats-grid { grid-template-columns: 1fr; } }
</style>
