<template>
  <div class="salary-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('salary') }}</h1>
      <div class="header-actions">
        <el-date-picker v-model="month" type="month" value-format="YYYY-MM" />
        <el-button type="primary" :icon="Check" @click="confirmAll">{{ t('confirmAll') }}</el-button>
        <el-button :icon="Download">{{ t('export') }}</el-button>
      </div>
    </div>

    <div class="summary-row">
      <div class="summary-card">
        <span class="summary-label">{{ t('totalPayroll') }}</span>
        <span class="summary-value">¥{{ summary.total.toLocaleString() }}</span>
      </div>
      <div class="summary-card">
        <span class="summary-label">{{ t('confirmed') }}</span>
        <span class="summary-value">{{ summary.confirmed }}/{{ summary.count }}</span>
      </div>
      <div class="summary-card">
        <span class="summary-label">{{ t('pending') }}</span>
        <span class="summary-value pending">{{ summary.pending }}</span>
      </div>
    </div>

    <div class="salary-table">
      <el-table :data="salaryList" stripe>
        <el-table-column :label="t('employee')" min-width="140">
          <template #default="{ row }">
            <div class="employee-cell">
              <el-avatar :size="36">{{ row.name?.charAt(0) }}</el-avatar>
              <div class="employee-info">
                <span class="name">{{ row.name }}</span>
                <span class="dept">{{ t(row.department) }}</span>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('baseSalary')" width="100">
          <template #default="{ row }">¥{{ row.baseSalary }}</template>
        </el-table-column>
        <el-table-column :label="t('bonus')" width="80">
          <template #default="{ row }">
            <span class="bonus">+¥{{ row.bonus }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('overtime')" width="80">
          <template #default="{ row }">
            <span v-if="row.overtime > 0" class="overtime">+¥{{ row.overtime }}</span>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('deduction')" width="80">
          <template #default="{ row }">
            <span v-if="row.deduction > 0" class="deduction">-¥{{ row.deduction }}</span>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('actualSalary')" width="100">
          <template #default="{ row }">
            <span class="actual">¥{{ row.actualSalary }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('status')" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 'confirmed' ? 'success' : 'warning'" size="small">
              {{ t(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="120" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="viewDetail(row)">{{ t('detail') }}</el-button>
            <el-button link type="success" @click="confirmOne(row)" v-if="row.status !== 'confirmed'">
              {{ t('confirm') }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- {{ t('DetailDialog') }} -->
    <el-dialog v-model="showDetail" :title="t('salaryDetail')" width="600px" top="5vh" append-to-body>
      <template v-if="selectedStaff">
        <el-descriptions :column="2" border>
          <el-descriptions-item :label="t('employee')">{{ selectedStaff.name }}</el-descriptions-item>
          <el-descriptions-item :label="t('department')">{{ t(selectedStaff.department) }}</el-descriptions-item>
        </el-descriptions>

        <div class="salary-breakdown">
          <h4>{{ t('salaryBreakdown') }}</h4>
          <div class="breakdown-item">
            <span>{{ t('baseSalary') }}</span>
            <span>¥{{ selectedStaff.baseSalary }}</span>
          </div>
          <div class="breakdown-item add">
            <span>{{ t('performanceBonus') }}</span>
            <span>+¥{{ selectedStaff.bonus }}</span>
          </div>
          <div class="breakdown-item add" v-if="selectedStaff.overtime > 0">
            <span>{{ t('overtimePay') }}</span>
            <span>+¥{{ selectedStaff.overtime }}</span>
          </div>
          <div class="breakdown-item sub" v-if="selectedStaff.deduction > 0">
            <span>{{ t('lateDeduction') }}</span>
            <span>-¥{{ selectedStaff.deduction }}</span>
          </div>
          <div class="breakdown-item total">
            <span>{{ t('actualSalary') }}</span>
            <span>¥{{ selectedStaff.actualSalary }}</span>
          </div>
        </div>

        <div class="work-summary">
          <h4>{{ t('workSummary') }}</h4>
          <el-row :gutter="20">
            <el-col :span="8">
              <div class="summary-item">
                <span class="value">{{ selectedStaff.workDays || 22 }}</span>
                <span class="label">{{ t('workDays') }}</span>
              </div>
            </el-col>
            <el-col :span="8">
              <div class="summary-item">
                <span class="value">{{ selectedStaff.serviceCount || 156 }}</span>
                <span class="label">{{ t('serviceCount') }}</span>
              </div>
            </el-col>
            <el-col :span="8">
              <div class="summary-item">
                <span class="value">{{ selectedStaff.lateCount || 2 }}</span>
                <span class="label">{{ t('lateTimes') }}</span>
              </div>
            </el-col>
          </el-row>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Check, Download } from '@element-plus/icons-vue'
import { t } from '../i18n'


const month = ref(new Date().toISOString().slice(0, 7))
const showDetail = ref(false)
const selectedStaff = ref(null)

const salaryList = ref([
  { id: 1, name: '张三', department: 'front', baseSalary: 4500, bonus: 1200, overtime: 200, deduction: 50, actualSalary: 5850, status: 'confirmed', workDays: 22, serviceCount: 182, lateCount: 1 },
  { id: 2, name: '李四', department: 'service', baseSalary: 4000, bonus: 800, overtime: 0, deduction: 100, actualSalary: 4700, status: 'pending', workDays: 21, serviceCount: 165, lateCount: 2 },
  { id: 3, name: '王五', department: 'management', baseSalary: 8000, bonus: 1500, overtime: 500, deduction: 0, actualSalary: 10000, status: 'confirmed', workDays: 22, serviceCount: 0, lateCount: 0 },
  { id: 4, name: '赵六', department: 'finance', baseSalary: 4200, bonus: 500, overtime: 0, deduction: 0, actualSalary: 4700, status: 'pending', workDays: 22, serviceCount: 148, lateCount: 0 },
])

const summary = computed(() => ({
  total: salaryList.value.reduce((sum, s) => sum + s.actualSalary, 0),
  count: salaryList.value.length,
  confirmed: salaryList.value.filter(s => s.status === 'confirmed').length,
  pending: salaryList.value.filter(s => s.status === 'pending').length,
}))

const viewDetail = (staff) => {
  selectedStaff.value = staff
  showDetail.value = true
}

const confirmOne = async (staff) => {
  await ElMessageBox.confirm(t('confirmSalary'), t('confirm'), { type: 'info' })
  staff.status = 'confirmed'
  ElMessage.success(t('confirmSuccess'))
}

const confirmAll = async () => {
  await ElMessageBox.confirm(t('confirmAllSalary'), t('confirm'), { type: 'warning' })
  salaryList.value.forEach(s => s.status = 'confirmed')
  ElMessage.success(t('confirmSuccess'))
}
</script>

<style scoped>
.salary-page {
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
}

.summary-label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.summary-value {
  font-size: 28px;
  font-weight: 600;
}

.summary-value.pending {
  color: var(--accent-warning);
}

.salary-table {
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

.name { font-weight: 500; }
.dept { font-size: 12px; color: var(--text-secondary); }

.bonus { color: var(--accent-success); font-weight: 500; }
.overtime { color: var(--accent-success); }
.deduction { color: var(--accent-danger); font-weight: 500; }
.actual { font-weight: 600; font-size: 16px; }

.salary-breakdown {
  margin-top: 20px;
  padding: 16px;
  background: var(--hover-bg);
  border-radius: 8px;
}

.salary-breakdown h4 {
  margin: 0 0 16px 0;
  font-size: 14px;
  font-weight: 600;
}

.breakdown-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-color);
}

.breakdown-item.add span:last-child { color: var(--accent-success); }
.breakdown-item.sub span:last-child { color: var(--accent-danger); }

.breakdown-item.total {
  border-top: 2px solid var(--text-primary);
  border-bottom: none;
  margin-top: 8px;
  padding-top: 12px;
  font-weight: 600;
  font-size: 18px;
}

.work-summary {
  margin-top: 20px;
}

.work-summary h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
}

.summary-item {
  text-align: center;
  padding: 12px;
  background: var(--hover-bg);
  border-radius: 8px;
}

.summary-item .value {
  display: block;
  font-size: 24px;
  font-weight: 600;
}

.summary-item .label {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>
