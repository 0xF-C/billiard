import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";

let authToken = localStorage.getItem("token") || "";

export const setToken = (t) => {
  authToken = t;
  if (t) localStorage.setItem("token", t);
  else localStorage.removeItem("token");
};

export const getToken = () => authToken;

const handleError = (err, context = "") => {
  console.error(`[API Error] ${context}:`, err);
  if (err && typeof err === 'string' && (err.includes('未登录') || err.includes('过期'))) {
    localStorage.removeItem("token");
    authToken = "";
    window.location.href = "/login";
  }
  ElMessage.error(err.message || err.toString() || "操作失败");
  return Promise.reject(err);
};

// ======== 登录 ========
export const login = (username, password) =>
  invoke("cmd_login", { username, password })
    .then((res) => {
      if (res.success && res.token) setToken(res.token);
      return res;
    })
    .catch((err) => handleError(err, "登录"));

export const logout = () =>
  invoke("cmd_logout", { token: authToken }).catch(() => {});

export const changePassword = (userId, oldPassword, newPassword) =>
  invoke("cmd_change_password", { token: authToken, userId, oldPassword, newPassword })
    .catch((err) => handleError(err, "修改密码"));

export const setupFirstLogin = (userId, username, newPassword) =>
  invoke("cmd_setup_first_login", { token: authToken, userId, username, newPassword })
    .catch((err) => handleError(err, "首次登录设置"));

// ======== 台球桌 ========
export const getTables = (includeOrder = false) =>
  invoke("cmd_get_tables", { token: authToken, includeOrder }).catch((err) => handleError(err, "获取球桌"));

export const updateTable = (id, data) =>
  invoke("cmd_update_table", { token: authToken, id, data }).catch((err) => handleError(err, "更新球桌"));

export const deleteTable = (id) =>
  invoke("cmd_delete_table", { token: authToken, id }).catch((err) => handleError(err, "删除球桌"));

export const createTable = (name, tableType, ratePerHour, isPrivate, areaId) =>
  invoke("cmd_create_table", { token: authToken, name, tableType, ratePerHour, isPrivate, areaId })
    .catch((err) => handleError(err, "创建球桌"));

// ======== 订单 ========
export const getActiveOrders = () =>
  invoke("cmd_get_active_orders", { token: authToken }).catch((err) => handleError(err, "获取活跃订单"));

export const getOrderByTable = (tableId) =>
  invoke("cmd_get_order_by_table", { token: authToken, tableId }).catch((err) => handleError(err, "获取订单"));

export const openTable = (data) =>
  invoke("cmd_open_table", { token: authToken, req: data }).catch((err) => handleError(err, "开台"));

export const closeTable = (orderId, data) =>
  invoke("cmd_close_order", { token: authToken, orderId, req: data }).catch((err) => handleError(err, "关台"));

export const getOrders = (status) =>
  invoke("cmd_get_orders", { token: authToken, status }).catch((err) => handleError(err, "获取订单列表"));

export const cancelOrder = (orderId, reason) =>
  invoke("cmd_cancel_order", { token: authToken, orderId, reason }).catch((err) => handleError(err, "取消订单"));

// ======== 会员 ========
export const getMembers = (search) =>
  invoke("cmd_get_members", { token: authToken, search }).catch((err) => handleError(err, "获取会员"));

export const createMember = (name, phone, gender, discount) =>
  invoke("cmd_create_member", { token: authToken, name, phone, gender, discount }).catch((err) => handleError(err, "创建会员"));

export const updateMember = (id, data) =>
  invoke("cmd_update_member", { token: authToken, id, data }).catch((err) => handleError(err, "更新会员"));

export const deleteMember = (id) =>
  invoke("cmd_delete_member", { token: authToken, id }).catch((err) => handleError(err, "删除会员"));

export const rechargeMember = (memberId, amount, paymentMethod) =>
  invoke("cmd_recharge_member", { token: authToken, memberId, amount, paymentMethod })
    .catch((err) => handleError(err, "充值"));

export const getMemberBalanceLogs = (memberId) =>
  invoke("cmd_get_member_balance_logs", { token: authToken, memberId }).catch((err) => handleError(err, "获取余额日志"));

