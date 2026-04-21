<template>
  <div class="page-wrapper">
    <div class="page-header">
      <div>
        <h1 class="page-title">{{ t('nav_front') }}</h1>
        <p class="page-subtitle">{{ t('sub_table_manage') }} - {{ t('quickActionsDesc') }}</p>
      </div>
      <div class="header-actions">
        <el-button type="primary" size="large">
          <el-icon><Plus /></el-icon>
          {{ t('openTable') }}
        </el-button>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">{{ t('totalTables') }}</div>
        <div class="stat-value">24</div>
        <div class="stat-meta">{{ t('billiardTable') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('activeTables') }}</div>
        <div class="stat-value">8</div>
        <div class="stat-meta">{{ t('inUseStatus') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('todayRevenue') }}</div>
        <div class="stat-value">¥2,480</div>
        <div class="stat-meta">{{ t('realTimeUpdate') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('avgTurnoverRate') }}</div>
        <div class="stat-value">3.2</div>
        <div class="stat-meta">{{ t('timesPerHour') }}</div>
      </div>
    </div>

    <div class="content-grid">
      <div class="section">
        <h2 class="section-title">{{ t('quickActions') }}</h2>
        <div class="action-cards">
          <div class="action-card">
            <el-icon class="card-icon"><Plus /></el-icon>
            <div class="card-title">{{ t('openTable') }}</div>
            <p class="card-desc">{{ t('openTableNew') }}</p>
          </div>
          <div class="action-card">
            <el-icon class="card-icon"><Refresh /></el-icon>
            <div class="card-title">{{ t('transferTable') }}</div>
            <p class="card-desc">{{ t('transferTableDesc') }}</p>
          </div>
          <div class="action-card">
            <el-icon class="card-icon"><Merge /></el-icon>
            <div class="card-title">{{ t('mergeTable') }}</div>
            <p class="card-desc">{{ t('mergeTableDesc') }}</p>
          </div>
          <div class="action-card">
            <el-icon class="card-icon"><Delete /></el-icon>
            <div class="card-title">{{ t('clearTable') }}</div>
            <p class="card-desc">{{ t('clearTableDesc') }}</p>
          </div>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">{{ t('realtimeTableStatus') }}</h2>
        <div class="table-grid">
          <div v-for="i in 8" :key="i" class="table-card" :class="{ active: i <= 3 }">
            <div class="table-number">{{ i }}</div>
            <div class="table-status">
              <span v-if="i <= 3" class="status-badge in-use">{{ t('inUse') }}</span>
              <span v-else class="status-badge free">{{ t('free') }}</span>
            </div>
            <div v-if="i <= 3" class="table-info">
              <div class="info-row">
                <span>{{ t('durationLabel') }}:</span>
                <span>{{ 45 + i * 5 }}{{ t('minuteUnit') }}</span>
              </div>
              <div class="info-row">
                <span>{{ t('consumptionLabel') }}:</span>
                <span>¥{{ 90 + i * 20 }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Plus, Refresh, Delete, Connection as Merge } from '@element-plus/icons-vue'
import { t } from '../i18n'
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

.action-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 12px;
}

.action-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-muted);
  border-radius: var(--radius-md);
  padding: 16px;
  text-align: center;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.action-card:hover {
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

.table-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 12px;
}

.table-card {
  background: var(--bg-primary);
  border: 2px solid var(--border-muted);
  border-radius: var(--radius-md);
  padding: 16px;
  text-align: center;
  transition: all var(--transition-fast);
}

.table-card.active {
  border-color: var(--accent-primary);
  background: rgba(31, 111, 235, 0.08);
}

.table-number {
  font-size: 24px;
  font-weight: 700;
  color: var(--accent-primary);
  margin-bottom: 8px;
}

.table-status {
  margin-bottom: 8px;
}

.status-badge {
  display: inline-block;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
}

.status-badge.free {
  background: rgba(76, 175, 80, 0.15);
  color: #4caf50;
}

.status-badge.in-use {
  background: rgba(255, 152, 0, 0.15);
  color: #ff9800;
}

.table-info {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 8px;
  text-align: left;
}

.info-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 4px;
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    gap: 16px;
  }

  .action-cards {
    grid-template-columns: repeat(2, 1fr);
  }

  .table-grid {
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  }
}
</style>
