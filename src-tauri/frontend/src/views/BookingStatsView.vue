<template>
  <div class="booking-stats-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('bookingStats') }}</h1>
      <div class="header-actions">
        <el-date-picker v-model="dateRange" type="daterange" range-separator="~" value-format="YYYY-MM-DD" @change="loadData" />
        <el-button :icon="Download" @click="exportStats">{{ t('export') }}</el-button>
      </div>
    </div>

    <div class="summary-cards">
      <div class="summary-card">
        <span class="card-label">{{ t('totalBookings') }}</span>
        <span class="card-value">{{ stats.total }}</span>
      </div>
      <div class="summary-card">
        <span class="card-label">{{ t('confirmedBookings') }}</span>
        <span class="card-value">{{ stats.confirmed }}</span>
        <span class="card-rate">{{ confirmedRate }}%</span>
      </div>
      <div class="summary-card">
        <span class="card-label">{{ t('cancelledBookings') }}</span>
        <span class="card-value danger">{{ stats.cancelled }}</span>
        <span class="card-rate">{{ cancelledRate }}%</span>
      </div>
      <div class="summary-card">
        <span class="card-label">{{ t('noShow') }}</span>
        <span class="card-value danger">{{ stats.noShow }}</span>
      </div>
      <div class="summary-card">
        <span class="card-label">{{ t('totalDeposit') }}</span>
        <span class="card-value">¥{{ stats.deposit.toLocaleString() }}</span>
      </div>
    </div>

    <div class="charts-row">
      <div class="chart-card">
        <h3>{{ t('bookingTrend') }}</h3>
        <div class="bar-chart">
          <div v-for="(item, idx) in trendData" :key="idx" class="bar-group">
            <div class="bars">
              <div class="bar confirmed" :style="{ height: barHeight(item.confirmed) + '%' }" :title="`确认: ${item.confirmed}`"></div>
              <div class="bar cancelled" :style="{ height: barHeight(item.cancelled) + '%' }" :title="`取消: ${item.cancelled}`"></div>
            </div>
            <span class="bar-label">{{ item.label }}</span>
          </div>
        </div>
        <div class="chart-legend">
          <span class="legend-item"><span class="dot confirmed"></span>{{ t('confirmed') }}</span>
          <span class="legend-item"><span class="dot cancelled"></span>{{ t('cancelled') }}</span>
        </div>
      </div>

      <div class="chart-card">
        <h3>{{ t('peakHours') }}</h3>
        <div class="hourly-chart">
          <div v-for="(item, idx) in hourlyData" :key="idx" class="hour-item">
            <div class="hour-bar">
              <div class="bar" :style="{ height: barHeight(item.count) + '%' }"></div>
            </div>
            <span class="hour-label">{{ item.hour }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="detail-section">
      <div class="section-header">
        <h3>{{ t('dailyDetails') }}</h3>
      </div>
      <el-table :data="dailyDetails" stripe>
        <el-table-column :label="t('date')" width="120">
          <template #default="{ row }">{{ row.date }}</template>
        </el-table-column>
        <el-table-column :label="t('totalBookings')" width="100">
          <template #default="{ row }">{{ row.total }}</template>
        </el-table-column>
        <el-table-column :label="t('confirmed')" width="100">
          <template #default="{ row }">{{ row.confirmed }}</template>
        </el-table-column>
        <el-table-column :label="t('cancelled')" width="100">
          <template #default="{ row }">{{ row.cancelled }}</template>
        </el-table-column>
        <el-table-column :label="t('noShow')" width="80">
          <template #default="{ row }">{{ row.noShow }}</template>
        </el-table-column>
        <el-table-column :label="t('showRate')" width="100">
          <template #default="{ row }">
            <el-progress :percentage="row.showRate" :color="getProgressColor(row.showRate)" />
          </template>
        </el-table-column>
        <el-table-column :label="t('deposit')">
          <template #default="{ row }">¥{{ row.deposit.toLocaleString() }}</template>
        </el-table-column>
        <el-table-column :label="t('avgDeposit')">
          <template #default="{ row }">¥{{ row.avgDeposit }}</template>
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


const dateRange = ref([])

const stats = ref({ total: 286, confirmed: 218, cancelled: 42, noShow: 26, deposit: 32800 })

const confirmedRate = computed(() => ((stats.value.confirmed / stats.value.total) * 100).toFixed(1))
const cancelledRate = computed(() => ((stats.value.cancelled / stats.value.total) * 100).toFixed(1))

const trendData = ref([
  { label: '周一', confirmed: 25, cancelled: 5 },
  { label: '周二', confirmed: 22, cancelled: 4 },
  { label: '周三', confirmed: 28, cancelled: 6 },
  { label: '周四', confirmed: 30, cancelled: 5 },
  { label: '周五', confirmed: 38, cancelled: 8 },
  { label: '周六', confirmed: 48, cancelled: 9 },
  { label: '周日', confirmed: 42, cancelled: 7 },
])

const hourlyData = ref([
  { hour: '10', count: 8 }, { hour: '11', count: 15 },
  { hour: '12', count: 22 }, { hour: '13', count: 18 },
  { hour: '14', count: 25 }, { hour: '15', count: 20 },
  { hour: '16', count: 22 }, { hour: '17', count: 30 },
  { hour: '18', count: 45 }, { hour: '19', count: 50 },
  { hour: '20', count: 42 }, { hour: '21', count: 28 },
])

const dailyDetails = ref([
  { date: '2026-04-13', total: 38, confirmed: 30, cancelled: 5, noShow: 3, showRate: 79, deposit: 4200, avgDeposit: 140 },
  { date: '2026-04-14', total: 35, confirmed: 28, cancelled: 5, noShow: 2, showRate: 80, deposit: 3800, avgDeposit: 136 },
  { date: '2026-04-15', total: 42, confirmed: 33, cancelled: 6, noShow: 3, showRate: 79, deposit: 4600, avgDeposit: 139 },
  { date: '2026-04-16', total: 40, confirmed: 32, cancelled: 5, noShow: 3, showRate: 80, deposit: 4400, avgDeposit: 138 },
  { date: '2026-04-17', total: 48, confirmed: 38, cancelled: 7, noShow: 3, showRate: 79, deposit: 5200, avgDeposit: 137 },
  { date: '2026-04-18', total: 52, confirmed: 42, cancelled: 7, noShow: 3, showRate: 81, deposit: 5800, avgDeposit: 138 },
  { date: '2026-04-19', total: 45, confirmed: 36, cancelled: 6, noShow: 3, showRate: 80, deposit: 5000, avgDeposit: 139 },
])

const maxBar = computed(() => Math.max(...trendData.value.flatMap(d => [d.confirmed, d.cancelled]), ...hourlyData.value.map(h => h.count)))
const barHeight = (val) => maxBar.value ? (val / maxBar.value) * 100 : 0

const getProgressColor = (rate) => {
  if (rate >= 80) return '#10b981'
  if (rate >= 60) return '#f59e0b'
  return '#ef4444'
}

const loadData = () => {}
const exportStats = () => ElMessage.success(t('exportSuccess'))
</script>

<style scoped>
.booking-stats-page { padding: 24px; }

.page-title { font-size: 24px; font-weight: 600; margin: 0 0 24px 0; }

.header-actions { display: flex; gap: 12px; }

.summary-cards {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.summary-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  text-align: center;
}

.card-label { display: block; font-size: 13px; color: var(--text-secondary); margin-bottom: 8px; }
.card-value { display: block; font-size: 28px; font-weight: 600; }
.card-value.danger { color: var(--accent-danger); }
.card-rate { display: block; font-size: 12px; color: var(--text-secondary); margin-top: 4px; }

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

.chart-card h3 { margin: 0 0 16px 0; font-size: 16px; font-weight: 600; }

.bar-chart {
  display: flex;
  align-items: flex-end;
  gap: 8px;
  height: 120px;
  padding-top: 20px;
}

.bar-group {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.bars {
  display: flex;
  gap: 2px;
  align-items: flex-end;
  height: 80px;
  width: 100%;
}

.bar {
  flex: 1;
  border-radius: 2px;
  min-height: 4px;
}

.bar.confirmed { background: var(--accent-success); }
.bar.cancelled { background: var(--accent-danger); }

.chart-card .bar { background: linear-gradient(to top, var(--primary-color), #6366f1); }

.bar-label { font-size: 11px; color: var(--text-secondary); margin-top: 4px; }

.chart-legend {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-top: 12px;
}

.legend-item { display: flex; align-items: center; gap: 6px; font-size: 12px; }

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.dot.confirmed { background: var(--accent-success); }
.dot.cancelled { background: var(--accent-danger); }

.hourly-chart {
  display: flex;
  align-items: flex-end;
  gap: 4px;
  height: 120px;
  padding-top: 20px;
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

.hour-label { font-size: 10px; color: var(--text-secondary); margin-top: 4px; }

.detail-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.section-header { margin-bottom: 16px; }
.section-header h3 { margin: 0; font-size: 16px; font-weight: 600; }
</style>
