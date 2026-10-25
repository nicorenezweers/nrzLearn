//! SQLite storage: settings, users, sessions and progress.

use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OptionalExtension, params};

pub struct Db(Mutex<Connection>);

pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub lang: String,
}

pub struct Stats {
    pub user_id: i64,
    pub username: String,
    pub xp: u32,
    pub streak: u32,
    pub levels_done: u32,
}

pub fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

impl Db {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS settings (
                 key   TEXT PRIMARY KEY,
                 value TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS users (
                 id            INTEGER PRIMARY KEY AUTOINCREMENT,
                 username      TEXT NOT NULL UNIQUE COLLATE NOCASE,
                 password_hash TEXT NOT NULL,
                 is_admin      INTEGER NOT NULL DEFAULT 0,
                 lang          TEXT NOT NULL DEFAULT 'en',
                 created_at    INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS sessions (
                 token_hash TEXT PRIMARY KEY,
                 user_id    INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                 expires_at INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS progress (
                 user_id     INTEGER PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
                 data        TEXT NOT NULL,
                 xp          INTEGER NOT NULL DEFAULT 0,
                 streak      INTEGER NOT NULL DEFAULT 0,
                 levels_done INTEGER NOT NULL DEFAULT 0,
                 updated_at  INTEGER NOT NULL
             );",
        )?;
        Ok(Self(Mutex::new(conn)))
    }

    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn setting(&self, key: &str) -> Option<String> {
        self.conn().query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| r.get(0)).optional().ok().flatten()
    }

    pub fn set_setting(&self, key: &str, value: &str) -> rusqlite::Result<()> {
        self.conn().execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn admin_count(&self) -> i64 {
        self.conn().query_row("SELECT COUNT(*) FROM users WHERE is_admin = 1", [], |r| r.get(0)).unwrap_or(0)
    }

    fn map_user(r: &rusqlite::Row<'_>) -> rusqlite::Result<User> {
        Ok(User { id: r.get(0)?, username: r.get(1)?, password_hash: r.get(2)?, is_admin: r.get::<_, i64>(3)? == 1, lang: r.get(4)? })
    }

    pub fn user_by_name(&self, name: &str) -> Option<User> {
        self.conn()
            .query_row("SELECT id, username, password_hash, is_admin, lang FROM users WHERE username = ?1", [name], Self::map_user)
            .optional()
            .ok()
            .flatten()
    }

    pub fn user_by_id(&self, id: i64) -> Option<User> {
        self.conn()
            .query_row("SELECT id, username, password_hash, is_admin, lang FROM users WHERE id = ?1", [id], Self::map_user)
            .optional()
            .ok()
            .flatten()
    }

    /// Returns `None` if the username is already taken.
    pub fn create_user(&self, name: &str, hash: &str, is_admin: bool, lang: &str) -> Option<i64> {
        let conn = self.conn();
        conn.execute(
            "INSERT INTO users (username, password_hash, is_admin, lang, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, hash, i64::from(is_admin), lang, now()],
        )
        .ok()?;
        Some(conn.last_insert_rowid())
    }

    pub fn set_password(&self, id: i64, hash: &str) -> rusqlite::Result<usize> {
        self.conn().execute("UPDATE users SET password_hash = ?1 WHERE id = ?2", params![hash, id])
    }

    pub fn set_admin(&self, id: i64, is_admin: bool) -> rusqlite::Result<usize> {
        self.conn().execute("UPDATE users SET is_admin = ?1 WHERE id = ?2", params![i64::from(is_admin), id])
    }

    pub fn set_lang(&self, id: i64, lang: &str) -> rusqlite::Result<usize> {
        self.conn().execute("UPDATE users SET lang = ?1 WHERE id = ?2", params![lang, id])
    }

    pub fn delete_user(&self, id: i64) -> rusqlite::Result<usize> {
        self.conn().execute("DELETE FROM users WHERE id = ?1", [id])
    }

    pub fn create_session(&self, token_hash: &str, user_id: i64, expires_at: i64) -> rusqlite::Result<()> {
        let conn = self.conn();
        conn.execute("DELETE FROM sessions WHERE expires_at < ?1", [now()])?;
        conn.execute("INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (?1, ?2, ?3)", params![token_hash, user_id, expires_at])?;
        Ok(())
    }

    pub fn session_user(&self, token_hash: &str) -> Option<i64> {
        self.conn()
            .query_row("SELECT user_id FROM sessions WHERE token_hash = ?1 AND expires_at > ?2", params![token_hash, now()], |r| r.get(0))
            .optional()
            .ok()
            .flatten()
    }

    pub fn delete_session(&self, token_hash: &str) -> rusqlite::Result<usize> {
        self.conn().execute("DELETE FROM sessions WHERE token_hash = ?1", [token_hash])
    }

    pub fn progress(&self, user_id: i64) -> Option<String> {
        self.conn().query_row("SELECT data FROM progress WHERE user_id = ?1", [user_id], |r| r.get(0)).optional().ok().flatten()
    }

    pub fn save_progress(&self, user_id: i64, data: &str, xp: u32, streak: u32, levels_done: u32) -> rusqlite::Result<()> {
        self.conn().execute(
            "INSERT INTO progress (user_id, data, xp, streak, levels_done, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(user_id) DO UPDATE SET data = excluded.data, xp = excluded.xp, streak = excluded.streak,
                 levels_done = excluded.levels_done, updated_at = excluded.updated_at",
            params![user_id, data, xp, streak, levels_done, now()],
        )?;
        Ok(())
    }

    /// All users ordered by XP (highest first).
    pub fn ranking(&self) -> Vec<Stats> {
        let conn = self.conn();
        let Ok(mut stmt) = conn.prepare(
            "SELECT u.id, u.username, COALESCE(p.xp, 0), COALESCE(p.streak, 0), COALESCE(p.levels_done, 0)
             FROM users u LEFT JOIN progress p ON p.user_id = u.id
             ORDER BY COALESCE(p.xp, 0) DESC, COALESCE(p.levels_done, 0) DESC, u.id ASC",
        ) else {
            return Vec::new();
        };
        stmt.query_map([], |r| Ok(Stats { user_id: r.get(0)?, username: r.get(1)?, xp: r.get(2)?, streak: r.get(3)?, levels_done: r.get(4)? }))
            .map(|rows| rows.flatten().collect())
            .unwrap_or_default()
    }

    pub fn admin_users(&self) -> Vec<nrzlearn_shared::AdminUser> {
        let conn = self.conn();
        let Ok(mut stmt) = conn.prepare(
            "SELECT u.id, u.username, u.is_admin, COALESCE(p.xp, 0), COALESCE(p.levels_done, 0), u.created_at
             FROM users u LEFT JOIN progress p ON p.user_id = u.id ORDER BY u.created_at ASC",
        ) else {
            return Vec::new();
        };
        stmt.query_map([], |r| {
            Ok(nrzlearn_shared::AdminUser {
                id: r.get(0)?,
                username: r.get(1)?,
                is_admin: r.get::<_, i64>(2)? == 1,
                xp: r.get(3)?,
                levels_done: r.get(4)?,
                created_at: r.get(5)?,
            })
        })
        .map(|rows| rows.flatten().collect())
        .unwrap_or_default()
    }
}
