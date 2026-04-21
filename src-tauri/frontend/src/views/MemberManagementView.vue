<template>
  <div class="page-wrapper">
    <div class="page-header">
      <div>
        <h1 class="page-title">{{ t('nav_member') }}</h1>
        <p class="page-subtitle">{{ t('sub_member_files') }} - 会员档案、卡种、充值、积分、分析</p>
      </div>
      <div class="header-actions">
        <el-button type="primary" size="large">
          <el-icon><Plus /></el-icon>
          {{ t('addMember') }}
        </el-button>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">{{ t('activeMembers') }}</div>
        <div class="stat-value">1,234</div>
        <div class="stat-meta">{{ t('activeMembers') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('monthRecharge') }}</div>
        <div class="stat-value">¥45,600</div>
        <div class="stat-meta">{{ t('totalRecharge') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('average') }}{{ t('consumption') }}</div>
        <div class="stat-value">¥156</div>
        <div class="stat-meta">{{ t('avgConsumption') }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">{{ t('churnWarning') }}</div>
        <div class="stat-value">23</div>
        <div class="stat-meta">{{ t('noConsumption30Days') }}</div>
      </div>
    </div>

    <div class="content-grid">
      <div class="section">
        <h2 class="section-title">{{ t('quickNav') }}</h2>
        <div class="nav-cards">
          <router-link to="/members" class="nav-card">
            <el-icon class="card-icon"><User /></el-icon>
            <div class="card-title">{{ t('sub_member_files') }}</div>
            <p class="card-desc">{{ t('memberFilesDesc') }}</p>
          </router-link>
          <router-link to="/member-levels" class="nav-card">
            <el-icon class="card-icon"><Trophy /></el-icon>
            <div class="card-title">{{ t('sub_member_cards') }}</div>
            <p class="card-desc">{{ t('memberCardsDesc') }}</p>
          </router-link>
          <router-link to="/recharge" class="nav-card">
            <el-icon class="card-icon"><Wallet /></el-icon>
            <div class="card-title">{{ t('sub_recharge_center') }}</div>
            <p class="card-desc">{{ t('rechargeCenterDesc') }}</p>
          </router-link>
          <router-link to="/points" class="nav-card">
            <el-icon class="card-icon"><Star /></el-icon>
            <div class="card-title">{{ t('sub_points_system') }}</div>
            <p class="card-desc">{{ t('pointsSystemDesc') }}</p>
          </router-link>
          <router-link to="/member-analysis" class="nav-card">
            <el-icon class="card-icon"><DataAnalysis /></el-icon>
            <div class="card-title">{{ t('sub_member_analysis') }}</div>
            <p class="card-desc">{{ t('memberAnalysisDesc') }}</p>
          </router-link>
          <router-link to="/blacklist" class="nav-card">
            <el-icon class="card-icon"><Delete /></el-icon>
            <div class="card-title">{{ t('sub_blacklist') }}</div>
            <p class="card-desc">{{ t('blacklistDesc') }}</p>
          </router-link>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">{{ t('memberLevelDistribution') }}</h2>
        <div class="level-distribution">
          <div v-for="level in memberLevels" :key="level.id" class="level-item">
            <div class="level-header">
              <span class="level-name">{{ level.name }}</span>
              <span class="level-count">{{ level.count }}人</span>
            </div>
            <div class="level-bar">
              <div class="level-progress" :style="{ width: level.percentage + '%', backgroundColor: level.color }"></div>
            </div>
          </div>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">{{ t('recentRecharges') }}</h2>
        <el-table :data="recentRecharges" stripe style="width: 100%">
          <el-table-column prop="memberName" label="会员名称" width="150" />
          <el-table-column prop="amount" label=t('rechargeAmount') width="120">
            <template #default="{ row }">
              <span style="color: var(--accent-primary)">¥{{ row.amount }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="bonus" label=t('bonusAmount') width="120" />
          <el-table-column prop="time" label=t('rechargeTime') width="180" />
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
import { Plus, User, Trophy, Wallet, Star, DataAnalysis, Delete } from '@element-plus/icons-vue'
import { t } from '../i18n'

const memberLevels = [
  { id: 1, name: t('normalMember'), count: 850, percentage: 69, color: '#4caf50' },
  { id: 2, name: t('silverMember'), count: 250, percentage: 20, color: '#2196f3' },
  { id: 3, name: t('goldMember'), count: 100, percentage: 8, color: '#ffc107' },
  { id: 4, name: t('diamondMember'), count: 34, percentage: 3, color: '#e91e63' }
]

const recentRecharges = ref([
  { memberName: '张三', amount: 500, bonus: 50, time: '2026-04-18 14:30' },
  { memberName: '李四', amount: 1000, bonus: 150, time: '2026-04-18 13:15' },
  { memberName: '王五', amount: 200, bonus: 20, time: '2026-04-18 12:00' },
  { memberName: '赵六', amount: 2000, bonus: 400, time: '2026-04-18 11:45' },
  { memberName: '孙七', amount: 300, bonus: 30, time: '2026-04-18 10:20' }
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
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
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

.level-distribution {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.level-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.level-header {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.level-name {
  color: var(--text-secondary);
  font-weight: 500;
}

.level-count {
  color: var(--text-tertiary);
}

.level-bar {
  height: 8px;
  background: var(--bg-primary);
  border-radius: 4px;
  overflow: hidden;
}

.level-progress {
  height: 100%;
  border-radius: 4px;
  transition: width 0.3s ease;
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
