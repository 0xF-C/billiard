#![allow(special_module_name)]
#[allow(special_module_name)]
mod lib;

use lib::*;
use log::info;

fn require_auth(token: &str) -> Result<(), String> {
    match verify_token(token) {
        Some(_) => Ok(()),
        None => Err("未登录或登录已过期".to_string()),
    }
}

fn get_db_path() -> String {
    lib::get_db_path().to_string_lossy().to_string()
}

// ---- Auth ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_login(username: String, password: String) -> LoginResponse {
    login(&username, &password)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_logout(token: String) {
    revoke_token(&token);
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_refresh_token(refresh_token: String) -> LoginResponse {
    // P3 #14 修复: Token 刷新机制
    refresh_access_token(&refresh_token).unwrap_or_else(|e| LoginResponse {
        success: false,
        token: None,
        user: None,
        message: Some(e),
    })
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_change_password(token: String, user_id: i64, old_password: String, new_password: String) -> Result<(), String> {
    require_auth(&token)?;
    change_password(user_id, &old_password, &new_password)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_setup_first_login(token: String, user_id: i64, username: String, new_password: String) -> Result<(), String> {
    require_auth(&token)?;
    setup_first_login(user_id, &username, &new_password)
}

// ---- Orders ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_tables(token: String, include_order: bool) -> Vec<Table> {
    if require_auth(&token).is_err() { return vec![]; }
    get_tables(include_order)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_open_table(token: String, req: OpenTableRequest) -> Result<Order, String> {
    require_auth(&token)?;
    open_table(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_close_order(token: String, order_id: i64, req: CloseTableRequest) -> Result<Order, String> {
    require_auth(&token)?;
    close_order(order_id, req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_active_orders(token: String) -> Vec<Order> {
    if require_auth(&token).is_err() { return vec![]; }
    get_active_orders()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_orders(token: String, status: Option<String>) -> Vec<Order> {
    if require_auth(&token).is_err() { return vec![]; }
    get_orders(status)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_order_by_table(token: String, table_id: i64) -> Result<Order, String> {
    require_auth(&token)?;
    let orders = get_active_orders();
    orders.into_iter().find(|o| o.table_id == table_id)
        .ok_or_else(|| "该桌当前无活跃订单".to_string())
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_cancel_order(token: String, order_id: i64, reason: String) -> Result<Order, String> {
    require_auth(&token)?;
    cancel_order(order_id, reason)
}

// ---- Members ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_members(token: String, search: Option<String>) -> Vec<Member> {
    if require_auth(&token).is_err() { return vec![]; }
    get_members(search)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_member(token: String, name: String, phone: String, gender: Option<String>, discount: Option<f64>) -> Result<Member, String> {
    require_auth(&token)?;
    create_member(name, phone, gender, discount)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_recharge_member(token: String, member_id: i64, amount: f64, payment_method: Option<String>) -> Result<Member, String> {
    require_auth(&token)?;
    recharge_member(member_id, amount, payment_method)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_member(token: String, id: i64, data: serde_json::Value) -> Result<Member, String> {
    require_auth(&token)?;
    update_member(id, data)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_member(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_member(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_member_balance_logs(token: String, member_id: i64) -> Vec<serde_json::Value> {
    if require_auth(&token).is_err() { return vec![]; }
    get_member_balance_logs(member_id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_member_levels(token: String) -> Vec<MemberLevel> {
    if require_auth(&token).is_err() { return vec![]; }
    get_member_levels()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_member_level(token: String, name: String, discount: f64) -> Result<MemberLevel, String> {
    require_auth(&token)?;
    update_member_level(&name, discount)
}

// ---- Dashboard & Stats ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_dashboard(token: String) -> DashboardStats {
    if require_auth(&token).is_err() { return DashboardStats { total_tables: 0, active_tables: 0, active_members: 0, today_revenue: 0.0, today_recharge: 0.0, order_count: 0, member_revenue: 0.0 }; }
    get_dashboard_stats()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_revenue_trend(token: String, days: i32) -> Vec<RevenueTrend> {
    if require_auth(&token).is_err() { return vec![]; }
    get_revenue_trend(days)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_table_usage(token: String, days: i32) -> Vec<TableUsage> {
    if require_auth(&token).is_err() { return vec![]; }
    get_table_usage(days)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_hourly_revenue(token: String, days: i32) -> Vec<HourlyRevenue> {
    if require_auth(&token).is_err() { return vec![]; }
    get_hourly_revenue(days)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_member_revenue(token: String, days: i32) -> Vec<MemberRevenue> {
    if require_auth(&token).is_err() { return vec![]; }
    get_member_revenue(days)
}

// ---- Areas ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_areas(token: String) -> Vec<Area> {
    if require_auth(&token).is_err() { return vec![]; }
    get_areas()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_area(token: String, name: String, rate_per_hour: f64) -> Result<Area, String> {
    require_auth(&token)?;
    create_area(name, rate_per_hour)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_area(token: String, id: i64, name: String, rate_per_hour: f64) -> Result<Area, String> {
    require_auth(&token)?;
    update_area(id, name, rate_per_hour)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_area(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_area(id)
}

// ---- Tables ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_create_table(token: String, name: String, table_type: String, rate_per_hour: f64, is_private: bool, area_id: Option<i64>) -> Result<Table, String> {
    require_auth(&token)?;
    create_table(name, table_type, rate_per_hour, is_private, area_id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_table(token: String, id: i64, data: serde_json::Value) -> Result<Table, String> {
    require_auth(&token)?;
    update_table(id, data)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_table(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_table(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_table_categories(token: String) -> Vec<serde_json::Value> {
    if require_auth(&token).is_err() { return vec![]; }
    get_table_categories()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_table_category(token: String, name: String, description: Option<String>) -> Result<serde_json::Value, String> {
    require_auth(&token)?;
    create_table_category(name, description)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_table_category(token: String, id: i64, name: String, description: Option<String>) -> Result<serde_json::Value, String> {
    require_auth(&token)?;
    update_table_category(id, name, description)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_table_category(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_table_category(id)
}

use crate::lib::services::Settings;

// ---- Settings ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_settings(token: String) -> Settings {
    if require_auth(&token).is_err() { return Settings::default(); }
    get_settings()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_save_settings(token: String, settings: Settings) -> Result<(), String> {
    // P3 #24 修复: 使用 Settings 结构体类型
    require_auth(&token)?;
    save_settings(settings)
}

// ---- Users ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_create_user(token: String, username: String, password: String, role: String) -> Result<User, String> {
    require_auth(&token)?;
    create_user(username, password, role)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_users(token: String) -> Vec<User> {
    if require_auth(&token).is_err() { return vec![]; }
    get_users()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_user(token: String, id: i64, role: String) -> Result<User, String> {
    require_auth(&token)?;
    update_user(id, role)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_user(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_user(id)
}

// ---- Pending Orders ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_pending_orders(token: String) -> Vec<PendingOrder> {
    if require_auth(&token).is_err() { return vec![]; }
    get_pending_orders()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_pending_order(token: String, data: CreatePendingOrderRequest) -> Result<PendingOrder, String> {
    require_auth(&token)?;
    create_pending_order(data)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_cancel_pending_order(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    cancel_pending_order(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_resume_pending_order(token: String, id: i64) -> Result<PendingOrder, String> {
    require_auth(&token)?;
    resume_pending_order(id)
}

// ---- Shift ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_current_shift(token: String) -> Option<ShiftRecord> {
    if require_auth(&token).is_err() { return None; }
    get_current_shift()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_shift_records(token: String) -> Vec<ShiftRecord> {
    if require_auth(&token).is_err() { return vec![]; }
    get_shift_records()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_shift_stats(token: String) -> ShiftStats {
    if require_auth(&token).is_err() { return ShiftStats { revenue: 0.0, order_count: 0, completed_count: 0, pending_count: 0, cash_amount: 0.0, wechat_amount: 0.0, alipay_amount: 0.0, member_amount: 0.0 }; }
    get_shift_stats()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_shift_record(token: String, data: CreateShiftRecordRequest) -> Result<ShiftRecord, String> {
    require_auth(&token)?;
    create_shift_record(data)
}

// ---- Products ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_products(token: String, category_id: Option<i64>, search: Option<String>) -> Vec<Product> {
    if require_auth(&token).is_err() { return vec![]; }
    get_products(category_id, search)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_product(token: String, req: CreateProductRequest) -> Result<Product, String> {
    require_auth(&token)?;
    create_product(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_product(token: String, id: i64, req: UpdateProductRequest) -> Result<Product, String> {
    require_auth(&token)?;
    update_product(id, req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_product(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_product(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_product_categories(token: String) -> Vec<ProductCategory> {
    if require_auth(&token).is_err() { return vec![]; }
    get_product_categories()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_product_category(token: String, name: String, description: Option<String>) -> Result<ProductCategory, String> {
    require_auth(&token)?;
    create_product_category(name, description)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_product_category(token: String, id: i64, name: String, description: Option<String>) -> Result<ProductCategory, String> {
    require_auth(&token)?;
    update_product_category(id, name, description)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_product_category(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_product_category(id)
}

// ---- Inventory ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_inventory(token: String, category_id: Option<i64>, search: Option<String>) -> Vec<InventoryItem> {
    if require_auth(&token).is_err() { return vec![]; }
    get_inventory(category_id, search)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_inventory(token: String, req: CreateInventoryRequest) -> Result<InventoryItem, String> {
    require_auth(&token)?;
    create_inventory(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_inventory(token: String, id: i64, req: UpdateInventoryRequest) -> Result<InventoryItem, String> {
    require_auth(&token)?;
    update_inventory(id, req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_inventory(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_inventory(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_inventory_categories(token: String) -> Vec<serde_json::Value> {
    if require_auth(&token).is_err() { return vec![]; }
    get_inventory_categories().into_iter().map(|c| serde_json::json!({"id": c.id, "name": c.name, "description": c.description})).collect()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_inventory_category(token: String, name: String, description: Option<String>) -> Result<serde_json::Value, String> {
    require_auth(&token)?;
    create_inventory_category(name, description)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_inventory_category(token: String, id: i64, name: String, description: Option<String>) -> Result<serde_json::Value, String> {
    require_auth(&token)?;
    update_inventory_category(id, name, description)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_inventory_category(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_inventory_category(id)
}

// ---- Expenses ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_expenses(token: String, remark: Option<String>) -> Vec<Expense> {
    if require_auth(&token).is_err() { return vec![]; }
    get_expenses(remark)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_expense(token: String, req: CreateExpenseRequest) -> Result<Expense, String> {
    require_auth(&token)?;
    create_expense(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_expense(token: String, id: i64, req: CreateExpenseRequest) -> Result<Expense, String> {
    require_auth(&token)?;
    update_expense(id, req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_expense(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_expense(id)
}

// ---- Bookings ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_bookings(token: String, status: Option<String>) -> Vec<Booking> {
    if require_auth(&token).is_err() { return vec![]; }
    get_bookings(status)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_booking(token: String, req: CreateBookingRequest) -> Result<Booking, String> {
    require_auth(&token)?;
    create_booking(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_booking(token: String, id: i64, req: UpdateBookingRequest) -> Result<Booking, String> {
    require_auth(&token)?;
    update_booking(id, req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_booking(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_booking(id)
}

// ---- Sales ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_sales_records(token: String, table_id: Option<i64>, member_id: Option<i64>, days: Option<i32>) -> Vec<Sale> {
    if require_auth(&token).is_err() { return vec![]; }
    get_sales_records(table_id, member_id, days)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_sale_product(token: String, req: SaleRequest) -> Result<Sale, String> {
    require_auth(&token)?;
    sale_product(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_sale_batch(token: String, req: BatchSaleRequest) -> Result<Vec<Sale>, String> {
    require_auth(&token)?;
    sale_batch(req)
}

// ---- Packages ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_packages(token: String) -> Vec<PackageEntity> {
    if require_auth(&token).is_err() { return vec![]; }
    get_packages()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_package(token: String, req: CreatePackageRequest) -> Result<PackageEntity, String> {
    require_auth(&token)?;
    create_package(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_package(token: String, id: i64, req: CreatePackageRequest) -> Result<PackageEntity, String> {
    require_auth(&token)?;
    update_package(id, req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_package(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_package(id)
}

// ---- Auto & Realtime ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_check_expired_packages(token: String) -> Vec<ExpiredOrderInfo> {
    if require_auth(&token).is_err() { return vec![]; }
    check_expired_packages()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_auto_close_expired(token: String, order_ids: Vec<i64>) -> Vec<Result<Order, String>> {
    if require_auth(&token).is_err() { return vec![Err("未登录或登录已过期".to_string())]; }
    auto_close_expired(order_ids)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_realtime_check(token: String, minutes: i32) -> Vec<RealtimeCheckResult> {
    if require_auth(&token).is_err() { return vec![]; }
    realtime_check(minutes)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_auto_close_exhausted(token: String) -> Vec<AutoCloseResult> {
    if require_auth(&token).is_err() { return vec![]; }
    auto_close_exhausted()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_realtime_billing_status(token: String) -> Vec<RealtimeBillingStatus> {
    if require_auth(&token).is_err() { return vec![]; }
    get_realtime_billing_status()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_process_billing_ticks(token: String) -> Vec<BillingTickResult> {
    if require_auth(&token).is_err() { return vec![]; }
    process_billing_ticks()
}

// ---- Backup ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_backup_database(token: String) -> BackupResult {
    if require_auth(&token).is_err() { return BackupResult { success: false, path: String::new(), size: 0, message: Some("未授权".to_string()) }; }
    backup_database()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_restore_database(token: String, path: String) -> Result<(), String> {
    require_auth(&token)?;
    restore_database(path)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_db_path() -> String {
    get_db_path()
}

// ---- Hardware ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_relay_status(token: String) -> Vec<RelayStatus> {
    if require_auth(&token).is_err() { return vec![]; }
    get_relay_status()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_set_relay(token: String, id: i64, on: bool) -> Result<RelayStatus, String> {
    require_auth(&token)?;
    set_relay(id, on)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_test_connection(token: String) -> ConnectionTestResult {
    if require_auth(&token).is_err() { return ConnectionTestResult { success: false, latency_ms: None, message: Some("未授权".to_string()) }; }
    test_connection()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_get_hardware_status(token: String) -> Vec<HardwareStatusItem> {
    if require_auth(&token).is_err() { return vec![]; }
    get_hardware_status()
}

// ---- Printer ----
#[tauri::command(rename_all = "camelCase")]
fn cmd_get_printers(token: String) -> Vec<Printer> {
    if require_auth(&token).is_err() { return vec![]; }
    get_printers()
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_create_printer(token: String, req: CreatePrinterRequest) -> Result<Printer, String> {
    require_auth(&token)?;
    create_printer(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_update_printer(token: String, id: i64, req: CreatePrinterRequest) -> Result<Printer, String> {
    require_auth(&token)?;
    update_printer(id, req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_delete_printer(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    delete_printer(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_set_default_printer(token: String, id: i64) -> Result<(), String> {
    require_auth(&token)?;
    set_default_printer(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_toggle_printer(token: String, id: i64) -> Result<Printer, String> {
    require_auth(&token)?;
    toggle_printer(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_print_receipt(token: String, req: PrintReceiptRequest) -> PrintResult {
    if require_auth(&token).is_err() {
        return PrintResult { success: false, message: "未登录或登录已过期".to_string() };
    }
    print_receipt(req)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_test_printer(token: String, id: i64) -> PrintResult {
    if require_auth(&token).is_err() {
        return PrintResult { success: false, message: "未登录或登录已过期".to_string() };
    }
    test_printer(id)
}

#[tauri::command(rename_all = "camelCase")]
fn cmd_list_usb_printers() -> Vec<String> {
    list_usb_printers()
}

// ======== 库存出入库 ========
#[tauri::command]
fn cmd_get_stock_io_records(token: String, product_id: Option<i64>, io_type: Option<String>, days: Option<i32>) -> Vec<serde_json::Value> {
    // P2 #13 修复: 添加 require_auth 检查
    if require_auth(&token).is_err() { return vec![]; }
    get_stock_io_records(product_id, io_type, days)
}

#[tauri::command]
fn cmd_create_stock_in(token: String, req: serde_json::Value) -> Result<serde_json::Value, String> {
    // P2 #13 修复: 添加 require_auth 检查
    require_auth(&token)?;
    let operator_id = verify_token(&token);
    let product_id = req.get("product_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let quantity = req.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0);
    let reason = req.get("reason").and_then(|v| v.as_str()).map(String::from);
    let remark = req.get("remark").and_then(|v| v.as_str()).map(String::from);
    let unit_price = req.get("unit_price").and_then(|v| v.as_f64());
    create_stock_in(product_id, quantity, reason, remark, unit_price, operator_id)
}

#[tauri::command]
fn cmd_create_stock_out(token: String, req: serde_json::Value) -> Result<serde_json::Value, String> {
    // P2 #13 修复: 添加 require_auth 检查
    require_auth(&token)?;
    let operator_id = verify_token(&token);
    let product_id = req.get("product_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let quantity = req.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0);
    let reason = req.get("reason").and_then(|v| v.as_str()).map(String::from);
    let remark = req.get("remark").and_then(|v| v.as_str()).map(String::from);
    create_stock_out(product_id, quantity, reason, remark, operator_id)
}

// ======== 积分管理 ========
#[tauri::command]
fn cmd_get_points_logs(token: String, member_id: Option<i64>, days: Option<i32>) -> Vec<serde_json::Value> {
    // P2 #13 修复: 添加 require_auth 检查
    if require_auth(&token).is_err() { return vec![]; }
    get_points_logs(member_id, days)
}

#[tauri::command]
fn cmd_add_points(token: String, req: serde_json::Value) -> Result<serde_json::Value, String> {
    // P2 #13 修复: 添加 require_auth 检查
    require_auth(&token)?;
    let operator_id = verify_token(&token);
    let member_id = req.get("member_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let points = req.get("points").and_then(|v| v.as_i64()).unwrap_or(0);
    let points_type = req.get("points_type").and_then(|v| v.as_str()).map(String::from);
    let remark = req.get("remark").and_then(|v| v.as_str()).map(String::from);
    let order_id = req.get("order_id").and_then(|v| v.as_i64());
    add_points(member_id, points, points_type, remark, order_id, operator_id)
}

// ======== 优惠券管理 ========
#[tauri::command]
fn cmd_get_coupons(token: String, status: Option<String>) -> Vec<serde_json::Value> {
    // P2 #13 修复: 添加 require_auth 检查
    if require_auth(&token).is_err() { return vec![]; }
    get_coupons(status)
}

#[tauri::command]
fn cmd_create_coupon(token: String, req: serde_json::Value) -> Result<serde_json::Value, String> {
    // P2 #13 修复: 添加 require_auth 检查
    require_auth(&token)?;
    verify_token(&token);
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let coupon_type = req.get("coupon_type").and_then(|v| v.as_str()).map(String::from);
    let discount = req.get("discount").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let min_amount = req.get("min_amount").and_then(|v| v.as_f64());
    let quantity = req.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0);
    let valid_from = req.get("valid_from").and_then(|v| v.as_str()).map(String::from);
    let valid_to = req.get("valid_to").and_then(|v| v.as_str()).map(String::from);
    let status = req.get("status").and_then(|v| v.as_str()).map(String::from);
    let description = req.get("description").and_then(|v| v.as_str()).map(String::from);
    create_coupon(name, coupon_type, discount, min_amount, quantity, valid_from, valid_to, status, description)
}

#[tauri::command]
fn cmd_update_coupon(token: String, id: i64, req: serde_json::Value) -> Result<serde_json::Value, String> {
    // P2 #13 修复: 添加 require_auth 检查
    require_auth(&token)?;
    verify_token(&token);
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let coupon_type = req.get("coupon_type").and_then(|v| v.as_str()).map(String::from);
    let discount = req.get("discount").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let min_amount = req.get("min_amount").and_then(|v| v.as_f64());
    let quantity = req.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0);
    let valid_from = req.get("valid_from").and_then(|v| v.as_str()).map(String::from);
    let valid_to = req.get("valid_to").and_then(|v| v.as_str()).map(String::from);
    let status = req.get("status").and_then(|v| v.as_str()).map(String::from);
    let description = req.get("description").and_then(|v| v.as_str()).map(String::from);
    update_coupon(id, name, coupon_type, discount, min_amount, quantity, valid_from, valid_to, status, description)
}

#[tauri::command]
fn cmd_delete_coupon(token: String, id: i64) -> Result<(), String> {
    // P2 #13 修复: 添加 require_auth 检查
    require_auth(&token)?;
    delete_coupon(id)
}

#[tauri::command]
fn cmd_claim_coupon(token: String, coupon_id: i64, member_id: i64) -> Result<serde_json::Value, String> {
    // P2 #13 修复: 添加 require_auth 检查
    require_auth(&token)?;
    claim_coupon(coupon_id, member_id)
}

// ======== 短信营销 ========
#[tauri::command]
fn cmd_get_sms_templates(token: String) -> Vec<serde_json::Value> {
    // P2 #13 修复: 添加 require_auth 检查
    if require_auth(&token).is_err() { return vec![]; }
    get_sms_templates()
}

#[tauri::command]
fn cmd_create_sms_template(token: String, req: serde_json::Value) -> Result<serde_json::Value, String> {
    // P2 #13 修复: 添加 require_auth 检查
    require_auth(&token)?;
    verify_token(&token);
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let template_type = req.get("template_type").and_then(|v| v.as_str()).map(String::from);
    create_sms_template(name, content, template_type)
}

#[tauri::command]
fn cmd_delete_sms_template(token: String, id: i64) -> Result<(), String> {
    verify_token(&token);
    delete_sms_template(id)
}

fn main() {
    env_logger::init();
    info!("Starting Billiard Manager Tauri App");
    
    lib::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmd_login,
            cmd_logout,
            cmd_refresh_token,
            cmd_change_password,
            cmd_setup_first_login,
            cmd_get_tables,
            cmd_open_table,
            cmd_close_order,
            cmd_get_active_orders,
            cmd_get_orders,
            cmd_get_members,
            cmd_create_member,
            cmd_recharge_member,
            cmd_get_dashboard,
            cmd_get_areas,
            cmd_get_settings,
            cmd_save_settings,
            cmd_get_member_levels,
            cmd_update_member_level,
            cmd_get_revenue_trend,
            cmd_get_order_by_table,
            cmd_cancel_order,
            cmd_get_member_balance_logs,
            cmd_update_member,
            cmd_delete_member,
            cmd_get_table_categories,
            cmd_create_table_category,
            cmd_update_table_category,
            cmd_delete_table_category,
            cmd_update_table,
            cmd_delete_table,
            cmd_create_table,
            cmd_create_area,
            cmd_update_area,
            cmd_delete_area,
            cmd_create_user,
            cmd_get_users,
            cmd_update_user,
            cmd_delete_user,
            cmd_check_expired_packages,
            cmd_get_pending_orders,
            cmd_create_pending_order,
            cmd_cancel_pending_order,
            cmd_resume_pending_order,
            cmd_get_current_shift,
            cmd_get_shift_records,
            cmd_get_shift_stats,
            cmd_create_shift_record,
            cmd_get_products,
            cmd_create_product,
            cmd_update_product,
            cmd_delete_product,
            cmd_get_product_categories,
            cmd_create_product_category,
            cmd_update_product_category,
            cmd_delete_product_category,
            cmd_get_inventory,
            cmd_create_inventory,
            cmd_update_inventory,
            cmd_delete_inventory,
            cmd_get_inventory_categories,
            cmd_create_inventory_category,
            cmd_update_inventory_category,
            cmd_delete_inventory_category,
            cmd_get_expenses,
            cmd_create_expense,
            cmd_update_expense,
            cmd_delete_expense,
            cmd_get_bookings,
            cmd_create_booking,
            cmd_update_booking,
            cmd_delete_booking,
            cmd_get_sales_records,
            cmd_sale_product,
            cmd_sale_batch,
            cmd_get_table_usage,
            cmd_get_hourly_revenue,
            cmd_get_member_revenue,
            cmd_get_packages,
            cmd_create_package,
            cmd_update_package,
            cmd_delete_package,
            cmd_check_expired_packages,
            cmd_auto_close_expired,
            cmd_realtime_check,
            cmd_auto_close_exhausted,
            cmd_get_realtime_billing_status,
            cmd_process_billing_ticks,
            cmd_backup_database,
            cmd_restore_database,
            cmd_get_relay_status,
            cmd_set_relay,
            cmd_test_connection,
            cmd_get_hardware_status,
            cmd_get_db_path,
            cmd_get_printers,
            cmd_create_printer,
            cmd_update_printer,
            cmd_delete_printer,
            cmd_set_default_printer,
            cmd_toggle_printer,
            cmd_print_receipt,
            cmd_test_printer,
            cmd_list_usb_printers,
            cmd_get_stock_io_records,
            cmd_create_stock_in,
            cmd_create_stock_out,
            cmd_get_points_logs,
            cmd_add_points,
            cmd_get_coupons,
            cmd_create_coupon,
            cmd_update_coupon,
            cmd_delete_coupon,
            cmd_claim_coupon,
            cmd_get_sms_templates,
            cmd_create_sms_template,
            cmd_delete_sms_template,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