export const getMemberLevels = () =>
  invoke("cmd_get_member_levels", { token: authToken }).catch((err) => handleError(err, "获取会员等级"));

export const updateMemberLevel = (name, discount) =>
  invoke("cmd_update_member_level", { token: authToken, name, discount }).catch((err) => handleError(err, "更新会员等级"));

// ======== 统计 ========
export const getDashboard = () =>
  invoke("cmd_get_dashboard", { token: authToken }).catch((err) => handleError(err, "获取统计"));

export const getRevenueTrend = (days = 7) =>
  invoke("cmd_get_revenue_trend", { token: authToken, days }).catch((err) => handleError(err, "获取趋势"));

// ======== 区域 ========
export const getAreas = () =>
  invoke("cmd_get_areas", { token: authToken }).catch((err) => handleError(err, "获取区域"));

export const createArea = (name, ratePerHour) =>
  invoke("cmd_create_area", { token: authToken, name, ratePerHour }).catch((err) => handleError(err, "创建区域"));

export const updateArea = (id, name, ratePerHour) =>
  invoke("cmd_update_area", { token: authToken, id, name, ratePerHour }).catch((err) => handleError(err, "更新区域"));

export const deleteArea = (id) =>
  invoke("cmd_delete_area", { token: authToken, id }).catch((err) => handleError(err, "删除区域"));

// ======== 系统设置 ========
export const getSettings = () =>
  invoke("cmd_get_settings", { token: authToken }).catch((err) => handleError(err, "获取设置"));

export const saveSettings = (settings) =>
  invoke("cmd_save_settings", { token: authToken, settings }).catch((err) => handleError(err, "保存设置"));

// ======== 硬件状态 ========
export const getHardwareStatus = () =>
  invoke("cmd_get_hardware_status", { token: authToken }).catch((err) => {
    console.error("[API Error] 获取硬件状态:", err);
    return [];
  });

// ======== 用户管理 ========
export const getUsers = () =>
  invoke("cmd_get_users", { token: authToken }).catch((err) => handleError(err, "获取用户"));

export const createUser = (username, password, role) =>
  invoke("cmd_create_user", { token: authToken, username, password, role }).catch((err) => handleError(err, "创建用户"));

export const updateUser = (id, role) =>
  invoke("cmd_update_user", { token: authToken, id, role }).catch((err) => handleError(err, "更新用户"));

export const deleteUser = (id) =>
  invoke("cmd_delete_user", { token: authToken, id }).catch((err) => handleError(err, "删除用户"));

// ======== 桌台类别 ========
export const getTableCategories = () =>
  invoke("cmd_get_table_categories", { token: authToken }).catch((err) => handleError(err, "获取桌台类别"));

export const createTableCategory = (name, description) =>
  invoke("cmd_create_table_category", { token: authToken, name, description }).catch((err) => handleError(err, "创建类别"));

// ======== 产品管理 ========
export const getProducts = (params = {}) =>
  invoke("cmd_get_products", { token: authToken, categoryId: params.categoryId || null, search: params.search || null })
    .catch((err) => handleError(err, "获取产品"));
export const createProduct = (req) =>
  invoke("cmd_create_product", { token: authToken, req }).catch((err) => handleError(err, "创建产品"));
export const updateProduct = (id, req) =>
  invoke("cmd_update_product", { token: authToken, id, req }).catch((err) => handleError(err, "更新产品"));
export const deleteProduct = (id) =>
  invoke("cmd_delete_product", { token: authToken, id }).catch((err) => handleError(err, "删除产品"));
export const getProductCategories = () =>
  invoke("cmd_get_product_categories", { token: authToken }).catch((err) => handleError(err, "获取产品分类"));
export const createProductCategory = (name, description) =>
  invoke("cmd_create_product_category", { token: authToken, name, description }).catch((err) => handleError(err, "创建产品分类"));
export const updateProductCategory = (id, name, description) =>
  invoke("cmd_update_product_category", { token: authToken, id, name, description }).catch((err) => handleError(err, "更新产品分类"));
export const deleteProductCategory = (id) =>
  invoke("cmd_delete_product_category", { token: authToken, id }).catch((err) => handleError(err, "删除产品分类"));

