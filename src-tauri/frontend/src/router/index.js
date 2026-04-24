import { createRouter, createWebHistory } from 'vue-router'

// 定义一个简单的占位组件
const Placeholder = {
  template: `
    <div style="padding: 40px; text-align: center; color: var(--text-secondary);">
      <h2 style="margin-bottom: 16px;">{{ $route.name }} 模块建设中</h2>
      <p>正在为您接入最新的业务流程，敬请期待...</p>
    </div>
  `
}

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('../views/LoginView.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/',
    component: () => import('../components/Layout.vue'),
    meta: { requiresAuth: true },
    children: [
      { path: '', name: 'Dashboard', component: () => import('../views/DashboardView.vue') },
      
      // 前台营业
      { path: 'tables', name: 'TableManage', component: () => import('../views/TablesView.vue') },
      { path: 'table-overview', name: 'TableOverview', component: () => import('../views/TableOverviewView.vue') },
      { path: 'table-rate', name: 'TableRate', component: () => import('../views/TableRateView.vue') },
      { path: 'products', name: 'Products', component: () => import('../views/ProductsView.vue') },
      { path: 'finance', name: 'Finance', component: () => import('../views/FinanceView.vue') },
      { path: 'orders-pending', name: 'PendingOrders', component: () => import('../views/PendingOrdersView.vue') },
      { path: 'shift-change', name: 'ShiftChange', component: () => import('../views/ShiftChangeView.vue') },
      { path: 'orders', name: 'Orders', component: () => import('../views/OrdersView.vue') },
      { path: 'area-manage', name: 'AreaManage', component: () => import('../views/AreaManageView.vue') },
      { path: 'table-category', name: 'TableCategory', component: () => import('../views/TableCategoryManageView.vue') },

      // 会员管理
      { path: 'members', name: 'Members', component: () => import('../views/MembersView.vue') },
      { path: 'member-levels', name: 'MemberLevels', component: () => import('../views/MemberLevelsView.vue') },
      { path: 'member-manage', name: 'MemberManage', component: () => import('../views/MemberManagementView.vue') },
      { path: 'recharge', name: 'Recharge', component: () => import('../views/RechargeView.vue') },
      { path: 'points', name: 'PointsSystem', component: () => import('../views/PointsSystemView.vue') },
      { path: 'member-analysis', name: 'MemberAnalysis', component: () => import('../views/MemberAnalysisView.vue') },
      { path: 'blacklist', name: 'Blacklist', component: () => import('../views/BlacklistView.vue') },

      // 预订管理
      { path: 'booking', name: 'BookingManagement', component: () => import('../views/BookingManagementView.vue') },
      { path: 'booking-online', name: 'OnlineBooking', component: () => import('../views/OnlineBookingView.vue') },
      { path: 'booking-kanban', name: 'BookingKanban', component: () => import('../views/BookingKanbanView.vue') },
      { path: 'booking-audit', name: 'BookingAudit', component: () => import('../views/BookingAuditView.vue') },
      { path: 'booking-remind', name: 'BookingRemind', component: () => import('../views/BookingRemindView.vue') },
      { path: 'booking-stats', name: 'BookingStats', component: () => import('../views/BookingStatsView.vue') },

      // 财务统计
      { path: 'finance-manage', name: 'FinanceManage', component: () => import('../views/FinanceManagementView.vue') },
      { path: 'finance-daily', name: 'DailyReport', component: () => import('../views/DailyReportView.vue') },
      { path: 'revenue-analysis', name: 'RevenueAnalysis', component: () => import('../views/RevenueAnalysisView.vue') },
      { path: 'table-efficiency', name: 'TableEfficiency', component: () => import('../views/TableEfficiencyView.vue') },
      { path: 'payment-methods', name: 'PaymentMethods', component: () => import('../views/PaymentMethodsView.vue') },
      { path: 'cost-accounting', name: 'CostAccounting', component: () => import('../views/CostAccountingView.vue') },
      { path: 'member-recharge', name: 'MemberRecharge', component: () => import('../views/MemberRechargeView.vue') },
      { path: 'export-reports', name: 'ExportReports', component: () => import('../views/ExportReportsView.vue') },

      // 库存管理
      { path: 'inventory-manage', name: 'InventoryManage', component: () => import('../views/InventoryManageView.vue') },
      { path: 'inventory-system', name: 'InventorySystem', component: () => import('../views/InventorySystemView.vue') },
      { path: 'category-manage', name: 'CategoryManage', component: () => import('../views/CategoryManageView.vue') },
      { path: 'stock-io', name: 'StockIO', component: () => import('../views/StockIOView.vue') },
      { path: 'stock-alert', name: 'StockAlert', component: () => import('../views/StockAlertView.vue') },
      { path: 'suppliers', name: 'Suppliers', component: () => import('../views/SuppliersView.vue') },
      { path: 'loss-records', name: 'LossRecords', component: () => import('../views/LossRecordsView.vue') },

      // 员工管理
      { path: 'staff-manage', name: 'StaffManage', component: () => import('../views/StaffManagementView.vue') },
      { path: 'staff-files', name: 'StaffFiles', component: () => import('../views/StaffFilesView.vue') },
      { path: 'attendance', name: 'Attendance', component: () => import('../views/AttendanceView.vue') },
      { path: 'performance', name: 'Performance', component: () => import('../views/PerformanceView.vue') },
      { path: 'permissions', name: 'Permissions', component: () => import('../views/PermissionsView.vue') },
      { path: 'salary', name: 'Salary', component: () => import('../views/SalaryView.vue') },

      // 营销活动
      { path: 'marketing', name: 'MarketingManage', component: () => import('../views/MarketingManagementView.vue') },
      { path: 'coupons', name: 'Coupons', component: () => import('../views/CouponsView.vue') },
      { path: 'promotions', name: 'Promotions', component: () => import('../views/PromotionsView.vue') },
      { path: 'tournaments', name: 'Tournaments', component: () => import('../views/TournamentsView.vue') },
      { path: 'points-mall', name: 'PointsMall', component: () => import('../views/PointsMallView.vue') },
      { path: 'sms-marketing', name: 'SMSMarketing', component: () => import('../views/SMSMarketingView.vue') },

      // 系统设置
      { path: 'table-manage', name: 'TableSettings', component: () => import('../views/TableManageView.vue') },
      { path: 'payment-config', name: 'PaymentConfig', component: () => import('../views/PaymentConfigView.vue') },
      { path: 'settings', name: 'PrintSettings', component: () => import('../views/SettingsView.vue') },
      { path: 'data-security', name: 'DataSecurity', component: () => import('../views/DataSecurityView.vue') },
      { path: 'store-info', name: 'StoreInfo', component: () => import('../views/StoreInfoView.vue') },

      // 其他
      { path: 'hardware', name: 'Hardware', component: () => import('../views/HardwareView.vue') }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫
router.beforeEach((to, from) => {
  const token = localStorage.getItem('token')
  if (to.meta.requiresAuth && !token) {
    return '/login'
  } else if (to.path === '/login' && token) {
    return '/'
  }
})

export default router
