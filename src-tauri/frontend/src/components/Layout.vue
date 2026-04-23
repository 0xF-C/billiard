<template>
  <div class="layout">
    <!-- ── Sidebar ── -->
    <aside class="sidebar" :class="{ collapsed: sidebarCollapsed }">
      <!-- Logo -->
      <div class="sidebar-logo" @click="sidebarCollapsed = !sidebarCollapsed">
        <div class="logo-orb">🎱</div>
        <div class="logo-text-wrap" v-show="!sidebarCollapsed">
          <span class="logo-name">Billiard</span>
          <span class="logo-sub">Management</span>
        </div>
        <el-icon class="collapse-icon" v-show="!sidebarCollapsed">
          <ArrowLeft />
        </el-icon>
        <el-icon class="collapse-icon" v-show="sidebarCollapsed">
          <ArrowRight />
        </el-icon>
      </div>

      <!-- Nav -->
      <nav class="sidebar-nav">
        <!-- Dashboard -->
        <router-link to="/" class="nav-item" :class="{ active: route.path === '/' }">
          <span class="nav-icon"><el-icon><HomeFilled /></el-icon></span>
          <span class="nav-label" v-show="!sidebarCollapsed">{{ t('dashboard') }}</span>
          <span class="nav-active-bar" v-if="route.path === '/'"></span>
        </router-link>

        <!-- Groups -->
        <div v-for="module in navModules" :key="module.id" class="nav-group">
          <div
            class="nav-parent"
            :class="{ active: isModuleActive(module), open: openGroups[module.id] }"
            @click="toggleGroup(module.id)"
          >
            <span class="nav-icon"><el-icon><component :is="module.icon" /></el-icon></span>
            <span class="nav-label" v-show="!sidebarCollapsed">{{ t(module.titleKey) }}</span>
            <el-icon class="nav-arrow" v-if="!sidebarCollapsed">
              <ArrowRight />
            </el-icon>
          </div>

          <transition name="children-slide">
            <div class="nav-children" v-show="!sidebarCollapsed && openGroups[module.id]">
              <router-link
                v-for="sub in module.children"
                :key="sub.path"
                :to="sub.path"
                class="nav-child"
                :class="{ active: route.path === sub.path }"
              >
                <span class="child-dot"></span>
                {{ t(sub.titleKey) }}
              </router-link>
            </div>
          </transition>
        </div>

        <!-- Hardware -->
        <router-link to="/hardware" class="nav-item" :class="{ active: route.path === '/hardware' }">
          <span class="nav-icon"><el-icon><Monitor /></el-icon></span>
          <span class="nav-label" v-show="!sidebarCollapsed">{{ t('hardware') }}</span>
        </router-link>
      </nav>

      <!-- Sidebar footer -->
      <div class="sidebar-footer" v-show="!sidebarCollapsed">
        <div class="footer-user">
          <el-avatar :size="28" style="background: linear-gradient(135deg, #3b82f6, #60a5fa); font-size: 12px; flex-shrink:0">
            {{ userName?.charAt(0) || 'A' }}
          </el-avatar>
          <div class="footer-user-info">
            <span class="footer-username">{{ userName }}</span>
            <span class="footer-role">Administrator</span>
          </div>
          <el-button size="small" circle @click="logout" :icon="SwitchButton" class="footer-logout-btn" title="退出登录" />
        </div>
      </div>
    </aside>

    <!-- ── Main ── -->
    <div class="layout-main">
      <!-- Topbar -->
      <header class="topbar" data-tauri-drag-region>
        <div class="topbar-inner" data-tauri-drag-region>

          <!-- Stats -->
          <div class="topbar-stats">
            <div class="stat-chip">
              <el-icon class="chip-icon"><Grid /></el-icon>
              <span class="chip-val">{{ dashStats.totalTables }}</span>
              <span class="chip-lbl">{{ t('totalTables') }}</span>
            </div>
            <div class="stat-chip chip-red">
              <el-icon class="chip-icon"><Clock /></el-icon>
              <span class="chip-val">{{ dashStats.activeTables }}</span>
              <span class="chip-lbl">{{ t('activeTables') }}</span>
            </div>
            <div class="stat-chip chip-green">
              <el-icon class="chip-icon"><Money /></el-icon>
              <span class="chip-val">{{ dashStats.todayRevenue }}</span>
              <span class="chip-lbl">{{ t('todayRevenue') }}</span>
            </div>
            <div class="stat-chip chip-purple">
              <el-icon class="chip-icon"><User /></el-icon>
              <span class="chip-val">{{ dashStats.activeMembers }}</span>
              <span class="chip-lbl">{{ t('activeMembers') }}</span>
            </div>
          </div>

          <!-- Right controls -->
          <div class="topbar-actions">
            <!-- Hardware status -->
            <div class="hw-row" v-if="hardwareStatus.length">
              <div
                v-for="hw in hardwareStatus"
                :key="hw.device_type + hw.device_name"
                class="hw-dot"
                :class="{ offline: !hw.online }"
                :title="hw.device_name"
              >
                <el-icon v-if="hw.device_type === 'printer'"><Printer /></el-icon>
                <el-icon v-else><Monitor /></el-icon>
              </div>
            </div>

            <!-- Clock -->
            <div class="topbar-clock">
              <el-icon><Clock /></el-icon>
              <span>{{ clock }}</span>
            </div>

            <!-- Language -->
            <div class="lang-switcher">
              <button
                v-for="l in langs"
                :key="l.v"
                :class="['lang-btn', { active: curLang === l.v }]"
                @click="setLang(l.v)"
              >{{ l.v.toUpperCase() }}</button>
            </div>
          </div>
        </div>
      </header>

      <!-- Page content -->
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
  Monitor, Clock, ArrowLeft, ArrowRight, SwitchButton, Calendar,
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
    id: 'front', titleKey: 'nav_front', icon: Grid,
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
    id: 'member', titleKey: 'nav_member', icon: User,
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
    id: 'booking', titleKey: 'nav_booking', icon: Calendar,
    children: [
      { path: '/booking-online', titleKey: 'sub_online_booking' },
      { path: '/booking-kanban', titleKey: 'sub_booking_kanban' },
      { path: '/booking-audit', titleKey: 'sub_booking_audit' },
      { path: '/booking-remind', titleKey: 'sub_booking_remind' },
      { path: '/booking-stats', titleKey: 'sub_booking_stats' },
    ]
  },
  {
    id: 'finance', titleKey: 'nav_finance', icon: Money,
    children: [
      { path: '/finance-daily', titleKey: 'sub_daily_report' },
      { path: '/revenue-analysis', titleKey: 'sub_revenue_analysis' },
      { path: '/table-efficiency', titleKey: 'sub_table_efficiency' },
      { path: '/payment-methods', titleKey: 'sub_payment_methods' },
      { path: '/cost-accounting', titleKey: 'sub_cost_accounting' },
      { path: '/member-recharge', titleKey: 'sub_member_recharge' },
      { path: '/export-reports', titleKey: 'sub_export_reports' },
    ]
  },
  {
    id: 'inventory', titleKey: 'nav_inventory', icon: Box,
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
    id: 'staff', titleKey: 'nav_staff', icon: UserFilled,
    children: [
      { path: '/staff-files', titleKey: 'sub_staff_files' },
      { path: '/attendance', titleKey: 'sub_attendance' },
      { path: '/performance', titleKey: 'sub_performance' },
      { path: '/permissions', titleKey: 'sub_permissions' },
      { path: '/salary', titleKey: 'sub_salary' },
    ]
  },
  {
    id: 'marketing', titleKey: 'nav_marketing', icon: Ticket,
    children: [
      { path: '/coupons', titleKey: 'sub_coupons' },
      { path: '/promotions', titleKey: 'sub_promotions' },
      { path: '/tournaments', titleKey: 'sub_tournaments' },
      { path: '/points-mall', titleKey: 'sub_points_mall' },
      { path: '/sms-marketing', titleKey: 'sub_sms_marketing' },
    ]
  },
  {
    id: 'system', titleKey: 'nav_system', icon: Setting,
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

const isModuleActive = (module) => module.children.some(sub => route.path === sub.path)

const userName = computed(() => {
  try {
    return JSON.parse(localStorage.getItem('user') || '{}').name || 'Admin'
  } catch (e) {
    return 'Admin'
  }
})

const setLang = (l) => { setI18nLang(l); window.location.reload() }
const logout = () => { localStorage.removeItem('token'); router.push('/login') }
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
  try { hardwareStatus.value = await getHardwareStatus() }
  catch { hardwareStatus.value = [] }
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
/* ── Layout Shell ── */
.layout {
  min-height: 100vh;
  display: flex;
  background: var(--bg-primary);
}

/* ── Sidebar ── */
.sidebar {
  width: 236px;
  background: var(--gradient-sidebar);
  border-right: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  transition: width 0.25s cubic-bezier(0.4,0,0.2,1);
  overflow: hidden;
  flex-shrink: 0;
  height: 100vh;
  position: sticky;
  top: 0;
  z-index: 100;
}

.sidebar.collapsed { width: 60px; }

/* Logo */
.sidebar-logo {
  height: 60px;
  display: flex;
  align-items: center;
  padding: 0 14px;
  gap: 10px;
  cursor: pointer;
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
  user-select: none;
  transition: background var(--transition-fast);
}

.sidebar-logo:hover { background: var(--bg-hover); }
.sidebar.collapsed .sidebar-logo { justify-content: center; padding: 0; }

.logo-orb {
  font-size: 22px;
  flex-shrink: 0;
  filter: drop-shadow(0 0 8px rgba(251,191,36,0.4));
  transition: transform var(--transition-fast);
}
.sidebar-logo:hover .logo-orb { transform: scale(1.08); }

.logo-text-wrap {
  display: flex;
  flex-direction: column;
  line-height: 1.2;
}

.logo-name {
  font-size: 15px;
  font-weight: 700;
  background: var(--gradient-gold);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: 0.5px;
  white-space: nowrap;
}

.logo-sub {
  font-size: 9px;
  color: var(--text-muted);
  letter-spacing: 0.12em;
  text-transform: uppercase;
  font-weight: 500;
  white-space: nowrap;
}

.collapse-icon {
  margin-left: auto;
  color: var(--text-muted);
  font-size: 12px;
  transition: color var(--transition-fast);
}
.sidebar-logo:hover .collapse-icon { color: var(--text-tertiary); }

/* Nav */
.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 10px 8px;
  scrollbar-width: thin;
  scrollbar-color: transparent transparent;
}
.sidebar-nav:hover {
  scrollbar-color: rgba(255,255,255,0.07) transparent;
}