// ======== 库存管理 ========
export const getInventory = (params = {}) =>
  invoke("cmd_get_inventory", { token: authToken, categoryId: params.categoryId || null, search: params.search || null })
    .catch((err) => handleError(err, "获取库存"));
export const createInventory = (req) =>
  invoke("cmd_create_inventory", { token: authToken, req }).catch((err) => handleError(err, "创建库存"));
export const updateInventory = (id, req) =>
  invoke("cmd_update_inventory", { token: authToken, id, req }).catch((err) => handleError(err, "更新库存"));
export const deleteInventory = (id) =>
  invoke("cmd_delete_inventory", { token: authToken, id }).catch((err) => handleError(err, "删除库存"));
export const getInventoryCategories = () =>
  invoke("cmd_get_inventory_categories", { token: authToken }).catch((err) => handleError(err, "获取库存分类"));
export const createInventoryCategory = (name, description) =>
  invoke("cmd_create_inventory_category", { token: authToken, name, description }).catch((err) => handleError(err, "创建库存分类"));
export const updateInventoryCategory = (id, name, description) =>
  invoke("cmd_update_inventory_category", { token: authToken, id, name, description }).catch((err) => handleError(err, "更新库存分类"));
export const deleteInventoryCategory = (id) =>
  invoke("cmd_delete_inventory_category", { token: authToken, id }).catch((err) => handleError(err, "删除库存分类"));

// ======== 支出管理 ========
export const getExpenses = (remark) =>
  invoke("cmd_get_expenses", { token: authToken, remark: remark || null }).catch((err) => handleError(err, "获取支出"));
export const createExpense = (req) =>
  invoke("cmd_create_expense", { token: authToken, req }).catch((err) => handleError(err, "创建支出"));
export const updateExpense = (id, req) =>
  invoke("cmd_update_expense", { token: authToken, id, req }).catch((err) => handleError(err, "更新支出"));
export const deleteExpense = (id) =>
  invoke("cmd_delete_expense", { token: authToken, id }).catch((err) => handleError(err, "删除支出"));

// ======== 销售记录 ========
export const getSalesRecords = (params = {}) =>
  invoke("cmd_get_sales_records", { token: authToken, tableId: params.tableId || null, memberId: params.memberId || null, days: params.days || null })
    .catch((err) => handleError(err, "获取销售记录"));
export const saleProduct = (req) =>
  invoke("cmd_sale_product", { token: authToken, req }).catch((err) => handleError(err, "销售商品"));
export const saleBatch = (req) =>
  invoke("cmd_sale_batch", { token: authToken, req }).catch((err) => handleError(err, "批量销售"));

// ======== 统计 ========
export const getTableUsage = (days = 30) =>
  invoke("cmd_get_table_usage", { token: authToken, days }).catch((err) => handleError(err, "获取桌台使用"));
export const getHourlyRevenue = (days = 7) =>
  invoke("cmd_get_hourly_revenue", { token: authToken, days }).catch((err) => handleError(err, "获取时段收入"));
export const getMemberRevenue = (days = 30) =>
  invoke("cmd_get_member_revenue", { token: authToken, days }).catch((err) => handleError(err, "获取会员消费"));

// ======== 套餐 ========
export const getPackages = () =>
  invoke("cmd_get_packages", { token: authToken }).catch((err) => handleError(err, "获取套餐"));
export const createPackage = (req) =>
  invoke("cmd_create_package", { token: authToken, req }).catch((err) => handleError(err, "创建套餐"));
export const updatePackage = (id, req) =>
  invoke("cmd_update_package", { token: authToken, id, req }).catch((err) => handleError(err, "更新套餐"));
export const deletePackage = (id) =>
  invoke("cmd_delete_package", { token: authToken, id }).catch((err) => handleError(err, "删除套餐"));

// ======== 桌台类别 ========
export const updateTableCategory = (id, name, description) =>
  invoke("cmd_update_table_category", { token: authToken, id, name, description }).catch((err) => handleError(err, "更新桌台类别"));
export const deleteTableCategory = (id) =>
  invoke("cmd_delete_table_category", { token: authToken, id }).catch((err) => handleError(err, "删除桌台类别"));

