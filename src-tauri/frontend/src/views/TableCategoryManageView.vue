<template>
  <div class="category-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('tableTypeManage') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd">{{ t('add') }}</el-button>
    </div>

    <div class="card">
      <el-table :data="categories" stripe style="width: 100%">
        <el-table-column :label="t('name')" prop="name" min-width="200" />
        <el-table-column :label="t('remark')" prop="description" min-width="300">
          <template #default="{ row }">{{ row.description || '-' }}</template>
        </el-table-column>
        <el-table-column :label="t('createdAt')" prop="created_at" min-width="150">
          <template #default="{ row }">{{ row.created_at?.slice(0,10) }}</template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="160" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="doEdit(row)" :icon="Edit">{{ t('edit') }}</el-button>
            <el-button size="small" type="danger" plain @click="doDelete(row)" :icon="Delete">{{ t('remove') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="formDlg" :title="editTarget ? t('edit') : t('add')" width="420px" :close-on-click-modal="false" top="5vh" append-to-body>
      <el-form :model="form" label-position="top">
        <el-form-item :label="t('name')">
          <el-input v-model="form.name" :placeholder="t('name')" />
        </el-form-item>
        <el-form-item :label="t('remark')">
          <el-input v-model="form.description" :placeholder="t('remark')" type="textarea" :rows="3" />
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
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox, ElButton, ElTable, ElTableColumn, ElDialog, ElForm, ElFormItem, ElInput, ElIcon } from 'element-plus'
import { Plus, Edit, Delete } from '@element-plus/icons-vue'
import { getTableCategories, createTableCategory, updateTableCategory, deleteTableCategory } from '../api'
import { t } from '../i18n'

const categories = ref([])
const formDlg = ref(false)
const editTarget = ref(null)
const form = ref({ name: '', description: '' })

const loadCategories = async () => {
  try { categories.value = await getTableCategories() }
  catch { ElMessage.error(t('loadFailed')) }
}

const showAdd = () => {
  editTarget.value = null
  form.value = { name: '', description: '' }
  formDlg.value = true
}

const doEdit = (c) => {
  editTarget.value = c
  form.value = { name: c.name, description: c.description }
  formDlg.value = true
}

const cfmForm = async () => {
  if (!form.value.name) return ElMessage.warning(t('pleaseComplete'))
  try {
    if (editTarget.value) { await updateTableCategory(editTarget.value.id, form.value.name, form.value.description) }
    else { await createTableCategory(form.value.name, form.value.description) }
    ElMessage.success(t('saveSuccess'))
    formDlg.value = false
    await loadCategories()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const doDelete = async (c) => {
  try {
    await ElMessageBox.confirm(`${t('confirmDelete')}「${c.name}」？`, '⚠️', { type: 'warning' })
    await deleteTableCategory(c.id)
    ElMessage.success(t('deleteSuccess'))
    await loadCategories()
  } catch(error) { if (error !== 'cancel') ElMessage.error(t('operationFailed')) }
}

onMounted(loadCategories)
</script>

<style scoped>
.category-page { display: flex; flex-direction: column; gap: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: center; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
}
:deep(.el-table) { background: transparent !important; }
:deep(.el-table th) { background: var(--bg-primary) !important; }
:deep(.el-table tr) { background: transparent !important; }
:deep(.el-table td) { border-bottom: 1px solid var(--border-muted) !important; }
</style>