/* Nav item (top-level link) */
.nav-item {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 8px 10px;
  border-radius: var(--radius-md);
  color: var(--text-tertiary);
  text-decoration: none;
  font-size: 13.5px;
  font-weight: 500;
  transition: all var(--transition-fast);
  position: relative;
  white-space: nowrap;
  margin-bottom: 2px;
}

.nav-item:hover {
  color: var(--text-secondary);
  background: var(--bg-hover);
}

.nav-item.active {
  color: var(--accent-primary);
  background: var(--bg-active);
}

.sidebar.collapsed .nav-item { justify-content: center; padding: 9px 0; }

.nav-active-bar {
  position: absolute;
  right: 6px;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 16px;
  background: var(--accent-primary);
  border-radius: 2px;
  box-shadow: 0 0 8px rgba(96,165,250,0.5);
}

/* Nav group */
.nav-group { margin-bottom: 1px; }

.nav-parent {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 8px 10px;
  border-radius: var(--radius-md);
  color: var(--text-tertiary);
  font-size: 13.5px;
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
  user-select: none;
}

.nav-parent:hover {
  color: var(--text-secondary);
  background: var(--bg-hover);
}

.nav-parent.active { color: var(--text-primary); }

.nav-parent.open .nav-arrow { transform: rotate(90deg); color: var(--accent-primary); }

