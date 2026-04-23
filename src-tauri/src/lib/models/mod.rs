use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Area {
    pub id: i64,
    pub name: String,
    pub rate_per_hour: f64,
    pub created_at: Option<String>,
    #[serde(default)]
    pub table_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    pub id: i64,
    pub name: String,
    pub table_type: String,
    pub status: String,
    pub rate_per_hour: f64,
    pub is_private: bool,
    pub min_hours: i32,
    pub is_active: bool,
    pub area_id: Option<i64>,
    pub current_order: Option<CurrentOrder>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentOrder {
    pub id: i64,
    pub start_time: String,
    pub member_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: i64,
    pub table_id: i64,
    pub table_name: Option<String>,
    pub member_id: Option<i64>,
    pub member_name: String,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub duration_minutes: Option<i32>,
    pub total_amount: f64,
    pub discount_amount: f64,
    pub deposit: f64,
    pub change_amount: f64,
    pub final_amount: f64,
    pub status: String,
    pub package_id: Option<i64>,
    pub package_name: Option<String>,
    pub package_price: Option<f64>,
    pub package_hours: Option<f64>,
    pub last_deduction_time: Option<String>,
    pub cancel_time: Option<String>,
    pub cancel_reason: Option<String>,
    pub deposit_refunded: f64,
    pub refund_method: Option<String>,
    pub payment_method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Member {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub gender: String,
    pub birthday: Option<String>,
    pub id_card: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub remark: Option<String>,
    pub balance: f64,
    pub discount: f64,
    pub level: String,
    pub total_spent: f64,
    pub visit_count: i32,
    pub last_visit: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberLevel {
    pub id: i64,
    pub name: String,
    pub discount: f64,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub role: String,
    pub created_at: Option<String>,
    #[serde(default)]
    pub first_login: bool,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>,
    pub user: Option<User>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DashboardStats {
    pub total_tables: i32,
    pub active_tables: i32,
    pub active_members: i32,
    pub today_revenue: f64,
    pub today_recharge: f64,
    pub order_count: i32,
    pub member_revenue: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenTableRequest {
    pub table_id: i64,
    pub member_id: Option<i64>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub deposit: Option<f64>,
    pub package_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseTableRequest {
    pub payment_method: Option<String>,
    pub discount_amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevenueTrend {
    pub date: String,
    pub revenue: f64,
    pub recharge: f64,
    pub order_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingOrder {
    pub id: i64,
    pub order_no: String,
    pub table_id: i64,
    pub table_name: String,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub estimated_amount: f64,
    pub remark: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePendingOrderRequest {
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub table_id: i64,
    pub estimated_amount: f64,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShiftRecord {
    pub id: i64,
    pub shift_name: String,
    pub operator_id: i64,
    pub operator_name: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub revenue: f64,
    pub order_count: i32,
    pub status: String,
    #[serde(default)]
    pub handover_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftStats {
    pub revenue: f64,
    pub order_count: i32,
    pub completed_count: i32,
    pub pending_count: i32,
    pub cash_amount: f64,
    pub wechat_amount: f64,
    pub alipay_amount: f64,
    pub member_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShiftRecordRequest {
    pub next_operator_id: i64,
    pub handover_amount: f64,
    pub remark: Option<String>,
    pub shift_stats: ShiftStats,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductCategory {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub price: f64,
    pub cost: f64,
    pub unit: String,
    pub stock: i32,
    pub alert_stock: i32,
    pub image: Option<String>,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub category_id: Option<i64>,
    pub price: f64,
    pub cost: Option<f64>,
    pub unit: Option<String>,
    pub stock: Option<i32>,
    pub alert_stock: Option<i32>,
    pub image: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub price: Option<f64>,
    pub cost: Option<f64>,
    pub unit: Option<String>,
    pub stock: Option<i32>,
    pub alert_stock: Option<i32>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub id: i64,
    pub name: String,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub quantity: i32,
    pub price: f64,
    pub cost: f64,
    pub unit: String,
    pub supplier: Option<String>,
    pub alert_stock: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInventoryRequest {
    pub name: String,
    pub category_id: Option<i64>,
    pub quantity: i32,
    pub price: f64,
    pub cost: Option<f64>,
    pub unit: Option<String>,
    pub supplier: Option<String>,
    pub alert_stock: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInventoryRequest {
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub quantity: Option<i32>,
    pub price: Option<f64>,
    pub cost: Option<f64>,
    pub unit: Option<String>,
    pub supplier: Option<String>,
    pub alert_stock: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryCategory {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expense {
    pub id: i64,
    #[serde(rename = "type")]
    pub expense_type: String,
    pub amount: f64,
    pub date: String,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExpenseRequest {
    #[serde(rename = "type")]
    pub expense_type: String,
    pub amount: f64,
    pub date: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Booking {
    pub id: i64,
    pub customer_name: String,
    pub customer_phone: String,
    pub table_id: Option<i64>,
    pub table_name: Option<String>,
    pub booking_time: String,
    pub duration_hours: f64,
    pub status: String,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBookingRequest {
    pub customer_name: String,
    pub customer_phone: String,
    pub table_id: Option<i64>,
    pub booking_time: String,
    pub duration_hours: Option<f64>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBookingRequest {
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub table_id: Option<i64>,
    pub booking_time: Option<String>,
    pub duration_hours: Option<f64>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sale {
    pub id: i64,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_amount: f64,
    pub table_id: Option<i64>,
    pub table_name: Option<String>,
    pub member_id: Option<i64>,
    pub member_name: Option<String>,
    pub payment_method: String,
    pub remark: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleRequest {
    pub product_id: i64,
    pub quantity: i32,
    pub table_id: Option<i64>,
    pub member_id: Option<i64>,
    pub payment_method: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSaleRequest {
    pub items: Vec<SaleItem>,
    pub table_id: Option<i64>,
    pub member_id: Option<i64>,
    pub payment_method: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleItem {
    pub product_id: i64,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableUsage {
    pub table_name: String,
    pub area_name: String,
    pub usage_count: i32,
    pub total_hours: f64,
    pub revenue: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyRevenue {
    pub hour: i32,
    pub revenue: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberRevenue {
    pub member_name: String,
    pub phone: String,
    pub order_count: i32,
    pub total_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupResult {
    pub success: bool,
    pub path: String,
    pub size: i64,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelayStatus {
    pub id: i64,
    pub name: String,
    pub on: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub latency_ms: Option<u64>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareStatusItem {
    pub device_type: String,
    pub device_name: String,
    pub online: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageEntity {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub hours: f64,
    pub area_ids: String,
    pub table_ids: String,
    pub start_time: String,
    pub end_time: String,
    pub is_active: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePackageRequest {
    pub name: String,
    pub price: f64,
    pub hours: f64,
    pub area_ids: String,
    pub table_ids: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpiredOrderInfo {
    pub order_id: i64,
    pub table_id: i64,
    pub table_name: String,
    pub package_name: Option<String>,
    pub start_time: String,
    pub duration_minutes: i64,
    pub package_hours: Option<f64>,
    pub remaining_minutes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeCheckResult {
    pub order_id: i64,
    pub table_name: String,
    pub duration_minutes: i64,
    pub current_amount: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeBillingStatus {
    pub order_id: i64,
    pub table_id: i64,
    pub table_name: String,
    pub member_id: Option<i64>,
    pub member_name: String,
    pub start_time: String,
    pub duration_minutes: i64,
    pub bill_minutes: i64,
    pub current_amount: f64,
    pub hourly_rate: f64,
    pub package_name: Option<String>,
    pub package_remaining_minutes: i64,
    pub deposit: f64,
    pub member_balance: f64,
    pub available: f64,
    pub remaining_minutes: i64,
    pub remaining_warning: String,
    pub warning_level: String,
    pub discount_amount: f64,
    pub net_amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Printer {
    pub id: i64,
    pub name: String,
    pub connection_type: String,
    pub ip_address: Option<String>,
    pub port: Option<i32>,
    pub serial_port: Option<String>,
    pub baud_rate: Option<i32>,
    pub paper_width: i32,
    pub is_enabled: bool,
    pub is_default: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePrinterRequest {
    pub name: String,
    pub connection_type: String,
    pub ip_address: Option<String>,
    pub port: Option<i32>,
    pub serial_port: Option<String>,
    pub baud_rate: Option<i32>,
    pub paper_width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintReceiptRequest {
    pub printer_id: Option<i64>,
    pub shop_name: String,
    pub order_no: Option<String>,
    pub table_name: Option<String>,
    pub member_name: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub duration_minutes: Option<i32>,
    pub items: Option<Vec<ReceiptItem>>,
    pub total_amount: f64,
    pub discount_amount: Option<f64>,
    pub deposit: Option<f64>,
    pub final_amount: f64,
    pub payment_method: Option<String>,
    pub receipt_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReceiptItem {
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintResult {
    pub success: bool,
    pub message: String,
}
