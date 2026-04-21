<template>
  <div class="blacklist-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('blacklist') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('addBlacklist') }}</el-button>
    </div>

    <div class="stats-row">
      <div class="stat-item">
        <span class="stat-value">{{ blacklist.length }}</span>
        <span class="stat-label">{{ t('totalBlacklisted') }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{{ permanentCount }}</span>
        <span class="stat-label">{{ t('permanentBan') }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{{ temporaryCount }}</span>
        <span class="stat-label">{{ t('temporaryBan') }}</span>
      </div>
    </div>

    <div class="filter-bar">
      <el-input
        v-model="searchKw"
        :placeholder="t('searchPhone')"
        class="search-input"
        clearable
      >
        <template #prefix>
          <el-icon><Search /></el-icon>
        </template>
      </el-input>
      <el-select v-model="filterType" :placeholder="t('type')" clearable>
        <el-option :label="t('all')" value="" />
        <el-option :label="t('permanentBan')" value="permanent" />
        <el-option :label="t('temporaryBan')" value="temporary" />
      </el-select>
      <el-select v-model="filterReason" :placeholder="t('reason')" clearable>
        <el-option :label="t('all')" value="" />
        <el-option :label="t('reasonArrears')" value="arrears" />
        <el-option :label="t('reasonViolation')" value="violation" />
        <el-option :label="t('reasonFraud')" value="fraud" />
        <el-option :label="t('reasonOther')" value="other" />
      </el-select>
    </div>

    <div class="blacklist-table">
      <el-table :data="filteredBlacklist" stripe>
        <el-table-column :label="t('member')" min-width="160">
          <template #default="{ row }">
            <div class="member-cell">
              <el-avatar :size="36" style="background: #ef4444;">
                {{ row.name?.charAt(0) }}
              </el-avatar>
              <div class="member-info">
                <div class="member-name">{{ row.name }}</div>
                <div class="member-phone">{{ row.phone }}</div>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('banType')" width="100">
          <template #default="{ row }">
            <el-tag :type="row.type === 'permanent' ? 'danger' : 'warning'" size="small">
              {{ row.type === 'permanent' ? t('permanent') : t('temporary') }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('reason')" min-width="140">
          <template #default="{ row }">
            {{ t('reason_' + row.reason) || row.reason }}
          </template>
        </el-table-column>
        <el-table-column :label="t('description')" min-width="180">
          <template #default="{ row }">
            <span class="desc-text">{{ row.description || '-' }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('bannedAt')" width="120">
          <template #default="{ row }">
            {{ formatDate(row.bannedAt) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('expireAt')" width="120">
          <template #default="{ row }">
            <span v-if="row.type === 'permanent'" class="never">{{ t('never') }}</span>
            <span v-else-if="isExpired(row.expireAt)" class="expired">{{ t('expired') }}</span>
            <span v-else>{{ formatDate(row.expireAt) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('operator')" width="90">
          <template #default="{ row }">
            {{ row.operator }}
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="160" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="viewDetail(row)">{{ t('detail') }}</el-button>
            <el-button link type="success" @click="unblock(row)" v-if="canUnblock(row)">
              {{ t('unblock') }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 添加黑名单弹窗 -->
    <el-dialog v-model="showAdd" :title="t('addBlacklist')" width="500px">
      <el-form :model="addForm" :rules="addRules" ref="addFormRef" label-width="100px">
        <el-form-item :label="t('phone')" prop="phone">
          <el-input v-model="addForm.phone" :placeholder="t('inputPhone')">
            <template #append>
              <el-button @click="searchMember">{{ t('search') }}</el-button>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item :label="t('name')" v-if="foundMember">
          <el-input :value="foundMember.name" disabled />
        </el-form-item>
        <el-form-item :label="t('banType')" prop="type">
          <el-radio-group v-model="addForm.type">
            <el-radio value="permanent">{{ t('permanentBan') }}</el-radio>
            <el-radio value="temporary">{{ t('temporaryBan') }}</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item :label="t('expireAt')" v-if="addForm.type === 'temporary'" prop="expireAt">
          <el-date-picker
            v-model="addForm.expireAt"
            type="date"
            :placeholder="t('selectExpireDate')"
            value-format="YYYY-MM-DD"
          />
        </el-form-item>
        <el-form-item :label="t('reason')" prop="reason">
          <el-select v-model="addForm.reason" :placeholder="t('selectReason')">
            <el-option :label="t('reasonArrears')" value="arrears" />
            <el-option :label="t('reasonViolation')" value="violation" />
            <el-option :label="t('reasonFraud')" value="fraud" />
            <el-option :label="t('reasonOther')" value="other" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('description')">
          <el-input v-model="addForm.description" type="textarea" :rows="3" :placeholder="t('inputDescription')" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAdd = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitAdd">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- 详情弹窗 -->
    <el-dialog v-model="showDetail" :title="t('blacklistDetail')" width="500px">
      <el-descriptions :column="1" border>
        <el-descriptions-item :label="t('name')">{{ detailItem?.name }}</el-descriptions-item>
        <el-descriptions-item :label="t('phone')">{{ detailItem?.phone }}</el-descriptions-item>
        <el-descriptions-item :label="t('banType')">
          <el-tag :type="detailItem?.type === 'permanent' ? 'danger' : 'warning'">
            {{ detailItem?.type === 'permanent' ? t('permanent') : t('temporary') }}
          </el-tag>
        </el-descriptions-item>
        <el-descriptions-item :label="t('reason')">{{ t('reason_' + detailItem?.reason) }}</el-descriptions-item>
        <el-descriptions-item :label="t('description')">{{ detailItem?.description || '-' }}</el-descriptions-item>
        <el-descriptions-item :label="t('bannedAt')">{{ formatDateTime(detailItem?.bannedAt) }}</el-descriptions-item>
        <el-descriptions-item :label="t('expireAt')">
          {{ detailItem?.type === 'permanent' ? t('never') : formatDateTime(detailItem?.expireAt) }}
        </el-descriptions-item>
        <el-descriptions-item :label="t('operator')">{{ detailItem?.operator }}</el-descriptions-item>
      </el-descriptions>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, reactive } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Search } from '@element-plus/icons-vue'
import { getMembers } from '../api'
import { t } from '../i18n'


const searchKw = ref('')
const filterType = ref('')
const filterReason = ref('')

const blacklist = ref([
  { id: 1, name: '李四', phone: '13800138001', type: 'permanent', reason: 'arrears', description: '欠费500元未结清', bannedAt: new Date(Date.now() - 86400000 * 30), expireAt: null, operator: '管理员' },
  { id: 2, name: '王五', phone: '13800138002', type: 'temporary', reason: 'violation', description: '在店内吸烟屡次劝阻无效', bannedAt: new Date(Date.now() - 86400000 * 10), expireAt: new Date(Date.now() + 86400000 * 20), operator: '店长' },
  { id: 3, name: '赵六', phone: '13800138003', type: 'permanent', reason: 'fraud', description: '使用假币被抓获', bannedAt: new Date(Date.now() - 86400000 * 60), expireAt: null, operator: '管理员' },
])

const showAdd = ref(false)
const addFormRef = ref(null)
const addForm = reactive({
  phone: '',
  type: 'permanent',
  reason: '',
  description: '',
  expireAt: '',
})
const addRules = {
  phone: [{ required: true, message: t('required'), trigger: 'blur' }],
  type: [{ required: true, message: t('required'), trigger: 'change' }],
  reason: [{ required: true, message: t('required'), trigger: 'change' }],
}

const foundMember = ref(null)

const showDetail = ref(false)
const detailItem = ref(null)

const permanentCount = computed(() => blacklist.value.filter(b => b.type === 'permanent').length)
const temporaryCount = computed(() => blacklist.value.filter(b => b.type === 'temporary').length)

const filteredBlacklist = computed(() => {
  let list = blacklist.value
  if (searchKw.value) {
    list = list.filter(b => b.phone.includes(searchKw.value) || b.name.includes(searchKw.value))
  }
  if (filterType.value) {
    list = list.filter(b => b.type === filterType.value)
  }
  if (filterReason.value) {
    list = list.filter(b => b.reason === filterReason.value)
  }
  return list
})

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN')
}

const formatDateTime = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleString('zh-CN')
}

const isExpired = (expireAt) => {
  if (!expireAt) return false
  return new Date(expireAt) < new Date()
}

const canUnblock = (item) => {
  return item.type === 'temporary' || isExpired(item.expireAt)
}

const searchMember = async () => {
  if (!addForm.phone) return
  const res = await getMembers(addForm.phone)
  if (res && res.length > 0) {
    foundMember.value = res[0]
  } else {
    foundMember.value = null
    ElMessage.warning(t('memberNotFound'))
  }
}

const submitAdd = async () => {
  await addFormRef.value?.validate()
  blacklist.value.unshift({
    id: Date.now(),
    name: foundMember.value?.name || '未知',
    phone: addForm.phone,
    type: addForm.type,
    reason: addForm.reason,
    description: addForm.description,
    bannedAt: new Date(),
    expireAt: addForm.expireAt || null,
    operator: '当前用户',
  })
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
  addForm.phone = ''
  addForm.reason = ''
  addForm.description = ''
  foundMember.value = null
}

const viewDetail = (item) => {
  detailItem.value = item
  showDetail.value = true
}

const unblock = async (item) => {
  await ElMessageBox.confirm(t('confirmUnblock'), t('confirm'), { type: 'warning' })
  const idx = blacklist.value.findIndex(b => b.id === item.id)
  if (idx !== -1) {
    blacklist.value.splice(idx, 1)
    ElMessage.success(t('unblockSuccess'))
  }
}
</script>

<style scoped>
.blacklist-page {
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

.stats-row {
  display: flex;
  gap: 32px;
  margin-bottom: 24px;
  padding: 16px 20px;
  background: var(--card-bg);
  border-radius: 12px;
  border: 1px solid var(--border-color);
}

.stat-item {
  display: flex;
  flex-direction: column;
}

.stat-value {
  font-size: 28px;
  font-weight: 600;
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.filter-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.search-input {
  width: 200px;
}

.blacklist-table {
  background: var(--card-bg);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.member-cell {
  display: flex;
  align-items: center;
  gap: 12px;
}

.member-info {
  display: flex;
  flex-direction: column;
}

.member-name {
  font-weight: 500;
}

.member-phone {
  font-size: 12px;
  color: var(--text-secondary);
}

.desc-text {
  font-size: 13px;
  color: var(--text-secondary);
}

.never {
  color: var(--accent-danger);
  font-weight: 500;
}

.expired {
  color: var(--text-secondary);
}
</style>
