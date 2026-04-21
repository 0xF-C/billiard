<template>
  <div class="layout">
    <aside class="sidebar" :class="{ collapsed: sidebarCollapsed }">
      <div class="sidebar-header" @click="sidebarCollapsed = !sidebarCollapsed">
        <span class="logo-icon">🎱</span>
        <span class="logo-text" v-show="!sidebarCollapsed">Billiard</span>
        <el-icon class="collapse-btn"><ArrowLeft v-if="!sidebarCollapsed" /><ArrowRight v-else /></el-icon>
      </div>

      <nav class="sidebar-nav">
        <router-link to="/" class="sidebar-item" :class="{ active: route.path === '/' }">
          <el-icon><HomeFilled /></el-icon>
          <span v-show="!sidebarCollapsed">{{ t('dashboard') }}</span>
        </router-link>

        <div v-for="module in navModules" :key="module.id" class="sidebar-group">
          <div
            class="sidebar-parent"
            :class="{ active: isModuleActive(module), open: openGroups[module.id] }"
            @click="toggleGroup(module.id)"
          >
            <el-icon><component :is="module.icon" /></el-icon>
            <span v-show="!sidebarCollapsed">{{ t(module.titleKey) }}</span>
            <el-icon class="group-arrow" v-if="!sidebarCollapsed">
              <ArrowRight />
            </el-icon>
          </div>
          <div class="sidebar-children" v-show="!sidebarCollapsed && openGroups[module.id]">
            <router-link
              v-for="sub in module.children"
              :key="sub.path"
              :to="sub.path"
              class="sidebar-child"
              :class="{ active: route.path === sub.path }"
            >
              {{ t(sub.titleKey) }}
            </router-link>
          </div>
        </div>

        <router-link to="/hardware" class="sidebar-item" :class="{ active: route.path === '/hardware' }">
          <el-icon><Monitor /></el-icon>
          <span v-show="!sidebarCollapsed">{{ t('hardware') }}</span>
        </router-link>
      </nav>
    </aside>

    <div class="layout-main">
      <header class="topbar" data-tauri-drag-region>
        <div class="topbar-inner" data-tauri-drag-region>
          <div class="topbar-left">
            <div class="stat-bar">
              <div class="stat-pill">
                <el-icon><Grid /></el-icon>
                <span class="stat-val">{{ dashStats.totalTables }}</span>
                <span class="stat-lbl">{{ t('totalTables') }}</span>
              </div>
              <div class="stat-pill danger">
                <el-icon><Clock /></el-icon>
                <span class="stat-val">{{ dashStats.activeTables }}</span>
                <span class="stat-lbl">{{ t('activeTables') }}</span>
              </div>
              <div class="stat-pill success">
                <el-icon><Money /></el-icon>
                <span class="stat-val">{{ dashStats.todayRevenue }}</span>
                <span class="stat-lbl">{{ t('todayRevenue') }}</span>
              </div>
              <div class="stat-pill purple">
                <el-icon><User /></el-icon>
                <span class="stat-val">{{ dashStats.activeMembers }}</span>
                <span class="stat-lbl">{{ t('activeMembers') }}</span>
              </div>
            </div>
          </div>
          <div class="topbar-right">
            <div class="hardware-status">
              <div v-for="hw in hardwareStatus" :key="hw.device_type + hw.device_name" class="hw-indicator" :class="{ offline: !hw.online }">
                <el-icon v-if="hw.device_type === 'printer'"><Printer /></el-icon>
                <el-icon v-else><Monitor /></el-icon>
              </div>
            </div>
            <div class="live-clock">
              <el-icon><Clock /></el-icon>
              {{ clock }}
            </div>
            <div class="lang-group">
              <button
                v-for="l in langs"
                :key="l.v"
                :class="['lang-btn', { active: curLang === l.v }]"
                @click="setLang(l.v)"
              >
                {{ t(l.k) }}
              </button>
            </div>
            <div class="user-badge">
              <el-avatar :size="28" style="background: var(--accent-primary); font-size: 12px;">
                {{ userName?.charAt(0) || 'A' }}
              </el-avatar>
              <span class="user-name">{{ userName }}</span>
            </div>
            <el-button size="small" @click="logout" :icon="SwitchButton" class="logout-btn">{{ t('logout') }}</el-button>
          </div>
        </div>
      </header>

      <main class="main-content">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, reactive } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElIcon, ElAvatar, ElButton } from 'element-plus'
