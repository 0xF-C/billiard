<template>
  <div class="booking-kanban-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('bookingKanban') }}</h1>
      <div class="header-actions">
        <el-date-picker
          v-model="selectedDate"
          type="date"
          value-format="YYYY-MM-DD"
          @change="loadBookings"
        />
        <el-button type="primary" :icon="Plus" @click="showAdd = true">{{ t('addBooking') }}</el-button>
      </div>
    </div>

    <div class="date-nav">
      <el-button :icon="ArrowLeft" circle @click="prevDay" />
      <span class="current-date">{{ formatDisplayDate(selectedDate) }}</span>
      <el-button :icon="ArrowRight" circle @click="nextDay" />
      <el-button @click="goToday">{{ t('today') }}</el-button>
    </div>

    <div class="kanban-container">
      <div class="time-column">
        <div class="time-header">{{ t('time') }}</div>
        <div v-for="hour in hours" :key="hour" class="time-slot">
          {{ hour.toString().padStart(2, '0') }}:00
        </div>
      </div>

      <div class="tables-container">
        <div v-for="table in tables" :key="table.id" class="table-column">
          <div class="table-header">
            <span class="table-name">{{ table.name }}</span>
            <el-tag size="small" :type="table.isPrivate ? 'warning' : 'info'">
              {{ table.isPrivate ? '包间' : '大厅' }}
            </el-tag>
          </div>
          <div class="table-slots">
            <div
              v-for="hour in hours"
              :key="hour"
              class="slot"
              :class="getSlotClass(table.id, hour)"
              @click="handleSlotClick(table.id, hour)"
            >
              <template v-if="getBooking(table.id, hour)">
                <div class="booking-info">
                  <div class="booking-name">{{ getBooking(table.id, hour).customerName }}</div>
                  <div class="booking-phone">{{ getBooking(table.id, hour).customerPhone }}</div>
                </div>
              </template>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="legend">
      <div class="legend-item">
        <span class="legend-color available"></span>
        <span>{{ t('available') }}</span>
      </div>
      <div class="legend-item">
        <span class="legend-color booked"></span>
        <span>{{ t('booked') }}</span>
      </div>
      <div class="legend-item">
        <span class="legend-color confirmed"></span>
        <span>{{ t('confirmed') }}</span>
      </div>
      <div class="legend-item">
        <span class="legend-color playing"></span>
        <span>{{ t('inUse') }}</span>
      </div>
    </div>

    <!-- Add Booking Dialog -->
    <el-dialog v-model="showAdd" :title="t('addBooking')" width="500px">
      <el-form :model="addForm" :rules="addRules" ref="addFormRef" label-width="100px">
        <el-form-item :label="t('table')" prop="tableId">
          <el-select v-model="addForm.tableId" :placeholder="t('selectTable')">
            <el-option v-for="t in tables" :key="t.id" :label="t.name" :value="t.id" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('date')">
          <el-date-picker v-model="addForm.date" type="date" value-format="YYYY-MM-DD" />
        </el-form-item>
        <el-form-item :label="t('time')" prop="startTime">
          <el-time-select
            v-model="addForm.startTime"
            :max-time="addForm.endTime"
            :placeholder="t('startTime')"
            start="08:00"
            step="01:00"
            end="23:00"
          />
          <span style="margin: 0 8px;">~</span>
          <el-time-select
            v-model="addForm.endTime"
            :min-time="addForm.startTime"
            :placeholder="t('endTime')"
            start="08:00"
            step="01:00"
            end="23:00"
          />
        </el-form-item>
        <el-form-item :label="t('customerName')" prop="customerName">
          <el-input v-model="addForm.customerName" />
        </el-form-item>
        <el-form-item :label="t('customerPhone')" prop="customerPhone">
          <el-input v-model="addForm.customerPhone" />
        </el-form-item>
        <el-form-item :label="t('deposit')">
          <el-input-number v-model="addForm.deposit" :min="0" :precision="0" />
        </el-form-item>
        <el-form-item :label="t('remark')">
          <el-input v-model="addForm.remark" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAdd = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitAdd">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- Booking Detail Dialog -->
    <el-dialog v-model="showDetail" :title="t('bookingDetail')" width="450px">
      <el-descriptions :column="1" border v-if="selectedBooking">
        <el-descriptions-item :label="t('table')">{{ selectedBooking.tableName }}</el-descriptions-item>
        <el-descriptions-item :label="t('time')">
          {{ selectedBooking.startTime }} ~ {{ selectedBooking.endTime }}
        </el-descriptions-item>
        <el-descriptions-item :label="t('customerName')">{{ selectedBooking.customerName }}</el-descriptions-item>
        <el-descriptions-item :label="t('customerPhone')">{{ selectedBooking.customerPhone }}</el-descriptions-item>
        <el-descriptions-item :label="t('deposit')">¥{{ selectedBooking.deposit || 0 }}</el-descriptions-item>
        <el-descriptions-item :label="t('status')">
          <el-tag :type="getStatusType(selectedBooking.status)">{{ t(selectedBooking.status) }}</el-tag>
        </el-descriptions-item>
        <el-descriptions-item :label="t('remark')">{{ selectedBooking.remark || '-' }}</el-descriptions-item>
        <el-descriptions-item :label="t('createdAt')">{{ formatDateTime(selectedBooking.createdAt) }}</el-descriptions-item>
      </el-descriptions>
      <template #footer>
        <el-button @click="showDetail = false">{{ t('close') }}</el-button>
        <el-button type="success" @click="confirmBooking" v-if="selectedBooking?.status === 'pending'">
          {{ t('confirmBooking') }}
        </el-button>
        <el-button type="danger" @click="cancelBooking">
          {{ t('cancelBooking') }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, ArrowLeft, ArrowRight } from '@element-plus/icons-vue'
