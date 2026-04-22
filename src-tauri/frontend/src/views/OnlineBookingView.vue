<template>
  <div class="online-booking-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('onlineBooking') }}</h1>
      <div class="header-actions">
        <el-button type="primary" :icon="Link" @click="showLink = true">{{ t('shareLink') }}</el-button>
      </div>
    </div>

    <div class="config-section">
      <div class="section-header">
        <h3>{{ t('bookingSettings') }}</h3>
      </div>
      <div class="config-grid">
        <div class="config-item">
          <span class="config-label">{{ t('allowOnlineBooking') }}</span>
          <el-switch v-model="config.allowOnline" />
        </div>
        <div class="config-item">
          <span class="config-label">{{ t('advanceDays') }}</span>
          <el-input-number v-model="config.advanceDays" :min="1" :max="30" />
        </div>
        <div class="config-item">
          <span class="config-label">{{ t('minDeposit') }}</span>
          <el-input-number v-model="config.minDeposit" :min="0" :precision="0" />
        </div>
        <div class="config-item">
          <span class="config-label">{{ t('autoConfirm') }}</span>
          <el-switch v-model="config.autoConfirm" />
        </div>
      </div>
    </div>

    <div class="channel-section">
      <div class="section-header">
        <h3>{{ t('bookingChannels') }}</h3>
        <el-button type="primary" size="small" :icon="Plus" @click="showAddChannel = true">
          {{ t('addChannel') }}
        </el-button>
      </div>
      <div class="channels-grid">
        <div v-for="ch in channels" :key="ch.id" class="channel-card" :class="{ enabled: ch.enabled }">
          <div class="channel-header">
            <span class="channel-name">{{ ch.name }}</span>
            <el-switch v-model="ch.enabled" />
          </div>
          <div class="channel-stats">
            <div class="stat">
              <span class="stat-value">{{ ch.todayOrders }}</span>
              <span class="stat-label">{{ t('todayOrders') }}</span>
            </div>
            <div class="stat">
              <span class="stat-value">{{ ch.pendingOrders }}</span>
              <span class="stat-label">{{ t('pendingOrders') }}</span>
            </div>
          </div>
          <div class="channel-url">
            <el-input :value="ch.url" readonly size="small">
              <template #append>
                <el-button @click="copyLink(ch.url)">{{ t('copy') }}</el-button>
              </template>
            </el-input>
          </div>
        </div>
      </div>
    </div>

    <div class="orders-section">
      <div class="section-header">
        <h3>{{ t('onlineBookingOrders') }}</h3>
        <el-radio-group v-model="filterStatus" size="small">
          <el-radio-button value="">{{ t('all') }}</el-radio-button>
          <el-radio-button value="pending">{{ t('pending') }}</el-radio-button>
          <el-radio-button value="confirmed">{{ t('confirmed') }}</el-radio-button>
          <el-radio-button value="cancelled">{{ t('cancelled') }}</el-radio-button>
        </el-radio-group>
      </div>
      <el-table :data="filteredOrders" stripe>
        <el-table-column :label="t('orderNo')" width="100">
          <template #default="{ row }">#{{ row.id }}</template>
        </el-table-column>
        <el-table-column :label="t('channel')" width="100">
          <template #default="{ row }">
            <el-tag size="small">{{ row.channel }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('customer')" min-width="140">
          <template #default="{ row }">
            <div class="customer-cell">
              <span class="name">{{ row.customerName }}</span>
              <span class="phone">{{ row.customerPhone }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('table')" width="100">
          <template #default="{ row }">{{ row.tableName || '-' }}</template>
        </el-table-column>
        <el-table-column :label="t('bookingTime')" width="160">
          <template #default="{ row }">{{ formatTime(row.bookingTime) }}</template>
        </el-table-column>
        <el-table-column :label="t('duration')" width="80">
          <template #default="{ row }">{{ row.duration }}h</template>
        </el-table-column>
        <el-table-column :label="t('deposit')" width="80">
          <template #default="{ row }">¥{{ row.deposit }}</template>
        </el-table-column>
        <el-table-column :label="t('status')" width="90">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)" size="small">{{ t(row.status) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="140" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="confirmOrder(row)" v-if="row.status === 'pending'">
              {{ t('confirm') }}
            </el-button>
            <el-button link type="danger" @click="cancelOrder(row)" v-if="row.status !== 'cancelled'">
              {{ t('cancel') }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="showAddChannel" :title="t('addChannel')" width="400px" top="5vh" append-to-body>
      <el-form label-width="80px">
        <el-form-item :label="t('channelName')">
          <el-input v-model="newChannel.name" />
        </el-form-item>
        <el-form-item :label="t('channelUrl')">
          <el-input v-model="newChannel.url" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddChannel = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="addChannel">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { Link, Plus } from '@element-plus/icons-vue'
import { t } from '../i18n'


const filterStatus = ref('')
const showAddChannel = ref(false)
const showLink = ref(false)

const config = ref({
  allowOnline: true,
  advanceDays: 7,
  minDeposit: 50,
  autoConfirm: false,
})

const channels = ref([
  { id: 1, name: t('wechatPublicAccount'), url: 'https://billiard.example.com/booking/wx', enabled: true, todayOrders: 12, pendingOrders: 3 },
  { id: 2, name: t('miniProgram'), url: 'https://billiard.example.com/booking/mp', enabled: true, todayOrders: 8, pendingOrders: 1 },
  { id: 3, name: t('officialWebsite'), url: 'https://billiard.example.com/booking/web', enabled: true, todayOrders: 5, pendingOrders: 2 },
])

const orders = ref([
  { id: 1001, channel: t('wechatPublicAccount'), customerName: '张三', customerPhone: '13800138001', tableName: t('table1'), bookingTime: new Date(Date.now() + 3600000 * 2), duration: 2, deposit: 100, status: 'pending' },
  { id: 1002, channel: t('miniProgram'), customerName: '李四', customerPhone: '13800138002', tableName: 'VIP-1', bookingTime: new Date(Date.now() + 3600000 * 4), duration: 3, deposit: 200, status: 'confirmed' },
  { id: 1003, channel: t('officialWebsite'), customerName: '王五', customerPhone: '13800138003', tableName: t('table2'), bookingTime: new Date(Date.now() - 3600000), duration: 2, deposit: 100, status: 'pending' },
  { id: 1004, channel: t('wechatPublicAccount'), customerName: '赵六', customerPhone: '13800138004', tableName: null, bookingTime: new Date(Date.now() - 86400000), duration: 2, deposit: 50, status: 'cancelled' },
])

const filteredOrders = computed(() => {
  if (!filterStatus.value) return orders.value
  return orders.value.filter(o => o.status === filterStatus.value)
})

const newChannel = ref({ name: '', url: '' })

const getStatusType = (status) => {
  const types = { pending: 'warning', confirmed: 'success', cancelled: 'info' }
  return types[status] || 'info'
}

const formatTime = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleString(currentLang.value === 'zh' ? 'zh-CN' : (currentLang.value === 'en' ? 'en-US' : 'zh-CN'))
}

const copyLink = (url) => {
  navigator.clipboard?.writeText(url)
  ElMessage.success(t('copySuccess'))
}

const confirmOrder = (order) => {
  order.status = 'confirmed'
  ElMessage.success(t('confirmSuccess'))
}

const cancelOrder = (order) => {
  order.status = 'cancelled'
  ElMessage.success(t('cancelSuccess'))
}

const addChannel = () => {
  if (!newChannel.value.name) return
  channels.value.push({
    id: Date.now(),
    ...newChannel.value,
    enabled: true,
    todayOrders: 0,
    pendingOrders: 0,
  })
  showAddChannel.value = false
  newChannel.value = { name: '', url: '' }
}
</script>

<style scoped>
.online-booking-page { padding: 24px; }

.page-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0 0 24px 0;
}

.header-actions { display: flex; gap: 12px; }

.config-section, .channel-section, .orders-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h3 { margin: 0; font-size: 16px; font-weight: 600; }

.config-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 24px;
}

.config-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.config-label { font-size: 13px; color: var(--text-secondary); }

.channels-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.channel-card {
  background: var(--hover-bg);
  border-radius: 10px;
  padding: 16px;
  border: 1px solid var(--border-color);
}

.channel-card.enabled {
  border-color: var(--primary-color);
}

.channel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.channel-name { font-weight: 600; }

.channel-stats {
  display: flex;
  gap: 24px;
  margin-bottom: 12px;
}

.stat { text-align: center; }
.stat-value { display: block; font-size: 20px; font-weight: 600; }
.stat-label { font-size: 12px; color: var(--text-secondary); }

.customer-cell { display: flex; flex-direction: column; }
.customer-cell .name { font-weight: 500; }
.customer-cell .phone { font-size: 12px; color: var(--text-secondary); }
</style>
