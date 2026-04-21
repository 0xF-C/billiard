<template>
  <div class="store-info-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('storeInfo') }}</h1>
      <el-button type="primary" @click="save">{{ t('save') }}</el-button>
    </div>

    <div class="info-grid">
      <div class="info-section basic">
        <h3>{{ t('basicInfo') }}</h3>
        <el-form label-width="100px">
          <el-form-item :label="t('storeName')">
            <el-input v-model="form.name" />
          </el-form-item>
          <el-form-item :label="t('storeCode')">
            <el-input v-model="form.code" />
          </el-form-item>
          <el-form-item :label="t('contact')">
            <el-input v-model="form.contact" />
          </el-form-item>
          <el-form-item :label="t('phone')">
            <el-input v-model="form.phone" />
          </el-form-item>
        </el-form>
      </div>

      <div class="info-section location">
        <h3>{{ t('locationInfo') }}</h3>
        <el-form label-width="100px">
          <el-form-item :label="t('province')">
            <el-input v-model="form.province" />
          </el-form-item>
          <el-form-item :label="t('city')">
            <el-input v-model="form.city" />
          </el-form-item>
          <el-form-item :label="t('district')">
            <el-input v-model="form.district" />
          </el-form-item>
          <el-form-item :label="t('address')">
            <el-input v-model="form.address" />
          </el-form-item>
        </el-form>
      </div>

      <div class="info-section business">
        <h3>{{ t('businessInfo') }}</h3>
        <el-form label-width="100px">
          <el-form-item :label="t('businessHours')">
            <el-time-select v-model="form.openTime" placeholder="营业开始" style="width: 45%" />
            <span style="padding: 0 8px;">-</span>
            <el-time-select v-model="form.closeTime" placeholder="营业结束" style="width: 45%" />
          </el-form-item>
          <el-form-item :label="t('tableCount')">
            <el-input-number v-model="form.tableCount" :min="1" />
          </el-form-item>
          <el-form-item :label="t('area')">
            <el-input-number v-model="form.area" :min="0" />
            <span class="form-tip">m²</span>
          </el-form-item>
          <el-form-item :label="t('parking')">
            <el-input-number v-model="form.parking" :min="0" />
            <span class="form-tip">{{ t('parkingSpaces') }}</span>
          </el-form-item>
        </el-form>
      </div>

      <div class="info-section contact">
        <h3>{{ t('contactInfo') }}</h3>
        <el-form label-width="100px">
          <el-form-item :label="t('managerName')">
            <el-input v-model="form.managerName" />
          </el-form-item>
          <el-form-item :label="t('managerPhone')">
            <el-input v-model="form.managerPhone" />
          </el-form-item>
          <el-form-item :label="t('emergencyContact')">
            <el-input v-model="form.emergencyContact" />
          </el-form-item>
          <el-form-item :label="t('wechat')">
            <el-input v-model="form.wechat" />
          </el-form-item>
        </el-form>
      </div>
    </div>

    <div class="map-section">
      <h3>{{ t('storeMap') }}</h3>
      <div class="map-placeholder">
        <el-icon :size="48"><Location /></el-icon>
        <span>{{ t('mapPlaceholder') }}</span>
        <el-button size="small" @click="openMap">{{ t('openMap') }}</el-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { Location } from '@element-plus/icons-vue'
import { t } from '../i18n'


const form = reactive({
  name: '星牌台球俱乐部',
  code: 'BILLIARD-001',
  contact: '张经理',
  phone: '010-12345678',
  province: '北京市',
  city: '北京市',
  district: '朝阳区',
  address: '建国路88号SOHO现代城A座1层',
  openTime: '09:00',
  closeTime: '23:00',
  tableCount: 12,
  area: 350,
  parking: 20,
  managerName: '张经理',
  managerPhone: '13800138001',
  emergencyContact: '13900139001',
  wechat: 'xingpai_billiard',
})

const save = () => ElMessage.success(t('saveSuccess'))
const openMap = () => ElMessage.info(t('openMap'))
</script>

<style scoped>
.store-info-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }

.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-bottom: 24px;
}

.info-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.info-section h3 { margin: 0 0 16px 0; font-size: 16px; font-weight: 600; }

.form-tip { margin-left: 8px; font-size: 12px; color: var(--text-secondary); }

.map-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.map-section h3 { margin: 0 0 16px 0; font-size: 16px; font-weight: 600; }

.map-placeholder {
  height: 200px;
  background: var(--hover-bg);
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--text-secondary);
}
</style>