import { getTables } from '../api'
import { t } from '../i18n'


const selectedDate = ref(new Date().toISOString().split('T')[0])
const hours = Array.from({ length: 15 }, (_, i) => i + 8) // 8:00 - 22:00

const tables = ref([])
const bookings = ref([])

const showAdd = ref(false)
const addFormRef = ref(null)
const addForm = reactive({
  tableId: null,
  date: selectedDate.value,
  startTime: '14:00',
  endTime: '16:00',
  customerName: '',
  customerPhone: '',
  deposit: 0,
  remark: '',
})
const addRules = {
  tableId: [{ required: true, message: t('required'), trigger: 'change' }],
  customerName: [{ required: true, message: t('required'), trigger: 'blur' }],
  customerPhone: [{ required: true, message: t('required'), trigger: 'blur' }],
}

const showDetail = ref(false)
const selectedBooking = ref(null)

const formatDisplayDate = (d) => {
  if (!d) return '-'
  const date = new Date(d)
  return date.toLocaleDateString('zh-CN', { month: 'long', day: 'numeric', weekday: 'short' })
}

const formatDateTime = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleString('zh-CN')
}

const prevDay = () => {
  const d = new Date(selectedDate.value)
  d.setDate(d.getDate() - 1)
  selectedDate.value = d.toISOString().split('T')[0]
  loadBookings()
}

const nextDay = () => {
  const d = new Date(selectedDate.value)
  d.setDate(d.getDate() + 1)
  selectedDate.value = d.toISOString().split('T')[0]
  loadBookings()
}

const goToday = () => {
  selectedDate.value = new Date().toISOString().split('T')[0]
  loadBookings()
}

const getBooking = (tableId, hour) => {
  return bookings.value.find(b => {
    if (b.tableId !== tableId) return false
    const startHour = parseInt(b.startTime.split(':')[0])
    const endHour = parseInt(b.endTime.split(':')[0])
    return hour >= startHour && hour < endHour
  })
}

const getSlotClass = (tableId, hour) => {
  const booking = getBooking(tableId, hour)
  if (!booking) return 'available'
  const statusMap = {
    'pending': 'booked',
    'confirmed': 'confirmed',
    'playing': 'playing',
  }
  return statusMap[booking.status] || 'booked'
}

const handleSlotClick = (tableId, hour) => {
  const booking = getBooking(tableId, hour)
  if (booking) {
    selectedBooking.value = booking
    showDetail.value = true
  } else {
    addForm.tableId = tableId
    addForm.startTime = `${hour.toString().padStart(2, '0')}:00`
    addForm.endTime = `${(hour + 2).toString().padStart(2, '0')}:00`
    showAdd.value = true
  }
}

const getStatusType = (status) => {
  const types = {
    'pending': 'warning',
    'confirmed': 'success',
    'playing': 'primary',
    'cancelled': 'info',
  }
  return types[status] || 'info'
}

const loadTables = async () => {
  const res = await getTables()
  tables.value = (res || []).filter(t => t.status !== 'maintenance')
}

