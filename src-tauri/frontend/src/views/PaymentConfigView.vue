<template>
  <div class="payment-config-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('paymentConfig') }}</h1>
    </div>

    <div class="config-sections">
      <div class="config-section">
        <h3>{{ t('onlinePayment') }}</h3>
        <div class="config-list">
          <div class="config-item" v-for="p in onlinePayments" :key="p.id">
            <div class="item-info">
              <el-icon :size="20" :color="p.color"><component :is="p.icon" /></el-icon>
              <span class="item-name">{{ p.name }}</span>
            </div>
            <div class="item-actions">
              <el-tag :type="p.enabled ? 'success' : 'info'" size="small">{{ p.enabled ? t('enabled') : t('disabled') }}</el-tag>
              <el-switch v-model="p.enabled" />
            </div>
          </div>
        </div>
      </div>

      <div class="config-section">
        <h3>{{ t('cashSettings') }}</h3>
        <el-form label-width="120px">
          <el-form-item :label="t('allowCash')">
            <el-switch v-model="cash.allow" />
          </el-form-item>
          <el-form-item :label="t('floatAmount')">
            <el-input-number v-model="cash.float" :min="0" :precision="0" />
            <span class="form-tip">{{ t('floatAmountTip') }}</span>
          </el-form-item>
          <el-form-item :label="t('cashAuditInterval')">
            <el-select v-model="cash.auditInterval">
              <el-option :label="t('everyShift')" :value="0" />
              <el-option :label="t('everyDay')" :value="1" />
            </el-select>
          </el-form-item>
        </el-form>
      </div>

      <div class="config-section">
        <h3>{{ t('autoCheckout') }}</h3>
        <el-form label-width="140px">
          <el-form-item :label="t('enableAutoCheckout')">
            <el-switch v-model="auto.enable" />
          </el-form-item>
          <el-form-item :label="t('maxIdleMinutes')">
            <el-input-number v-model="auto.maxIdleMinutes" :min="30" :max="480" :step="30" />
            <span class="form-tip">{{ t('maxIdleTip') }}</span>
          </el-form-item>
          <el-form-item :label="t('warningBefore')">
            <el-input-number v-model="auto.warningMinutes" :min="5" :max="30" />
            <span class="form-tip">{{ t('warningTip') }}</span>
          </el-form-item>
        </el-form>
      </div>
    </div>

    <div class="action-bar">
      <el-button type="primary" size="large" @click="save">{{ t('save') }}</el-button>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { t } from '../i18n'


const onlinePayments = ref([
  { id: 'wechat', name: '微信支付', enabled: true, color: '#07c160', icon: 'Wallet' },
  { id: 'alipay', name: '支付宝', enabled: true, color: '#1677ff', icon: 'Money' },
  { id: 'unionpay', name: '银联云闪付', enabled: false, color: '#e74c3c', icon: 'CreditCard' },
  { id: 'jdpay', name: '京东支付', enabled: false, color: '#c0392b', icon: 'ShoppingCart' },
])

const cash = reactive({ allow: true, float: 500, auditInterval: 0 })
const auto = reactive({ enable: true, maxIdleMinutes: 120, warningMinutes: 10 })

const save = () => {
  ElMessage.success(t('saveSuccess'))
}
</script>

<style scoped>
.payment-config-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0 0 24px 0; }

.config-sections {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.config-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.config-section h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
}

.config-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.config-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--hover-bg);
  border-radius: 8px;
}

.item-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.item-name { font-weight: 500; }
.item-actions { display: flex; align-items: center; gap: 12px; }

.form-tip { margin-left: 12px; font-size: 12px; color: var(--text-secondary); }

.action-bar {
  display: flex;
  justify-content: flex-end;
  margin-top: 24px;
}
</style>
