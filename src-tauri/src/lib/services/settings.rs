use crate::lib::db::DB;
use rusqlite::params;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Settings {
    #[serde(rename = "businessHours")]
    pub business_hours: BusinessHours,
    #[serde(rename = "specialRates")]
    pub special_rates: SpecialRates,
    pub packages: Vec<Package>,
    #[serde(rename = "memberDay")]
    pub member_day: MemberDay,
    #[serde(rename = "autoClose")]
    pub auto_close: AutoClose,
    #[serde(rename = "autoPrintOnClose", default = "default_true")]
    pub auto_print_on_close: Option<bool>,
    #[serde(rename = "shopName", default)]
    pub shop_name: Option<String>,
    #[serde(rename = "printFooter", default)]
    pub print_footer: Option<String>,
    #[serde(rename = "billingRules", default)]
    pub billing_rules: BillingRules,
}

fn default_true() -> Option<bool> { Some(true) }

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct BillingRules {
    #[serde(rename = "freeMinutes", default = "default_free_minutes")]
    pub free_minutes: i32,
    #[serde(rename = "billingInterval", default = "default_billing_interval")]
    pub billing_interval: i32,
    #[serde(rename = "applyRounding", default = "default_true_bool")]
    pub apply_rounding: bool,
}

fn default_free_minutes() -> i32 { 5 }
fn default_billing_interval() -> i32 { 30 }
fn default_true_bool() -> bool { true }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AutoClose {
    pub enabled: bool,
}

impl Default for AutoClose {
    fn default() -> Self {
        AutoClose { enabled: false }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct BusinessHours {
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "hoursPerDay")]
    pub hours_per_day: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SpecialRates {
    #[serde(rename = "weekendDiscount")]
    pub weekend_discount: Option<WeekendDiscount>,
    #[serde(rename = "memberDay")]
    pub member_day: Option<MemberDay>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WeekendDiscount {
    pub enabled: bool,
    pub discount: i32,
    pub days: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct MemberDay {
    pub enabled: bool,
    pub dates: String,
    pub discount: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub hours: f64,
    #[serde(rename = "areaIds")]
    pub area_ids: String,
    #[serde(rename = "tableIds")]
    pub table_ids: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
}

pub fn load_settings() -> Settings {
    let conn = DB.lock();
    let settings_json: Option<String> = conn.query_row("SELECT value FROM settings WHERE key = 'system_settings'", [], |r| r.get(0)).ok();
    if let Some(json) = settings_json {
        serde_json::from_str(&json).unwrap_or_else(|_| default_settings())
    } else {
        default_settings()
    }
}

fn default_settings() -> Settings {
    Settings {
        business_hours: BusinessHours { start_time: "10:00".to_string(), end_time: "02:00".to_string(), hours_per_day: 16 },
        special_rates: SpecialRates::default(),
        packages: vec![],
        member_day: MemberDay { enabled: false, dates: "".to_string(), discount: 0 },
        auto_close: AutoClose::default(),
        auto_print_on_close: Some(true),
        shop_name: Some("台球厅".to_string()),
        print_footer: Some("欢迎下次光临".to_string()),
        billing_rules: BillingRules::default(),
    }
}

pub fn get_member_day_discount() -> i32 {
    let settings = load_settings();
    
    // Fix #9: Check weekend discount
    if let Some(ref wd) = settings.special_rates.weekend_discount {
        if wd.enabled {
            // Use strftime('%w') for day of week: 0=Sun, 1=Mon, ..., 6=Sat
            let day_of_week: String = chrono::Local::now().format("%w").to_string();
            if let Ok(dow) = day_of_week.parse::<u32>() {
                // Convert to Mon=1, ..., Sun=7 format
                let mon_based = if dow == 0 { 7 } else { dow };
                if wd.days.split(',').any(|d| {
                    d.trim().parse::<u32>().ok() == Some(mon_based)
                }) {
                    return wd.discount;
                }
            }
        }
    }
    
    // Check member day discount
    if settings.member_day.enabled {
        let today = chrono::Local::now().format("%m-%d").to_string();
        if settings.member_day.dates.split(',').any(|d| d.trim() == today) {
            return settings.member_day.discount;
        }
    }
    0
}

pub fn get_settings() -> HashMap<String, serde_json::Value> {
    let conn = DB.lock();
    let settings = HashMap::new();
    let result: Result<String, _> = conn.query_row("SELECT value FROM settings WHERE key = 'system_settings'", [], |r| r.get(0));
    if let Ok(json) = result {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json) {
            return serde_json::from_value(parsed).unwrap_or_default();
        }
    }
    settings
}

pub fn save_settings(settings: HashMap<String, serde_json::Value>) -> Result<(), String> {
    let conn = DB.lock();
    let json = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
    let existing: Option<i64> = conn.query_row("SELECT id FROM settings WHERE key = 'system_settings'", [], |r| r.get(0)).ok();
    if existing.is_some() {
        conn.execute("UPDATE settings SET value = ?1, updated_at = datetime('now') WHERE key = 'system_settings'", params![json]).map_err(|e| e.to_string())?;
    } else {
        conn.execute("INSERT INTO settings (key, value, updated_at) VALUES ('system_settings', ?1, datetime('now'))", params![json]).map_err(|e| e.to_string())?;
    }
    Ok(())
}
