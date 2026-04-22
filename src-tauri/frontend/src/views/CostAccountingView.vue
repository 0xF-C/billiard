<template>
  <div class="cost-accounting-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('costAccounting') }}</h1>
      <div class="header-actions">
        <el-date-picker
          v-model="month"
          type="month"
          :placeholder="t('selectMonth')"
          value-format="YYYY-MM"
          @change="loadData"
        />
        <el-button type="primary" :icon="Plus" @click="showAddExpense = true">
          {{ t('addExpense') }}
        </el-button>
      </div>
    </div>

    <div class="summary-row">
      <div class="summary-card income">
        <div class="card-label">{{ t('totalIncome') }}</div>
        <div class="card-value">¥{{ summary.income.toLocaleString() }}</div>
      </div>
      <div class="summary-card expense">
        <div class="card-label">{{ t('totalExpense') }}</div>
        <div class="card-value">¥{{ summary.expense.toLocaleString() }}</div>
      </div>
      <div class="summary-card profit" :class="{ loss: summary.profit < 0 }">
        <div class="card-label">{{ t('netProfit') }}</div>
        <div class="card-value">¥{{ summary.profit.toLocaleString() }}</div>
        <div class="card-rate">{{ t('profitRate') }}: {{ summary.profitRate.toFixed(1) }}%</div>
      </div>
    </div>

    <div class="main-grid">
      <div class="expense-section">
        <div class="section-header">
          <h3>{{ t('expenseBreakdown') }}</h3>
        </div>
        <div class="expense-categories">
          <div v-for="cat in expenseCategories" :key="cat.name" class="category-row">
            <div class="category-info">
              <span class="category-icon" :style="{ background: cat.color }">
                <el-icon><component :is="cat.icon" /></el-icon>
              </span>
              <span class="category-name">{{ t(cat.name) }}</span>
            </div>
            <div class="category-bar">
              <div class="bar-fill" :style="{ width: cat.percent + '%', background: cat.color }"></div>
            </div>
            <div class="category-values">
              <span class="category-amount">¥{{ cat.amount.toLocaleString() }}</span>
              <span class="category-percent">{{ cat.percent.toFixed(1) }}%</span>
            </div>
          </div>
        </div>
      </div>

      <div class="chart-section">
        <h3>{{ t('incomeVsExpense') }}</h3>
        <div class="bar-chart">
          <div v-for="(item, idx) in monthlyData" :key="idx" class="bar-group">
            <div class="bar-item income">
              <div class="bar" :style="{ height: barHeight(item.income, maxChartValue) + '%' }"></div>
            </div>
            <div class="bar-item expense">
              <div class="bar" :style="{ height: barHeight(item.expense, maxChartValue) + '%' }"></div>
            </div>
            <span class="bar-label">{{ item.month }}</span>
          </div>
        </div>
        <div class="chart-legend">
          <span class="legend-item"><span class="dot income"></span>{{ t('income') }}</span>
          <span class="legend-item"><span class="dot expense"></span>{{ t('expense') }}</span>
        </div>
      </div>
    </div>

    <div class="detail-section">
      <div class="section-header">
        <h3>{{ t('expenseRecords') }}</h3>
        <el-select v-model="filterCategory" :placeholder="t('all')" clearable size="small">
          <el-option v-for="cat in expenseCategories" :key="cat.name" :label="t(cat.name)" :value="cat.name" />
        </el-select>
      </div>
      <el-table :data="filteredExpenses" stripe>
        <el-table-column :label="t('date')" width="120">
          <template #default="{ row }">
            {{ formatDate(row.date) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('category')" width="120">
          <template #default="{ row }">
            <el-tag size="small">{{ t(row.category) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('amount')">
          <template #default="{ row }">
            <span class="expense-amount">¥{{ row.amount.toLocaleString() }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('remark')">
          <template #default="{ row }">
            {{ row.remark || '-' }}
          </template>
        </el-table-column>
        <el-table-column :label="t('operator')" width="100">
          <template #default="{ row }">
            {{ row.operator }}
          </template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="100" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="editExpense(row)">{{ t('edit') }}</el-button>
            <el-button link type="danger" @click="deleteExpense(row)">{{ t('delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- {{ t('AddExpenseDialog') }} -->
    <el-dialog v-model="showAddExpense" :title="t('addExpense')" width="450px" top="5vh" append-to-body>
      <el-form :model="expenseForm" label-width="80px">
        <el-form-item :label="t('date')">
          <el-date-picker v-model="expenseForm.date" type="date" value-format="YYYY-MM-DD" />
        </el-form-item>
        <el-form-item :label="t('category')">
          <el-select v-model="expenseForm.category" :placeholder="t('selectCategory')">
            <el-option v-for="cat in expenseCategories" :key="cat.name" :label="t(cat.name)" :value="cat.name" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('amount')">
          <el-input-number v-model="expenseForm.amount" :min="0" :precision="2" />
        </el-form-item>
        <el-form-item :label="t('remark')">
          <el-input v-model="expenseForm.remark" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddExpense = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitExpense">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, House, User, Lightning, Coffee, ShoppingCart, MoreFilled } from '@element-plus/icons-vue'
import { getExpenses, createExpense } from '../api'
import { t } from '../i18n'


const month = ref(new Date().toISOString().slice(0, 7))
const filterCategory = ref('')
const showAddExpense = ref(false)

const expenseForm = ref({
  date: new Date().toISOString().split('T')[0],
  category: '',
  amount: 0,
  remark: '',
})

const summary = ref({
  income: 85600,
  expense: 28400,
  profit: 57200,
  profitRate: 66.8,
})

const expenseCategories = ref([
  { name: 'rent', icon: House, color: '#3b82f6', amount: 12000, percent: 42.3 },
  { name: 'salary', icon: User, color: '#10b981', amount: 8500, percent: 29.9 },
  { name: 'utilities', icon: Lightning, color: '#f59e0b', amount: 3200, percent: 11.3 },
  { name: 'supplies', icon: Coffee, color: '#8b5cf6', amount: 2800, percent: 9.9 },
  { name: 'maintenance', icon: ShoppingCart, color: '#ef4444', amount: 1200, percent: 4.2 },
  { name: 'other', icon: MoreFilled, color: '#6b7280', amount: 700, percent: 2.4 },
])

const monthlyData = ref([
  { month: '1月', income: 78000, expense: 26000 },
  { month: '2月', income: 65000, expense: 24000 },
  { month: '3月', income: 82000, expense: 28000 },
  { month: '4月', income: 85600, expense: 28400 },
])

const expenses = ref([
  { id: 1, date: '2026-04-01', category: 'rent', amount: 12000, remark: t('rentApril'), operator: '管理员' },
  { id: 2, date: '2026-04-05', category: 'salary', amount: 2500, remark: t('staffSalaryZhang'), operator: '店长' },
  { id: 3, date: '2026-04-10', category: 'utilities', amount: 800, remark: t('electricityFee'), operator: '管理员' },
  { id: 4, date: '2026-04-12', category: 'supplies', amount: 350, remark: t('consumables'), operator: '店长' },
  { id: 5, date: '2026-04-15', category: 'maintenance', amount: 200, remark: t('tableRepair'), operator: '管理员' },
])

const maxChartValue = computed(() => {
  return Math.max(...monthlyData.value.flatMap(d => [d.income, d.expense]))
})

const filteredExpenses = computed(() => {
  if (!filterCategory.value) return expenses.value
  return expenses.value.filter(e => e.category === filterCategory.value)
})

const barHeight = (value, max) => {
  if (!max) return 0
  return (value / max) * 100
}

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString(currentLang.value === 'zh' ? 'zh-CN' : (currentLang.value === 'en' ? 'en-US' : 'zh-CN'), { month: 'short', day: 'numeric' })
}

const loadData = async () => {
  await getExpenses().catch(() => null)
}

const submitExpense = async () => {
  if (!expenseForm.value.category || expenseForm.value.amount <= 0) {
    ElMessage.warning(t('pleaseComplete'))
    return
  }
  expenses.value.unshift({
    id: Date.now(),
    date: expenseForm.value.date,
    category: expenseForm.value.category,
    amount: expenseForm.value.amount,
    remark: expenseForm.value.remark,
    operator: '当前用户',
  })
  ElMessage.success(t('addSuccess'))
  showAddExpense.value = false
  expenseForm.value = { date: new Date().toISOString().split('T')[0], category: '', amount: 0, remark: '' }
}

const editExpense = (row) => {
  ElMessage.info(t('editFeatureInProgress'))
}

const deleteExpense = async (row) => {
  await ElMessageBox.confirm(t('confirmDelete'), t('confirm'), { type: 'warning' })
  const idx = expenses.value.findIndex(e => e.id === row.id)
  if (idx !== -1) {
    expenses.value.splice(idx, 1)
    ElMessage.success(t('deleteSuccess'))
  }
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
.cost-accounting-page {
  padding: 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
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

.summary-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.summary-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.summary-card.income { border-left: 4px solid var(--accent-success); }
.summary-card.expense { border-left: 4px solid var(--accent-danger); }
.summary-card.profit { border-left: 4px solid var(--accent-primary); }
.summary-card.profit.loss { border-left-color: var(--accent-danger); }

.card-label {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.card-value {
  font-size: 28px;
  font-weight: 600;
}

.card-rate {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 4px;
}

.main-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-bottom: 24px;
}

.expense-section, .chart-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h3, h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.expense-categories {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.category-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.category-info {
  width: 140px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.category-icon {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 14px;
}

.category-name {
  font-size: 13px;
}

.category-bar {
  flex: 1;
  height: 8px;
  background: var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  border-radius: 4px;
}

.category-values {
  width: 100px;
  text-align: right;
}

.category-amount {
  font-weight: 500;
  margin-right: 8px;
}

.category-percent {
  font-size: 12px;
  color: var(--text-secondary);
}

.bar-chart {
  display: flex;
  align-items: flex-end;
  gap: 16px;
  height: 160px;
  padding-top: 20px;
}

.bar-group {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.bar-item {
  width: 24px;
  height: 100px;
  display: flex;
  align-items: flex-end;
}

.bar-item.income .bar {
  background: linear-gradient(to top, #10b981, #34d399);
}

.bar-item.expense .bar {
  background: linear-gradient(to top, #ef4444, #f87171);
}

.bar-item .bar {
  width: 100%;
  border-radius: 4px 4px 0 0;
  min-height: 4px;
}

.bar-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.chart-legend {
  display: flex;
  justify-content: center;
  gap: 24px;
  margin-top: 12px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.dot.income { background: #10b981; }
.dot.expense { background: #ef4444; }

.detail-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.expense-amount {
  color: var(--accent-danger);
  font-weight: 500;
}
</style>
