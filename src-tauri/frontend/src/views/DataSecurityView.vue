<template>
  <div class="data-security-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('dataSecurity') }}</h1>
    </div>

    <div class="sections-grid">
      <!-- 数据库备份 -->
      <div class="card">
        <div class="card-title-row">
          <el-icon class="card-icon backup-icon-color"><FolderOpened /></el-icon>
          <h3>{{ t('backupRestore') }}</h3>
        </div>
        <p class="card-desc">{{ t('downloadDb') || '将当前数据库备份到本地文件，妥善保管以防数据丢失。' }}</p>
        <div class="db-path-row">
          <el-icon><DataLine /></el-icon>
          <span class="db-path-label">{{ t('dbPath') || '数据库路径' }}:</span>
          <span class="db-path-value">{{ dbPath || '加载中...' }}</span>
        </div>
        <div class="action-row">
          <el-button type="primary" :loading="backingUp" @click="doBackup" :icon="Download">
            {{ t('createBackup') || '立即备份' }}
          </el-button>
          <el-button :icon="Upload" @click="triggerRestore">
            {{ t('restoreData') || '从备份恢复' }}
          </el-button>
        </div>
        <input ref="restoreInput" type="file" accept=".db,.sqlite" style="display:none" @change="doRestore" />

        <!-- 最近备份记录 -->
        <div v-if="backupHistory.length" class="backup-history">
          <div class="history-title">{{ t('recentBackups') || '最近备份记录' }}</div>
          <div v-for="(item, i) in backupHistory" :key="i" class="history-item">
            <el-icon class="history-icon"><DocumentCopy /></el-icon>
            <span class="history-name">{{ item.name }}</span>
            <span class="history-size">{{ item.size }}</span>
            <el-tag type="success" size="small">{{ t('success') || '成功' }}</el-tag>
          </div>
        </div>
      </div>

      <!-- 数据保留策略 -->
      <div class="card">
        <div class="card-title-row">
          <el-icon class="card-icon retention-icon-color"><Clock /></el-icon>
          <h3>{{ t('dataRetention') || '数据保留策略' }}</h3>
        </div>
        <p class="card-desc">{{ '设置历史数据的保留周期，过期数据将自动清理以释放存储空间。' }}</p>
        <div class="setting-list">
          <div class="setting-item">
            <div class="setting-label">{{ t('orderRetention') || '订单保留周期' }}</div>
            <el-select v-model="retention.orders" style="width:150px">
              <el-option :label="t('3months') || '3个月'" :value="3" />
              <el-option :label="t('6months') || '6个月'" :value="6" />
              <el-option :label="t('1year') || '1年'" :value="12" />
              <el-option :label="t('permanent') || '永久'" :value="999" />
            </el-select>
          </div>
          <div class="setting-item">
            <div class="setting-label">{{ t('logRetention') || '日志保留周期' }}</div>
            <el-select v-model="retention.logs" style="width:150px">
              <el-option :label="t('7days') || '7天'" :value="0.25" />
              <el-option :label="t('30days') || '30天'" :value="1" />
              <el-option :label="t('3months') || '3个月'" :value="3" />
              <el-option :label="t('6months') || '6个月'" :value="6" />
            </el-select>
          </div>
        </div>
        <div class="card-footer">
          <el-button type="primary" @click="saveRetention">{{ t('save') }}</el-button>
        </div>
      </div>

      <!-- 访问控制 -->
      <div class="card">
        <div class="card-title-row">
          <el-icon class="card-icon access-icon-color"><Lock /></el-icon>
          <h3>{{ t('accessControl') || '访问控制' }}</h3>
        </div>
        <p class="card-desc">{{ '配置登录会话和密码安全策略，保护系统安全。' }}</p>
        <div class="setting-list">
          <div class="setting-item">
            <div class="setting-label">
              {{ t('loginTimeout') || '登录超时' }}
              <span class="label-desc">{{ t('loginTimeoutDesc') || '无操作后自动登出' }}</span>
            </div>
            <div class="input-with-unit">
              <el-input-number v-model="access.timeout" :min="5" :max="120" style="width:120px" />
              <span class="unit">{{ t('minutes') || '分钟' }}</span>
            </div>
          </div>
          <div class="setting-item">
            <div class="setting-label">{{ t('passwordPolicy') || '密码策略' }}</div>
            <el-select v-model="access.passwordPolicy" style="width:150px">
              <el-option :label="t('normal') || '普通'" value="normal" />
              <el-option :label="t('strong') || '强密码'" value="strong" />
            </el-select>
          </div>
          <div class="setting-item">
            <div class="setting-label">{{ t('loginLog') || '记录登录日志' }}</div>
            <el-switch v-model="access.logLogin" />
          </div>
        </div>
        <div class="card-footer">
          <el-button type="primary" @click="saveAccess">{{ t('save') }}</el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Download, Upload, FolderOpened, Clock, Lock, DataLine, DocumentCopy } from '@element-plus/icons-vue'