// ======== 自动化 ========
export const checkExpiredPackages = () =>
  invoke("cmd_check_expired_packages", { token: authToken }).catch((err) => handleError(err, "检查套餐过期"));
export const autoCloseExpired = (orderIds) =>
  invoke("cmd_auto_close_expired", { token: authToken, orderIds }).catch((err) => handleError(err, "自动结账"));
export const realtimeCheck = (minutes = 30) =>
  invoke("cmd_realtime_check", { token: authToken, minutes }).catch((err) => handleError(err, "实时检查"));

export const autoCloseExhausted = () =>
  invoke("cmd_auto_close_exhausted", { token: authToken }).catch((err) => handleError(err, "余额不足自动关台"));

export const getDbPath = () =>
  invoke("cmd_get_db_path").catch(() => "未知");
  invoke("cmd_auto_close_exhausted", { token: authToken }).catch((err) => handleError(err, "余额不足自动关台"));

// ======== 数据库 ========
export const backupDatabase = () =>
  invoke("cmd_backup_database", { token: authToken }).catch((err) => handleError(err, "备份数据库"));
export const restoreDatabase = (path) =>
  invoke("cmd_restore_database", { token: authToken, path }).catch((err) => handleError(err, "恢复数据库"));

// ======== 硬件继电器 ========
export const getRelayStatus = () =>
  invoke("cmd_get_relay_status", { token: authToken }).catch((err) => handleError(err, "获取继电器状态"));
export const setRelay = (id, on) =>
  invoke("cmd_set_relay", { token: authToken, id, on }).catch((err) => handleError(err, "控制继电器"));
export const testConnection = () =>
  invoke("cmd_test_connection", { token: authToken }).catch((err) => handleError(err, "连接测试"));

// ======== 挂单管理 ========
export const getPendingOrders = () =>
  invoke("cmd_get_pending_orders", { token: authToken }).catch((err) => handleError(err, "获取挂单"));
export const createPendingOrder = (data) =>
  invoke("cmd_create_pending_order", { token: authToken, data }).catch((err) => handleError(err, "创建挂单"));
export const cancelPendingOrder = (id) =>
  invoke("cmd_cancel_pending_order", { token: authToken, id }).catch((err) => handleError(err, "取消挂单"));
export const resumePendingOrder = (id) =>
  invoke("cmd_resume_pending_order", { token: authToken, id }).catch((err) => handleError(err, "恢复挂单"));

// ======== 交班对账 ========
export const getCurrentShift = () =>
  invoke("cmd_get_current_shift", { token: authToken }).catch((err) => handleError(err, "获取当前班次"));
export const getShiftRecords = () =>
  invoke("cmd_get_shift_records", { token: authToken }).catch((err) => handleError(err, "获取班次记录"));
export const getShiftStats = () =>
  invoke("cmd_get_shift_stats", { token: authToken }).catch((err) => handleError(err, "获取班次统计"));
export const createShiftRecord = (data) =>
  invoke("cmd_create_shift_record", { token: authToken, data }).catch((err) => handleError(err, "创建班次记录"));

// ======== 预订管理 ========
export const getBookings = (status) =>
  invoke("cmd_get_bookings", { token: authToken, status }).catch((err) => handleError(err, "获取预订"));
export const createBooking = (req) =>
  invoke("cmd_create_booking", { token: authToken, req }).catch((err) => handleError(err, "创建预订"));
export const updateBooking = (id, req) =>
  invoke("cmd_update_booking", { token: authToken, id, req }).catch((err) => handleError(err, "更新预订"));
export const deleteBooking = (id) =>
  invoke("cmd_delete_booking", { token: authToken, id }).catch((err) => handleError(err, "删除预订"));

// ======== 打印机管理 ========
export const getPrinters = () =>
  invoke("cmd_get_printers", { token: authToken }).catch((err) => handleError(err, "获取打印机列表"));

export const createPrinter = (data) =>
  invoke("cmd_create_printer", { token: authToken, req: data }).catch((err) => handleError(err, "添加打印机"));

export const updatePrinter = (id, data) =>
  invoke("cmd_update_printer", { token: authToken, id, req: data }).catch((err) => handleError(err, "更新打印机"));

