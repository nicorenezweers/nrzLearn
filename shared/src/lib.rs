//! Types exchanged between the nrzLearn web app and the server (`/api/...`).

use serde::{Deserialize, Serialize};

/// XP needed per user level. Used by the app and the leaderboard.
pub const XP_PER_LEVEL: u32 = 250;

/// Languages the app ships with. `en` is the default.
pub const LANGUAGES: &[&str] = &["en", "de", "es"];

pub const USERNAME_MIN: usize = 3;
pub const USERNAME_MAX: usize = 24;
pub const PASSWORD_MIN: usize = 8;

/// Public information about the instance, available without logging in.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Status {
    /// No admin exists yet: the first visitor runs the onboarding.
    pub setup_needed: bool,
    pub instance_name: String,
    pub registration_open: bool,
    pub default_lang: String,
}

/// Onboarding: creates the instance settings and the first admin.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SetupRequest {
    pub instance_name: String,
    pub username: String,
    pub password: String,
    pub registration_open: bool,
    pub default_lang: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    /// Language chosen on the sign-up screen (ignored on login).
    #[serde(default)]
    pub lang: Option<String>,
}

/// The logged-in user, including the saved progress as a JSON string.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Me {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
    pub lang: String,
    pub progress: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SaveProgress {
    /// The progress serialized by the app. The server reads `xp`, `serie`
    /// and `knoten` from it for the leaderboard.
    pub progress: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SetLanguage {
    pub lang: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LeaderEntry {
    pub rank: u32,
    pub username: String,
    pub xp: u32,
    pub level: u32,
    pub streak: u32,
    pub levels_done: u32,
    pub is_me: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Leaderboard {
    pub entries: Vec<LeaderEntry>,
    /// The current user, also when outside the top entries.
    pub me: Option<LeaderEntry>,
    pub total_users: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AdminUser {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
    pub xp: u32,
    pub levels_done: u32,
    /// Unix timestamp (seconds).
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InstanceSettings {
    pub instance_name: String,
    pub registration_open: bool,
    pub default_lang: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SetAdmin {
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct NewPassword {
    pub password: String,
}

/// Error body. `error` is a stable code the app translates.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ApiError {
    pub error: String,
}

/// Error codes returned by the server.
pub mod code {
    pub const INVALID_CREDENTIALS: &str = "invalid_credentials";
    pub const USERNAME_TAKEN: &str = "username_taken";
    pub const INVALID_USERNAME: &str = "invalid_username";
    pub const WEAK_PASSWORD: &str = "weak_password";
    pub const REGISTRATION_CLOSED: &str = "registration_closed";
    pub const SETUP_DONE: &str = "setup_done";
    pub const UNAUTHORIZED: &str = "unauthorized";
    pub const FORBIDDEN: &str = "forbidden";
    pub const LAST_ADMIN: &str = "last_admin";
    pub const NOT_FOUND: &str = "not_found";
    pub const INVALID_INPUT: &str = "invalid_input";
}

/// Username rules, checked by both sides.
pub fn valid_username(name: &str) -> bool {
    let n = name.chars().count();
    (USERNAME_MIN..=USERNAME_MAX).contains(&n)
        && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
}

pub fn level_for(xp: u32) -> u32 {
    xp / XP_PER_LEVEL + 1
}
