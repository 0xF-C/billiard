use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::path::PathBuf;
use log::{info, warn};
use uuid::Uuid;

pub(crate) static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    match init_database() {
        Ok(conn) => Mutex::new(conn),
        Err(e) => {
            log::error!("Failed to initialize database: {}", e);
            panic!("Database initialization failed: {}", e);
        }
    }
});

pub fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("billiard-manager");
    std::fs::create_dir_all(&path).ok();
    path.push("data.db");
    path
}

pub fn init_database() -> SqliteResult<Connection> {
    let conn = Connection::open(get_db_path())?;
    
    // Enable foreign key enforcement (SQLite default is OFF)
    conn.execute_batch("PRAGMA foreign_keys = ON")?;
    
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS areas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            rate_per_hour REAL DEFAULT 30.0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS tables (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            table_type TEXT DEFAULT '普通',
            status TEXT DEFAULT '空闲',
            rate_per_hour REAL DEFAULT 30.0,
            is_private INTEGER DEFAULT 0,
            min_hours INTEGER DEFAULT 0,
            is_active INTEGER DEFAULT 1,
            area_id INTEGER,
            FOREIGN KEY (area_id) REFERENCES areas(id)
        );

        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            table_id INTEGER NOT NULL,
            member_id INTEGER,
            customer_name TEXT,
            customer_phone TEXT,
            start_time TEXT,
            end_time TEXT,
            duration_minutes INTEGER,
            total_amount REAL DEFAULT 0.0,
            discount_amount REAL DEFAULT 0.0,
            deposit REAL DEFAULT 0.0,
            change_amount REAL DEFAULT 0.0,
            final_amount REAL DEFAULT 0.0,
            status TEXT DEFAULT '进行中',
            package_id INTEGER,
            package_name TEXT,
            package_price REAL,
            package_hours REAL,
            last_deduction_time TEXT,
            cancel_time TEXT,
            cancel_reason TEXT,
            deposit_refunded REAL DEFAULT 0.0,
            refund_method TEXT,
            payment_method TEXT,
            FOREIGN KEY (table_id) REFERENCES tables(id),
            FOREIGN KEY (member_id) REFERENCES members(id)
        );

        CREATE TABLE IF NOT EXISTS members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            phone TEXT UNIQUE NOT NULL,
            gender TEXT DEFAULT '未知',
            birthday TEXT,
            id_card TEXT,
            email TEXT,
            address TEXT,
            remark TEXT,
            balance REAL DEFAULT 0.0,
            discount REAL DEFAULT 1.0,
            level TEXT DEFAULT '普通会员',
            total_spent REAL DEFAULT 0.0,
            visit_count INTEGER DEFAULT 0,
            last_visit TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS balance_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            member_id INTEGER NOT NULL,
            amount REAL DEFAULT 0.0,
            balance_before REAL DEFAULT 0.0,
            balance_after REAL DEFAULT 0.0,
            reason TEXT,
            order_id INTEGER,
            operator TEXT DEFAULT '系统',
            payment_method TEXT DEFAULT 'balance',
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (member_id) REFERENCES members(id)
        );

        CREATE TABLE IF NOT EXISTS revenues (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type TEXT,
            amount REAL DEFAULT 0.0,
            payment_method TEXT,
            table_id INTEGER,
            order_id INTEGER,
            member_id INTEGER,
            remark TEXT,
            date TEXT DEFAULT (date('now')),
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS table_categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS inventory_categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS inventory (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            category_id INTEGER,
            quantity INTEGER DEFAULT 0,
            price REAL DEFAULT 0.0,
            unit TEXT DEFAULT '个',
            supplier TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (category_id) REFERENCES inventory_categories(id)
        );

        CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type TEXT NOT NULL,
            amount REAL DEFAULT 0.0,
            date TEXT DEFAULT (date('now')),
            remark TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS sales (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            inventory_id INTEGER,
            product_name TEXT,
            quantity INTEGER DEFAULT 1,
            unit_price REAL DEFAULT 0,
            total_amount REAL DEFAULT 0,
            table_id INTEGER,
            table_name TEXT,
            member_id INTEGER,
            payment_method TEXT DEFAULT 'cash',
            remark TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS member_levels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            discount REAL DEFAULT 1.0,
            color TEXT DEFAULT 'rgba(88,166,255,0.2)',
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT,
            salt TEXT,
            bcrypt_hash TEXT,
            role TEXT DEFAULT '服务员',
            first_login INTEGER DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS user_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );

        CREATE TABLE IF NOT EXISTS permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            category TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS role_permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            role TEXT NOT NULL,
            permission TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(role, permission)
        );

        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            key TEXT UNIQUE NOT NULL,
            value TEXT,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS pending_orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_no TEXT UNIQUE NOT NULL,
            table_id INTEGER NOT NULL,
            customer_name TEXT,
            customer_phone TEXT,
            estimated_amount REAL DEFAULT 0.0,
            remark TEXT,
            status TEXT DEFAULT 'pending',
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (table_id) REFERENCES tables(id)
        );

        CREATE TABLE IF NOT EXISTS shift_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            shift_name TEXT NOT NULL,
            operator_id INTEGER NOT NULL,
            start_time TEXT DEFAULT CURRENT_TIMESTAMP,
            end_time TEXT,
            revenue REAL DEFAULT 0.0,
            order_count INTEGER DEFAULT 0,
            status TEXT DEFAULT 'active',
            handover_amount REAL DEFAULT 0.0,
            FOREIGN KEY (operator_id) REFERENCES users(id)
        );

        CREATE INDEX IF NOT EXISTS idx_order_table_status ON orders(table_id, status);
        CREATE INDEX IF NOT EXISTS idx_order_member_status ON orders(member_id, status);
        CREATE INDEX IF NOT EXISTS idx_member_phone ON members(phone);
        CREATE INDEX IF NOT EXISTS idx_balance_log_member ON balance_logs(member_id);
        CREATE INDEX IF NOT EXISTS idx_revenues_date ON revenues(date);

        CREATE TABLE IF NOT EXISTS printers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            connection_type TEXT NOT NULL DEFAULT 'network',
            ip_address TEXT,
            port INTEGER,
            serial_port TEXT,
            baud_rate INTEGER,
            paper_width INTEGER NOT NULL DEFAULT 80,
            is_enabled INTEGER NOT NULL DEFAULT 1,
            is_default INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now'))
        );
        ",
    )?;

    run_migrations(&conn)?;
    Ok(conn)
}

