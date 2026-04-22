<template>
  <div class="tournaments-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('tournaments') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('createTournament') }}</el-button>
    </div>

    <div class="filter-bar">
      <el-radio-group v-model="filterStatus" size="default">
        <el-radio-button value="">{{ t('all') }}</el-radio-button>
        <el-radio-button value="upcoming">{{ t('upcoming') }}</el-radio-button>
        <el-radio-button value="ongoing">{{ t('ongoing') }}</el-radio-button>
        <el-radio-button value="ended">{{ t('ended') }}</el-radio-button>
      </el-radio-group>
    </div>

    <div class="tournaments-grid">
      <div v-for="t in filteredTournaments" :key="t.id" class="tournament-card" :class="t.status">
        <div class="card-header">
          <div class="trophy-icon">
            <el-icon><Trophy /></el-icon>
          </div>
          <div class="tournament-info">
            <span class="tournament-name">{{ t.name }}</span>
            <el-tag :type="getStatusTag(t.status)" size="small">{{ t(getStatusTag(t.status)) }}</el-tag>
          </div>
        </div>
        <div class="card-meta">
          <div class="meta-item">
            <el-icon><Calendar /></el-icon>
            <span>{{ formatDate(t.startDate) }}</span>
          </div>
          <div class="meta-item">
            <el-icon><Location /></el-icon>
            <span>{{ t.location }}</span>
          </div>
        </div>
        <div class="card-desc">{{ t.description }}</div>
        <div class="card-stats">
          <div class="stat">
            <span class="stat-value">{{ t.registered }}</span>
            <span class="stat-label">{{ t('registered') }}</span>
          </div>
          <div class="stat">
            <span class="stat-value">{{ t.capacity }}</span>
            <span class="stat-label">{{ t('capacity') }}</span>
          </div>
          <div class="stat">
            <span class="stat-value">¥{{ t.entryFee }}</span>
            <span class="stat-label">{{ t('entryFee') }}</span>
          </div>
          <div class="stat">
            <span class="stat-value">¥{{ t.prize }}</span>
            <span class="stat-label">{{ t('prizePool') }}</span>
          </div>
        </div>
        <div class="card-actions">
          <el-button type="primary" size="small" @click="viewDetail(t)">{{ t('detail') }}</el-button>
          <el-button size="small" @click="registerList(t)">{{ t('participants') }}</el-button>
          <el-button type="success" size="small" @click="publish(t)" v-if="t.status === 'draft'">{{ t('publish') }}</el-button>
        </div>
      </div>
    </div>

    <el-dialog v-model="showAdd" :title="t('createTournament')" width="550px" top="5vh" append-to-body>
      <el-form :model="form" label-width="100px">
        <el-form-item :label="t('tournamentName')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item :label="t('startDate')">
              <el-date-picker v-model="form.startDate" type="date" value-format="YYYY-MM-DD" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('location')">
              <el-input v-model="form.location" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item :label="t('capacity')">
              <el-input-number v-model="form.capacity" :min="2" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('entryFee')">
              <el-input-number v-model="form.entryFee" :min="0" :precision="0" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item :label="t('prizePool')">
              <el-input-number v-model="form.prize" :min="0" :precision="0" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item :label="t('description')">
          <el-input v-model="form.description" type="textarea" :rows="3" />
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
import { Plus, Trophy, Calendar, Location } from '@element-plus/icons-vue'
import { t } from '../i18n'


const filterStatus = ref('upcoming')
const showAdd = ref(false)

const form = ref({
  name: '', startDate: '', location: '', capacity: 16, entryFee: 50, prize: 2000, description: '',
})

const tournaments = ref([
  { id: 1, name: '2026春季台球争霸赛', startDate: '2026-05-01', location: '本店大厅', description: '本店举办的首届台球争霸赛，冠军奖金2000元', status: 'upcoming', registered: 12, capacity: 16, entryFee: 50, prize: 2000 },
  { id: 2, name: '周末会员交流赛', startDate: '2026-04-19', location: '本店VIP室', description: '每周六会员内部交流赛，氛围友好', status: 'ongoing', registered: 8, capacity: 8, entryFee: 0, prize: 500 },
  { id: 3, name: '2025年度总决赛', startDate: '2025-12-31', location: '本店大厅', description: '年度总决赛，决出年度冠军', status: 'ended', registered: 32, capacity: 32, entryFee: 100, prize: 5000 },
])

const filteredTournaments = computed(() => {
  if (!filterStatus.value) return tournaments.value
  return tournaments.value.filter(t => t.status === filterStatus.value)
})

const getStatusTag = (status) => {
  const types = { upcoming: 'primary', ongoing: 'success', ended: 'info' }
  return types[status] || 'info'
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('zh-CN', { month: 'long', day: 'numeric' })
}

const submit = () => {
  if (!form.value.name) { ElMessage.warning(t('pleaseComplete')); return }
  tournaments.value.push({ id: Date.now(), status: 'draft', registered: 0, ...form.value })
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
}

const viewDetail = (t) => ElMessage.info(`${t.name}: ${t.description}`)
const registerList = (t) => ElMessage.info(`${t('participants')}: ${t.registered}/${t.capacity}`)
const publish = (t) => { t.status = 'upcoming'; ElMessage.success(t('publishSuccess')) }
</script>

<style scoped>
.tournaments-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.filter-bar { margin-bottom: 24px; }

.tournaments-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.tournament-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.tournament-card.upcoming { border-left: 4px solid var(--accent-primary); }
.tournament-card.ongoing { border-left: 4px solid var(--accent-success); }
.tournament-card.ended { opacity: 0.7; }

.card-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 12px;
}

.trophy-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, #ffd700, #e6a23c);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 24px;
}

.tournament-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.tournament-name { font-weight: 600; font-size: 16px; }

.card-meta {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 12px;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-secondary);
}

.card-desc {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 12px;
  line-height: 1.5;
}

.card-stats {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
  padding: 12px;
  background: var(--hover-bg);
  border-radius: 8px;
  margin-bottom: 12px;
}

.stat { text-align: center; }
.stat-value { display: block; font-size: 16px; font-weight: 600; }
.stat-label { font-size: 10px; color: var(--text-secondary); }

.card-actions { display: flex; gap: 8px; }
</style>
