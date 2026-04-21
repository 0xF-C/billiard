<template>
  <div class="points-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('pointsSystem') }}</h1>
      <div class="header-actions">
        <el-button type="primary" :icon="Setting" @click="showSettings = true">
          {{ t('pointsRules') }}
        </el-button>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon purple">
          <el-icon><Star /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('totalPointsIssued') }}</span>
          <span class="stat-value">{{ stats.totalIssued.toLocaleString() }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon green">
          <el-icon><Goods /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('pointsRedeemed') }}</span>
          <span class="stat-value">{{ stats.totalRedeemed.toLocaleString() }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon blue">
          <el-icon><User /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('membersWithPoints') }}</span>
          <span class="stat-value">{{ stats.membersCount }}</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon orange">
          <el-icon><TrendCharts /></el-icon>
        </div>
        <div class="stat-body">
          <span class="stat-label">{{ t('avgPointsPerMember') }}</span>
          <span class="stat-value">{{ stats.avgPoints }}</span>
        </div>
      </div>
    </div>

    <div class="main-content">
      <!-- 左侧：会员积分列表 -->
      <div class="member-points-panel">
        <div class="panel-header">
          <h3>{{ t('memberPoints') }}</h3>
          <el-input
            v-model="searchKw"
            :placeholder="t('searchMember')"
            class="search-input"
            clearable
            @keyup.enter="loadMembers"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
        </div>

        <el-table :data="members" stripe>
          <el-table-column :label="t('member')" min-width="140">
            <template #default="{ row }">
              <div class="member-cell">
                <el-avatar :size="32" :style="{ background: getLevelColor(row.level) }">
                  {{ row.name?.charAt(0) }}
                </el-avatar>
                <span class="member-name">{{ row.name }}</span>
              </div>
            </template>
          </el-table-column>
          <el-table-column prop="level" :label="t('level')" width="100">
            <template #default="{ row }">
              <el-tag size="small" :type="getLevelType(row.level)">{{ row.level }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column :label="t('points')" width="120">
            <template #default="{ row }">
              <span class="points-value">{{ row.points || 0 }}</span>
            </template>
          </el-table-column>
          <el-table-column :label="t('actions')" width="140" fixed="right">
            <template #default="{ row }">
              <el-button link type="primary" @click="adjustPoints(row, 'add')">
                <el-icon><Plus /></el-icon>
              </el-button>
              <el-button link type="warning" @click="adjustPoints(row, 'deduct')">
                <el-icon><Minus /></el-icon>
              </el-button>
              <el-button link type="primary" @click="showHistory(row)">
                {{ t('history') }}
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <!-- 右侧：积分商城 -->
      <div class="points-mall-panel">
        <div class="panel-header">
          <h3>{{ t('pointsMall') }}</h3>
          <el-button type="primary" size="small" :icon="Plus" @click="showAddGift = true">
            {{ t('addGift') }}
          </el-button>
        </div>

        <div class="gifts-grid">
          <div v-for="gift in gifts" :key="gift.id" class="gift-card">
            <div class="gift-image">
              <el-icon :size="48"><Present /></el-icon>
            </div>
            <div class="gift-info">
              <div class="gift-name">{{ gift.name }}</div>
              <div class="gift-points">
                <el-icon><Star /></el-icon>
                {{ gift.points }} {{ t('points') }}
              </div>
              <div class="gift-stock">{{ t('stock') }}: {{ gift.stock }}</div>
            </div>
            <el-button
              type="primary"
              size="small"
              :disabled="gift.stock <= 0"
              @click="redeemGift(gift)"
            >
              {{ t('redeem') }}
            </el-button>
          </div>
        </div>
      </div>
    </div>

    <!-- 积分调整弹窗 -->
    <el-dialog v-model="showAdjust" :title="adjustType === 'add' ? t('addPoints') : t('deductPoints')" width="400px">
      <el-form :model="adjustForm" label-width="80px">
        <el-form-item :label="t('member')">
          <el-input :value="adjustMember?.name" disabled />
        </el-form-item>
        <el-form-item :label="t('currentPoints')">
          <span>{{ adjustMember?.points || 0 }}</span>
        </el-form-item>
        <el-form-item :label="t('amount')">
          <el-input-number v-model="adjustForm.amount" :min="1" :max="10000" />
        </el-form-item>
        <el-form-item :label="t('reason')">
          <el-input v-model="adjustForm.reason" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAdjust = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitAdjust">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- 积分规则设置弹窗 -->
    <el-dialog v-model="showSettings" :title="t('pointsRules')" width="500px">
      <el-form :model="rulesForm" label-width="120px">
        <el-form-item :label="t('pointsPerYuan')">
          <el-input-number v-model="rulesForm.pointsPerYuan" :min="0" :max="100" />
          <span class="form-tip">{{ t('pointsPerYuanTip') }}</span>
        </el-form-item>
        <el-form-item :label="t('memberDayBonus')">
          <el-input-number v-model="rulesForm.memberDayBonus" :min="1" :max="10" :step="0.1" :precision="1" />
          <span class="form-tip">{{ t('memberDayBonusTip') }}</span>
        </el-form-item>
        <el-form-item :label="t('pointsExpiry')">
          <el-input-number v-model="rulesForm.expiryMonths" :min="0" :max="36" />
          <span class="form-tip">{{ t('pointsExpiryTip') }}</span>
        </el-form-item>
        <el-form-item :label="t('minRedeemPoints')">
          <el-input-number v-model="rulesForm.minRedeem" :min="0" :max="10000" :step="100" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showSettings = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="saveRules">{{ t('save') }}</el-button>
      </template>
    </el-dialog>

    <!-- 积分历史弹窗 -->
    <el-dialog v-model="showHistoryDialog" :title="t('pointsHistory')" width="600px">
      <el-table :data="pointsHistory" stripe max-height="400">
        <el-table-column prop="created_at" :label="t('time')" width="160">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('type')" width="100">
          <template #default="{ row }">
            <el-tag :type="row.type === 'earn' ? 'success' : 'warning'" size="small">
              {{ row.type === 'earn' ? t('earn') : t('redeem') }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('points')" width="100">
          <template #default="{ row }">
            <span :class="row.type === 'earn' ? 'points-earn' : 'points-spend'">
              {{ row.type === 'earn' ? '+' : '-' }}{{ row.points }}
            </span>
          </template>
        </el-table-column>
        <el-table-column prop="description" :label="t('description')" />
      </el-table>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Setting, Star, Goods, User, TrendCharts, Search, Plus, Minus, Present } from '@element-plus/icons-vue'
import { getMembers } from '../api'
import { t } from '../i18n'


const searchKw = ref('')
const members = ref([])
const gifts = ref([
  { id: 1, name: t('drinkCoupon'), points: 100, stock: 50 },
  { id: 2, name: t('hourCoupon'), points: 300, stock: 20 },
  { id: 3, name: t('50YuanCoupon'), points: 500, stock: 10 },
  { id: 4, name: t('premiumCue'), points: 5000, stock: 2 },
])

const stats = reactive({
  totalIssued: 125680,
  totalRedeemed: 45230,
  membersCount: 328,
  avgPoints: 245,
})

const showAdjust = ref(false)
const adjustType = ref('add')
const adjustMember = ref(null)
const adjustForm = reactive({
  amount: 10,
  reason: '',
})

const showSettings = ref(false)
const rulesForm = reactive({
  pointsPerYuan: 1,
  memberDayBonus: 1.5,
  expiryMonths: 12,
  minRedeem: 100,
})

const showHistoryDialog = ref(false)
const pointsHistory = ref([])

const showAddGift = ref(false)

const getLevelColor = (level) => {
  const colors = {
    t('normalMember'): '#909399',
    t('silverMember'): '#909399',
    t('goldMember'): '#e6a23c',
    '白金会员': '#409eff',
    t('diamondMember'): '#a855f7',
  }
  return colors[level] || '#909399'
}

const getLevelType = (level) => {
  const types = {
    t('normalMember'): 'info',
    t('silverMember'): 'info',
    t('goldMember'): 'warning',
    '白金会员': 'primary',
    t('diamondMember'): 'danger',
  }
  return types[level] || 'info'
}

const formatTime = (ts) => {
  if (!ts) return '-'
  return new Date(ts).toLocaleString(currentLang.value === 'zh' ? 'zh-CN' : (currentLang.value === 'en' ? 'en-US' : 'zh-CN'))
}

const loadMembers = async () => {
  const res = await getMembers(searchKw.value)
  members.value = (res || []).map(m => ({ ...m, points: Math.floor(Math.random() * 1000) }))
}

const adjustPoints = (member, type) => {
  adjustMember.value = member
  adjustType.value = type
  adjustForm.amount = 10
  adjustForm.reason = ''
  showAdjust.value = true
}

const submitAdjust = () => {
  if (!adjustForm.amount || adjustForm.amount <= 0) {
    ElMessage.warning(t('inputValidAmount'))
    return
  }

  const idx = members.value.findIndex(m => m.id === adjustMember.value.id)
  if (idx !== -1) {
    if (adjustType.value === 'add') {
      members.value[idx].points += adjustForm.amount
    } else {
      members.value[idx].points = Math.max(0, members.value[idx].points - adjustForm.amount)
    }
    ElMessage.success(t('success'))
    showAdjust.value = false
  }
}

const showHistory = (member) => {
  pointsHistory.value = [
    { created_at: new Date(Date.now() - 86400000 * 5), type: 'earn', points: 50, description: t('consumptionGift') },
    { created_at: new Date(Date.now() - 86400000 * 3), type: 'redeem', points: 100, description: t('redeemDrinkCoupon') },
    { created_at: new Date(Date.now() - 86400000), type: 'earn', points: 200, description: t('rechargeGift') },
  ]
  showHistoryDialog.value = true
}

const redeemGift = (gift) => {
  ElMessage.success(`${t('redeemSuccess')}: ${gift.name}`)
  gift.stock--
}

const saveRules = () => {
  ElMessage.success(t('saveSuccess'))
  showSettings.value = false
}

onMounted(() => {
  loadMembers()
})
</script>

<style scoped>
.points-page {
  padding: 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  border: 1px solid var(--border-color);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}

.stat-icon.purple { background: rgba(163, 113, 247, 0.1); color: #a855f7; }
.stat-icon.green { background: rgba(63, 185, 80, 0.1); color: var(--accent-success); }
.stat-icon.blue { background: rgba(64, 158, 255, 0.1); color: var(--accent-primary); }
.stat-icon.orange { background: rgba(229, 165, 48, 0.1); color: var(--accent-warning); }

.stat-body {
  display: flex;
  flex-direction: column;
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.stat-value {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
}

.main-content {
  display: grid;
  grid-template-columns: 1fr 400px;
  gap: 24px;
}

.member-points-panel,
.points-mall-panel {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.member-cell {
  display: flex;
  align-items: center;
  gap: 10px;
}

.member-name {
  font-weight: 500;
}

.points-value {
  color: var(--accent-primary);
  font-weight: 600;
}

.gifts-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.gift-card {
  background: var(--hover-bg);
  border-radius: 10px;
  padding: 16px;
  text-align: center;
  border: 1px solid var(--border-color);
}

.gift-image {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--accent-primary);
  margin-bottom: 12px;
}

.gift-name {
  font-weight: 500;
  margin-bottom: 8px;
}

.gift-points {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  color: var(--accent-warning);
  font-weight: 500;
  margin-bottom: 4px;
}

.gift-stock {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.form-tip {
  margin-left: 12px;
  font-size: 12px;
  color: var(--text-secondary);
}

.points-earn {
  color: var(--accent-success);
  font-weight: 500;
}

.points-spend {
  color: var(--accent-warning);
  font-weight: 500;
}
</style>
