<template>
  <div class="page-wrapper">
    <div class="page-header">
      <div>
        <h1 class="page-title">{{ t('nav_booking') }}</h1>
        <p class="page-subtitle">{{ t('sub_online_booking') }} - 预订看板、审核、提醒、统计</p>
      </div>
      <div class="header-actions">
        <el-button type="primary" size="large">
          <el-icon><Plus /></el-icon>
          新建预订
        </el-button>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">{{ t('todayBookings') }}</div>
        <div class="stat-value">12</div>
        <div class="stat-meta">{{ t('countBooking') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('pending') }}</div>
        <div class="stat-value">3</div>
        <div class="stat-meta">{{ t('pending') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">预订转化率</div>
        <div class="stat-value">87%</div>
        <div class="stat-meta">本周平均</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">爽约率</div>
        <div class="stat-value">8%</div>
        <div class="stat-meta">本周数据</div>
      </div>
    </div>

    <div class="content-grid">
      <div class="section">
        <h2 class="section-title">快速导航</h2>
        <div class="nav-cards">
          <router-link to="/booking-online" class="nav-card">
            <el-icon class="card-icon"><Plus /></el-icon>
            <div class="card-title">{{ t('sub_online_booking') }}</div>
            <p class="card-desc">微信/APP预订入口</p>
          </router-link>
          <router-link to="/booking-kanban" class="nav-card">
            <el-icon class="card-icon"><DocumentCopy /></el-icon>
            <div class="card-title">{{ t('sub_booking_kanban') }}</div>
            <p class="card-desc">日历视图预订情况</p>
          </router-link>
          <router-link to="/booking-audit" class="nav-card">
            <el-icon class="card-icon"><Check /></el-icon>
            <div class="card-title">{{ t('sub_booking_audit') }}</div>
            <p class="card-desc">自动/人工确认</p>
          </router-link>
          <router-link to="/booking-remind" class="nav-card">
            <el-icon class="card-icon"><Bell /></el-icon>
            <div class="card-title">{{ t('sub_booking_remind') }}</div>
            <p class="card-desc">短信/微信提醒</p>
          </router-link>
          <router-link to="/booking-stats" class="nav-card">
            <el-icon class="card-icon"><DataAnalysis /></el-icon>
            <div class="card-title">{{ t('sub_booking_stats') }}</div>
            <p class="card-desc">预订数据分析</p>
          </router-link>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">{{ t('pendingAuditBookings') }}</h2>
        <el-table :data="pendingBookings" stripe style="width: 100%">
          <el-table-column prop="bookingNo" :label="t('orderNo')" width="120" />
          <el-table-column prop="memberName" :label="t('name')" width="120" />
          <el-table-column prop="tableType" :label="t('tableType')" width="100" />
          <el-table-column prop="bookingTime" :label="t('bookingTime')" width="180" />
          <el-table-column prop="duration" :label="t('duration')" width="80" />
          <el-table-column prop="status" :label="t('status')" width="100">
            <template #default="{ row }">
              <el-tag :type="row.status === 'pending' ? 'warning' : 'success'">
                {{ row.status === 'pending' ? t('pending') : t('approved') }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column :label="t('operation')" width="150">
            <template #default>
              <el-button link type="success" size="small">{{ t('approve') }}</el-button>
              <el-button link type="danger" size="small">{{ t('reject') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <div class="section">
        <h2 class="section-title">本周预订趋势</h2>
        <div class="chart-placeholder">
          <p>预订量统计图表</p>
          <p style="font-size: 12px; color: var(--text-tertiary);">周一到周日的预订数量对比</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { Plus, DocumentCopy, Check, Bell, DataAnalysis } from '@element-plus/icons-vue'
import { t } from '../i18n'

const pendingBookings = ref([
  { bookingNo: 'BK20260418001', memberName: '张三', tableType: '斯诺克', bookingTime: '2026-04-18 18:00', duration: '2小时', status: 'pending' },
  { bookingNo: 'BK20260418002', memberName: '李四', tableType: '九球', bookingTime: '2026-04-18 19:30', duration: '1.5小时', status: 'pending' },
  { bookingNo: 'BK20260418003', memberName: '王五', tableType: '中式八球', bookingTime: '2026-04-18 20:00', duration: '3小时', status: 'pending' }
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
