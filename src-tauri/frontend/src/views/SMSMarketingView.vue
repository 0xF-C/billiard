<template>
  <div class="sms-marketing-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('smsMarketing') }}</h1>
      <div class="header-actions">
        <span class="balance">{{ t('smsBalance') }}: {{ balance }} {{ t('smsCount') }}</span>
        <el-button type="primary" :icon="Plus" @click="showSend = true">{{ t('sendSMS') }}</el-button>
      </div>
    </div>

    <div class="send-form">
      <div class="form-header">
        <h3>{{ t('quickSend') }}</h3>
      </div>
      <div class="form-body">
        <div class="form-row">
          <el-radio-group v-model="sendType">
            <el-radio value="all">{{ t('allMembers') }}</el-radio>
            <el-radio value="level">{{ t('byLevel') }}</el-radio>
            <el-radio value="custom">{{ t('custom') }}</el-radio>
          </el-radio-group>
        </div>
        <div class="form-row" v-if="sendType === 'level'">
          <el-checkbox-group v-model="selectedLevels">
            <el-checkbox v-for="l in levels" :key="l" :value="l">{{ t(l) }}</el-checkbox>
          </el-checkbox-group>
        </div>
        <div class="form-row" v-if="sendType === 'custom'">
          <el-input v-model="customPhones" type="textarea" :rows="2" :placeholder="t('inputPhones')" />
        </div>
        <div class="form-row">
          <el-input v-model="smsContent" type="textarea" :rows="4" :placeholder="t('inputSMSContent')" :maxlength="200" show-word-limit />
        </div>
        <div class="form-row">
          <span>{{ t('recipientCount') }}: {{ recipientCount }} {{ t('people') }}</span>
          <span class="sms-count">{{ t('smsCountTip') }}: {{ Math.ceil(smsContent.length / 70) }}</span>
        </div>
        <div class="form-row">
          <el-button type="primary" size="large" :disabled="!smsContent" @click="sendSMS">
            {{ t('sendNow') }}
          </el-button>
        </div>
      </div>
    </div>

    <div class="sms-templates">
      <div class="section-header">
        <h3>{{ t('smsTemplates') }}</h3>
        <el-button type="primary" size="small" :icon="Plus" @click="showTemplate = true">{{ t('addTemplate') }}</el-button>
      </div>
      <div class="template-grid">
        <div v-for="tmpl in templates" :key="tmpl.id" class="template-card" @click="useTemplate(tmpl)">
          <div class="template-name">{{ tmpl.name }}</div>
          <div class="template-content">{{ tmpl.content }}</div>
          <div class="template-meta">{{ tmpl.usageCount }}{{ t('usageCount') }}</div>
        </div>
      </div>
    </div>

    <div class="send-history">
      <div class="section-header">
        <h3>{{ t('sendHistory') }}</h3>
      </div>
      <el-table :data="history" stripe max-height="300">
        <el-table-column :label="t('sendTime')" width="160">
          <template #default="{ row }">{{ formatTime(row.time) }}</template>
        </el-table-column>
        <el-table-column :label="t('content')" min-width="200">
          <template #default="{ row }">
            <span class="sms-content-preview">{{ row.content }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('recipient')" width="80">
          <template #default="{ row }">{{ row.count }}</template>
        </el-table-column>
        <el-table-column :label="t('status')" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 'success' ? 'success' : 'danger'" size="small">{{ t(row.status) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('smsCount')" width="80">
          <template #default="{ row }">{{ row.smsCount }}</template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="showTemplate" :title="t('addTemplate')" width="400px">
      <el-form label-width="80px">
        <el-form-item :label="t('templateName')">
          <el-input v-model="newTemplate.name" />
        </el-form-item>
        <el-form-item :label="t('content')">
          <el-input v-model="newTemplate.content" type="textarea" :rows="4" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showTemplate = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="addTemplate">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { t } from '../i18n'


const balance = ref(1850)
const sendType = ref('all')
const selectedLevels = ref([])
const customPhones = ref('')
const smsContent = ref('')
const showSend = ref(false)
const showTemplate = ref(false)
const newTemplate = ref({ name: '', content: '' })

const levels = [t('normalMember'), t('silverMember'), t('goldMember'), '白金会员', t('diamondMember')]

const recipientCount = computed(() => {
  if (sendType.value === 'all') return 328
  if (sendType.value === 'level') return selectedLevels.value.length * 60
  return (customPhones.value.match(/\d{11}/g) || []).length
})

const templates = ref([
  { id: 1, name: t('promotionActivity'), content: t('smsPromoTemplate'), usageCount: 56 },
  { id: 2, name: t('bookingRemind'), content: t('smsBookingTemplate'), usageCount: 128 },
  { id: 3, name: t('rechargeOffer'), content: t('smsRechargeTemplate'), usageCount: 34 },
  { id: 4, name: t('holidayGreetings'), content: t('smsHolidayTemplate'), usageCount: 89 },
])

const history = ref([
  { id: 1, time: new Date(), content: t('weekend50OffSms'), count: 328, status: 'success', smsCount: 7 },
  { id: 2, time: new Date(Date.now() - 86400000), content: t('bookingRemindSms'), count: 12, status: 'success', smsCount: 1 },
  { id: 3, time: new Date(Date.now() - 86400000 * 3), content: t('rechargeOfferSms'), count: 156, status: 'success', smsCount: 3 },
])

const formatTime = (d) => new Date(d).toLocaleString(currentLang.value === 'zh' ? 'zh-CN' : (currentLang.value === 'en' ? 'en-US' : 'zh-CN'))

const sendSMS = () => {
  if (!smsContent.value) return
  history.value.unshift({
    id: Date.now(), time: new Date(), content: smsContent.value,
    count: recipientCount.value, status: 'success', smsCount: Math.ceil(smsContent.value.length / 70)
  })
  balance.value -= Math.ceil(smsContent.value.length / 70)
  smsContent.value = ''
  ElMessage.success(t('sendSuccess'))
}

const useTemplate = (tmpl) => { smsContent.value = tmpl.content }

const addTemplate = () => {
  if (!newTemplate.value.name) return
  templates.value.push({ id: Date.now(), usageCount: 0, ...newTemplate.value })
  showTemplate.value = false
  ElMessage.success(t('addSuccess'))
}
</script>

<style scoped>
.sms-marketing-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.header-actions { display: flex; align-items: center; gap: 16px; }

.balance {
  padding: 8px 16px;
  background: var(--hover-bg);
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
}

.send-form, .sms-templates, .send-history {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  margin-bottom: 24px;
}

.form-header, .section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.form-header h3, .section-header h3 { margin: 0; font-size: 16px; font-weight: 600; }

.form-body { display: flex; flex-direction: column; gap: 16px; }

.form-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.sms-count { color: var(--text-secondary); font-size: 12px; }

.template-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.template-card {
  background: var(--hover-bg);
  border-radius: 8px;
  padding: 12px;
  cursor: pointer;
  border: 1px solid var(--border-color);
  transition: border-color 0.2s;
}

.template-card:hover { border-color: var(--primary-color); }

.template-name { font-weight: 500; margin-bottom: 6px; }
.template-content { font-size: 12px; color: var(--text-secondary); line-height: 1.5; margin-bottom: 8px; }
.template-meta { font-size: 11px; color: var(--text-secondary); }

.sms-content-preview {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 300px;
}
</style>