import { 
  HomeFilled, Grid, User, ShoppingCart, Money, Setting, 
  Monitor, Clock, ArrowDown, ArrowLeft, ArrowRight, SwitchButton, Calendar, 
  Box, UserFilled, Ticket, List, Printer 
} from '@element-plus/icons-vue'
import { t, currentLang, setLang as setI18nLang } from '../i18n'
import { getDashboard, getHardwareStatus } from '../api'

const route = useRoute()
const router = useRouter()

const sidebarCollapsed = ref(false)
const openGroups = ref({})
const dashStats = reactive({ totalTables: 0, activeTables: 0, todayRevenue: '¥0', activeMembers: 0 })
const hardwareStatus = ref([])

const langs = [
  { v: 'zh', k: 'chinese' },
  { v: 'ug', k: 'uighur' },
  { v: 'en', k: 'languageEnglish' },
]

const navModules = [
  {
    id: 'front',
    titleKey: 'nav_front',
    icon: Grid,
    children: [
      { path: '/tables', titleKey: 'sub_table_manage' },
      { path: '/table-overview', titleKey: 'sub_realtime_tables' },
      { path: '/products', titleKey: 'sub_goods_consume' },
      { path: '/finance', titleKey: 'sub_checkout' },
      { path: '/orders-pending', titleKey: 'sub_pending_orders' },
      { path: '/shift-change', titleKey: 'sub_shift_change' },
    ]
  },
  {
    id: 'member',
    titleKey: 'nav_member',
    icon: User,
    children: [
      { path: '/members', titleKey: 'sub_member_files' },
      { path: '/member-levels', titleKey: 'sub_member_cards' },
      { path: '/recharge', titleKey: 'sub_recharge_center' },
      { path: '/points', titleKey: 'sub_points_system' },
      { path: '/member-analysis', titleKey: 'sub_member_analysis' },
      { path: '/blacklist', titleKey: 'sub_blacklist' },
    ]
  },
  {
    id: 'booking',
    titleKey: 'nav_booking',
    icon: Calendar,
    children: [
      { path: '/booking-online', titleKey: 'sub_online_booking' },
      { path: '/booking-kanban', titleKey: 'sub_booking_kanban' },
      { path: '/booking-audit', titleKey: 'sub_booking_audit' },
      { path: '/booking-remind', titleKey: 'sub_booking_remind' },
      { path: '/booking-stats', titleKey: 'sub_booking_stats' },
    ]
  },
  {
    id: 'finance',
    titleKey: 'nav_finance',
    icon: Money,
    children: [
      { path: '/finance-daily', titleKey: 'sub_daily_report' },
      { path: '/revenue-analysis', titleKey: 'sub_revenue_analysis' },
      { path: '/table-efficiency', titleKey: 'sub_table_efficiency' },
      { path: '/payment-methods', titleKey: 'sub_payment_methods' },
      { path: '/cost-accounting', titleKey: 'sub_cost_accounting' },
      { path: '/export-reports', titleKey: 'sub_export_reports' },
    ]
  },
  {
    id: 'inventory',
    titleKey: 'nav_inventory',
    icon: Box,
    children: [
      { path: '/inventory-manage', titleKey: 'sub_goods_manage' },
      { path: '/category-manage', titleKey: 'sub_category_manage' },
      { path: '/stock-io', titleKey: 'sub_stock_io' },
      { path: '/stock-alert', titleKey: 'sub_stock_alert' },
      { path: '/suppliers', titleKey: 'sub_suppliers' },
      { path: '/loss-records', titleKey: 'sub_loss_records' },
    ]
  },
  {
    id: 'staff',
    titleKey: 'nav_staff',
    icon: UserFilled,
    children: [
      { path: '/staff-files', titleKey: 'sub_staff_files' },
      { path: '/attendance', titleKey: 'sub_attendance' },
      { path: '/performance', titleKey: 'sub_performance' },
      { path: '/permissions', titleKey: 'sub_permissions' },
      { path: '/salary', titleKey: 'sub_salary' },
    ]
  },
  {
    id: 'marketing',
    titleKey: 'nav_marketing',
    icon: Ticket,
    children: [
      { path: '/coupons', titleKey: 'sub_coupons' },
      { path: '/promotions', titleKey: 'sub_promotions' },
      { path: '/tournaments', titleKey: 'sub_tournaments' },
      { path: '/points-mall', titleKey: 'sub_points_mall' },
      { path: '/sms-marketing', titleKey: 'sub_sms_marketing' },
    ]
  },
  {
    id: 'system',
    titleKey: 'nav_system',
    icon: Setting,
    children: [
      { path: '/area-manage', titleKey: 'areaSettings' },
      { path: '/table-manage', titleKey: 'sub_table_settings' },
      { path: '/table-category', titleKey: 'sub_table_category' },
      { path: '/table-rate', titleKey: 'sub_billing' },
      { path: '/payment-config', titleKey: 'sub_payment_config' },
      { path: '/settings', titleKey: 'sub_print_settings' },
      { path: '/data-security', titleKey: 'sub_data_security' },
      { path: '/store-info', titleKey: 'sub_store_info' },
    ]
  }
]

