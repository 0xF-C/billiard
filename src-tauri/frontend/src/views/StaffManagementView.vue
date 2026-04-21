<template>
  <div class="page-wrapper">
    <div class="page-header">
      <div>
        <h1 class="page-title">{{ t('nav_staff') }}</h1>
        <p class="page-subtitle">{{ t('sub_staff_files') }} - 排班考勤、绩效、权限、工资</p>
      </div>
      <div class="header-actions">
        <el-button type="primary" size="large">
          <el-icon><Plus /></el-icon>
          {{ t('addStaff') }}
        </el-button>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">{{ t('activeStaff') }}</div>
        <div class="stat-value">24</div>
        <div class="stat-meta">{{ t('people') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('todayAttendance') }}</div>
        <div class="stat-value">22</div>
        <div class="stat-meta">{{ t('times') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('totalPayroll') }}</div>
        <div class="stat-value">¥48,600</div>
        <div class="stat-meta">{{ t('pendingPayment') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('avgPerformance') }}</div>
        <div class="stat-value">8.6</div>
        <div class="stat-meta">{{ t('scoreMax10') }}</div>
      </div>
    </div>

    <div class="content-grid">
      <div class="section">
        <h2 class="section-title">{{ t('quickNav') }}</h2>
        <div class="nav-cards">
          <router-link to="/staff-files" class="nav-card">
            <el-icon class="card-icon"><User /></el-icon>
            <div class="card-title">{{ t('sub_staff_files') }}</div>
            <p class="card-desc">{{ t('staffFilesDesc') }}</p>
          </router-link>
          <router-link to="/attendance" class="nav-card">
            <el-icon class="card-icon"><Clock /></el-icon>
            <div class="card-title">{{ t('sub_attendance') }}</div>
            <p class="card-desc">{{ t('attendanceDesc') }}</p>
          </router-link>
          <router-link to="/performance" class="nav-card">
            <el-icon class="card-icon"><DataAnalysis /></el-icon>
            <div class="card-title">{{ t('sub_performance') }}</div>
            <p class="card-desc">{{ t('performanceDesc') }}</p>
          </router-link>
          <router-link to="/permissions" class="nav-card">
            <el-icon class="card-icon"><Setting /></el-icon>
            <div class="card-title">{{ t('sub_permissions') }}</div>
            <p class="card-desc">{{ t('permissionsDesc') }}</p>
          </router-link>
          <router-link to="/salary" class="nav-card">
            <el-icon class="card-icon"><Money /></el-icon>
            <div class="card-title">{{ t('sub_salary') }}</div>
            <p class="card-desc">{{ t('salaryDesc') }}</p>
          </router-link>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">{{ t('staffRoleDistribution') }}</h2>
        <div class="role-distribution">
          <div v-for="role in staffRoles" :key="role.id" class="role-item">
            <div class="role-header">
              <span class="role-name">{{ role.name }}</span>
              <span class="role-count">{{ role.count }}人</span>
            </div>
            <div class="role-bar">
              <div class="role-progress" :style="{ width: role.percentage + '%', backgroundColor: role.color }"></div>
            </div>
            <div class="role-footer">
              <span class="role-percentage">{{ role.percentage }}%</span>
              <span class="role-salary">¥{{ role.avgSalary }}/月</span>
            </div>
          </div>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">{{ t('monthAttendanceStats') }}</h2>
        <el-table :data="attendanceStats" stripe style="width: 100%">
          <el-table-column prop="staffName" :label="t('staffName')" width="120" />
          <el-table-column prop="position" :label="t('position')" width="100" />
          <el-table-column prop="workDays" :label="t('workDays')" width="100" />
          <el-table-column prop="leaveDays" :label="t('leaveDays')" width="100" />
          <el-table-column prop="attendance" :label="t('attendanceRate')" width="100">
            <template #default="{ row }">
              <el-progress :percentage="row.attendance" />
            </template>
          </el-table-column>
          <el-table-column label="操作" width="100">
            <template #default>
              <el-button link type="primary" size="small">{{ t('detail') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { Plus, User, Clock, DataAnalysis, Setting, Money } from '@element-plus/icons-vue'
import { t } from '../i18n'

const staffRoles = ref([
  { id: 1, name: t('manager'), count: 2, percentage: 8, color: '#e91e63', avgSalary: 8000 },
  { id: 2, name: t('cashier'), count: 4, percentage: 17, color: '#2196f3', avgSalary: 3500 },
  { id: 3, name: t('waiter'), count: 14, percentage: 58, color: '#4caf50', avgSalary: 2800 },
  { id: 4, name: t('assistant'), count: 4, percentage: 17, color: '#ffc107', avgSalary: 3200 }
])

const attendanceStats = ref([
  { staffName: '王经理', position: t('manager'), workDays: 22, leaveDays: 1, attendance: 96 },
  { staffName: '李收银', position: t('cashier'), workDays: 21, leaveDays: 2, attendance: 91 },
  { staffName: '张服务', position: t('waiter'), workDays: 20, leaveDays: 3, attendance: 87 },
  { staffName: '刘助教', position: t('assistant'), workDays: 22, leaveDays: 1, attendance: 96 },
  { staffName: '陈服务', position: t('waiter'), workDays: 19, leaveDays: 4, attendance: 83 }
])
</script>

<style scoped>
.page-wrapper {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.page-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.stat-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stat-label {
  font-size: 12px;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  color: var(--accent-primary);
}

.stat-meta {
  font-size: 13px;
  color: var(--text-secondary);
}

.content-grid {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.section {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 24px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.nav-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 12px;
}

.nav-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-muted);
  border-radius: var(--radius-md);
  padding: 16px;
  text-align: center;
  cursor: pointer;
  transition: all var(--transition-fast);
  text-decoration: none;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.nav-card:hover {
  border-color: var(--accent-primary);
  background: var(--bg-active);
}

.card-icon {
  font-size: 24px;
  color: var(--accent-primary);
  margin-bottom: 8px;
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.card-desc {
  font-size: 12px;
  color: var(--text-tertiary);
  margin: 4px 0 0 0;
}

.role-distribution {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.role-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.role-header {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.role-name {
  color: var(--text-secondary);
  font-weight: 500;
}

.role-count {
  color: var(--text-tertiary);
}

.role-bar {
  height: 12px;
  background: var(--bg-primary);
  border-radius: 6px;
  overflow: hidden;
}

.role-progress {
  height: 100%;
  border-radius: 6px;
  transition: width 0.3s ease;
}

.role-footer {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-tertiary);
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    gap: 16px;
  }

  .nav-cards {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>
