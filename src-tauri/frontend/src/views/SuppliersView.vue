<template>
  <div class="suppliers-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('suppliers') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('addSupplier') }}</el-button>
    </div>

    <div class="suppliers-grid">
      <div v-for="s in suppliers" :key="s.id" class="supplier-card">
        <div class="card-header">
          <div class="supplier-avatar">{{ s.name?.charAt(0) }}</div>
          <div class="supplier-info">
            <span class="supplier-name">{{ s.name }}</span>
            <span class="supplier-category">{{ s.category }}</span>
          </div>
          <el-tag :type="s.active ? 'success' : 'info'" size="small">{{ s.active ? t('active') : t('inactive') }}</el-tag>
        </div>
        <div class="card-body">
          <div class="info-row">
            <el-icon><Phone /></el-icon>
            <span>{{ s.phone }}</span>
          </div>
          <div class="info-row">
            <el-icon><Message /></el-icon>
            <span>{{ s.contact || '-' }}</span>
          </div>
          <div class="info-row">
            <el-icon><Location /></el-icon>
            <span>{{ s.address || '-' }}</span>
          </div>
        </div>
        <div class="card-stats">
          <div class="stat">
            <span class="stat-value">{{ s.orderCount }}</span>
            <span class="stat-label">{{ t('totalOrders') }}</span>
          </div>
          <div class="stat">
            <span class="stat-value">¥{{ s.totalAmount.toLocaleString() }}</span>
            <span class="stat-label">{{ t('totalAmount') }}</span>
          </div>
        </div>
        <div class="card-actions">
          <el-button link type="primary" @click="viewDetail(s)">{{ t('detail') }}</el-button>
          <el-button link type="primary" @click="editSupplier(s)">{{ t('edit') }}</el-button>
          <el-button link type="danger" @click="deleteSupplier(s)">{{ t('delete') }}</el-button>
        </div>
      </div>
    </div>

    <!-- 添加供应商弹窗 -->
    <el-dialog v-model="showAdd" :title="t('addSupplier')" width="500px">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="t('supplierName')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="t('category')">
          <el-select v-model="form.category" :placeholder="t('selectCategory')">
            <el-option label="酒水饮料" value="酒水饮料" />
            <el-option label="器材用品" value="器材用品" />
            <el-option label="维修配件" value="维修配件" />
            <el-option label="其他" value="其他" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('phone')">
          <el-input v-model="form.phone" />
        </el-form-item>
        <el-form-item :label="t('contact')">
          <el-input v-model="form.contact" />
        </el-form-item>
        <el-form-item :label="t('address')">
          <el-input v-model="form.address" type="textarea" :rows="2" />
        </el-form-item>
        <el-form-item :label="t('remark')">
          <el-input v-model="form.remark" type="textarea" :rows="2" />
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
import { ref, reactive } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Phone, Message, Location } from '@element-plus/icons-vue'
import { t } from '../i18n'


const showAdd = ref(false)
const form = reactive({
  name: '',
  category: '',
  phone: '',
  contact: '',
  address: '',
  remark: '',
})

const suppliers = ref([
  { id: 1, name: '可乐饮料批发', category: '酒水饮料', phone: '13800138001', contact: '张经理', address: '北京市朝阳区XX路12号', active: true, orderCount: 28, totalAmount: 15600 },
  { id: 2, name: '星牌台球器材', category: '器材用品', phone: '13800138002', contact: '李经理', address: '上海市浦东新区XX街8号', active: true, orderCount: 15, totalAmount: 28000 },
  { id: 3, name: '力健体育用品', category: '器材用品', phone: '13800138003', contact: '王经理', address: '广州市天河区XX路20号', active: true, orderCount: 10, totalAmount: 18500 },
  { id: 4, name: '顺发维修服务', category: '维修配件', phone: '13800138004', contact: '赵师傅', address: '北京市海淀区XX巷5号', active: false, orderCount: 6, totalAmount: 3200 },
])

const submit = () => {
  if (!form.name) { ElMessage.warning(t('pleaseComplete')); return }
  suppliers.value.push({ id: Date.now(), ...form, active: true, orderCount: 0, totalAmount: 0 })
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
  Object.assign(form, { name: '', category: '', phone: '', contact: '', address: '', remark: '' })
}

const viewDetail = (s) => ElMessage.info(`${t('detail')}: ${s.name}`)
const editSupplier = (s) => ElMessage.info(`${t('edit')}: ${s.name}`)
const deleteSupplier = async (s) => {
  await ElMessageBox.confirm(t('confirmDelete'), t('confirm'), { type: 'warning' })
  const idx = suppliers.value.findIndex(x => x.id === s.id)
  if (idx !== -1) { suppliers.value.splice(idx, 1); ElMessage.success(t('deleteSuccess')) }
}
</script>

<style scoped>
.suppliers-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }

.suppliers-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.supplier-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.supplier-avatar {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: var(--primary-color);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 600;
}

.supplier-info { flex: 1; display: flex; flex-direction: column; }
.supplier-name { font-weight: 600; }
.supplier-category { font-size: 12px; color: var(--text-secondary); }

.card-body { margin-bottom: 16px; }
.info-row { display: flex; align-items: center; gap: 8px; font-size: 13px; color: var(--text-secondary); margin-bottom: 6px; }

.card-stats {
  display: flex;
  gap: 24px;
  padding: 12px;
  background: var(--hover-bg);
  border-radius: 8px;
  margin-bottom: 12px;
}

.stat { text-align: center; }
.stat-value { display: block; font-size: 18px; font-weight: 600; }
.stat-label { font-size: 11px; color: var(--text-secondary); }

.card-actions { display: flex; gap: 8px; }
</style>
