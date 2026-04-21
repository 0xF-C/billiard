<template>
  <div class="coupons-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('coupons') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('createCoupon') }}</el-button>
    </div>

    <div class="filter-bar">
      <el-select v-model="filterStatus" :placeholder="t('status')" clearable>
        <el-option :label="t('all')" value="" />
        <el-option :label="t('active')" value="active" />
        <el-option :label="t('paused')" value="paused" />
        <el-option :label="t('expired')" value="expired" />
      </el-select>
    </div>

    <div class="coupons-grid">
      <div v-for="c in filteredCoupons" :key="c.id" class="coupon-card" :class="c.type">
        <div class="coupon-header">
          <div class="coupon-value">
            <span class="currency">¥</span>
            <span class="amount">{{ c.discount }}</span>
            <span class="type-label">{{ c.type === 'cash' ? t('cashCoupon') : t('discountCoupon') }}</span>
          </div>
          <el-tag :type="getStatusTag(c.status)" size="small">{{ t(c.status) }}</el-tag>
        </div>
        <div class="coupon-info">
          <div class="coupon-name">{{ c.name }}</div>
          <div class="coupon-desc">{{ c.description }}</div>
          <div class="coupon-meta">
            <span v-if="c.minAmount > 0">{{ t('minimum') }}¥{{ c.minAmount }}{{ t('available') }}</span>
            <span>{{ t('validUntil') }} {{ formatDate(c.expireDate) }}</span>
          </div>
        </div>
        <div class="coupon-stats">
          <div class="stat">
            <span class="stat-value">{{ c.totalCount }}</span>
            <span class="stat-label">{{ t('totalCount') }}</span>
          </div>
          <div class="stat">
            <span class="stat-value">{{ c.usedCount }}</span>
            <span class="stat-label">{{ t('usedCount') }}</span>
          </div>
          <div class="stat">
            <span class="stat-value">{{ c.redeemCount }}</span>
            <span class="stat-label">{{ t('redeemedCount') }}</span>
          </div>
        </div>
        <div class="coupon-actions">
          <el-button size="small" @click="editCoupon(c)">{{ t('edit') }}</el-button>
          <el-button size="small" @click="toggleStatus(c)">
            {{ c.status === 'active' ? t('pause') : t('activate') }}
          </el-button>
        </div>
      </div>
    </div>

    <!-- 创建优惠券弹窗 -->
    <el-dialog v-model="showAdd" :title="t('createCoupon')" width="500px">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="t('couponName')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="t('couponType')">
          <el-radio-group v-model="form.type">
            <el-radio value="cash">{{ t('cashCoupon') }}</el-radio>
            <el-radio value="discount">{{ t('discountCoupon') }}</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item :label="t('discountAmount')">
          <el-input-number v-model="form.discount" :min="1" :precision="form.type === 'discount' ? 1 : 0" />
          <span class="form-tip">{{ form.type === 'discount' ? '折' : '元' }}</span>
        </el-form-item>
        <el-form-item :label="t('minAmount')">
          <el-input-number v-model="form.minAmount" :min="0" />
          <span class="form-tip">{{ t('minAmountTip') }}</span>
        </el-form-item>
        <el-form-item :label="t('totalCount')">
          <el-input-number v-model="form.totalCount" :min="1" />
        </el-form-item>
        <el-form-item :label="t('expireDate')">
          <el-date-picker v-model="form.expireDate" type="date" value-format="YYYY-MM-DD" />
        </el-form-item>
        <el-form-item :label="t('description')">
          <el-input v-model="form.description" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAdd = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitCoupon">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { t } from '../i18n'


const filterStatus = ref('')
const showAdd = ref(false)
const form = ref({
  name: '',
  type: 'cash',
  discount: 50,
  minAmount: 100,
  totalCount: 100,
  expireDate: '',
  description: '',
})

const coupons = ref([
  { id: 1, name: '新人50元券', type: 'cash', discount: 50, minAmount: 200, totalCount: 500, usedCount: 128, redeemCount: 156, expireDate: '2026-05-31', status: 'active', description: '新用户首充赠送' },
  { id: 2, name: '充值200送30', type: 'cash', discount: 30, minAmount: 200, totalCount: 200, usedCount: 45, redeemCount: 60, expireDate: '2026-04-30', status: 'active', description: '充值满200可用' },
  { id: 3, name: '8折折扣券', type: 'discount', discount: 8, minAmount: 100, totalCount: 100, usedCount: 12, redeemCount: 20, expireDate: '2026-04-15', status: 'expired', description: '全场商品8折' },
  { id: 4, name: 'VIP专属100元券', type: 'cash', discount: 100, minAmount: 500, totalCount: 50, usedCount: 8, redeemCount: 10, expireDate: '2026-06-30', status: 'active', description: 'VIP会员专属' },
])

const filteredCoupons = computed(() => {
  if (!filterStatus.value) return coupons.value
  return coupons.value.filter(c => c.status === filterStatus.value)
})

const getStatusTag = (status) => {
  const types = { active: 'success', paused: 'warning', expired: 'info' }
  return types[status] || 'info'
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
}

const toggleStatus = (c) => {
  c.status = c.status === 'active' ? 'paused' : 'active'
  ElMessage.success(`${t('success')}: ${c.name}`)
}

const submitCoupon = () => {
  if (!form.value.name) { ElMessage.warning(t('pleaseComplete')); return }
  coupons.value.unshift({ id: Date.now(), ...form.value, usedCount: 0, redeemCount: 0, status: 'active' })
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
}

const editCoupon = (c) => ElMessage.info(`${t('edit')}: ${c.name}`)
</script>

<style scoped>
.coupons-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.filter-bar { margin-bottom: 24px; display: flex; gap: 12px; }

.coupons-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.coupon-card {
  background: var(--card-bg);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.coupon-card.cash {
  border-top: 3px solid var(--accent-primary);
}

.coupon-card.discount {
  border-top: 3px solid var(--accent-warning);
}

.coupon-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 16px 16px 12px;
}

.coupon-value {
  display: flex;
  align-items: baseline;
  gap: 2px;
}

.currency { font-size: 16px; font-weight: 600; color: var(--accent-danger); }
.amount { font-size: 36px; font-weight: 700; color: var(--accent-danger); }
.type-label { font-size: 13px; color: var(--text-secondary); margin-left: 4px; }

.coupon-info { padding: 0 16px 12px; }
.coupon-name { font-weight: 600; margin-bottom: 4px; }
.coupon-desc { font-size: 12px; color: var(--text-secondary); margin-bottom: 8px; }
.coupon-meta { font-size: 12px; color: var(--text-secondary); display: flex; flex-direction: column; gap: 2px; }

.coupon-stats {
  display: flex;
  gap: 16px;
  padding: 12px 16px;
  background: var(--hover-bg);
}

.stat { text-align: center; }
.stat-value { display: block; font-size: 18px; font-weight: 600; }
.stat-label { font-size: 11px; color: var(--text-secondary); }

.coupon-actions {
  display: flex;
  gap: 8px;
  padding: 12px 16px;
}

.form-tip { margin-left: 8px; font-size: 12px; color: var(--text-secondary); }
</style>
