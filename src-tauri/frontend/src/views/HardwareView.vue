<template>
  <div class="hardware-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('hardware') }}</h1>
      <div class="conn-status" :class="connOk === true ? 'ok' : connOk === false ? 'fail' : 'idle'">
        <el-icon v-if="connOk === true"><SuccessFilled /></el-icon>
        <el-icon v-else-if="connOk === false"><CircleCloseFilled /></el-icon>
        <el-icon v-else><Loading /></el-icon>
        <span>{{ connOk === true ? t('status') + ' OK' : connOk === false ? t('operationFailed') : t('relayTest') + '...' }}</span>
      </div>
    </div>

    <div class="header-actions">
      <el-button @click="testConn" :icon="Connection">{{ t('relayTest') }}</el-button>
      <el-button @click="loadStatus" :icon="Refresh">{{ t('relayStatus') }}</el-button>
    </div>

    <div class="relay-grid">
      <div v-for="(on, idx) in relayList" :key="idx" class="relay-card" :class="on ? 'on' : 'off'">
        <div class="relay-header">
          <div class="relay-icon" :class="on ? 'on' : 'off'">
            <el-icon><Connection /></el-icon>
          </div>
          <div class="relay-info">
            <span class="relay-name">{{ t('relayTable') }} {{ idx + 1 }}</span>
            <el-tag :type="on ? 'success' : 'info'" size="small">
              {{ on ? t('relayOn') : t('relayOff') }}
            </el-tag>
          </div>
        </div>
        <el-button
          :type="on ? 'danger' : 'success'"
          class="toggle-btn"
          @click="toggle(idx + 1, !on)"
        >
          <el-icon><Switch /></el-icon>
          {{ on ? t('relayOff') : t('relayOn') }}
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElButton, ElIcon, ElTag } from 'element-plus'
import { Connection, Refresh, SuccessFilled, CircleCloseFilled, Loading, Switch } from '@element-plus/icons-vue'
import { getRelayStatus, testConnection, setRelay } from '../api'
import { t } from '../i18n'

const relayStatus = ref({})
const connOk = ref(null)

const relayList = computed(() => {
  const list = []
  for (let i = 1; i <= 12; i++) list.push(relayStatus.value[i] ?? false)
  return list
})

const loadStatus = async () => {
  try {
    const res = await getRelayStatus()
    relayStatus.value = res.relays || {}
    connOk.value = res.status !== 'error'
  } catch { connOk.value = false; ElMessage.error(t('loadFailed')) }
}

const toggle = async (id, turnOn) => {
  try {
    await setRelay(id, turnOn ? 'on' : 'off')
    ElMessage.success(`${id}${turnOn ? t('relayOn') : t('relayOff')}`)
    await loadStatus()
  } catch(e) { ElMessage.error(e.response?.data?.message || t('operationFailed')) }
}

const testConn = async () => {
  try {
    const res = await testConnection()
    connOk.value = res.connected
    ElMessage.success(res.connected ? t('status') + ' OK' : t('operationFailed'))
  } catch { connOk.value = false; ElMessage.error(t('operationFailed')) }
}

onMounted(loadStatus)
</script>

<style scoped>
.hardware-page { display: flex; flex-direction: column; gap: 24px; }
.page-header { display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }
.conn-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border-radius: 20px;
  font-size: 13px;
  font-weight: 600;
}
.conn-status.ok { background: rgba(63,185,80,0.15); color: var(--accent-success); border: 1px solid rgba(63,185,80,0.3); }
.conn-status.fail { background: rgba(248,81,73,0.15); color: var(--accent-danger); border: 1px solid rgba(248,81,73,0.3); }
.conn-status.idle { background: var(--bg-tertiary); color: var(--text-tertiary); border: 1px solid var(--border-default); }
.header-actions { display: flex; gap: 8px; }
.relay-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 16px; }
.relay-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: 20px;
  transition: all 0.2s;
}
.relay-card:hover { border-color: var(--accent-primary); }
.relay-card.on { border-color: rgba(63,185,80,0.3); }
.relay-header { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }
.relay-icon {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}
.relay-icon.on { background: rgba(63,185,80,0.15); color: var(--accent-success); }
.relay-icon.off { background: var(--bg-tertiary); color: var(--text-tertiary); }
.relay-info { display: flex; flex-direction: column; gap: 4px; }
.relay-name { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.toggle-btn { width: 100%; }
</style>
