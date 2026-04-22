<template>
  <div class="page-wrapper">
    <div class="page-header">
      <div>
        <h1 class="page-title">{{ t('memberAnalysis') }}</h1>
        <p class="page-subtitle">会员数据总览、等级分布与消费画像</p>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">{{ t('totalMembers') }}</div>
        <div class="stat-value">{{ stats.total }}</div>
        <div class="stat-meta">系统累计会员总数</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('newThisMonth') }}</div>
        <div class="stat-value" style="color: var(--accent-success);">+{{ stats.newThisMonth }}</div>
        <div class="stat-meta">本月新增注册</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('totalBalance') }}</div>
        <div class="stat-value">¥{{ stats.totalBalance.toLocaleString() }}</div>
        <div class="stat-meta">会员账户沉淀资金</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('totalConsumption') }}</div>
        <div class="stat-value">¥{{ stats.totalConsumption.toLocaleString() }}</div>
        <div class="stat-meta">会员累计消费总额</div>
      </div>
    </div>

    <div class="charts-grid">
      <div class="section chart-card">
        <div class="section-title">{{ t('memberLevelDistribution') }}</div>
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

      <div class="section chart-card">
        <div class="chart-header">
          <div class="section-title">{{ t('newMembersTrend') }}</div>
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

      <div class="section chart-card">
        <div class="section-title">{{ t('consumptionDistribution') }}</div>
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

    <div class="section detail-section">
      <div class="section-header">
        <h2 class="section-title" style="margin: 0;">{{ t('memberList') }}</h2>
        <div class="header-actions">
          <el-input
            v-model="searchKw"
            :placeholder="t('searchMember')"
            clearable
            @keyup.enter="loadMembers"
            style="width: 200px;"
          >
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
          <el-select v-model="filterLevel" :placeholder="t('level')" clearable style="width: 140px;">
            <el-option :label="t('all')" value="" />
            <el-option v-for="l in levelList" :key="l" :label="t(l)" :value="l" />
          </el-select>
        </div>
      </div>
      
      <el-table :data="filteredMembers" stripe max-height="500" style="width: 100%">
        <el-table-column :label="t('member')" min-width="180">
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
        <el-table-column :label="t('level')" width="120">
          <template #default="{ row }">
            <el-tag :type="getLevelTagType(row.level)" size="small" effect="light">{{ row.level }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('balance')" width="120">
          <template #default="{ row }">
            <span class="balance-text">¥{{ row.balance?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('totalConsumption')" width="140">
          <template #default="{ row }">
            <span class="consumption-text">¥{{ row.totalSpent?.toFixed(2) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('lastVisit')" width="140">
          <template #default="{ row }">
            <span class="date-text">{{ formatDate(row.lastVisit) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('visitCount')" width="100">
          <template #default="{ row }">
            {{ row.visitCount }}{{ t('times') }}
          </template>
        </el-table-column>
        <el-table-column :label="t('avgConsumption')" width="120">
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
import { Search } from '@element-plus/icons-vue'
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
  { level: '银卡会员', count: 95, percent: 29.0, color: '#a1a1aa' },
  { level: '金卡会员', count: 60, percent: 18.3, color: '#fbbf24' },
  { level: '白金会员', count: 38, percent: 11.6, color: '#60a5fa' },
  { level: '钻石会员', count: 15, percent: 4.5, color: '#a78bfa' },
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
  { range: '500-1000', count: 95, percent: 29.0, color: '#60a5fa' },
  { range: '1000-3000', count: 80, percent: 24.4, color: '#34d399' },
  { range: '3000-5000', count: 45, percent: 13.7, color: '#fbbf24' },
  { range: '>5000', count: 28, percent: 8.5, color: '#a78bfa' },
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
  const colors = { '普通会员': '#909399', '银卡会员': '#a1a1aa', '金卡会员': '#fbbf24', '白金会员': '#60a5fa', '钻石会员': '#a78bfa' }
  return colors[level] || 'var(--accent-primary)'
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
/* 容器规范：复用全局布局 */
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

/* 核心数据卡片：采用你最常用的 stat-card 风格 */
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
  font-size: 13px;
  color: var(--text-tertiary);
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
}

.stat-meta {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 统一区块风格：使用全局的 .section 类名逻辑 */
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

/* 图表区网格 */
.charts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 24px;
}

.chart-card {
  display: flex;
  flex-direction: column;
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.chart-header .section-title {
  margin: 0;
}

/* 环形图设计融入暗黑/浅色规范 */
.donut-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex: 1;
}

.donut {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: conic-gradient(#909399 0% 36.6%, #a1a1aa 36.6% 65.6%, #fbbf24 65.6% 83.9%, #60a5fa 83.9% 95.5%, #a78bfa 95.5% 100%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.donut-center {
  width: 84px;
  height: 84px;
  border-radius: 50%;
  background: var(--bg-secondary); /* 动态适应卡片底色 */
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.donut-total { font-size: 20px; font-weight: 700; color: var(--text-primary); }
.donut-label { font-size: 12px; color: var(--text-tertiary); }

.donut-legend { 
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.legend-item {
  display: flex;
  align-items: center;
  font-size: 13px;
  color: var(--text-secondary);
}

.legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-right: 8px;
}

.legend-name { width: 64px; }
.legend-count { font-weight: 600; color: var(--text-primary); width: 32px; text-align: right; margin-right: 8px; }
.legend-percent { font-size: 12px; width: 44px; text-align: right; }

/* 柱状图规范 */
.trend-chart {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  height: 130px;
  flex: 1;
  padding-top: 10px;
}

.bar-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.bar-wrap {
  width: 100%;
  height: 90px;
  display: flex;
  align-items: flex-end;
  justify-content: center;
  margin-bottom: 8px;
}

.bar {
  width: 14px;
  background: var(--accent-primary); /* 使用全局主色 */
  border-radius: 4px 4px 0 0;
  min-height: 4px;
  transition: opacity 0.2s;
}

.bar-item:hover .bar {
  opacity: 0.8;
}

.bar-label { font-size: 12px; color: var(--text-tertiary); }
.bar-count { font-size: 12px; font-weight: 600; color: var(--text-primary); margin-top: 4px; }

/* 横向进度条规范 */
.bar-horizontal {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  flex: 1;
  gap: 14px;
}

.range-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.range-label {
  width: 70px;
  font-size: 12px;
  color: var(--text-secondary);
}

.range-bar {
  flex: 1;
  height: 8px;
  background: var(--bg-primary); /* 使用更深的底托色 */
  border-radius: 4px;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  border-radius: 4px;
}

.range-count {
  width: 60px;
  text-align: right;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

/* 底部列表区 */
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.member-cell {
  display: flex;
  align-items: center;
  gap: 12px;
}

.member-info { display: flex; flex-direction: column; }
.member-name { font-weight: 600; color: var(--text-primary); font-size: 14px; margin-bottom: 2px;}
.member-phone { font-size: 12px; color: var(--text-tertiary); }

.balance-text { color: var(--accent-success); font-weight: 600; }
.consumption-text { color: var(--text-primary); font-weight: 600; }
.date-text { color: var(--text-secondary); font-size: 13px; }

/* 深度覆写 Element Table 样式适配你的暗黑系统 */
:deep(.el-table) {
  background: transparent !important;
  --el-table-border-color: var(--border-muted);
}
:deep(.el-table th.el-table__cell) {
  background: var(--bg-tertiary) !important;
  color: var(--text-secondary) !important;
  font-weight: 600;
}
:deep(.el-table tr) {
  background: transparent !important;
}
:deep(.el-table td.el-table__cell) {
  border-bottom: 1px solid var(--border-muted) !important;
}
:deep(.el-table--striped .el-table__body tr.el-table__row--striped td.el-table__cell) {
  background: var(--bg-primary) !important; /* 斑马纹适应你的底层色 */
}
</style>