<template>
  <div class="promotions-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('promotions') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('createPromotion') }}</el-button>
    </div>

    <div class="active-promotions">
      <h3>{{ t('activePromotions') }}</h3>
      <div class="promo-grid">
        <div v-for="p in activePromos" :key="p.id" class="promo-card active">
          <div class="promo-badge running">{{ t('running') }}</div>
          <div class="promo-header">
            <span class="promo-icon"><el-icon><Gift /></el-icon></span>
            <span class="promo-name">{{ p.name }}</span>
          </div>
          <div class="promo-desc">{{ p.description }}</div>
          <div class="promo-period">
            <el-icon><Calendar /></el-icon>
            {{ formatDate(p.startDate) }} ~ {{ formatDate(p.endDate) }}
          </div>
          <div class="promo-stats">
            <div class="stat">
              <span class="stat-value">{{ p.participants }}</span>
              <span class="stat-label">{{ t('participants') }}</span>
            </div>
            <div class="stat">
              <span class="stat-value">¥{{ p.revenue }}</span>
              <span class="stat-label">{{ t('revenue') }}</span>
            </div>
          </div>
          <div class="promo-actions">
            <el-button size="small" @click="viewDetail(p)">{{ t('detail') }}</el-button>
            <el-button size="small" type="danger" @click="endPromotion(p)">{{ t('endEarly') }}</el-button>
          </div>
        </div>
      </div>
    </div>

    <div class="all-promotions">
      <h3>{{ t('allPromotions') }}</h3>
      <el-table :data="allPromos" stripe>
        <el-table-column :label="t('name')" min-width="160">
          <template #default="{ row }">
            <div class="promo-name-cell">
              <span class="promo-icon"><el-icon><Gift /></el-icon></span>
              {{ row.name }}
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('type')" width="100">
          <template #default="{ row }">
            <el-tag size="small">{{ t(row.type) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('period')" width="180">
          <template #default="{ row }">
            {{ formatDate(row.startDate) }} ~ {{ formatDate(row.endDate) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('status')" width="80">
          <template #default="{ row }">
            <el-tag :type="getStatusTag(row.status)" size="small">{{ t(row.status) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('participants')" width="90">
          <template #default="{ row }">{{ row.participants }}</template>
        </el-table-column>
        <el-table-column :label="t('revenue')" width="100">
          <template #default="{ row }">¥{{ row.revenue.toLocaleString() }}</template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="120" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="editPromo(row)">{{ t('edit') }}</el-button>
            <el-button link type="danger" @click="deletePromo(row)">{{ t('delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="showAdd" :title="t('createPromotion')" width="550px">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="t('promotionName')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="t('type')">
          <el-select v-model="form.type">
            <el-option :label="t('discount')" value="discount" />
            <el-option :label="t('gift')" value="gift" />
            <el-option :label="t('bundle')" value="bundle" />
            <el-option :label="t('hourly')" value="hourly" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('description')">
          <el-input v-model="form.description" type="textarea" :rows="2" />
        </el-form-item>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item :label="t('startDate')">
              <el-date-picker v-model="form.startDate" type="date" value-format="YYYY-MM-DD" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('endDate')">
              <el-date-picker v-model="form.endDate" type="date" value-format="YYYY-MM-DD" />
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
      <template #footer>
        <el-button @click="showAdd = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submit">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Present as Gift, Calendar } from '@element-plus/icons-vue'
import { t } from '../i18n'


const showAdd = ref(false)
const form = ref({ name: '', type: '', description: '', startDate: '', endDate: '' })

const activePromos = ref([
  { id: 1, name: t('weekend50Off'), type: 'discount', description: t('weekend50OffDesc'), startDate: '2026-04-01', endDate: '2026-04-30', status: 'running', participants: 286, revenue: 42800 },
  { id: 2, name: t('recharge300Get100'), type: 'gift', description: t('recharge300Get100Desc'), startDate: '2026-04-10', endDate: '2026-05-10', status: 'running', participants: 52, revenue: 15600 },
])

const allPromos = ref([
  { id: 1, name: t('weekend50Off'), type: 'discount', description: '周六周日全场5折', startDate: '2026-04-01', endDate: '2026-04-30', status: 'running', participants: 286, revenue: 42800 },
  { id: 2, name: t('recharge300Get100'), type: 'gift', description: t('recharge300Get100Desc'), startDate: '2026-04-10', endDate: '2026-05-10', status: 'running', participants: 52, revenue: 15600 },
  { id: 3, name: t('newStore80Off'), type: 'discount', description: t('newStore80OffDesc'), startDate: '2026-02-01', endDate: '2026-02-28', status: 'ended', participants: 420, revenue: 52000 },
  { id: 4, name: t('nightPackageSpecial'), type: 'bundle', description: t('nightPackageSpecialDesc'), startDate: '2026-03-01', endDate: '2026-03-31', status: 'ended', participants: 186, revenue: 16368 },
])

const getStatusTag = (status) => {
  const types = { running: 'success', scheduled: 'primary', ended: 'info' }
  return types[status] || 'info'
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString(currentLang.value === 'zh' ? 'zh-CN' : (currentLang.value === 'en' ? 'en-US' : 'zh-CN'), { month: 'short', day: 'numeric' })
}

const submit = () => {
  if (!form.value.name) { ElMessage.warning(t('pleaseComplete')); return }
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
}

const viewDetail = (p) => ElMessage.info(`${p.name}: ${p.participants}人参与，营收¥${p.revenue.toLocaleString()}`)
const editPromo = (p) => ElMessage.info(`${t('edit')}: ${p.name}`)
const endPromotion = async (p) => {
  await ElMessageBox.confirm(t('confirmEndEarly'), t('confirm'), { type: 'warning' })
  p.status = 'ended'
  ElMessage.success(t('success'))
}
const deletePromo = async (p) => {
  await ElMessageBox.confirm(t('confirmDelete'), t('confirm'), { type: 'warning' })
  const idx = allPromos.value.findIndex(x => x.id === p.id)
  if (idx !== -1) { allPromos.value.splice(idx, 1); ElMessage.success(t('deleteSuccess')) }
}
</script>

<style scoped>
.promotions-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }

.active-promotions { margin-bottom: 24px; }
.active-promotions h3, .all-promotions h3 { margin: 0 0 16px 0; font-size: 16px; font-weight: 600; }

.promo-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.promo-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  position: relative;
}

.promo-card.active {
  border-color: var(--accent-success);
}

.promo-badge {
  position: absolute;
  top: 12px;
  right: 12px;
  padding: 2px 8px;
  border-radius: 10px;
  font-size: 11px;
  font-weight: 500;
}

.promo-badge.running {
  background: rgba(63, 185, 80, 0.1);
  color: var(--accent-success);
}

.promo-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.promo-icon {
  color: var(--accent-primary);
}

.promo-name { font-weight: 600; font-size: 16px; }
.promo-desc { font-size: 13px; color: var(--text-secondary); margin-bottom: 12px; }
.promo-period { font-size: 12px; color: var(--text-secondary); margin-bottom: 12px; display: flex; align-items: center; gap: 4px; }

.promo-stats {
  display: flex;
  gap: 24px;
  padding: 12px;
  background: var(--hover-bg);
  border-radius: 8px;
  margin-bottom: 12px;
}

.stat { text-align: center; }
.stat-value { display: block; font-size: 18px; font-weight: 600; }
.stat-label { font-size: 11px; color: var(--text-secondary); }

.promo-actions { display: flex; gap: 8px; }

.all-promotions {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.promo-name-cell { display: flex; align-items: center; gap: 8px; font-weight: 500; }
</style>