const CURRENT_SCHEMA_VERSION: i32 = 4;

fn get_current_version(conn: &Connection) -> SqliteResult<i32> {
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM schema_version", [], |r| r.get(0))?;
    if count == 0 {
        return Ok(0);
    }
    conn.query_row("SELECT version FROM schema_version ORDER BY version DESC LIMIT 1", [], |r| r.get(0))
}

fn run_migrations(conn: &Connection) -> SqliteResult<()> {
    let current_version = get_current_version(conn).unwrap_or(0);
    
    if current_version < 1 {
        info!("Running migration v1: inserting default data (checking existing)");
        
        let area_count: i32 = conn.query_row("SELECT COUNT(*) FROM areas", [], |r| r.get(0)).unwrap_or(0);
        if area_count == 0 {
            conn.execute("INSERT INTO areas (name, rate_per_hour) VALUES ('大厅', 30.0)", [])?;
            conn.execute("INSERT INTO areas (name, rate_per_hour) VALUES ('包房', 40.0)", [])?;
            conn.execute("INSERT INTO areas (name, rate_per_hour) VALUES ('VIP区', 60.0)", [])?;
        }

        let cat_count: i32 = conn.query_row("SELECT COUNT(*) FROM table_categories", [], |r| r.get(0)).unwrap_or(0);
        if cat_count == 0 {
            conn.execute("INSERT INTO table_categories (name, description) VALUES ('普通', '普通台球桌')", [])?;
            conn.execute("INSERT INTO table_categories (name, description) VALUES ('VIP', 'VIP专属球桌')", [])?;
            conn.execute("INSERT INTO table_categories (name, description) VALUES ('斯诺克', '斯诺克球桌')", [])?;
        }

        let table_count: i32 = conn.query_row("SELECT COUNT(*) FROM tables", [], |r| r.get(0)).unwrap_or(0);
        if table_count == 0 {
            for i in 1..=8 {
                conn.execute(
                    "INSERT INTO tables (name, table_type, rate_per_hour, is_private, area_id) VALUES (?1, '普通', 30.0, 0, 1)",
                    rusqlite::params![format!("{}号普通", i)],
                )?;
            }
            conn.execute(
                "INSERT INTO tables (name, table_type, rate_per_hour, is_private, area_id) VALUES ('VIP-A', 'VIP', 60.0, 1, 3)",
                [],
            )?;
            conn.execute(
                "INSERT INTO tables (name, table_type, rate_per_hour, is_private, area_id) VALUES ('VIP-B', 'VIP', 60.0, 1, 3)",
                [],
            )?;
        }

        let level_count: i32 = conn.query_row("SELECT COUNT(*) FROM member_levels", [], |r| r.get(0)).unwrap_or(0);
        if level_count == 0 {
            conn.execute("INSERT INTO member_levels (name, discount, color) VALUES ('普通会员', 1.0, 'rgba(88,166,255,0.2)')", [])?;
            conn.execute("INSERT INTO member_levels (name, discount, color) VALUES ('银卡会员', 0.95, 'rgba(192,192,192,0.2)')", [])?;
            conn.execute("INSERT INTO member_levels (name, discount, color) VALUES ('金卡会员', 0.9, 'rgba(255,215,0,0.2)')", [])?;
            conn.execute("INSERT INTO member_levels (name, discount, color) VALUES ('VIP会员', 0.85, 'rgba(156,39,176,0.2)')", [])?;
            conn.execute("INSERT INTO member_levels (name, discount, color) VALUES ('钻石会员', 0.8, 'rgba(163,113,247,0.2)')", [])?;
        }

        let user_count: i32 = conn.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0)).unwrap_or(0);
        if user_count == 0 {
            // Generate a random password for first login
            let random_password = Uuid::new_v4().to_string().chars().take(8).collect::<String>();
            let bcrypt = {
                use bcrypt::{hash, DEFAULT_COST};
                hash(&random_password, DEFAULT_COST).unwrap_or_default()
            };
            conn.execute(
                "INSERT INTO users (username, bcrypt_hash, role, first_login) VALUES ('admin', ?1, '管理员', 1)",
                params![bcrypt],
            )?;
            info!("⚠️  Default admin created with random password: {} (MUST CHANGE ON FIRST LOGIN)", random_password);
        } else {
            let admin_exists: i32 = conn.query_row(
                "SELECT COUNT(*) FROM users WHERE username = 'admin'",
                [],
                |r| r.get(0)
            ).unwrap_or(0);
            if admin_exists == 0 {
                let random_password = Uuid::new_v4().to_string().chars().take(8).collect::<String>();
                let bcrypt = {
                    use bcrypt::{hash, DEFAULT_COST};
                    hash(&random_password, DEFAULT_COST).unwrap_or_default()
                };
                conn.execute(
                    "INSERT INTO users (username, bcrypt_hash, role, first_login) VALUES ('admin', ?1, '管理员', 1)",
                    params![bcrypt],
                )?;
                info!("⚠️  Default admin created with random password: {} (MUST CHANGE ON FIRST LOGIN)", random_password);
            }
        }

        const ROLE_PERMISSIONS: &[(&str, &str)] = &[
            ("管理员", "table:*,member:*,order:*,finance:*,inventory:*,user:*,settings:*,report:*"),
            ("店长", "table:*,member:*,order:*,finance:view,inventory:*,settings:view,report:view"),
            ("服务员", "table:view,table:open,table:close,order:*,member:view,order:create"),
            ("收银员", "table:view,order:*,finance:*,member:recharge"),
            ("经理", "table:*,member:*,order:*,finance:view,inventory:*,report:view,settings:view"),
        ];

        for (role, perms) in ROLE_PERMISSIONS {
            for perm in perms.split(',') {
                conn.execute(
                    "INSERT OR IGNORE INTO role_permissions (role, permission) VALUES (?1, ?2)",
                    params![role, perm],
                ).ok();
            }
        }

        let version_count: i32 = conn.query_row("SELECT COUNT(*) FROM schema_version", [], |r| r.get(0)).unwrap_or(0);
        if version_count == 0 {
            conn.execute("INSERT INTO schema_version (version) VALUES (1)", [])?;
        }
    }

    if current_version < 2 {
        info!("Running migration v2: adding new tables and columns");
        
        conn.execute("ALTER TABLE inventory ADD COLUMN cost REAL DEFAULT 0", []).ok();
        conn.execute("ALTER TABLE inventory ADD COLUMN alert_stock INTEGER DEFAULT 0", []).ok();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product_categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).ok();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                category_id INTEGER,
                price REAL DEFAULT 0,
                cost REAL DEFAULT 0,
                unit TEXT DEFAULT '个',
                stock INTEGER DEFAULT 0,
                alert_stock INTEGER DEFAULT 0,
                image TEXT,
                description TEXT,
                is_active INTEGER DEFAULT 1,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (category_id) REFERENCES product_categories(id)
            )",
            [],
        ).ok();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS packages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                price REAL NOT NULL,
                hours REAL NOT NULL,
                area_ids TEXT DEFAULT '',
                table_ids TEXT DEFAULT '',
                start_time TEXT DEFAULT '00:00',
                end_time TEXT DEFAULT '23:59',
                is_active INTEGER DEFAULT 1,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).ok();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_name TEXT NOT NULL,
                customer_phone TEXT NOT NULL,
                table_id INTEGER,
                booking_time TEXT NOT NULL,
                duration_hours REAL DEFAULT 2,
                status TEXT DEFAULT 'pending',
                remark TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (table_id) REFERENCES tables(id)
            )",
            [],
        ).ok();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS relays (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                state INTEGER DEFAULT 0,
                table_id INTEGER,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).ok();
        
        let relay_count: i32 = conn.query_row("SELECT COUNT(*) FROM relays", [], |r| r.get(0)).unwrap_or(0);
        if relay_count == 0 {
            conn.execute("INSERT INTO relays (name, state) VALUES ('灯控1', 0)", []).ok();
            conn.execute("INSERT INTO relays (name, state) VALUES ('灯控2', 0)", []).ok();
            conn.execute("INSERT INTO relays (name, state) VALUES ('电源1', 0)", []).ok();
            conn.execute("INSERT INTO relays (name, state) VALUES ('电源2', 0)", []).ok();
        }
        
        conn.execute("INSERT INTO schema_version (version) VALUES (2)", []).ok();
    }

    if current_version < 3 {
        info!("Running migration v3: adding token revocation table");
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS revoked_tokens (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token TEXT UNIQUE NOT NULL,
                revoked_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).ok();
        
        conn.execute("INSERT INTO schema_version (version) VALUES (3)", []).ok();
    }

    if current_version < 4 {
        info!("Running migration v4: adding printers table");
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS printers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                connection_type TEXT NOT NULL DEFAULT 'network',
                ip_address TEXT,
                port INTEGER,
                serial_port TEXT,
                baud_rate INTEGER,
                paper_width INTEGER NOT NULL DEFAULT 80,
                is_enabled INTEGER NOT NULL DEFAULT 1,
                is_default INTEGER NOT NULL DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now'))
            )",
            [],
        ).ok();
        
        conn.execute("INSERT INTO schema_version (version) VALUES (4)", []).ok();
    }

    if current_version < CURRENT_SCHEMA_VERSION {
        warn!("Schema version {} is newer than current {}, consider running additional migrations", CURRENT_SCHEMA_VERSION, current_version);
    }

    info!("Database migrations completed, version: {}", current_version.max(1));
    Ok(())
}

