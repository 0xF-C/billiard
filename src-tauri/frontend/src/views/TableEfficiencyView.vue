<template>
  <div class="table-efficiency-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('tableEfficiency') }}</h1>
      <div class="header-actions">
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          range-separator="~"
          :start-placeholder="t('startDate')"
          :end-placeholder="t('endDate')"
          value-format="YYYY-MM-DD"
          @change="loadData"
        />
      </div>
    </div>

    <div class="overview-cards">
      <div class="overview-card">
        <div class="overview-icon blue">
          <el-icon><Timer /></el-icon>
        </div>
        <div class="overview-body">
          <div class="overview-value">{{ stats.avgUsageHours.toFixed(1) }}h</div>
          <div class="overview-label">{{ t('avgDailyUsage') }}</div>
        </div>
      </div>
      <div class="overview-card">
        <div class="overview-icon green">
          <el-icon><TrendCharts /></el-icon>
        </div>
        <div class="overview-body">
          <div class="overview-value">{{ stats.usageRate.toFixed(1) }}%</div>
          <div class="overview-label">{{ t('overallUsageRate') }}</div>
        </div>
      </div>
      <div class="overview-card">
        <div class="overview-icon orange">
          <el-icon><Coin /></el-icon>
        </div>
        <div class="overview-body">
          <div class="overview-value">¥{{ stats.revenuePerHour }}</div>
          <div class="overview-label">{{ t('hourlyRevenue') }}</div>
        </div>
      </div>
      <div class="overview-card">
        <div class="overview-icon purple">
          <el-icon><Avatar /></el-icon>
        </div>
        <div class="overview-body">
          <div class="overview-value">{{ stats.turnoverRate.toFixed(1) }}</div>
          <div class="overview-label">{{ t('dailyTurnover') }}</div>
        </div>
      </div>
    </div>

    <div class="main-content">
      <div class="table-rank-section">
        <div class="section-header">
          <h3>{{ t('tableRanking') }}</h3>
          <el-radio-group v-model="sortBy" size="small">
            <el-radio-button value="revenue">{{ t('revenue') }}</el-radio-button>
            <el-radio-button value="hours">{{ t('hours') }}</el-radio-button>
            <el-radio-button value="turnover">{{ t('turnover') }}</el-radio-button>
          </el-radio-group>
        </div>
        <div class="table-list">
          <div v-for="(table, idx) in sortedTables" :key="table.id" class="table-row">
            <div class="rank">{{ idx + 1 }}</div>
            <div class="table-info">
              <span class="table-name">{{ table.name }}</span>
              <el-tag size="small" :type="table.isPrivate ? 'warning' : 'info'">
                {{ table.isPrivate ? '包间' : '大厅' }}
              </el-tag>
            </div>
            <div class="table-stats">
              <div class="stat">
                <span class="stat-value">¥{{ table.revenue.toLocaleString() }}</span>
                <span class="stat-label">{{ t('revenue') }}</span>
              </div>
              <div class="stat">
                <span class="stat-value">{{ table.hours }}h</span>
                <span class="stat-label">{{ t('usage') }}</span>
              </div>
              <div class="stat">
                <span class="stat-value">{{ table.turnover }}</span>
                <span class="stat-label">{{ t('orders') }}</span>
              </div>
            </div>
            <div class="usage-bar">
              <div class="bar-fill" :style="{ width: (table.usageRate || 0) + '%' }"></div>
            </div>
            <span class="usage-rate">{{ (table.usageRate || 0).toFixed(0) }}%</span>
          </div>
        </div>
      </div>

      <div class="charts-section">
        <div class="chart-card">
          <h3>{{ t('hourlyDistribution') }}</h3>
          <div class="hourly-bars">
            <div v-for="(item, idx) in hourlyData" :key="idx" class="hour-item">
              <div class="hour-bar">
                <div class="bar" :style="{ height: (item.rate) + '%' }"></div>
              </div>
              <span class="hour-label">{{ item.hour }}</span>
            </div>
          </div>
        </div>

        <div class="chart-card">
          <h3>{{ t('weekdayDistribution') }}</h3>
          <div class="weekday-chart">
            <div v-for="(item, idx) in weekdayData" :key="idx" class="weekday-row">
              <span class="weekday-label">{{ item.day }}</span>
              <div class="weekday-bar">
                <div class="bar" :style="{ width: (item.rate) + '%' }"></div>
              </div>
              <span class="weekday-rate">{{ item.rate.toFixed(0) }}%</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { Timer, TrendCharts, Coin, Avatar } from '@element-plus/icons-vue'
import { getTables, getTableUsage, getHourlyRevenue } from '../api'
import { t } from '../i18n'


const dateRange = ref([])
const sortBy = ref('revenue')