.nav-arrow {
  margin-left: auto;
  font-size: 11px;
  transition: transform var(--transition-normal), color var(--transition-fast);
  color: var(--text-muted);
  flex-shrink: 0;
}

.sidebar.collapsed .nav-parent { justify-content: center; padding: 9px 0; }

/* Nav icon */
.nav-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  flex-shrink: 0;
  font-size: 15px;
}

/* Nav label */
.nav-label { flex: 1; min-width: 0; white-space: nowrap; overflow: hidden; }

/* Children */
.nav-children {
  margin: 2px 0 4px 14px;
  padding-left: 10px;
  border-left: 1px solid var(--border-subtle);
  overflow: hidden;
}

.nav-child {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  text-decoration: none;
  font-size: 12.5px;
  font-weight: 400;
  transition: all var(--transition-fast);
  white-space: nowrap;
  margin-bottom: 1px;
}

.nav-child:hover {
  color: var(--text-secondary);
  background: var(--bg-hover);
}

.nav-child.active {
  color: var(--accent-primary);
  background: var(--bg-active);
  font-weight: 500;
}

.child-dot {
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: currentColor;
  flex-shrink: 0;
  opacity: 0.5;
  transition: opacity var(--transition-fast);
}
.nav-child.active .child-dot { opacity: 1; }

