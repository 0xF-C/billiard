use crate::lib::db::DB;
use crate::lib::models::*;
use crate::lib::utils::{round_to_two, today_local};
use chrono::Duration;
use rusqlite::params;
use log::{warn, error};

pub fn get_dashboard_stats() -> DashboardStats {
    let conn = DB.lock();
    let today = today_local();

    let total_tables: i32 = conn
        .query_row("SELECT COUNT(*) FROM tables WHERE is_active = 1", [], |r| r.get(0))
        .unwrap_or_else(|e| { error!("total_tables query failed: {}", e); 0 });

    let active_orders: Vec<Option<i64>> = {
        let mut stmt = match conn.prepare("SELECT member_id FROM orders WHERE status = '进行中'") {
            Ok(s) => s,
            Err(e) => { error!("active_orders prepare failed: {}", e); return DashboardStats::default(); }
        };
        let rows: Vec<Option<i64>> = match stmt.query_map([], |row| row.get(0)) {
            Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
            Err(e) => { error!("active_orders query failed: {}", e); return DashboardStats::default(); }
        };
        rows
    };
    let active_tables = active_orders.len() as i32;
    let active_members = active_orders.iter().filter(|mid| mid.is_some()).count() as i32;

    let today_orders: Vec<f64> = {
        let mut stmt = match conn.prepare("SELECT final_amount FROM orders WHERE date(end_time) = ?1 AND status = '已结账'") {
            Ok(s) => s,
            Err(e) => { error!("today_orders prepare failed: {}", e); return DashboardStats::default(); }
        };
        let rows: Vec<f64> = match stmt.query_map(params![today], |row| row.get(0)) {
            Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
            Err(e) => { error!("today_orders query failed: {}", e); return DashboardStats::default(); }
        };
        rows
    };
    let today_revenue: f64 = today_orders.iter().sum();
    let order_count = today_orders.len() as i32;

    let member_revenue: f64 = {
        match conn.prepare("SELECT COALESCE(SUM(final_amount), 0) FROM orders WHERE date(end_time) = ?1 AND status = '已结账' AND member_id IS NOT NULL") {
            Ok(mut stmt) => stmt.query_row(params![today], |r| r.get(0)).unwrap_or_else(|e| { warn!("member_revenue query failed: {}", e); 0.0 }),
            Err(e) => { warn!("member_revenue prepare failed: {}", e); 0.0 }
        }
    };

    let today_recharge: f64 = {
        match conn.prepare("SELECT COALESCE(SUM(amount), 0) FROM revenues WHERE date = ?1 AND type = 'member_recharge'") {
            Ok(mut stmt) => stmt.query_row(params![today], |r| r.get(0)).unwrap_or_else(|e| { warn!("today_recharge query failed: {}", e); 0.0 }),
            Err(e) => { warn!("today_recharge prepare failed: {}", e); 0.0 }
        }
    };

    DashboardStats {
        total_tables, active_tables, active_members,
        today_revenue: round_to_two(today_revenue),
        today_recharge: round_to_two(today_recharge),
        order_count,
        member_revenue: round_to_two(member_revenue),
    }
}

pub fn get_revenue_trend(days: i32) -> Vec<RevenueTrend> {
    let conn = DB.lock();
    let mut trend = Vec::new();
    let shanghai = chrono::Utc::now().with_timezone(&chrono_tz::Asia::Shanghai);
    let today = shanghai.date_naive();
    for i in (0..days).rev() {
        let date = today - Duration::days(i as i64);
        let date_str = date.format("%Y-%m-%d").to_string();
        let revenue: f64 = conn.query_row(
            "SELECT COALESCE(SUM(final_amount), 0) FROM orders WHERE date(end_time) = ?1 AND status = '已结账'",
            params![date_str], |r| r.get(0)).unwrap_or(0.0);
        let recharge: f64 = conn.query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM revenues WHERE date = ?1 AND type = 'member_recharge'",
            params![date_str], |r| r.get(0)).unwrap_or(0.0);
        let order_count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM orders WHERE date(end_time) = ?1 AND status = '已结账'",
            params![date_str], |r| r.get(0)).unwrap_or(0);
        trend.push(RevenueTrend {
            date: date_str,
            revenue: round_to_two(revenue),
            recharge: round_to_two(recharge),
            order_count,
        });
    }
    trend
}