const curLang = computed(() => currentLang.value)
const clock = ref('')
let timer = null
let dashTimer = null

const toggleGroup = (id) => {
  openGroups.value = { ...openGroups.value, [id]: !openGroups.value[id] }
}

const isModuleActive = (module) => {
  return module.children.some(sub => route.path === sub.path)
}

const currentPageTitle = computed(() => {
  if (route.path === '/') return ''
  if (route.path === '/hardware') return t('hardware')
  for (const mod of navModules) {
    for (const sub of mod.children) {
      if (route.path === sub.path) return t(sub.titleKey)
    }
  }
  return ''
})

const userName = computed(() => {
  try {
    return JSON.parse(localStorage.getItem('user') || '{}').name || 'Admin'
  } catch (e) {
    return 'Admin'
  }
})

const setLang = (l) => {
  setI18nLang(l)
  window.location.reload()
}

const logout = () => {
  localStorage.removeItem('token')
  router.push('/login')
}

const updateClock = () => {
  const now = new Date()
  clock.value = now.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' })
}

const loadDashStats = async () => {
  try {
    const d = await getDashboard()
    dashStats.totalTables = d.total_tables || 0
    dashStats.activeTables = d.active_tables || 0
    dashStats.todayRevenue = '¥' + (d.today_revenue?.toFixed(0) || 0)
    dashStats.activeMembers = d.active_members || 0
  } catch {}
}

const loadHardwareStatus = async () => {
  try {
    hardwareStatus.value = await getHardwareStatus()
  } catch {
    hardwareStatus.value = []
  }
}

onMounted(() => {
  updateClock()
  timer = setInterval(updateClock, 1000)
  const activeMod = navModules.find(m => isModuleActive(m))
  if (activeMod) openGroups.value[activeMod.id] = true
  loadDashStats()
  loadHardwareStatus()
  dashTimer = setInterval(() => { loadDashStats(); loadHardwareStatus() }, 30000)
})
onUnmounted(() => { clearInterval(timer); clearInterval(dashTimer) })
</script>

<style scoped>
.layout {
  min-height: 100vh;
  display: flex;
  background: var(--bg-primary);
}

.sidebar {
  width: 240px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-default);
  display: flex;
  flex-direction: column;
  transition: width 0.25s ease;
  overflow: hidden;
  flex-shrink: 0;
  height: 100vh;
  position: sticky;
  top: 0;
  z-index: 100;
}

.sidebar.collapsed {
  width: 64px;
}

.sidebar-header {
  height: 64px;
  display: flex;
  align-items: center;
  padding: 0 16px;
  gap: 10px;
  cursor: pointer;
  border-bottom: 1px solid var(--border-default);
  flex-shrink: 0;
  background: var(--bg-secondary);
  user-select: none;
}

.sidebar.collapsed .sidebar-header {
  justify-content: center;
  padding: 0;
}

.logo-icon {
  font-size: 24px;
  flex-shrink: 0;
  filter: drop-shadow(0 0 6px rgba(255,215,0,0.3));
}

.logo-text {
  background: var(--gradient-gold);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  font-size: 18px;
  font-weight: 700;
  letter-spacing: 0.5px;
  white-space: nowrap;
}

.collapse-btn {
  margin-left: auto;
  color: var(--text-tertiary);
  transition: all var(--transition-fast);
}

.sidebar-header:hover .collapse-btn {
  color: var(--accent-primary);
}

.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 8px;
  scrollbar-width: thin;
  scrollbar-color: var(--bg-elevated) transparent;
}

.sidebar-nav::-webkit-scrollbar {
  width: 4px;
}

.sidebar-nav::-webkit-scrollbar-thumb {
  background: var(--bg-elevated);
  border-radius: 2px;
}

.sidebar-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-fast);
  white-space: nowrap;
  cursor: pointer;
}

.sidebar-item:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.sidebar-item.active {
  color: var(--accent-primary);
  background: var(--bg-active);
}