export const deletePrinter = (id) =>
  invoke("cmd_delete_printer", { token: authToken, id }).catch((err) => handleError(err, "删除打印机"));

export const setDefaultPrinter = (id) =>
  invoke("cmd_set_default_printer", { token: authToken, id }).catch((err) => handleError(err, "设置默认打印机"));

export const togglePrinter = (id, enabled) =>
  invoke("cmd_toggle_printer", { token: authToken, id, enabled }).catch((err) => handleError(err, "切换打印机"));

export const printReceipt = (data) =>
  invoke("cmd_print_receipt", { token: authToken, data }).catch((err) => handleError(err, "打印小票"));

export const testPrinter = (id) =>
  invoke("cmd_test_printer", { token: authToken, id }).catch((err) => handleError(err, "测试打印机"));

export const listUsbPrinters = () =>
  invoke("cmd_list_usb_printers").catch((err) => handleError(err, "枚举USB打印机"));

// ======== 支付接口(预留) ========
export const createPaymentQr = (method, amount, description) =>
  invoke("cmd_create_payment_qr", { token: authToken, method, amount, description })
    .catch((err) => handleError(err, "创建支付二维码"));

export const queryPaymentStatus = (method, orderNo) =>
  invoke("cmd_query_payment_status", { token: authToken, method, orderNo })
    .catch((err) => handleError(err, "查询支付状态"));

export const createRefund = (method, orderId, amount, reason) =>
  invoke("cmd_create_refund", { token: authToken, method, orderId, amount, reason })
    .catch((err) => handleError(err, "申请退款"));

export const generateTableQr = (tableId) =>
  invoke("cmd_generate_table_qr", { token: authToken, tableId })
    .catch((err) => handleError(err, "生成桌台二维码"));

export const generateRechargeQr = (memberId) =>
  invoke("cmd_generate_recharge_qr", { token: authToken, memberId })
    .catch((err) => handleError(err, "生成充值二维码"));

export const sendTemplateMessage = (openid, templateId, data) =>
  invoke("cmd_send_template_message", { token: authToken, openid, templateId, data })
    .catch((err) => handleError(err, "发送模板消息"));

// ======== 默认导出 ========
export default {
  login,
  logout,
  getTables,
  updateTable,
  deleteTable,
  createTable,
  getActiveOrders,
  getOrderByTable,
  openTable,
  closeTable,
  getOrders,
  cancelOrder,
  getMembers,
  createMember,
  updateMember,
  deleteMember,
  rechargeMember,
  getMemberBalanceLogs,
  getMemberLevels,
  updateMemberLevel,
  getDashboard,
  getRevenueTrend,
  getAreas,
  createArea,
  updateArea,
  deleteArea,
  getSettings,
  saveSettings,
  getUsers,
  createUser,
  updateUser,
  deleteUser,
  getTableCategories,
  createTableCategory,
  getTableUsage,
  getHourlyRevenue,
  getMemberRevenue,
  getSalesRecords,
  getInventory,
  createInventory,
  updateInventory,
  deleteInventory,
  getInventoryCategories,
  createInventoryCategory,
  updateInventoryCategory,
  deleteInventoryCategory,
  getProducts,
  createProduct,
  updateProduct,
  deleteProduct,
  getPackages,
  createPackage,
  updatePackage,
  deletePackage,
  getExpenses,
  createExpense,
  updateExpense,
  deleteExpense,
  backupDatabase,
  restoreDatabase,
  getRelayStatus,
  testConnection,
  setRelay,
  checkExpiredPackages,
  realtimeCheck,
  autoCloseExpired,
  getPendingOrders,
  createPendingOrder,
  cancelPendingOrder,
  resumePendingOrder,
  getCurrentShift,
  getShiftRecords,
  getShiftStats,
  createShiftRecord,
  getBookings,
  createBooking,
  updateBooking,
  deleteBooking,
  changePassword,
  setupFirstLogin,
  getPrinters,
  createPrinter,
  updatePrinter,
  deletePrinter,
  setDefaultPrinter,
  togglePrinter,
  printReceipt,
  testPrinter,
  listUsbPrinters,
  createPaymentQr,
  queryPaymentStatus,
  createRefund,
  generateTableQr,
  generateRechargeQr,
  sendTemplateMessage,
};