import { t } from '../i18n'
import { backupDatabase, restoreDatabase, getDbPath } from '../api'

const restoreInput = ref(null)
const backingUp = ref(false)
const dbPath = ref('')
const backupHistory = ref([])

const retention = reactive({ orders: 12, logs: 1 })
const access = reactive({ timeout: 30, passwordPolicy: 'normal', logLogin: true })

onMounted(async () => {
  dbPath.value = await getDbPath()
})

const doBackup = async () => {
  backingUp.value = true
  try {
    const result = await backupDatabase()
    if (result && result.success) {
      const sizeKB = result.size ? (result.size / 1024).toFixed(1) + ' KB' : ''
      backupHistory.value.unshift({ name: result.path.split(/[/\\]/).pop(), size: sizeKB })
      if (backupHistory.value.length > 5) backupHistory.value.pop()
      ElMessage.success(`${t('backupSuccess') || '备份成功'}${result.path ? '：' + result.path : ''}`)
    } else {
      ElMessage.error(result?.message || t('operationFailed'))
    }
  } catch (e) {
    ElMessage.error(String(e) || t('operationFailed'))
  } finally {
    backingUp.value = false
  }
}

const triggerRestore = () => restoreInput.value?.click()

const doRestore = async (e) => {
  const file = e.target.files[0]
  if (!file) return
  e.target.value = ''
  try {
    await ElMessageBox.confirm(
      t('restoreConfirm') || '恢复后当前数据将被覆盖且无法撤销，确定要继续吗？',
      t('confirm') || '确认恢复',
      { type: 'warning', confirmButtonText: t('confirm') || '确定', cancelButtonText: t('cancel') || '取消' }
    )
    await restoreDatabase(file.path || file.name)
    ElMessage.success(t('restoreSuccess') || '恢复成功，请重启应用')
  } catch (err) {
    if (err !== 'cancel') ElMessage.error(String(err) || t('operationFailed'))
  }
}

const saveRetention = () => {
  ElMessage.success(t('saveSuccess'))
}

const saveAccess = () => {
  ElMessage.success(t('saveSuccess'))
}
</script>

<style scoped>
.data-security-page {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.page-header { display: flex; align-items: center; }
.page-title { font-size: 24px; font-weight: 600; color: var(--text-primary); margin: 0; }

.sections-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
  gap: 20px;
  align-items: start;
}

.card {
  background: var(--card-bg, #fff);
  border: 1px solid var(--border-color, #e4e7ed);
  border-radius: 12px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.card-title-row {
  display: flex;
  align-items: center;
  gap: 10px;
}
.card-title-row h3 { margin: 0; font-size: 16px; font-weight: 600; color: var(--text-primary); }
.card-icon { font-size: 20px; flex-shrink: 0; }
.backup-icon-color { color: #409eff; }
.retention-icon-color { color: #67c23a; }
.access-icon-color { color: #e6a23c; }

.card-desc { margin: 0; font-size: 13px; color: var(--text-secondary, #909399); line-height: 1.6; }

.db-path-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: var(--bg-secondary, #f5f7fa);
  border-radius: 8px;
  font-size: 12px;
  color: var(--text-secondary);
  word-break: break-all;
}
.db-path-label { font-weight: 500; white-space: nowrap; }
.db-path-value { color: var(--text-primary); font-family: monospace; }

.action-row { display: flex; gap: 12px; flex-wrap: wrap; }

.backup-history { display: flex; flex-direction: column; gap: 8px; }
.history-title { font-size: 13px; font-weight: 500; color: var(--text-secondary); }
.history-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  background: var(--bg-secondary, #f5f7fa);
  border-radius: 8px;
  font-size: 13px;
}
.history-icon { color: var(--text-tertiary, #c0c4cc); }
.history-name { flex: 1; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.history-size { color: var(--text-secondary); white-space: nowrap; }

.setting-list { display: flex; flex-direction: column; gap: 0; }
.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 14px 0;
  border-bottom: 1px solid var(--border-color, #e4e7ed);
}
.setting-item:last-child { border-bottom: none; }
.setting-label {
  display: flex;
  flex-direction: column;
  gap: 2px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}
.label-desc { font-size: 12px; font-weight: 400; color: var(--text-secondary); }

.input-with-unit { display: flex; align-items: center; gap: 8px; }
.unit { font-size: 13px; color: var(--text-secondary); white-space: nowrap; }

.card-footer { display: flex; justify-content: flex-end; padding-top: 4px; }
</style>
