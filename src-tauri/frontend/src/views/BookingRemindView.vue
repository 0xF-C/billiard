<template>
  <div class="booking-remind-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('bookingRemind') }}</h1>
      <div class="header-actions">
        <el-button :icon="Refresh" @click="loadData">{{ t('refresh') }}</el-button>
      </div>
    </div>

    <div class="stats-row">
      <div class="stat-card warning">
        <el-icon :size="28"><Bell /></el-icon>
        <div class="stat-body">
          <span class="stat-value">{{ upcomingBookings.length }}</span>
          <span class="stat-label">{{ t('upcomingBookings') }}</span>
        </div>
      </div>
      <div class="stat-card info">
        <el-icon :size="28"><Clock /></el-icon>
        <div class="stat-body">
          <span class="stat-value">{{ remindingCount }}</span>
          <span class="stat-label">{{ t('remindingCount') }}</span>
        </div>
      </div>
      <div class="stat-card success">
        <el-icon :size="28"><Message /></el-icon>
        <div class="stat-body">
          <span class="stat-value">{{ confirmedCount }}</span>
          <span class="stat-label">{{ t('confirmedToday') }}</span>
        </div>
      </div>
    </div>

    <div class="filter-bar">
      <el-date-picker v-model="selectedDate" type="date" value-format="YYYY-MM-DD" />
      <el-select v-model="filterStatus" :placeholder="t('status')" clearable>
        <el-option :label="t('all')" value="" />
        <el-option :label="t('notRemind')" value="not_remind" />
        <el-option :label="t('reminded')" value="reminded" />
        <el-option :label="t('confirmed')" value="confirmed" />
      </el-select>
    </div>

    <div class="remind-list">
      <div v-for="item in filteredList" :key="item.id" class="remind-item" :class="item.status">
        <div class="item-time">
          <span class="time-value">{{ item.time }}</span>
          <span class="date-value">{{ item.date }}</span>
        </div>
        <div class="item-info">
          <div class="customer-name">{{ item.customerName }}</div>
          <div class="customer-phone">{{ item.customerPhone }}</div>
          <div class="booking-detail">{{ item.tableName }} · {{ item.duration }}h</div>
        </div>
        <div class="item-status">
          <el-tag :type="getStatusTag(item.status)" size="small">{{ t(item.status) }}</el-tag>
        </div>
        <div class="item-actions">
          <el-button type="primary" size="small" @click="sendRemind(item)" v-if="item.status !== 'confirmed'">
            <el-icon><Message /></el-icon> {{ t('sendRemind') }}
          </el-button>
          <el-button size="small" @click="callCustomer(item)">
            <el-icon><Phone /></el-icon> {{ t('call') }}
          </el-button>
          <el-button type="success" size="small" @click="confirmBooking(item)">
            {{ t('confirm') }}
          </el-button>
        </div>
      </div>
      <el-empty v-if="filteredList.length === 0" :description="t('noData')" />
    </div>

    <div class="auto-remind-config">
      <div class="config-header">
        <h3>{{ t('autoRemindConfig') }}</h3>
        <el-switch v-model="autoRemind" />
      </div>
      <div class="config-items" v-if="autoRemind">
        <div class="config-item">
          <span>{{ t('remindBeforeHours') }}</span>
          <el-input-number v-model="remindHours[0]" :min="1" :max="48" />
          <span class="unit">{{ t('hoursBefore') }}</span>
        </div>
        <div class="config-item">
          <span>{{ t('remindBeforeHours') }}</span>
          <el-input-number v-model="remindHours[1]" :min="1" :max="24" />
          <span class="unit">{{ t('hoursBefore') }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { Bell, Clock, Message, Refresh, Phone } from '@element-plus/icons-vue'
import { t } from '../i18n'


const selectedDate = ref(new Date().toISOString().split('T')[0])
const filterStatus = ref('')
const autoRemind = ref(true)
const remindHours = ref([2, 24])

const list = ref([
  { id: 1, customerName: '张三', customerPhone: '13800138001', tableName: '1号桌', date: '2026-04-20', time: '14:00', duration: 2, status: 'not_remind' },
  { id: 2, customerName: '李四', customerPhone: '13800138002', tableName: 'VIP-1', date: '2026-04-20', time: '18:00', duration: 3, status: 'reminded' },
  { id: 3, customerName: '王五', customerPhone: '13800138003', tableName: '2号桌', date: '2026-04-20', time: '10:00', duration: 2, status: 'confirmed' },
  { id: 4, customerName: '赵六', customerPhone: '13800138004', tableName: '3号桌', date: '2026-04-21', time: '15:00', duration: 2, status: 'not_remind' },
])

const upcomingBookings = computed(() => list.value.filter(o => o.status !== 'confirmed'))
const remindingCount = computed(() => list.value.filter(o => o.status === 'reminded').length)
const confirmedCount = computed(() => list.value.filter(o => o.status === 'confirmed').length)

const filteredList = computed(() => {
  let items = list.value
  if (filterStatus.value) items = items.filter(o => o.status === filterStatus.value)
  return items
})

const getStatusTag = (status) => {
  const types = { not_remind: 'warning', reminded: 'info', confirmed: 'success' }
  return types[status] || 'info'
}

const loadData = () => {
  // reload
}

const sendRemind = (item) => {
  item.status = 'reminded'
  ElMessage.success(t('remindSent'))
}

const callCustomer = (item) => {
  ElMessage.info(`${t('calling')}: ${item.customerPhone}`)
}

const confirmBooking = (item) => {
  item.status = 'confirmed'
  ElMessage.success(t('confirmSuccess'))
}
</script>

<style scoped>
.booking-remind-page { padding: 24px; }

.page-title { font-size: 24px; font-weight: 600; margin: 0 0 24px 0; }

.header-actions { display: flex; gap: 12px; }

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
  display: flex;
  align-items: center;
  gap: 16px;
  border: 1px solid var(--border-color);
  border-left: 4px solid var(--accent-primary);
}

.stat-card.warning { border-left-color: var(--accent-warning); }
.stat-card.warning .el-icon { color: var(--accent-warning); }
.stat-card.info { border-left-color: var(--accent-primary); }
.stat-card.info .el-icon { color: var(--accent-primary); }
.stat-card.success { border-left-color: var(--accent-success); }
.stat-card.success .el-icon { color: var(--accent-success); }

.stat-value { display: block; font-size: 24px; font-weight: 600; }
.stat-label { font-size: 13px; color: var(--text-secondary); }

.filter-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.remind-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 24px;
}

.remind-item {
  background: var(--card-bg);
  border-radius: 10px;
  padding: 16px 20px;
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  gap: 20px;
  border-left: 3px solid var(--accent-warning);
}

.remind-item.confirmed {
  border-left-color: var(--accent-success);
  opacity: 0.7;
}

.item-time {
  text-align: center;
  min-width: 60px;
}

.time-value { display: block; font-size: 20px; font-weight: 600; }
.date-value { font-size: 12px; color: var(--text-secondary); }

.item-info { flex: 1; }

.customer-name { font-weight: 600; margin-bottom: 4px; }
.customer-phone { font-size: 13px; color: var(--text-secondary); margin-bottom: 2px; }
.booking-detail { font-size: 12px; color: var(--text-secondary); }

.item-status { min-width: 80px; }

.item-actions { display: flex; gap: 8px; }

.auto-remind-config {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.config-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.config-header h3 { margin: 0; font-size: 16px; font-weight: 600; }

.config-items {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.config-item {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
}

.unit { color: var(--text-secondary); }
</style>
