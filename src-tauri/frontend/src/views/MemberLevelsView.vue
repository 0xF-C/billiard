<template>
  <div class="member-levels-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('levelSettings') }}</h1>
    </div>

    <div class="levels-grid">
      <div v-for="level in levels" :key="level.name" class="level-card">
        <div class="level-header">
          <div class="level-icon" :style="{ background: level.color }">
            <el-icon><Star /></el-icon>
          </div>
          <div class="level-info">
            <span class="level-name">{{ level.name }}</span>
            <span class="level-count">{{ level.memberCount }} 会员</span>
          </div>
        </div>
        <div class="level-discount">
          <span class="discount-value">{{ (level.discount * 10).toFixed(1) }}</span>
          <span class="discount-unit">折</span>
        </div>
        <div class="level-actions">
          <el-button size="small" @click="openEditDialog(level)" :icon="Edit">{{ t('edit') }}</el-button>
        </div>
      </div>
    </div>

    <!-- {{ t('EditDiscountDialog') }} -->
    <el-dialog v-model="editDlg" :title="t('edit') + ' - ' + (editTarget?.name || '')" width="360px" top="5vh" append-to-body>
      <div class="edit-discount-form" v-if="editTarget">
        <div class="discount-label">{{ t('discount') }}</div>
        <div class="discount-input-group">
          <el-input-number v-model="editDiscount" :min="0.1" :max="1" :step="0.1" :precision="1" />
          <span class="discount-suffix">折</span>
        </div>
        <div class="discount-preview">{{ (editDiscount * 10).toFixed(1) }}{{ t('discount') }}</div>
      </div>
      <template #footer>
        <el-button @click="editDlg = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="saveEdit">{{ t('save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElIcon, ElButton, ElMessage, ElDialog, ElInputNumber } from 'element-plus'
import { Star, Edit } from '@element-plus/icons-vue'
import { t } from '../i18n'
import { getMemberLevels, updateMemberLevel } from '../api'

const levels = ref([])
const loading = ref(false)
const editDlg = ref(false)
const editTarget = ref(null)
const editDiscount = ref(1.0)

const loadLevels = async () => {
  loading.value = true
  try {
    levels.value = await getMemberLevels()
  } catch { levels.value = [] }
  loading.value = false
}

const openEditDialog = (level) => {
  editTarget.value = level
  editDiscount.value = level.discount
  editDlg.value = true
}

const saveEdit = async () => {
  try {
    await updateMemberLevel(editTarget.value.name, editDiscount.value)
    ElMessage.success(t('updateSuccess'))
    editDlg.value = false
    await loadLevels()
  } catch { ElMessage.error(t('operationFailed')) }
}

onMounted(loadLevels)
</script>

<style scoped>
.member-levels-page { display: flex; flex-direction: column; gap: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: center; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.levels-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 16px; }
.level-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  transition: all 0.2s;
}
.level-card:hover { border-color: var(--accent-primary); }
.level-header { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }
.level-icon {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  color: var(--text-secondary);
}
.level-info { display: flex; flex-direction: column; gap: 2px; }
.level-name { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.level-count { font-size: 12px; color: var(--text-tertiary); }
.level-discount {
  display: flex;
  align-items: baseline;
  gap: 4px;
  margin-bottom: 16px;
}
.discount-value { font-size: 36px; font-weight: 700; color: var(--accent-primary); }
.discount-unit { font-size: 16px; color: var(--text-secondary); }
.level-actions { display: flex; gap: 8px; }

.edit-discount-form { display: flex; flex-direction: column; gap: 16px; }
.discount-label { font-size: 14px; font-weight: 500; color: var(--text-secondary); }
.discount-input-group { display: flex; align-items: center; gap: 8px; }
.discount-suffix { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.discount-preview {
  text-align: center;
  padding: 12px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  font-size: 24px;
  font-weight: 700;
  color: var(--accent-primary);
}
</style>
