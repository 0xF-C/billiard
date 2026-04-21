<template>
  <div class="page-wrapper">
    <div class="page-header">
      <div>
        <h1 class="page-title">{{ t('nav_inventory') }}</h1>
        <p class="page-subtitle">{{ t('sub_goods_manage') }} - 商品、入库出库、预警、供应商、损耗</p>
      </div>
      <div class="header-actions">
        <el-button type="primary" size="large">
          <el-icon><Plus /></el-icon>
          {{ t('add') }}{{ t('product') }}
        </el-button>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">商品总数</div>
        <div class="stat-value">156</div>
        <div class="stat-meta">种商品</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">库存总值</div>
        <div class="stat-value">¥24,800</div>
        <div class="stat-meta">成本价</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">低库存预警</div>
        <div class="stat-value">8</div>
        <div class="stat-meta">需补货</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">本月销售</div>
        <div class="stat-value">¥18,600</div>
        <div class="stat-meta">销售额</div>
      </div>
    </div>

    <div class="content-grid">
      <div class="section">
        <h2 class="section-title">快速导航</h2>
        <div class="nav-cards">
          <router-link to="/inventory-manage" class="nav-card">
            <el-icon class="card-icon"><Box /></el-icon>
            <div class="card-title">{{ t('sub_goods_manage') }}</div>
            <p class="card-desc">商品信息管理</p>
          </router-link>
          <router-link to="/category-manage" class="nav-card">
            <el-icon class="card-icon"><Menu /></el-icon>
            <div class="card-title">{{ t('sub_category_manage') }}</div>
            <p class="card-desc">商品分类管理</p>
          </router-link>
          <router-link to="/stock-io" class="nav-card">
            <el-icon class="card-icon"><Download /></el-icon>
            <div class="card-title">{{ t('sub_stock_io') }}</div>
            <p class="card-desc">采购入库、销售出库</p>
          </router-link>
          <router-link to="/stock-alert" class="nav-card">
            <el-icon class="card-icon"><Warning /></el-icon>
            <div class="card-title">{{ t('sub_stock_alert') }}</div>
            <p class="card-desc">低库存和过期提醒</p>
          </router-link>
          <router-link to="/suppliers" class="nav-card">
            <el-icon class="card-icon"><User /></el-icon>
            <div class="card-title">{{ t('sub_suppliers') }}</div>
            <p class="card-desc">供应商档案管理</p>
          </router-link>
          <router-link to="/loss-records" class="nav-card">
            <el-icon class="card-icon"><Delete /></el-icon>
            <div class="card-title">{{ t('sub_loss_records') }}</div>
            <p class="card-desc">破损、过期、赠送</p>
          </router-link>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">库存预警</h2>
        <el-table :data="stockAlerts" stripe style="width: 100%">
<el-table-column prop="productName" :label="t('productName')" width="150" />
          <el-table-column prop="category" :label="t('categoryName')" width="100" />
          <el-table-column prop="currentStock" :label="t('currentStock')" width="100" />
          <el-table-column prop="minStock" :label="t('minStock')" width="100" />
          <el-table-column prop="status" :label="t('status')" width="100">
          <template #default="{ row }">
            <el-tag :type="row.currentStock <= row.minStock ? 'danger' : 'success'" size="small">
              {{ row.currentStock <= row.minStock ? t('lowStock') : t('normal') }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="120">
            <template #default>
              <el-button link type="primary" size="small">{{ t('replenishment') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <div class="section">
        <h2 class="section-title">商品分类统计</h2>
        <div class="category-stats">
          <div v-for="cat in categoryStats" :key="cat.id" class="category-item">
            <div class="category-header">
              <span class="category-name">{{ cat.name }}</span>
              <span class="category-count">{{ cat.count }}件</span>
            </div>
            <div class="category-bar">
              <div class="category-progress" :style="{ width: cat.percentage + '%', backgroundColor: cat.color }"></div>
            </div>
            <div class="category-footer">
              <span class="category-value">¥{{ cat.value }}</span>
              <span class="category-percentage">{{ cat.percentage }}%</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { Plus, Box, Download, Warning, User, Delete, Menu } from '@element-plus/icons-vue'
import { t } from '../i18n'

const stockAlerts = ref([
  { productName: '瓶装啤酒', category: '酒水', currentStock: 5, minStock: 20, status: '紧急' },
  { productName: '可乐', category: '饮料', currentStock: 12, minStock: 30, status: '预警' },
  { productName: '花生', category: '小食', currentStock: 8, minStock: 25, status: '紧急' },
  { productName: '台球粉', category: '配件', currentStock: 3, minStock: 10, status: '紧急' },
  { productName: '球杆', category: '配件', currentStock: 15, minStock: 20, status: '预警' }
])

const categoryStats = ref([
  { id: 1, name: '酒水饮料', count: 45, value: 8600, percentage: 35, color: '#4caf50' },
  { id: 2, name: '小食零食', count: 32, value: 5200, percentage: 21, color: '#2196f3' },
  { id: 3, name: '台球配件', count: 56, value: 8400, percentage: 34, color: '#ffc107' },
  { id: 4, name: '其他', count: 23, value: 2600, percentage: 10, color: '#9c27b0' }
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

.category-stats {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.category-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.category-header {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.category-name {
  color: var(--text-secondary);
  font-weight: 500;
}

.category-count {
  color: var(--text-tertiary);
}

.category-bar {
  height: 12px;
  background: var(--bg-primary);
  border-radius: 6px;
  overflow: hidden;
}

.category-progress {
  height: 100%;
  border-radius: 6px;
  transition: width 0.3s ease;
}

.category-footer {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
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
