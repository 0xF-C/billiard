<template>
  <div class="member-analysis-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('memberAnalysis') }}</h1>
    </div>

    <div class="overview-cards">
      <div class="overview-card">
        <div class="card-icon blue">
          <el-icon><User /></el-icon>
        </div>
        <div class="card-body">
          <span class="card-value">{{ stats.total }}</span>
          <span class="card-label">{{ t('totalMembers') }}</span>
        </div>
      </div>
      <div class="overview-card">
        <div class="card-icon green">
          <el-icon><TrendCharts /></el-icon>
        </div>
        <div class="card-body">
          <span class="card-value">{{ stats.newThisMonth }}</span>
          <span class="card-label">{{ t('newThisMonth') }}</span>
        </div>
      </div>
      <div class="overview-card">
        <div class="card-icon purple">
          <el-icon><Coin /></el-icon>
        </div>
        <div class="card-body">
          <span class="card-value">¥{{ stats.totalBalance }}</span>
          <span class="card-label">{{ t('totalBalance') }}</span>
        </div>
      </div>
      <div class="overview-card">
        <div class="card-icon orange">
          <el-icon><Money /></el-icon>
        </div>
        <div class="card-body">
          <span class="card-value">¥{{ stats.totalConsumption }}</span>
          <span class="card-label">{{ t('totalConsumption') }}</span>
        </div>
      </div>
    </div>

    <div class="charts-grid">
      <div class="chart-card">
        <div class="card-header">
          <h3>{{ t('memberLevelDistribution') }}</h3>
        </div>
        <div class="donut-wrapper">
          <div class="donut">
            <div class="donut-center">
              <span class="donut-total">{{ stats.total }}</span>
              <span class="donut-label">{{ t('members') }}</span>
            </div>
          </div>
          <div class="donut-legend">
            <div v-for="item in levelDist" :key="item.level" class="legend-item">
              <span class="legend-dot" :style="{ background: item.color }"></span>
              <span class="legend-name">{{ t(item.level) }}</span>
              <span class="legend-count">{{ item.count }}</span>
              <span class="legend-percent">{{ item.percent }}%</span>
            </div>
          </div>
        </div>
      </div>

      <div class="chart-card">
        <div class="card-header">
          <h3>{{ t('newMembersTrend') }}</h3>
          <el-radio-group v-model="trendPeriod" size="small">
            <el-radio-button value="week">{{ t('week') }}</el-radio-button>
            <el-radio-button value="month">{{ t('month') }}</el-radio-button>
          </el-radio-group>
        </div>
        <div class="trend-chart">
          <div v-for="(item, idx) in trendData" :key="idx" class="bar-item">
            <div class="bar-wrap">
              <div class="bar" :style="{ height: barHeight(item.count) + '%' }"></div>
            </div>
            <span class="bar-label">{{ item.label }}</span>
            <span class="bar-count">{{ item.count }}</span>
          </div>
        </div>
      </div>

      <div class="chart-card">
        <div class="card-header">
          <h3>{{ t('consumptionDistribution') }}</h3>
        </div>
        <div class="bar-horizontal">
          <div v-for="item in consumptionDist" :key="item.range" class="range-row">
            <span class="range-label">{{ item.range }}</span>
            <div class="range-bar">
              <div class="bar-fill" :style="{ width: item.percent + '%', background: item.color }"></div>
            </div>
            <span class="range-count">{{ item.count }}{{ t('people') }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="detail-section">
      <div class="section-header">
        <h3>{{ t('memberList') }}</h3>
        <div class="header-actions">
          <el-input
            v-model="searchKw"
            :placeholder="t('searchMember')"
            clearable
            @keyup.enter="loadMembers"
          >
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
          <el-select v-model="filterLevel" :placeholder="t('level')" clearable>
            <el-option :label="t('all')" value="" />
            <el-option v-for="l in levelList" :key="l" :label="t(l)" :value="l" />
          </el-select>
        </div>
      </div>
      <el-table :data="filteredMembers" stripe max-height="400">
        <el-table-column :label="t('member')" min-width="160">
          <template #default="{ row }">
            <div class="member-cell">
              <el-avatar :size="36" :style="{ background: getLevelColor(row.level) }">
                {{ row.name?.charAt(0) }}
              </el-avatar>
              <div class="member-info">
                <span class="member-name">{{ row.name }}</span>
                <span class="member-phone">{{ row.phone }}</span>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('level')" width="100">
          <template #default="{ row }">
            <el-tag :type="getLevelTagType(row.level)" size="small">{{ row.level }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('balance')" width="100">
          <template #default="{ row }">
            <span class="balance">¥{{ row.balance?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('totalConsumption')" width="120">
          <template #default="{ row }">
            <span>¥{{ row.totalSpent?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('lastVisit')" width="120">
          <template #default="{ row }">
            {{ formatDate(row.lastVisit) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('visitCount')" width="80">
          <template #default="{ row }">
            {{ row.visitCount }}
          </template>
        </el-table-column>
        <el-table-column :label="t('avgConsumption')" width="100">
          <template #default="{ row }">
            ¥{{ row.avgConsumption?.toFixed(0) }}
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { User, TrendCharts, Coin, Money, Search } from '@element-plus/icons-vue'
import { getMembers } from '../api'
import { t } from '../i18n'


const searchKw = ref('')
const filterLevel = ref('')
const trendPeriod = ref('week')

const levelList = ['普通会员', '银卡会员', '金卡会员', '白金会员', '钻石会员']

const stats = ref({
  total: 328,
  newThisMonth: 42,
  totalBalance: 186520,
  totalConsumption: 2894600,
})

const levelDist = ref([
  { level: '普通会员', count: 120, percent: 36.6, color: '#909399' },
  { level: '银卡会员', count: 95, percent: 29.0, color: '#c0c0c0' },
  { level: '金卡会员', count: 60, percent: 18.3, color: '#e6a23c' },
  { level: '白金会员', count: 38, percent: 11.6, color: '#409eff' },
  { level: '钻石会员', count: 15, percent: 4.5, color: '#a855f7' },
])

const trendData = ref([
  { label: '周一', count: 5 },
  { label: '周二', count: 8 },
  { label: '周三', count: 6 },
  { label: '周四', count: 9 },
  { label: '周五', count: 12 },
  { label: '周六', count: 18 },
  { label: '周日', count: 15 },
])

const consumptionDist = ref([
  { range: '0-500', count: 80, percent: 24.4, color: '#909399' },
  { range: '500-1000', count: 95, percent: 29.0, color: '#409eff' },
  { range: '1000-3000', count: 80, percent: 24.4, color: '#10b981' },
  { range: '3000-5000', count: 45, percent: 13.7, color: '#f59e0b' },
  { range: '>5000', count: 28, percent: 8.5, color: '#a855f7' },
])

const members = ref([])

const filteredMembers = computed(() => {
  let list = members.value
  if (searchKw.value) {
    list = list.filter(m => m.name?.includes(searchKw.value) || m.phone?.includes(searchKw.value))
  }
  if (filterLevel.value) {
    list = list.filter(m => m.level === filterLevel.value)
  }
  return list
})

const maxTrend = computed(() => Math.max(...trendData.value.map(d => d.count)))
const barHeight = (count) => maxTrend.value ? (count / maxTrend.value) * 100 : 0

const getLevelColor = (level) => {
  const colors = { '普通会员': '#909399', '银卡会员': '#c0c0c0', '金卡会员': '#e6a23c', '白金会员': '#409eff', '钻石会员': '#a855f7' }
  return colors[level] || '#909399'
}

const getLevelTagType = (level) => {
  const types = { '普通会员': 'info', '银卡会员': 'info', '金卡会员': 'warning', '白金会员': 'primary', '钻石会员': 'danger' }
  return types[level] || 'info'
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
}

const loadMembers = async () => {
  const res = await getMembers(searchKw.value)
  members.value = (res || []).map(m => ({
    ...m,
    avgConsumption: m.visitCount ? (m.totalSpent / m.visitCount) : 0,
  }))
}

onMounted(() => {
  loadMembers()
})
</script>

<style scoped>
.member-analysis-page { padding: 24px; }

.page-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0 0 24px 0;
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

.card-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}

.card-icon.blue { background: rgba(59, 130, 246, 0.1); color: #3b82f6; }
.card-icon.green { background: rgba(16, 185, 129, 0.1); color: #10b981; }
.card-icon.purple { background: rgba(139, 92, 246, 0.1); color: #a855f7; }
.card-icon.orange { background: rgba(245, 158, 11, 0.1); color: #f59e0b; }

.card-value { display: block; font-size: 24px; font-weight: 600; }
.card-label { font-size: 13px; color: var(--text-secondary); }

.charts-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 24px;
  margin-bottom: 24px;
}

.chart-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.card-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.donut-wrapper {
  display: flex;
  align-items: center;
  gap: 24px;
}

.donut {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: conic-gradient(#909399 0% 36.6%, #c0c0c0 36.6% 65.6%, #e6a23c 65.6% 83.9%, #409eff 83.9% 95.5%, #a855f7 95.5% 100%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.donut-center {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: var(--card-bg);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.donut-total { font-size: 20px; font-weight: 600; }
.donut-label { font-size: 11px; color: var(--text-secondary); }

.donut-legend { flex: 1; }

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
  font-size: 13px;
}

.legend-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.legend-name { flex: 1; }
.legend-count { font-weight: 500; }
.legend-percent { color: var(--text-secondary); font-size: 12px; width: 40px; text-align: right; }

.trend-chart {
  display: flex;
  align-items: flex-end;
  gap: 8px;
  height: 120px;
  padding-top: 20px;
}

.bar-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.bar-wrap {
  width: 100%;
  height: 80px;
  display: flex;
  align-items: flex-end;
}

.bar {
  width: 100%;
  background: linear-gradient(to top, var(--primary-color), #6366f1);
  border-radius: 4px 4px 0 0;
  min-height: 4px;
}

.bar-label { font-size: 11px; color: var(--text-secondary); margin-top: 4px; }
.bar-count { font-size: 11px; color: var(--text-secondary); }

.bar-horizontal {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.range-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.range-label {
  width: 60px;
  font-size: 12px;
  color: var(--text-secondary);
}

.range-bar {
  flex: 1;
  height: 10px;
  background: var(--border-color);
  border-radius: 5px;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  border-radius: 5px;
}

.range-count {
  width: 60px;
  text-align: right;
  font-size: 13px;
  font-weight: 500;
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

.header-actions {
  display: flex;
  gap: 12px;
}

.member-cell {
  display: flex;
  align-items: center;
  gap: 10px;
}

.member-info { display: flex; flex-direction: column; }
.member-name { font-weight: 500; }
.member-phone { font-size: 12px; color: var(--text-secondary); }

.balance { color: var(--accent-success); font-weight: 500; }
</style>
