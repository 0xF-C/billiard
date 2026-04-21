<template>
  <div class="page-wrapper">
    <div class="page-header">
      <div>
        <h1 class="page-title">{{ t('nav_finance') }}</h1>
        <p class="page-subtitle">{{ t('sub_daily_report') }} - 营收分析、桌台效率、支付方式、成本核算</p>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">{{ t('todayRevenue') }}</div>
        <div class="stat-value">¥8,960</div>
        <div class="stat-meta">较昨日 +12%</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('monthRevenue') }}</div>
        <div class="stat-value">¥186,400</div>
        <div class="stat-meta">目标进度 78%</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('avgTableFee') }}</div>
        <div class="stat-value">¥186</div>
        <div class="stat-meta">{{ t('avgConsumption') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">商品销售</div>
        <div class="stat-value">¥2,340</div>
        <div class="stat-meta">占比 26%</div>
      </div>
    </div>

    <div class="content-grid">
      <div class="section">
        <h2 class="section-title">快速导航</h2>
        <div class="nav-cards">
          <router-link to="/finance-daily" class="nav-card">
            <el-icon class="card-icon"><DocumentCopy /></el-icon>
            <div class="card-title">{{ t('sub_daily_report') }}</div>
            <p class="card-desc">日报表、班次对比</p>
          </router-link>
          <router-link to="/revenue-analysis" class="nav-card">
            <el-icon class="card-icon"><DataAnalysis /></el-icon>
            <div class="card-title">{{ t('sub_revenue_analysis') }}</div>
            <p class="card-desc">台费、商品、充值</p>
          </router-link>
          <router-link to="/table-efficiency" class="nav-card">
            <el-icon class="card-icon"><Histogram /></el-icon>
            <div class="card-title">{{ t('sub_table_efficiency') }}</div>
            <p class="card-desc">翻台率、平均时长</p>
          </router-link>
          <router-link to="/payment-methods" class="nav-card">
            <el-icon class="card-icon"><CreditCard /></el-icon>
            <div class="card-title">{{ t('sub_payment_methods') }}</div>
            <p class="card-desc">支付渠道占比</p>
          </router-link>
          <router-link to="/cost-accounting" class="nav-card">
            <el-icon class="card-icon"><Money /></el-icon>
            <div class="card-title">{{ t('sub_cost_accounting') }}</div>
            <p class="card-desc">水电、人工、物料</p>
          </router-link>
          <router-link to="/export-reports" class="nav-card">
            <el-icon class="card-icon"><Download /></el-icon>
            <div class="card-title">{{ t('sub_export_reports') }}</div>
            <p class="card-desc">Excel/PDF导出</p>
          </router-link>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">支付方式分布</h2>
        <div class="payment-distribution">
          <div v-for="payment in paymentMethods" :key="payment.id" class="payment-item">
            <div class="payment-header">
              <span class="payment-name">{{ payment.name }}</span>
              <span class="payment-amount">¥{{ payment.amount }}</span>
            </div>
            <div class="payment-bar">
              <div class="payment-progress" :style="{ width: payment.percentage + '%', backgroundColor: payment.color }"></div>
            </div>
            <div class="payment-footer">
              <span class="payment-percentage">{{ payment.percentage }}%</span>
              <span class="payment-count">{{ payment.count }}{{ t('count') }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">最近7天营收趋势</h2>
        <div class="chart-placeholder">
          <p>营收趋势图表</p>
          <p style="font-size: 12px; color: var(--text-tertiary);">显示每日营收变化</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { DocumentCopy, DataAnalysis, Histogram, CreditCard, Money, Download } from '@element-plus/icons-vue'
import { t } from '../i18n'

const paymentMethods = ref([
  { id: 1, name: '现金', amount: 3200, percentage: 36, color: '#4caf50', count: 45 },
  { id: 2, name: '微信支付', amount: 2800, percentage: 31, color: '#1f6feb', count: 52 },
  { id: 3, name: '支付宝', amount: 1600, percentage: 18, color: '#1890ff', count: 28 },
  { id: 4, name: '会员卡', amount: 960, percentage: 11, color: '#ffc107', count: 18 },
  { id: 5, name: '积分抵扣', amount: 400, percentage: 4, color: '#9c27b0', count: 8 }
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

.payment-distribution {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.payment-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.payment-header {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.payment-name {
  color: var(--text-secondary);
  font-weight: 500;
}

.payment-amount {
  color: var(--accent-primary);
  font-weight: 600;
}

.payment-bar {
  height: 12px;
  background: var(--bg-primary);
  border-radius: 6px;
  overflow: hidden;
}

.payment-progress {
  height: 100%;
  border-radius: 6px;
  transition: width 0.3s ease;
}

.payment-footer {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-tertiary);
}

.chart-placeholder {
  height: 200px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  border: 1px dashed var(--border-muted);
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
