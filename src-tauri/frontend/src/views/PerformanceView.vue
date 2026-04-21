<template>
  <div class="performance-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('performance') }}</h1>
      <div class="header-actions">
        <el-date-picker v-model="month" type="month" value-format="YYYY-MM" />
        <el-button type="primary" :icon="Setting" @click="showConfig = true">{{ t('config') }}</el-button>
      </div>
    </div>

    <div class="ranking-section">
      <h3>{{ t('performanceRanking') }}</h3>
      <div class="ranking-list">
        <div v-for="(staff, idx) in rankingList" :key="staff.id" class="ranking-item">
          <div class="rank-badge" :class="`rank-${idx + 1}`">
            {{ idx + 1 }}
          </div>
          <el-avatar :size="40" :style="{ background: getAvatarColor(staff.id) }">
            {{ staff.name?.charAt(0) }}
          </el-avatar>
          <div class="staff-info">
            <span class="staff-name">{{ staff.name }}</span>
            <span class="staff-dept">{{ t(staff.department) }}</span>
          </div>
          <div class="score-bar">
            <div class="bar-fill" :style="{ width: (staff.score / 100) * 100 + '%' }"></div>
          </div>
          <span class="score-value">{{ staff.score }}</span>
        </div>
      </div>
    </div>

    <div class="metrics-grid">
      <div class="metric-card" v-for="metric in metrics" :key="metric.id">
        <div class="metric-header">
          <span class="metric-name">{{ t(metric.name) }}</span>
          <span class="metric-weight">{{ t('weight') }}: {{ metric.weight }}%</span>
        </div>
        <div class="metric-value">{{ metric.avgValue }}</div>
        <div class="metric-desc">{{ t(metric.desc) }}</div>
      </div>
    </div>

    <div class="detail-section">
      <div class="section-header">
        <h3>{{ t('performanceDetails') }}</h3>
        <el-select v-model="filterStaff" :placeholder="t('allStaff')" clearable size="small">
          <el-option v-for="s in staffList" :key="s.id" :label="s.name" :value="s.id" />
        </el-select>
      </div>
      <el-table :data="filteredDetails" stripe>
        <el-table-column :label="t('employee')" min-width="120">
          <template #default="{ row }">
            <div class="employee-cell">
              <el-avatar :size="32">{{ row.name?.charAt(0) }}</el-avatar>
              <span>{{ row.name }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('serviceCount')" width="100">
          <template #default="{ row }">{{ row.serviceCount }}</template>
        </el-table-column>
        <el-table-column :label="t('revenue')" width="100">
          <template #default="{ row }">¥{{ row.revenue }}</template>
        </el-table-column>
        <el-table-column :label="t('satisfaction')" width="80">
          <template #default="{ row }">
            <el-tag size="small">{{ row.satisfaction }}%</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('attendance')" width="80">
          <template #default="{ row }">
            <el-tag :type="row.attendance === 100 ? 'success' : 'warning'" size="small">
              {{ row.attendance }}%
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('bonus')" width="100">
          <template #default="{ row }">
            <span class="bonus">+¥{{ row.bonus }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('totalScore')" width="80">
          <template #default="{ row }">
            <span class="score">{{ row.score }}</span>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="showConfig" :title="t('performanceConfig')" width="500px">
      <el-form label-width="120px">
        <el-form-item :label="t('serviceWeight')">
          <el-slider :model-value="40" :max="100" show-input />
        </el-form-item>
        <el-form-item :label="t('revenueWeight')">
          <el-slider :model-value="30" :max="100" show-input />
        </el-form-item>
        <el-form-item :label="t('satisfactionWeight')">
          <el-slider :model-value="20" :max="100" show-input />
        </el-form-item>
        <el-form-item :label="t('attendanceWeight')">
          <el-slider :model-value="10" :max="100" show-input />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showConfig = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="showConfig = false">{{ t('save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { Setting } from '@element-plus/icons-vue'
import { t } from '../i18n'


const month = ref(new Date().toISOString().slice(0, 7))
const filterStaff = ref('')
const showConfig = ref(false)

const rankingList = ref([
  { id: 1, name: '张三', department: 'front', score: 95 },
  { id: 2, name: '李四', department: 'service', score: 88 },
  { id: 3, name: '王五', department: 'management', score: 85 },
  { id: 4, name: '赵六', department: 'finance', score: 82 },
])

const metrics = ref([
  { id: 1, name: 'serviceCount', weight: 40, avgValue: '156单', desc: '本月服务单数' },
  { id: 2, name: 'revenue', weight: 30, avgValue: '¥12,350', desc: '个人贡献营收' },
  { id: 3, name: 'satisfaction', weight: 20, avgValue: '96%', desc: '客户满意度' },
  { id: 4, name: 'attendance', weight: 10, avgValue: '98%', desc: '出勤率' },
])

const staffList = ref([
  { id: 1, name: '张三' },
  { id: 2, name: '李四' },
  { id: 3, name: '王五' },
  { id: 4, name: '赵六' },
])

const details = ref([
  { id: 1, name: '张三', serviceCount: 182, revenue: 14560, satisfaction: 98, attendance: 100, bonus: 1200, score: 95 },
  { id: 2, name: '李四', serviceCount: 165, revenue: 12350, satisfaction: 94, attendance: 96, bonus: 800, score: 88 },
  { id: 3, name: '王五', serviceCount: 0, revenue: 28000, satisfaction: 96, attendance: 100, bonus: 1500, score: 85 },
  { id: 4, name: '赵六', serviceCount: 148, revenue: 10200, satisfaction: 92, attendance: 94, bonus: 500, score: 82 },
])

const filteredDetails = computed(() => {
  if (!filterStaff.value) return details.value
  return details.value.filter(d => d.id === filterStaff.value)
})

const getAvatarColor = (id) => {
  const colors = ['#3b82f6', '#10b981', '#f59e0b', '#8b5cf6', '#ef4444']
  return colors[id % colors.length]
}
</script>

<style scoped>
.performance-page {
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

.ranking-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  margin-bottom: 24px;
}

.ranking-section h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
}

.ranking-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.ranking-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--hover-bg);
  border-radius: 8px;
}

.rank-badge {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 14px;
  background: var(--border-color);
}

.rank-badge.rank-1 { background: #ffd700; color: #fff; }
.rank-badge.rank-2 { background: #c0c0c0; color: #fff; }
.rank-badge.rank-3 { background: #cd7f32; color: #fff; }

.staff-info {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.staff-name {
  font-weight: 500;
}

.staff-dept {
  font-size: 12px;
  color: var(--text-secondary);
}

.score-bar {
  width: 120px;
  height: 8px;
  background: var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--primary-color), #6366f1);
  border-radius: 4px;
}

.score-value {
  width: 40px;
  text-align: right;
  font-weight: 600;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.metric-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.metric-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 12px;
}

.metric-name {
  font-weight: 600;
}

.metric-weight {
  font-size: 12px;
  color: var(--text-secondary);
}

.metric-value {
  font-size: 28px;
  font-weight: 600;
  margin-bottom: 4px;
}

.metric-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

.detail-section {
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

.section-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.employee-cell {
  display: flex;
  align-items: center;
  gap: 8px;
}

.bonus {
  color: var(--accent-success);
  font-weight: 500;
}

.score {
  font-weight: 600;
}
</style>