pub fn get_table_usage(days: i32) -> Vec<TableUsage> {
    let conn = DB.lock();
    let shanghai = chrono::Utc::now().with_timezone(&chrono_tz::Asia::Shanghai);
    let start_date = (shanghai - Duration::days(days as i64)).format("%Y-%m-%d").to_string();
    let mut stmt = match conn.prepare(
        "SELECT t.name, COALESCE(a.name, ''), COUNT(o.id) as cnt, COALESCE(SUM(o.duration_minutes),0), COALESCE(SUM(o.final_amount),0) FROM tables t LEFT JOIN areas a ON t.area_id = a.id LEFT JOIN orders o ON o.table_id = t.id AND o.status = '已结账' AND date(o.end_time) >= ?1 WHERE t.is_active = 1 GROUP BY t.id ORDER BY cnt DESC"
    ) { Ok(s) => s, Err(_) => return vec![] };
    let rows: Vec<TableUsage> = match stmt.query_map(params![start_date], |row| {
        Ok(TableUsage {
            table_name: row.get(0)?,
            area_name: row.get(1)?,
            usage_count: row.get(2)?,
            total_hours: (row.get::<_, i64>(3).unwrap_or(0) as f64) / 60.0,
            revenue: row.get(4)?,
        })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    rows
}

pub fn get_hourly_revenue(days: i32) -> Vec<HourlyRevenue> {
    let conn = DB.lock();
    let shanghai = chrono::Utc::now().with_timezone(&chrono_tz::Asia::Shanghai);
    let start_date = (shanghai - Duration::days(days as i64)).format("%Y-%m-%d").to_string();
    let mut stmt = match conn.prepare(
        "SELECT CAST(strftime('%H', end_time) AS INTEGER) as hr, COALESCE(SUM(final_amount), 0) FROM orders WHERE status = '已结账' AND date(end_time) >= ?1 GROUP BY hr ORDER BY hr"
    ) { Ok(s) => s, Err(_) => return vec![] };
    let rows: Vec<HourlyRevenue> = match stmt.query_map(params![start_date], |row| {
        Ok(HourlyRevenue { hour: row.get(0)?, revenue: row.get(1)? })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    rows
}

pub fn get_member_revenue(days: i32) -> Vec<MemberRevenue> {
    let conn = DB.lock();
    let shanghai = chrono::Utc::now().with_timezone(&chrono_tz::Asia::Shanghai);
    let start_date = (shanghai - Duration::days(days as i64)).format("%Y-%m-%d").to_string();
    let mut stmt = match conn.prepare(
        "SELECT COALESCE(m.name, '散客'), COALESCE(m.phone, ''), COUNT(o.id), COALESCE(SUM(o.final_amount), 0) FROM orders o LEFT JOIN members m ON o.member_id = m.id WHERE o.status = '已结账' AND date(o.end_time) >= ?1 AND o.member_id IS NOT NULL GROUP BY o.member_id ORDER BY SUM(o.final_amount) DESC LIMIT 50"
    ) { Ok(s) => s, Err(_) => return vec![] };
    let rows: Vec<MemberRevenue> = match stmt.query_map(params![start_date], |row| {
        Ok(MemberRevenue {
            member_name: row.get(0)?,
            phone: row.get(1)?,
            order_count: row.get(2)?,
            total_amount: row.get(3)?,
        })
    }) { Ok(iter) => iter.filter_map(|r| r.ok()).collect(), Err(_) => vec![] };
    rows
}
