<template>
  <div class="staff-files-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('staffFiles') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('addStaff') }}</el-button>
    </div>

    <div class="filter-bar">
      <el-input v-model="searchKw" :placeholder="t('searchName')" class="search-input" clearable>
        <template #prefix><el-icon><Search /></el-icon></template>
      </el-input>
      <el-select v-model="filterDept" :placeholder="t('department')" clearable>
        <el-option :label="t('all')" value="" />
        <el-option :label="t('frontDesk')" value="front" />
        <el-option :label="t('service')" value="service" />
        <el-option :label="t('finance')" value="finance" />
        <el-option :label="t('management')" value="management" />
      </el-select>
      <el-select v-model="filterStatus" :placeholder="t('status')" clearable>
        <el-option :label="t('all')" value="" />
        <el-option :label="t('active')" value="active" />
        <el-option :label="t('onLeave')" value="leave" />
        <el-option :label="t('resigned')" value="resigned" />
      </el-select>
    </div>

    <div class="staff-table">
      <el-table :data="filteredStaff" stripe>
        <el-table-column :label="t('employee')" min-width="160">
          <template #default="{ row }">
            <div class="employee-cell">
              <el-avatar :size="40" :style="{ background: getAvatarColor(row.id) }">
                {{ row.name?.charAt(0) }}
              </el-avatar>
              <div class="employee-info">
                <span class="employee-name">{{ row.name }}</span>
                <span class="employee-id">ID: {{ row.employeeId }}</span>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('department')" width="100">
          <template #default="{ row }">
            {{ t(row.department) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('position')" width="100">
          <template #default="{ row }">
            {{ row.position }}
          </template>
        </el-table-column>
        <el-table-column :label="t('phone')" width="120">
          <template #default="{ row }">
            {{ row.phone }}
          </template>
        </el-table-column>
        <el-table-column :label="t('joinDate')" width="100">
          <template #default="{ row }">
            {{ formatDate(row.joinDate) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('status')" width="80">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)" size="small">
              {{ t(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="160" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="viewDetail(row)">{{ t('detail') }}</el-button>
            <el-button link type="primary" @click="editStaff(row)">{{ t('edit') }}</el-button>
            <el-button link type="danger" @click="deleteStaff(row)">{{ t('delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 添加员工弹窗 -->
    <el-dialog v-model="showAdd" :title="t('addStaff')" width="600px">
      <el-form :model="staffForm" label-width="100px">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item :label="t('name')">
              <el-input v-model="staffForm.name" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('phone')">
              <el-input v-model="staffForm.phone" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item :label="t('department')">
              <el-select v-model="staffForm.department" :placeholder="t('select')">
                <el-option :label="t('frontDesk')" value="front" />
                <el-option :label="t('service')" value="service" />
                <el-option :label="t('finance')" value="finance" />
                <el-option :label="t('management')" value="management" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('position')">
              <el-input v-model="staffForm.position" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item :label="t('joinDate')">
              <el-date-picker v-model="staffForm.joinDate" type="date" value-format="YYYY-MM-DD" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('status')">
              <el-select v-model="staffForm.status">
                <el-option :label="t('active')" value="active" />
                <el-option :label="t('onLeave')" value="leave" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item :label="t('idCard')">
          <el-input v-model="staffForm.idCard" />
        </el-form-item>
        <el-form-item :label="t('address')">
          <el-input v-model="staffForm.address" />
        </el-form-item>
        <el-form-item :label="t('emergencyContact')">
          <el-input v-model="staffForm.emergencyContact" />
        </el-form-item>
        <el-form-item :label="t('baseSalary')">
          <el-input-number v-model="staffForm.baseSalary" :min="0" :precision="0" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAdd = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitStaff">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- 详情弹窗 -->
    <el-dialog v-model="showDetail" :title="t('staffDetail')" width="600px">
      <el-descriptions :column="2" border v-if="selectedStaff">
        <el-descriptions-item :label="t('name')">{{ selectedStaff.name }}</el-descriptions-item>
        <el-descriptions-item :label="t('employeeId')">{{ selectedStaff.employeeId }}</el-descriptions-item>
        <el-descriptions-item :label="t('department')">{{ t(selectedStaff.department) }}</el-descriptions-item>
        <el-descriptions-item :label="t('position')">{{ selectedStaff.position }}</el-descriptions-item>
        <el-descriptions-item :label="t('phone')">{{ selectedStaff.phone }}</el-descriptions-item>
        <el-descriptions-item :label="t('joinDate')">{{ formatDate(selectedStaff.joinDate) }}</el-descriptions-item>
        <el-descriptions-item :label="t('status')">
          <el-tag :type="getStatusType(selectedStaff.status)">{{ t(selectedStaff.status) }}</el-tag>
        </el-descriptions-item>
        <el-descriptions-item :label="t('baseSalary')">¥{{ selectedStaff.baseSalary }}</el-descriptions-item>
        <el-descriptions-item :label="t('idCard')">{{ selectedStaff.idCard || '-' }}</el-descriptions-item>
        <el-descriptions-item :label="t('address')">{{ selectedStaff.address || '-' }}</el-descriptions-item>
        <el-descriptions-item :label="t('emergencyContact')">{{ selectedStaff.emergencyContact || '-' }}</el-descriptions-item>
      </el-descriptions>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Search } from '@element-plus/icons-vue'
import { t } from '../i18n'


const searchKw = ref('')
const filterDept = ref('')
const filterStatus = ref('')
const showAdd = ref(false)
const showDetail = ref(false)
const selectedStaff = ref(null)

const staffList = ref([
  { id: 1, employeeId: 'EMP001', name: '张三', department: 'front', position: '前台接待', phone: '13800138001', joinDate: '2024-01-15', status: 'active', baseSalary: 4500, idCard: '110101199001011234', address: '北京市朝阳区', emergencyContact: '李四 13900139001' },
  { id: 2, employeeId: 'EMP002', name: '李四', department: 'service', position: '服务员', phone: '13800138002', joinDate: '2024-03-01', status: 'active', baseSalary: 4000 },
  { id: 3, employeeId: 'EMP003', name: '王五', department: 'management', position: '店长', phone: '13800138003', joinDate: '2023-06-01', status: 'active', baseSalary: 8000 },
  { id: 4, employeeId: 'EMP004', name: '赵六', department: 'finance', position: '收银员', phone: '13800138004', joinDate: '2024-02-20', status: 'leave', baseSalary: 4200 },
])

const staffForm = reactive({
  name: '',
  phone: '',
  department: '',
  position: '',
  joinDate: new Date().toISOString().split('T')[0],
  status: 'active',
  idCard: '',
  address: '',
  emergencyContact: '',
  baseSalary: 4000,
})

const filteredStaff = computed(() => {
  let list = staffList.value
  if (searchKw.value) {
    list = list.filter(s => s.name.includes(searchKw.value) || s.phone.includes(searchKw.value))
  }
  if (filterDept.value) {
    list = list.filter(s => s.department === filterDept.value)
  }
  if (filterStatus.value) {
    list = list.filter(s => s.status === filterStatus.value)
  }
  return list
})

const getAvatarColor = (id) => {
  const colors = ['#3b82f6', '#10b981', '#f59e0b', '#8b5cf6', '#ef4444']
  return colors[id % colors.length]
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN')
}

const getStatusType = (status) => {
  const types = { active: 'success', leave: 'warning', resigned: 'info' }
  return types[status] || 'info'
}

const viewDetail = (staff) => {
  selectedStaff.value = staff
  showDetail.value = true
}

const editStaff = (staff) => {
  ElMessage.info('编辑功能开发中')
}

const deleteStaff = async (staff) => {
  await ElMessageBox.confirm(t('confirmDelete'), t('confirm'), { type: 'warning' })
  const idx = staffList.value.findIndex(s => s.id === staff.id)
  if (idx !== -1) {
    staffList.value.splice(idx, 1)
    ElMessage.success(t('deleteSuccess'))
  }
}

const submitStaff = () => {
  if (!staffForm.name || !staffForm.phone) {
    ElMessage.warning(t('pleaseComplete'))
    return
  }
  staffList.value.unshift({
    id: Date.now(),
    employeeId: `EMP${String(staffList.value.length + 1).padStart(3, '0')}`,
    ...staffForm,
  })
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
  Object.assign(staffForm, { name: '', phone: '', department: '', position: '', joinDate: new Date().toISOString().split('T')[0], status: 'active', idCard: '', address: '', emergencyContact: '', baseSalary: 4000 })
}
</script>

<style scoped>
.staff-files-page {
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

.filter-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.search-input {
  width: 200px;
}

.staff-table {
  background: var(--card-bg);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.employee-cell {
  display: flex;
  align-items: center;
  gap: 12px;
}

.employee-info {
  display: flex;
  flex-direction: column;
}

.employee-name {
  font-weight: 500;
}

.employee-id {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>