const ROLE_PERMISSIONS: &[(&str, &str)] = &[
    ("管理员", "table:*,member:*,order:*,finance:*,inventory:*,user:*,settings:*,report:*"),
    ("店长", "table:*,member:*,order:*,finance:view,inventory:*,settings:view,report:view"),
    ("服务员", "table:view,table:open,table:close,order:*,member:view,order:create"),
    ("收银员", "table:view,order:*,finance:*,member:recharge"),
    ("经理", "table:*,member:*,order:*,finance:view,inventory:*,report:view,settings:view"),
];

#[allow(dead_code)]
pub fn check_permission(role: &str, resource: &str, action: &str) -> bool {
    if role == "管理员" {
        return true;
    }

    let conn = DB.lock();
    let has_permission: SqliteResult<i32> = conn.query_row(
        "SELECT COUNT(*) FROM role_permissions WHERE role = ?1 AND permission = ?2",
        params![role, format!("{}:{}", resource, action)],
        |r| r.get(0),
    );

    if has_permission.unwrap_or(0) > 0 {
        return true;
    }

    if let Ok(perms) = conn.query_row(
        "SELECT permission FROM role_permissions WHERE role = ?1",
        params![role],
        |row| row.get::<_, String>(0),
    ) {
        if perms.contains(resource) {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
pub fn get_role_permissions(role: &str) -> Vec<String> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT permission FROM role_permissions WHERE role = ?1") {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let result: Vec<String> = match stmt.query_map(params![role], |row| row.get(0)) {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    };
    result
}

#[allow(dead_code)]
pub fn init_role_permissions() {
    let conn = DB.lock();

    for (role, _perms) in ROLE_PERMISSIONS {
        let exists: i32 = conn.query_row(
            "SELECT COUNT(*) FROM role_permissions WHERE role = ?1",
            params![role],
            |r| r.get(0),
        ).unwrap_or(0);

        if exists == 0 {
            let perms_str = _perms;
            for perm in perms_str.split(',') {
                let parts: Vec<&str> = perm.split(':').collect();
                if parts.len() == 2 {
                    conn.execute(
                        "INSERT OR IGNORE INTO role_permissions (role, permission) VALUES (?1, ?2)",
                        params![role, perm],
                    ).ok();
                }
            }
        }
    }
    info!("Role permissions initialized");
}
