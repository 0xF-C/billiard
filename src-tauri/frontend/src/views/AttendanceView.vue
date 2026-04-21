<template>
  <div class="attendance-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('attendance') }}</h1>
      <div class="header-actions">
        <el-date-picker v-model="month" type="month" value-format="YYYY-MM" />
        <el-button :icon="Download">{{ t('export') }}</el-button>
      </div>
    </div>

    <div class="today-section">
      <div class="section-header">
        <h3>{{ t('todayAttendance') }}</h3>
        <el-button type="primary" size="small" @click="clockIn" v-if="!hasClockedIn">
          {{ t('clockIn') }}
        </el-button>
        <el-button type="warning" size="small" @click="clockOut" v-else>
          {{ t('clockOut') }}
        </el-button>
      </div>
      <div class="attendance-cards">
        <div class="card">
          <span class="card-label">{{ t('clockInTime') }}</span>
          <span class="card-value">{{ todayRecord.clockIn || '--:--' }}</span>
        </div>
        <div class="card">
          <span class="card-label">{{ t('clockOutTime') }}</span>
          <span class="card-value">{{ todayRecord.clockOut || '--:--' }}</span>
        </div>
        <div class="card">
          <span class="card-label">{{ t('workHours') }}</span>
          <span class="card-value">{{ todayRecord.hours || 0 }}h</span>
        </div>
        <div class="card">
          <span class="card-label">{{ t('status') }}</span>
          <el-tag :type="todayRecord.status === 'normal' ? 'success' : 'warning'">
            {{ t(todayRecord.status || 'pending') }}
          </el-tag>
        </div>
      </div>
    </div>

    <div class="stats-row">
      <div class="stat-card">
        <span class="stat-value">{{ stats.totalDays }}</span>
        <span class="stat-label">{{ t('workDays') }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{{ stats.normalDays }}</span>
        <span class="stat-label">{{ t('normalDays') }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{{ stats.lateDays }}</span>
        <span class="stat-label">{{ t('lateDays') }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{{ stats.leaveDays }}</span>
        <span class="stat-label">{{ t('leaveDays') }}</span>
      </div>
    </div>

    <div class="records-section">
      <div class="section-header">
        <h3>{{ t('attendanceRecords') }}</h3>
        <el-select v-model="filterStaff" :placeholder="t('allStaff')" clearable size="small">
          <el-option v-for="s in staffList" :key="s.id" :label="s.name" :value="s.id" />
        </el-select>
      </div>
      <el-table :data="filteredRecords" stripe>
        <el-table-column :label="t('date')" width="120">
          <template #default="{ row }">
            {{ formatDate(row.date) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('employee')" width="100">
          <template #default="{ row }">
            {{ row.staffName }}
          </template>
        </el-table-column>
        <el-table-column :label="t('clockIn')" width="80">
          <template #default="{ row }">
            <span :class="{ late: isLate(row.clockIn) }">{{ row.clockIn || '-' }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('clockOut')" width="80">
          <template #default="{ row }">
            <span :class="{ early: isEarly(row.clockOut) }">{{ row.clockOut || '-' }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('hours')" width="60">
          <template #default="{ row }">
            {{ row.hours || 0 }}h
          </template>
        </el-table-column>
        <el-table-column :label="t('status')" width="80">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)" size="small">{{ t(row.status) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('remark')">
          <template #default="{ row }">
            {{ row.remark || '-' }}
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { Download } from '@element-plus/icons-vue'
import { t } from '../i18n'


const month = ref(new Date().toISOString().slice(0, 7))
const filterStaff = ref('')
const hasClockedIn = ref(false)

const todayRecord = ref({
  clockIn: '09:02',
  clockOut: null,
  hours: 0,
  status: 'late',
})

const stats = ref({
  totalDays: 18,
  normalDays: 15,
  lateDays: 2,
  leaveDays: 1,
})

const staffList = ref([
  { id: 1, name: '张三' },
  { id: 2, name: '李四' },
  { id: 3, name: '王五' },
])

const records = ref([
  { date: '2026-04-19', staffId: 1, staffName: '张三', clockIn: '09:02', clockOut: null, hours: 0, status: 'late' },
  { date: '2026-04-18', staffId: 1, staffName: '张三', clockIn: '08:55', clockOut: '18:30', hours: 9.5, status: 'normal' },
  { date: '2026-04-17', staffId: 1, staffName: '张三', clockIn: '09:15', clockOut: '18:00', hours: 8.5, status: 'late', remark: '堵车' },
  { date: '2026-04-16', staffId: 2, staffName: '李四', clockIn: null, clockOut: null, hours: 0, status: 'leave', remark: '事假' },
  { date: '2026-04-15', staffId: 1, staffName: '张三', clockIn: '08:50', clockOut: '18:00', hours: 9, status: 'normal' },
])

const filteredRecords = computed(() => {
  if (!filterStaff.value) return records.value
  return records.value.filter(r => r.staffId === filterStaff.value)
})

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
}

const isLate = (time) => {
  if (!time) return false
  const [h, m] = time.split(':').map(Number)
  return h > 9 || (h === 9 && m > 0)
}

const isEarly = (time) => {
  if (!time) return false
  const [h, m] = time.split(':').map(Number)
  return h < 18
}

const getStatusType = (status) => {
  const types = { normal: 'success', late: 'warning', early: 'warning', leave: 'info', absent: 'danger' }
  return types[status] || 'info'
}

const clockIn = () => {
  const now = new Date()
  const time = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}`
  todayRecord.value.clockIn = time
  todayRecord.value.status = isLate(time) ? 'late' : 'normal'
  hasClockedIn.value = true
  ElMessage.success(`${t('clockIn')}: ${time}`)
}

const clockOut = () => {
  const now = new Date()
  const time = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}`
  todayRecord.value.clockOut = time
  const [inH, inM] = todayRecord.value.clockIn.split(':').map(Number)
  const outH = now.getHours(), outM = now.getMinutes()
  todayRecord.value.hours = ((outH * 60 + outM) - (inH * 60 + inM)) / 60
  ElMessage.success(`${t('clockOut')}: ${time}`)
}
</script>

<style scoped>
.attendance-page {
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

.today-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.attendance-cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}

.card {
  text-align: center;
  padding: 16px;
  background: var(--hover-bg);
  border-radius: 8px;
}

.card-label {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.card-value {
  font-size: 24px;
  font-weight: 600;
}

.stats-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  text-align: center;
  border: 1px solid var(--border-color);
}

.stat-value {
  display: block;
  font-size: 28px;
  font-weight: 600;
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.records-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.late { color: var(--accent-warning); }
.early { color: var(--accent-danger); }
</style>
