<template>
  <div class="points-mall-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('pointsMall') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('addGift') }}</el-button>
    </div>

    <div class="stats-row">
      <div class="stat-card">
        <span class="stat-value">{{ gifts.length }}</span>
        <span class="stat-label">{{ t('totalGifts') }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{{ totalStock }}</span>
        <span class="stat-label">{{ t('totalStock') }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-value">{{ redeemedCount }}</span>
        <span class="stat-label">{{ t('totalRedeemed') }}</span>
      </div>
    </div>

    <div class="gifts-grid">
      <div v-for="g in gifts" :key="g.id" class="gift-card">
        <div class="gift-image">
          <el-icon :size="48" color="var(--accent-primary)"><Gift /></el-icon>
        </div>
        <div class="gift-body">
          <div class="gift-name">{{ g.name }}</div>
          <div class="gift-desc">{{ g.description }}</div>
          <div class="gift-points">
            <el-icon><Star /></el-icon>
            {{ g.points }} {{ t('points') }}
          </div>
          <div class="gift-stock">
            {{ t('stock') }}: {{ g.stock }}
            <span v-if="g.stock <= 5" class="low-stock">{{ t('lowStock') }}</span>
          </div>
        </div>
        <div class="gift-actions">
          <el-button type="primary" size="small" :disabled="g.stock <= 0" @click="redeem(g)">
            {{ t('redeem') }}
          </el-button>
          <el-button size="small" @click="editGift(g)">{{ t('edit') }}</el-button>
        </div>
      </div>
    </div>

    <div class="redeem-history">
      <div class="section-header">
        <h3>{{ t('redeemHistory') }}</h3>
      </div>
      <el-table :data="history" stripe max-height="300">
        <el-table-column :label="t('time')" width="160">
          <template #default="{ row }">{{ formatTime(row.time) }}</template>
        </el-table-column>
        <el-table-column :label="t('member')" min-width="120">
          <template #default="{ row }">{{ row.memberName }}</template>
        </el-table-column>
        <el-table-column :label="t('gift')" min-width="120">
          <template #default="{ row }">{{ row.giftName }}</template>
        </el-table-column>
        <el-table-column :label="t('points')" width="100">
          <template #default="{ row }">
            <span class="points-deduct">-{{ row.points }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('status')" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 'claimed' ? 'success' : 'warning'" size="small">
              {{ t(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="100">
          <template #default="{ row }">
            <el-button link type="success" size="small" @click="claim(row)" v-if="row.status === 'pending'">
              {{ t('claim') }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="showAdd" :title="t('addGift')" width="450px">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="t('giftName')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="t('description')">
          <el-input v-model="form.description" type="textarea" :rows="2" />
        </el-form-item>
        <el-form-item :label="t('points')">
          <el-input-number v-model="form.points" :min="1" />
        </el-form-item>
        <el-form-item :label="t('stock')">
          <el-input-number v-model="form.stock" :min="0" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAdd = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submit">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { Plus, Present as Gift, Star } from '@element-plus/icons-vue'
import { t } from '../i18n'


const showAdd = ref(false)
const form = ref({ name: '', description: '', points: 100, stock: 10 })

const gifts = ref([
  { id: 1, name: '饮料券', description: '可兑换任意饮料一瓶', points: 100, stock: 50 },
  { id: 2, name: '1小时打球券', description: '可免费打球1小时', points: 300, stock: 20 },
  { id: 3, name: '50元代金券', description: '消费满200可用', points: 500, stock: 10 },
  { id: 4, name: '精美台球手套', description: '品牌台球手套一副', points: 800, stock: 5 },
  { id: 5, name: '200元代金券', description: '消费满500可用', points: 2000, stock: 3 },
  { id: 6, name: '专属球杆租赁', description: 'VIP专属球杆免费租赁3次', points: 3000, stock: 2 },
])

const history = ref([
  { id: 1, time: new Date(), memberName: '张三', giftName: '饮料券', points: 100, status: 'claimed' },
  { id: 2, time: new Date(Date.now() - 3600000), memberName: '李四', giftName: '1小时打球券', points: 300, status: 'pending' },
  { id: 3, time: new Date(Date.now() - 7200000), memberName: '王五', giftName: '精美台球手套', points: 800, status: 'claimed' },
])

const totalStock = computed(() => gifts.value.reduce((sum, g) => sum + g.stock, 0))
const redeemedCount = computed(() => history.value.filter(h => h.status === 'claimed').length)

const formatTime = (d) => new Date(d).toLocaleString('zh-CN')

const redeem = (g) => {
  if (g.stock <= 0) { ElMessage.warning(t('outOfStock')); return }
  ElMessage.success(`${t('redeemSuccess')}: ${g.name}`)
  history.value.unshift({ id: Date.now(), time: new Date(), memberName: '当前会员', giftName: g.name, points: g.points, status: 'pending' })
}

const claim = (row) => { row.status = 'claimed'; ElMessage.success(t('claimSuccess')) }
const editGift = (g) => ElMessage.info(`${t('edit')}: ${g.name}`)

const submit = () => {
  if (!form.value.name) { ElMessage.warning(t('pleaseComplete')); return }
  gifts.value.push({ id: Date.now(), ...form.value })
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
}
</script>

<style scoped>
.points-mall-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }

.stats-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  text-align: center;
  border: 1px solid var(--border-color);
}

.stat-value { display: block; font-size: 28px; font-weight: 600; }
.stat-label { font-size: 13px; color: var(--text-secondary); }

.gifts-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.gift-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.gift-image {
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 12px;
}

.gift-name { font-weight: 600; margin-bottom: 4px; }
.gift-desc { font-size: 12px; color: var(--text-secondary); margin-bottom: 8px; }
.gift-points {
  display: flex;
  align-items: center;
  gap: 4px;
  color: var(--accent-warning);
  font-weight: 600;
  margin-bottom: 4px;
}

.gift-stock {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.low-stock { color: var(--accent-danger); margin-left: 4px; }

.gift-actions { display: flex; gap: 8px; }

.redeem-history {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.section-header { margin-bottom: 16px; }
.section-header h3 { margin: 0; font-size: 16px; font-weight: 600; }

.points-deduct { color: var(--accent-warning); font-weight: 500; }
</style>
