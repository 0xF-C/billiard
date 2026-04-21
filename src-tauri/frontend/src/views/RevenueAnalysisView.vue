<template>
  <div class="revenue-analysis-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('revenueAnalysis') }}</h1>
      <div class="header-actions">
        <el-radio-group v-model="dateRange" @change="loadData">
          <el-radio-button value="7">{{ t('last7Days') }}</el-radio-button>
          <el-radio-button value="30">{{ t('last30Days') }}</el-radio-button>
          <el-radio-button value="90">{{ t('last90Days') }}</el-radio-button>
        </el-radio-group>
      </div>
    </div>

    <div class="summary-cards">
      <div class="summary-card">
        <div class="card-title">{{ t('totalRevenue') }}</div>
        <div class="card-value">¥{{ summary.total.toLocaleString() }}</div>
        <div class="card-trend up" v-if="summary.trend > 0">
          <el-icon><Top /></el-icon> {{ summary.trend.toFixed(1) }}%
        </div>
        <div class="card-trend down" v-else>
          <el-icon><Bottom /></el-icon> {{ Math.abs(summary.trend).toFixed(1) }}%
        </div>
      </div>
      <div class="summary-card">
        <div class="card-title">{{ t('avgDaily') }}</div>
        <div class="card-value">¥{{ summary.avgDaily.toLocaleString() }}</div>
      </div>
      <div class="summary-card">
        <div class="card-title">{{ t('peakDay') }}</div>
        <div class="card-value">{{ summary.peakDay }}</div>
        <div class="card-sub">¥{{ summary.peakRevenue.toLocaleString() }}</div>
      </div>
      <div class="summary-card">
        <div class="card-title">{{ t('memberRevenue') }}</div>
        <div class="card-value">¥{{ summary.memberRevenue.toLocaleString() }}</div>
        <div class="card-sub">{{ percent(summary.memberRevenue, summary.total) }}%</div>
      </div>
    </div>

    <div class="charts-row">
      <div class="chart-card">
        <div class="chart-header">
          <h3>{{ t('revenueTrend') }}</h3>
        </div>
        <div class="chart-body">
          <div class="simple-chart">
            <div class="chart-y-axis">
              <span>{{ maxRevenue.toLocaleString() }}</span>
              <span>{{ (maxRevenue / 2).toLocaleString() }}</span>
              <span>0</span>
            </div>
            <div class="chart-bars">
              <div v-for="(item, idx) in trendData" :key="idx" class="bar-item">
                <div class="bar" :style="{ height: barHeight(item.revenue) + '%' }">
                  <span class="bar-tooltip">¥{{ item.revenue.toLocaleString() }}</span>
                </div>
                <span class="bar-label">{{ item.label }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="chart-card">
        <div class="chart-header">
          <h3>{{ t('revenueBreakdown') }}</h3>
        </div>
        <div class="chart-body">
          <div class="donut-chart">
            <div class="donut-center">
              <span class="donut-value">{{ summary.total.toLocaleString() }}</span>
              <span class="donut-label">{{ t('total') }}</span>
            </div>
            <div class="donut-legend">
              <div v-for="item in breakdown" :key="item.name" class="legend-item">
                <span class="legend-color" :style="{ background: item.color }"></span>
                <span class="legend-name">{{ t(item.name) }}</span>
                <span class="legend-value">¥{{ item.value.toLocaleString() }}</span>
                <span class="legend-percent">{{ item.percent }}%</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="detail-section">
      <div class="section-header">
        <h3>{{ t('dailyDetails') }}</h3>
        <el-button :icon="Download" size="small">{{ t('export') }}</el-button>
      </div>
      <el-table :data="dailyData" stripe>
        <el-table-column :label="t('date')" width="120">
          <template #default="{ row }">
            {{ formatDate(row.date) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('weekday')" width="80">
          <template #default="{ row }">
            {{ getWeekday(row.date) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('tableRevenue')">
          <template #default="{ row }">
            ¥{{ row.tableRevenue.toLocaleString() }}
          </template>
        </el-table-column>
        <el-table-column :label="t('productRevenue')">
          <template #default="{ row }">
            ¥{{ row.productRevenue.toLocaleString() }}
          </template>
        </el-table-column>
        <el-table-column :label="t('rechargeAmount')">
          <template #default="{ row }">
            ¥{{ row.recharge.toLocaleString() }}
          </template>
        </el-table-column>
        <el-table-column :label="t('totalRevenue')">
          <template #default="{ row }">
            <span class="total-cell">¥{{ row.total.toLocaleString() }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('orderCount')" width="80">
          <template #default="{ row }">
            {{ row.orders }}
          </template>
        </el-table-column>
        <el-table-column :label="t('avgOrder')" width="100">
          <template #default="{ row }">
            ¥{{ row.avgOrder }}
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { Top, Bottom, Download } from '@element-plus/icons-vue'
import { getRevenueTrend } from '../api'
import { t } from '../i18n'


const dateRange = ref('7')

const summary = ref({
  total: 28560,
  avgDaily: 4080,
  trend: 12.5,
  peakDay: '周六',
  peakRevenue: 6800,
  memberRevenue: 15200,
})

const trendData = ref([
  { label: '周一', revenue: 3200 },
  { label: '周二', revenue: 2800 },
  { label: '周三', revenue: 3100 },
  { label: '周四', revenue: 3500 },
  { label: '周五', revenue: 4800 },
  { label: '周六', revenue: 6800 },
  { label: '周日', revenue: 4360 },
])

const breakdown = ref([
  { name: 'tableRevenue', value: 18200, percent: 63.7, color: '#3b82f6' },
  { name: 'productRevenue', value: 5360, percent: 18.8, color: '#10b981' },
  { name: 'rechargeAmount', value: 5000, percent: 17.5, color: '#f59e0b' },
])

const dailyData = ref([
  { date: '2026-04-13', tableRevenue: 2800, productRevenue: 450, recharge: 500, total: 3750, orders: 18, avgOrder: 208 },
  { date: '2026-04-14', tableRevenue: 2400, productRevenue: 400, recharge: 200, total: 3000, orders: 15, avgOrder: 200 },
  { date: '2026-04-15', tableRevenue: 3100, productRevenue: 800, recharge: 1000, total: 4900, orders: 22, avgOrder: 223 },
  { date: '2026-04-16', tableRevenue: 2800, productRevenue: 700, recharge: 300, total: 3800, orders: 20, avgOrder: 190 },
  { date: '2026-04-17', tableRevenue: 3200, productRevenue: 600, recharge: 400, total: 4200, orders: 19, avgOrder: 221 },
  { date: '2026-04-18', tableRevenue: 4500, productRevenue: 1300, recharge: 1000, total: 6800, orders: 28, avgOrder: 243 },
  { date: '2026-04-19', tableRevenue: 3400, productRevenue: 960, recharge: 600, total: 4960, orders: 24, avgOrder: 207 },
])

const maxRevenue = computed(() => Math.max(...trendData.value.map(d => d.revenue)))

const percent = (part, total) => {
  if (!total) return 0
  return ((part / total) * 100).toFixed(1)
}

const barHeight = (revenue) => {
  if (!maxRevenue.value) return 0
  return (revenue / maxRevenue.value) * 100
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
}

const getWeekday = (d) => {
  if (!d) return '-'
  const days = ['周日', '周一', '周二', '周三', '周四', '周五', '周六']
  return days[new Date(d).getDay()]
}

const loadData = async () => {
  const days = parseInt(dateRange.value)
  await getRevenueTrend(days).catch(() => null)
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
.revenue-analysis-page {
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

.summary-cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.summary-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.card-title {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.card-value {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 4px;
}

.card-trend {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 13px;
}

.card-trend.up { color: var(--accent-success); }
.card-trend.down { color: var(--accent-danger); }

.card-sub {
  font-size: 12px;
  color: var(--text-secondary);
}

.charts-row {
  display: grid;
  grid-template-columns: 1.5fr 1fr;
  gap: 24px;
  margin-bottom: 24px;
}

.chart-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.chart-header {
  margin-bottom: 16px;
}

.chart-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.simple-chart {
  display: flex;
  height: 200px;
}

.chart-y-axis {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding-right: 8px;
  font-size: 11px;
  color: var(--text-secondary);
}

.chart-bars {
  flex: 1;
  display: flex;
  align-items: flex-end;
  gap: 8px;
}

.bar-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.bar {
  width: 100%;
  background: linear-gradient(to top, var(--primary-color), #6366f1);
  border-radius: 4px 4px 0 0;
  min-height: 4px;
  position: relative;
  transition: height 0.3s;
}

.bar:hover .bar-tooltip {
  display: block;
}

.bar-tooltip {
  display: none;
  position: absolute;
  top: -24px;
  left: 50%;
  transform: translateX(-50%);
  background: var(--text-primary);
  color: #fff;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  white-space: nowrap;
}

.bar-label {
  margin-top: 8px;
  font-size: 11px;
  color: var(--text-secondary);
}

.donut-chart {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.donut-center {
  text-align: center;
  margin-bottom: 24px;
}

.donut-value {
  display: block;
  font-size: 28px;
  font-weight: 600;
}

.donut-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.donut-legend {
  width: 100%;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-color);
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 3px;
}

.legend-name {
  flex: 1;
  font-size: 13px;
}

.legend-value {
  font-weight: 500;
}

.legend-percent {
  font-size: 12px;
  color: var(--text-secondary);
  width: 40px;
  text-align: right;
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

.total-cell {
  font-weight: 600;
  color: var(--accent-success);
}
</style>