const loadBookings = () => {
  // 模拟预订数据
  bookings.value = [
    { id: 1, tableId: tables.value[0]?.id, tableName: tables.value[0]?.name, customerName: '张三', customerPhone: '13800138001', startTime: '10:00', endTime: '12:00', status: 'confirmed', deposit: 100, remark: '', createdAt: new Date() },
    { id: 2, tableId: tables.value[1]?.id, tableName: tables.value[1]?.name, customerName: '李四', customerPhone: '13800138002', startTime: '14:00', endTime: '16:00', status: 'pending', deposit: 50, remark: '朋友聚会', createdAt: new Date() },
    { id: 3, tableId: tables.value[0]?.id, tableName: tables.value[0]?.name, customerName: '王五', customerPhone: '13800138003', startTime: '18:00', endTime: '20:00', status: 'playing', deposit: 0, remark: '', createdAt: new Date() },
  ].filter(b => b.tableId)
}

const submitAdd = async () => {
  await addFormRef.value?.validate()
  const table = tables.value.find(t => t.id === addForm.tableId)
  bookings.value.push({
    id: Date.now(),
    tableId: addForm.tableId,
    tableName: table?.name,
    customerName: addForm.customerName,
    customerPhone: addForm.customerPhone,
    startTime: addForm.startTime,
    endTime: addForm.endTime,
    status: 'pending',
    deposit: addForm.deposit,
    remark: addForm.remark,
    createdAt: new Date(),
  })
  ElMessage.success(t('addSuccess'))
  showAdd.value = false
  addForm.customerName = ''
  addForm.customerPhone = ''
  addForm.remark = ''
}

const confirmBooking = () => {
  if (selectedBooking.value) {
    selectedBooking.value.status = 'confirmed'
    ElMessage.success(t('confirmSuccess'))
    showDetail.value = false
  }
}

const cancelBooking = async () => {
  await ElMessageBox.confirm(t('confirmCancel'), t('confirm'), { type: 'warning' })
  const idx = bookings.value.findIndex(b => b.id === selectedBooking.value.id)
  if (idx !== -1) {
    bookings.value.splice(idx, 1)
    ElMessage.success(t('cancelSuccess'))
    showDetail.value = false
  }
}

onMounted(async () => {
  await loadTables()
  loadBookings()
})
</script>

<style scoped>
.booking-kanban-page {
  padding: 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.date-nav {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}

.current-date {
  font-size: 18px;
  font-weight: 500;
  min-width: 200px;
  text-align: center;
}

.kanban-container {
  display: flex;
  background: var(--card-bg);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  overflow: hidden;
  margin-bottom: 16px;
}

.time-column {
  width: 60px;
  flex-shrink: 0;
  border-right: 1px solid var(--border-color);
}

.time-header {
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 500;
  background: var(--hover-bg);
  border-bottom: 1px solid var(--border-color);
}

.time-slot {
  height: 50px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  color: var(--text-secondary);
  border-bottom: 1px solid var(--border-color);
}

.tables-container {
  display: flex;
  overflow-x: auto;
}

.table-column {
  min-width: 120px;
  border-right: 1px solid var(--border-color);
}

.table-header {
  height: 48px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: var(--hover-bg);
  border-bottom: 1px solid var(--border-color);
  padding: 4px;
}

.table-name {
  font-weight: 500;
  font-size: 13px;
}

.table-slots {
  position: relative;
}

.slot {
  height: 50px;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 4px;
}

.slot:hover {
  background: var(--hover-bg);
}

.slot.available {
  background: transparent;
}

.slot.booked {
  background: rgba(229, 165, 48, 0.15);
  border-left: 3px solid var(--accent-warning);
}

.slot.confirmed {
  background: rgba(63, 185, 80, 0.15);
  border-left: 3px solid var(--accent-success);
}

.slot.playing {
  background: rgba(64, 158, 255, 0.15);
  border-left: 3px solid var(--accent-primary);
}

.booking-info {
  text-align: center;
}

.booking-name {
  font-size: 12px;
  font-weight: 500;
}

.booking-phone {
  font-size: 10px;
  color: var(--text-secondary);
}

.legend {
  display: flex;
  gap: 24px;
  padding: 12px 16px;
  background: var(--card-bg);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
}

.legend-color {
  width: 16px;
  height: 16px;
  border-radius: 4px;
}

.legend-color.available {
  background: var(--card-bg);
  border: 1px solid var(--border-color);
}

.legend-color.booked {
  background: rgba(229, 165, 48, 0.3);
  border-left: 3px solid var(--accent-warning);
}

.legend-color.confirmed {
  background: rgba(63, 185, 80, 0.3);
  border-left: 3px solid var(--accent-success);
}

.legend-color.playing {
  background: rgba(64, 158, 255, 0.3);
  border-left: 3px solid var(--accent-primary);
}
</style>
