<template>
  <div class="page-wrapper">
    <div class="page-header">
      <div>
        <h1 class="page-title">{{ t('nav_marketing') }}</h1>
        <p class="page-subtitle">{{ t('sub_coupons') }} - 促销、赛事、积分商城、短信营销</p>
      </div>
      <div class="header-actions">
        <el-button type="primary" size="large">
          <el-icon><Plus /></el-icon>
          新建活动
        </el-button>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">活跃优惠券</div>
        <div class="stat-value">8</div>
        <div class="stat-meta">张</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">本月促销收入</div>
        <div class="stat-value">¥12,400</div>
        <div class="stat-meta">新增收入</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">优惠券核销率</div>
        <div class="stat-value">64%</div>
        <div class="stat-meta">本月数据</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">会员参与度</div>
        <div class="stat-value">78%</div>
        <div class="stat-meta">活动参与</div>
      </div>
    </div>

    <div class="content-grid">
      <div class="section">
        <h2 class="section-title">快速导航</h2>
        <div class="nav-cards">
          <router-link to="/coupons" class="nav-card">
            <el-icon class="card-icon"><Ticket /></el-icon>
            <div class="card-title">{{ t('sub_coupons') }}</div>
            <p class="card-desc">满减、折扣、体验券</p>
          </router-link>
          <router-link to="/promotions" class="nav-card">
            <el-icon class="card-icon"><Promotion /></el-icon>
            <div class="card-title">{{ t('sub_promotions') }}</div>
            <p class="card-desc">时段特价、节假日</p>
          </router-link>
          <router-link to="/tournaments" class="nav-card">
            <el-icon class="card-icon"><Trophy /></el-icon>
            <div class="card-title">{{ t('sub_tournaments') }}</div>
            <p class="card-desc">比赛报名、赛程</p>
          </router-link>
          <router-link to="/points-mall" class="nav-card">
            <el-icon class="card-icon"><ShoppingCart /></el-icon>
            <div class="card-title">{{ t('sub_points_mall') }}</div>
            <p class="card-desc">积分兑换商品</p>
          </router-link>
          <router-link to="/sms-marketing" class="nav-card">
            <el-icon class="card-icon"><Message /></el-icon>
            <div class="card-title">{{ t('sub_sms_marketing') }}</div>
            <p class="card-desc">短信营销推送</p>
          </router-link>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">当前活动</h2>
        <div class="activity-list">
          <div v-for="activity in activeActivities" :key="activity.id" class="activity-card">
            <div class="activity-header">
              <div>
                <div class="activity-title">{{ activity.title }}</div>
                <div class="activity-meta">{{ activity.type }} · {{ activity.startDate }}至{{ activity.endDate }}</div>
              </div>
              <el-tag :type="activity.status === '进行中' ? 'success' : 'info'">
                {{ activity.status }}
              </el-tag>
            </div>
            <div class="activity-stats">
              <div class="stat">
                <span class="label">参与人数</span>
                <span class="value">{{ activity.participants }}</span>
              </div>
              <div class="stat">
                <span class="label">核销金额</span>
                <span class="value">¥{{ activity.amount }}</span>
              </div>
              <div class="stat">
                <span class="label">核销率</span>
                <span class="value">{{ activity.rate }}%</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">营销效果对比</h2>
        <div class="chart-placeholder">
          <p>营销活动效果对比图表</p>
          <p style="font-size: 12px; color: var(--text-tertiary);">显示各活动的参与度和转化率</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { Plus, Ticket, Promotion, Trophy, ShoppingCart, Message } from '@element-plus/icons-vue'
import { t } from '../i18n'

const activeActivities = ref([
  {
    id: 1,
    title: '周末特价活动',
    type: '时段特价',
    startDate: '2026-04-18',
    endDate: '2026-04-20',
    status: '进行中',
    participants: 156,
    amount: 3200,
    rate: 72
  },
  {
    id: 2,
    title: '会员充值送礼',
    type: '充值优惠',
    startDate: '2026-04-15',
    endDate: '2026-04-30',
    status: '进行中',
    participants: 234,
    amount: 8600,
    rate: 68
  },
  {
    id: 3,
    title: '新会员体验券',
    type: '体验券',
    startDate: '2026-04-01',
    endDate: '2026-04-30',
    status: '进行中',
    participants: 89,
    amount: 2400,
    rate: 58
  }
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

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.activity-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-muted);
  border-radius: var(--radius-md);
  padding: 16px;
}

.activity-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
}

.activity-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.activity-meta {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-top: 4px;
}

.activity-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.stat {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat .label {
  font-size: 12px;
  color: var(--text-tertiary);
}

.stat .value {
  font-size: 16px;
  font-weight: 600;
  color: var(--accent-primary);
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

  .activity-stats {
    grid-template-columns: 1fr;
  }
}
</style>
