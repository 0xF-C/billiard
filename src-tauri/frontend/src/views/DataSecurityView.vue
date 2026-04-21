<template>
  <div class="data-security-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('dataSecurity') }}</h1>
    </div>

    <div class="security-sections">
      <div class="section">
        <h3>{{ t('backupSettings') }}</h3>
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-name">{{ t('autoBackup') }}</span>
            <span class="setting-desc">{{ t('autoBackupDesc') }}</span>
          </div>
          <el-switch v-model="backup.auto" />
        </div>
        <div class="setting-row" v-if="backup.auto">
          <div class="setting-info">
            <span class="setting-name">{{ t('backupInterval') }}</span>
          </div>
          <el-select v-model="backup.interval">
            <el-option :label="t('everyDay')" value="daily" />
            <el-option :label="t('everyWeek')" value="weekly" />
          </el-select>
        </div>
        <div class="backup-history">
          <h4>{{ t('recentBackups') }}</h4>
          <el-table :data="backupHistory" size="small" max-height="200">
            <el-table-column :label="t('time')" width="160">
              <template #default="{ row }">{{ formatTime(row.time) }}</template>
            </el-table-column>
            <el-table-column :label="t('size')" width="100">
              <template #default="{ row }">{{ row.size }}</template>
            </el-table-column>
            <el-table-column :label="t('status')" width="80">
              <template #default="{ row }">
                <el-tag :type="row.status === 'success' ? 'success' : 'danger'" size="small">{{ t(row.status) }}</el-tag>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </div>

      <div class="section">
        <h3>{{ t('dataRetention') }}</h3>
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-name">{{ t('orderRetention') }}</span>
          </div>
          <el-select v-model="retention.orders">
            <el-option label=t('3months') :value="3" />
            <el-option label=t('6months') :value="6" />
            <el-option label=t('1year') :value="12" />
            <el-option label=t('permanent') :value="999" />
          </el-select>
        </div>
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-name">{{ t('logRetention') }}</span>
          </div>
          <el-select v-model="retention.logs">
            <el-option label=t('7days') :value="0.25" />
            <el-option label=t('30days') :value="1" />
            <el-option label=t('3months') :value="3" />
            <el-option label=t('6months') :value="6" />
          </el-select>
        </div>
      </div>

      <div class="section">
        <h3>{{ t('accessControl') }}</h3>
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-name">{{ t('loginTimeout') }}</span>
            <span class="setting-desc">{{ t('loginTimeoutDesc') }}</span>
          </div>
          <el-input-number v-model="access.timeout" :min="5" :max="120" />
          <span class="unit">{{ t('minutes') }}</span>
        </div>
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-name">{{ t('passwordPolicy') }}</span>
          </div>
          <el-select v-model="access.passwordPolicy">
            <el-option :label="t('normal')" value="normal" />
            <el-option :label="t('strong')" value="strong" />
          </el-select>
        </div>
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-name">{{ t('loginLog') }}</span>
          </div>
          <el-switch v-model="access.logLogin" />
        </div>
      </div>
    </div>

    <div class="action-bar">
      <el-button type="primary" @click="save">{{ t('save') }}</el-button>
      <el-button @click="manualBackup">{{ t('manualBackup') }}</el-button>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { t } from '../i18n'


const backup = reactive({ auto: true, interval: 'daily' })
const retention = reactive({ orders: 12, logs: 1 })
const access = reactive({ timeout: 30, passwordPolicy: 'normal', logLogin: true })

const backupHistory = ref([
  { time: new Date(), size: '12.5 MB', status: 'success' },
  { time: new Date(Date.now() - 86400000), size: '12.3 MB', status: 'success' },
  { time: new Date(Date.now() - 86400000 * 3), size: '12.1 MB', status: 'success' },
  { time: new Date(Date.now() - 86400000 * 7), size: '11.8 MB', status: 'success' },
])

const formatTime = (d) => new Date(d).toLocaleString(currentLang.value === 'zh' ? 'zh-CN' : (currentLang.value === 'en' ? 'en-US' : 'zh-CN'))

const save = () => ElMessage.success(t('saveSuccess'))
const manualBackup = () => ElMessage.success(t('backupSuccess'))
</script>

<style scoped>
.data-security-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0 0 24px 0; }

.security-sections { display: flex; flex-direction: column; gap: 24px; }

.section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.section h3 { margin: 0 0 16px 0; font-size: 16px; font-weight: 600; }

.setting-row {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 0;
  border-bottom: 1px solid var(--border-color);
}

.setting-row:last-child { border-bottom: none; }

.setting-info { flex: 1; display: flex; flex-direction: column; }
.setting-name { font-weight: 500; }
.setting-desc { font-size: 12px; color: var(--text-secondary); }

.unit { font-size: 13px; color: var(--text-secondary); }

.backup-history { margin-top: 16px; }
.backup-history h4 { margin: 0 0 12px 0; font-size: 14px; font-weight: 500; }

.action-bar {
  display: flex;
  gap: 12px;
  margin-top: 24px;
  justify-content: flex-end;
}
</style>