.sidebar.collapsed .sidebar-item {
  justify-content: center;
  padding: 10px;
}

.sidebar-group {
  margin-bottom: 2px;
}

.sidebar-parent {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
  user-select: none;
}

.sidebar-parent:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.sidebar-parent.active {
  color: var(--accent-primary);
}

.sidebar-parent.open .group-arrow {
  transform: rotate(90deg);
}

.group-arrow {
  margin-left: auto;
  font-size: 12px;
  transition: transform var(--transition-fast);
  color: var(--text-tertiary);
}

.sidebar.collapsed .sidebar-parent {
  justify-content: center;
  padding: 10px;
}

.sidebar-children {
  margin-left: 12px;
  border-left: 2px solid var(--border-default);
  padding-left: 4px;
}

.sidebar-child {
  display: block;
  padding: 7px 14px;
  border-radius: var(--radius-sm);
  color: var(--text-tertiary);
  text-decoration: none;
  font-size: 13px;
  transition: all var(--transition-fast);
  white-space: nowrap;
  cursor: pointer;
}

.sidebar-child:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.sidebar-child.active {
  color: var(--accent-primary);
  background: var(--bg-active);
  font-weight: 500;
}

.layout-main {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.topbar {
  position: sticky;
  top: 0;
  z-index: 99;
  background: rgba(17,24,32,0.85);
  backdrop-filter: blur(16px) saturate(180%);
  -webkit-backdrop-filter: blur(16px) saturate(180%);
  border-bottom: 1px solid var(--border-default);
}

.topbar-inner {
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
}

.topbar-left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  min-width: 0;
}

.stat-bar {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-left: 8px;
}

.stat-pill {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  background: rgba(88,166,255,0.1);
  font-size: 12px;
  white-space: nowrap;
}

.stat-pill .el-icon {
  font-size: 13px;
  color: var(--accent-primary);
}

.stat-pill .stat-val {
  font-weight: 700;
  color: var(--text-primary);
  font-size: 13px;
}

.stat-pill .stat-lbl {
  color: var(--text-tertiary);
  font-size: 11px;
}

.stat-pill.danger { background: rgba(248,81,73,0.1); }
.stat-pill.danger .el-icon { color: var(--accent-danger); }
.stat-pill.success { background: rgba(63,185,80,0.1); }
.stat-pill.success .el-icon { color: var(--accent-success); }
.stat-pill.purple { background: rgba(163,113,247,0.1); }
.stat-pill.purple .el-icon { color: var(--accent-purple); }

.hardware-status {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-left: 15px;
}

.hw-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border-radius: 50%;
  background: rgba(63,185,80,0.15);
  color: var(--accent-success);
  font-size: 14px;
}

.hw-indicator.offline {
  background: rgba(248,81,73,0.15);
  color: var(--accent-danger);
}

.page-breadcrumb {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.topbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.live-clock {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-tertiary);
  font-size: 13px;
  font-family: var(--font-mono);
  padding: 6px 12px;
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-muted);
}

.lang-group {
  display: flex;
  gap: 2px;
  background: var(--bg-secondary);
  padding: 4px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-muted);
}

.lang-btn {
  padding: 4px 10px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-tertiary);
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.lang-btn:hover { color: var(--text-secondary); }
.lang-btn.active {
  background: var(--accent-primary);
  color: #fff;
}

.user-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 12px 4px 4px;
  background: var(--bg-secondary);
  border-radius: 20px;
  border: 1px solid var(--border-muted);
}

.user-name {
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
}

.logout-btn {
  border: 1px solid rgba(248,81,73,0.3) !important;
  background: rgba(248,81,73,0.08) !important;
  color: var(--accent-danger) !important;
}

.main-content {
  flex: 1;
  padding: 24px;
  max-width: 1600px;
  margin: 0 auto;
  width: 100%;
}

@media (max-width: 1024px) {
  .sidebar { width: 64px; }
  .sidebar .logo-text,
  .sidebar span:not(.logo-icon) { display: none; }
  .sidebar .sidebar-children { display: none; }
  .sidebar .sidebar-parent { justify-content: center; padding: 10px; }
  .sidebar .sidebar-item { justify-content: center; padding: 10px; }
  .sidebar .collapse-btn { display: none; }
  .sidebar .group-arrow { display: none; }
  .sidebar .sidebar-nav { padding: 4px; }
  .sidebar-group { margin-bottom: 0; }
  .sidebar-children { margin-left: 0; border-left: none; padding-left: 0; }
}
</style>