const stats = ref({
  avgUsageHours: 6.5,
  usageRate: 68.4,
  revenuePerHour: 85,
  turnoverRate: 3.2,
})

const tables = ref([
  { id: 1, name: '1号桌', isPrivate: false, revenue: 3500, hours: 42, turnover: 18, usageRate: 70 },
  { id: 2, name: '2号桌', isPrivate: false, revenue: 3200, hours: 38, turnover: 16, usageRate: 63 },
  { id: 3, name: 'VIP-1', isPrivate: true, revenue: 4800, hours: 32, turnover: 12, usageRate: 80 },
  { id: 4, name: '3号桌', isPrivate: false, revenue: 2800, hours: 35, turnover: 15, usageRate: 58 },
  { id: 5, name: '4号桌', isPrivate: false, revenue: 2600, hours: 30, turnover: 14, usageRate: 50 },
  { id: 6, name: 'VIP-2', isPrivate: true, revenue: 4200, hours: 28, turnover: 10, usageRate: 75 },
])

const hourlyData = ref([
  { hour: '10', rate: 20 },
  { hour: '11', rate: 45 },
  { hour: '12', rate: 70 },
  { hour: '13', rate: 85 },
  { hour: '14', rate: 75 },
  { hour: '15', rate: 55 },
  { hour: '16', rate: 40 },
  { hour: '17', rate: 50 },
  { hour: '18', rate: 80 },
  { hour: '19', rate: 95 },
  { hour: '20', rate: 90 },
  { hour: '21', rate: 60 },
])

const weekdayData = ref([
  { day: '周一', rate: 55 },
  { day: '周二', rate: 50 },
  { day: '周三', rate: 58 },
  { day: '周四', rate: 62 },
  { day: '周五', rate: 78 },
  { day: '周六', rate: 95 },
  { day: '周日', rate: 85 },
])

const sortedTables = computed(() => {
  const sorted = [...tables.value]
  sorted.sort((a, b) => {
    if (sortBy.value === 'revenue') return b.revenue - a.revenue
    if (sortBy.value === 'hours') return b.hours - a.hours
    return b.turnover - a.turnover
  })
  return sorted
})

const loadData = async () => {
  await Promise.all([
    getTables(),
    getTableUsage(30),
    getHourlyRevenue(7),
  ]).catch(() => null)
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
.table-efficiency-page {
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

.overview-cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.overview-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  border: 1px solid var(--border-color);
}

.overview-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}

.overview-icon.blue { background: rgba(59, 130, 246, 0.1); color: #3b82f6; }
.overview-icon.green { background: rgba(16, 185, 129, 0.1); color: #10b981; }
.overview-icon.orange { background: rgba(245, 158, 11, 0.1); color: #f59e0b; }
.overview-icon.purple { background: rgba(139, 92, 246, 0.1); color: #8b5cf6; }

.overview-value {
  font-size: 24px;
  font-weight: 600;
}

.overview-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.main-content {
  display: grid;
  grid-template-columns: 1fr 400px;
  gap: 24px;
}

.table-rank-section {
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

.table-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.table-row {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px;
  background: var(--hover-bg);
  border-radius: 8px;
}

.rank {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: var(--primary-color);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
}

.table-info {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
}

.table-name {
  font-weight: 500;
}

.table-stats {
  display: flex;
  gap: 24px;
}

.stat {
  text-align: center;
}

.stat-value {
  font-weight: 600;
}

.stat-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.usage-bar {
  width: 80px;
  height: 8px;
  background: var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-primary), #6366f1);
  border-radius: 4px;
}

.usage-rate {
  width: 40px;
  text-align: right;
  font-size: 13px;
  font-weight: 500;
}

.charts-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.chart-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.chart-card h3 {
  margin: 0 0 16px 0;
  font-size: 14px;
  font-weight: 600;
}

.hourly-bars {
  display: flex;
  align-items: flex-end;
  gap: 4px;
  height: 100px;
}

.hour-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.hour-bar {
  width: 100%;
  height: 80px;
  display: flex;
  align-items: flex-end;
}

.hour-bar .bar {
  width: 100%;
  background: linear-gradient(to top, var(--primary-color), #6366f1);
  border-radius: 2px;
  min-height: 4px;
}

.hour-label {
  font-size: 10px;
  color: var(--text-secondary);
  margin-top: 4px;
}

.weekday-chart {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.weekday-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.weekday-label {
  width: 36px;
  font-size: 12px;
}

.weekday-bar {
  flex: 1;
  height: 12px;
  background: var(--border-color);
  border-radius: 6px;
  overflow: hidden;
}

.weekday-bar .bar {
  height: 100%;
  background: linear-gradient(90deg, var(--primary-color), #6366f1);
  border-radius: 6px;
}

.weekday-rate {
  width: 36px;
  text-align: right;
  font-size: 12px;
  font-weight: 500;
}
</style>
