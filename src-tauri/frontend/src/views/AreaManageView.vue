<template>
  <div class="area-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('areaSettings') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd">{{ t('add') }}</el-button>
    </div>

    <div class="areas-grid">
      <div v-for="area in areas" :key="area.id" class="area-card" @click="toggleExpand(area.id)">
        <div class="area-header">
          <div class="area-icon">
            <el-icon><Location /></el-icon>
          </div>
          <div class="area-info">
            <span class="area-name">{{ area.name }}</span>
            <span class="area-rate">¥{{ area.rate_per_hour }}/{{ t('hour') }}</span>
          </div>
          <span class="area-count">{{ area.table_count || 0 }} {{ t('tables') }}</span>
          <el-icon class="expand-arrow" :class="{ expanded: expandedAreas[area.id] }">
            <ArrowRight />
          </el-icon>
        </div>
        <div class="area-actions" @click.stop>
          <el-button size="small" @click="editArea(area)" :icon="Edit">{{ t('edit') }}</el-button>
          <el-button size="small" type="danger" plain @click="deleteAreaById(area)" :icon="Delete">{{ t('remove') }}</el-button>
        </div>
        <div v-if="expandedAreas[area.id]" class="area-tables" @click.stop>
          <div v-if="getAreaTables(area.id).length > 0" class="tables-list">
            <div v-for="tb in getAreaTables(area.id)" :key="tb.id" class="table-item">
              <span class="table-name">{{ tb.name }}</span>
              <span class="table-rate">¥{{ tb.rate_per_hour }}/{{ t('hour') }}</span>
              <el-tag :type="tb.status === '使用中' ? 'danger' : tb.status === '维护中' ? 'warning' : 'success'" size="small">
                {{ tb.status === '使用中' ? t('tableInUse') : tb.status === '维护中' ? t('tableMaintenance') : t('tableFree') }}
              </el-tag>
            </div>
          </div>
          <div v-else class="no-tables">{{ t('noData') }}</div>
        </div>
      </div>
      <div v-if="areas.length === 0" class="empty-card">
        <el-icon :size="48"><Warning /></el-icon>
        <span>{{ t('loadFailed') }}</span>
      </div>
    </div>

    <el-dialog v-model="formDlg" :title="editTarget ? t('edit') : t('add')" width="400px" :close-on-click-modal="false" top="5vh" append-to-body>
      <el-form :model="form" label-position="top">
        <el-form-item :label="t('name')">
          <el-input v-model="form.name" :placeholder="t('name')" />
        </el-form-item>
        <el-form-item :label="t('hourlyRate')">
          <el-input-number v-model="form.rate_per_hour" :min="0" :step="5" :precision="1" controls-position="right" style="width: 100%;" />
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
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Edit, Delete, Location, Warning, ArrowRight } from '@element-plus/icons-vue'
import { getAreas, createArea, updateArea, deleteArea, getTables } from '../api'
import { t } from '../i18n'

const areas = ref([])
const tables = ref([])
const formDlg = ref(false)
const editTarget = ref(null)
const form = ref({ name: '', rate_per_hour: 30 })
const expandedAreas = reactive({})

const getAreaTables = (areaId) => {
  return tables.value.filter(tb => tb.area_id === areaId)
}

const toggleExpand = (areaId) => {
  expandedAreas[areaId] = !expandedAreas[areaId]
}

const loadAreas = async () => {
  try { areas.value = await getAreas() }
  catch { ElMessage.error(t('loadFailed')) }
}

const loadTables = async () => {
  try { tables.value = await getTables() }
  catch { tables.value = [] }
}

const showAdd = () => {
  editTarget.value = null
  form.value = { name: '', rate_per_hour: 30 }
  formDlg.value = true
}

const editArea = (area) => {
  editTarget.value = area
  form.value = { name: area.name, rate_per_hour: area.rate_per_hour || 30 }
  formDlg.value = true
}

const cfmForm = async () => {
  if (!form.value.name) return ElMessage.warning(t('pleaseComplete'))
  try {
    if (editTarget.value) { await updateArea(editTarget.value.id, form.value.name, form.value.rate_per_hour) }
    else { await createArea(form.value.name, form.value.rate_per_hour) }
    ElMessage.success(t('saveSuccess'))
    formDlg.value = false
    await loadAreas()
  } catch(e) { ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

const deleteAreaById = async (area) => {
  try {
    await ElMessageBox.confirm(`${t('confirmDelete')}「${area.name}」？`, '⚠️', { type: 'warning' })
    await deleteArea(area.id)
    ElMessage.success(t('deleteSuccess'))
    await loadAreas()
    await loadTables()
  } catch(e) { if (e !== 'cancel') ElMessage.error(e.response?.data?.error || t('operationFailed')) }
}

onMounted(() => { loadAreas(); loadTables() })
</script>

<style scoped>
.area-page { display: flex; flex-direction: column; gap: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: center; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.areas-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }
.area-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  transition: all 0.2s;
  cursor: pointer;
}
.area-card:hover { border-color: var(--accent-primary); }
.area-header { display: flex; align-items: center; gap: 12px; margin-bottom: 12px; }
.area-icon {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  color: var(--accent-primary);
  flex-shrink: 0;
}
.area-info { display: flex; flex-direction: column; gap: 2px; flex: 1; }
.area-name { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.area-rate { font-size: 13px; color: var(--accent-success); }
.area-count { font-size: 12px; color: var(--text-tertiary); white-space: nowrap; }
.expand-arrow {
  transition: transform 0.2s;
  color: var(--text-tertiary);
  font-size: 14px;
}
.expand-arrow.expanded { transform: rotate(90deg); }
.area-actions { display: flex; gap: 8px; margin-bottom: 8px; }

.area-tables {
  border-top: 1px solid var(--border-default);
  padding-top: 12px;
  margin-top: 4px;
}
.tables-list { display: flex; flex-direction: column; gap: 8px; }
.table-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  font-size: 13px;
}
.table-item .table-name { flex: 1; font-weight: 500; color: var(--text-primary); }
.table-item .table-rate { color: var(--accent-success); font-weight: 500; }
.no-tables { text-align: center; color: var(--text-tertiary); font-size: 13px; padding: 12px; }

.empty-card {
  grid-column: 1 / -1;
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 60px;
  text-align: center;
  color: var(--text-tertiary);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}
</style>