/* Slide transition */
.children-slide-enter-active,
.children-slide-leave-active {
  transition: opacity 0.18s ease, transform 0.18s ease;
}
.children-slide-enter-from,
.children-slide-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* Sidebar footer */
.sidebar-footer {
  border-top: 1px solid var(--border-subtle);
  padding: 10px 12px;
  flex-shrink: 0;
}

.footer-user {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: var(--radius-md);
  background: var(--bg-hover);
  border: 1px solid var(--border-subtle);
}

.footer-user-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  line-height: 1.3;
}

.footer-username {
  font-size: 12.5px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.footer-role {
  font-size: 10px;
  color: var(--text-muted);
  letter-spacing: 0.04em;
}

.footer-logout-btn {
  width: 24px !important;
  height: 24px !important;
  padding: 0 !important;
  border: 1px solid rgba(248,113,113,0.2) !important;
  background: rgba(248,113,113,0.06) !important;
  color: var(--accent-danger) !important;
  flex-shrink: 0;
}
.footer-logout-btn:hover {
  background: rgba(248,113,113,0.14) !important;
}

/* ── Layout main ── */
.layout-main {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

/* ── Topbar ── */
.topbar {
  position: sticky;
  top: 0;
  z-index: 99;
  background: rgba(8,13,20,0.9);
  backdrop-filter: blur(20px) saturate(160%);
  -webkit-backdrop-filter: blur(20px) saturate(160%);
  border-bottom: 1px solid var(--border-subtle);
  box-shadow: 0 1px 0 rgba(96,165,250,0.03);
}

.topbar-inner {
  height: 52px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  gap: 12px;
}

/* Stat chips */
.topbar-stats {
  display: flex;
  align-items: center;
  gap: 5px;
}

.stat-chip {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 4px 10px;
  border-radius: var(--radius-pill);
  background: rgba(255,255,255,0.03);
  border: 1px solid var(--border-subtle);
  font-size: 12px;
  white-space: nowrap;
  transition: border-color var(--transition-fast);
}

.stat-chip:hover { border-color: var(--border-default); }

.chip-icon { font-size: 12px; color: var(--accent-primary); }
.chip-val { font-weight: 700; color: var(--text-primary); font-size: 12px; font-variant-numeric: tabular-nums; }
.chip-lbl { color: var(--text-muted); font-size: 11px; }

.chip-red .chip-icon  { color: var(--accent-danger); }
.chip-green .chip-icon { color: var(--accent-success); }
.chip-purple .chip-icon { color: var(--accent-purple); }

/* Right side actions */
.topbar-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* Hardware dots */
.hw-row { display: flex; align-items: center; gap: 3px; }

.hw-dot {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: rgba(52,211,153,0.1);
  color: var(--accent-success);
  font-size: 12px;
  border: 1px solid rgba(52,211,153,0.15);
}

.hw-dot.offline {
  background: rgba(248,113,113,0.1);
  color: var(--accent-danger);
  border-color: rgba(248,113,113,0.15);
}

/* Clock */
.topbar-clock {
  display: flex;
  align-items: center;
  gap: 5px;
  color: var(--text-tertiary);
  font-size: 12px;
  font-family: var(--font-mono);
  padding: 5px 10px;
  background: rgba(255,255,255,0.02);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  letter-spacing: 0.05em;
}
.topbar-clock .el-icon { font-size: 11px; }

/* Language switcher */
.lang-switcher {
  display: flex;
  gap: 1px;
  background: rgba(255,255,255,0.02);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: 3px;
}

.lang-btn {
  padding: 3px 8px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-muted);
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.05em;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.lang-btn:hover { color: var(--text-tertiary); }

.lang-btn.active {
  background: var(--accent-primary);
  color: #fff;
}

/* ── Main content ── */
.main-content {
  flex: 1;
  padding: 22px 24px;
  max-width: 1640px;
  margin: 0 auto;
  width: 100%;
}

/* ── Responsive ── */
@media (max-width: 1024px) {
  .sidebar { width: 60px; }
  .sidebar .nav-label,
  .sidebar .logo-text-wrap,
  .sidebar .nav-arrow,
  .sidebar .sidebar-footer { display: none; }
  .sidebar .nav-children { display: none; }
  .sidebar .nav-parent { justify-content: center; padding: 9px 0; }
  .sidebar .nav-item { justify-content: center; padding: 9px 0; }
  .sidebar .collapse-icon { display: none; }
  .sidebar-nav { padding: 6px 4px; }
}
</style>
