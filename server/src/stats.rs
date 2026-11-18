//! Anonymous learning statistics: how many levels are finished, per course,
//! interface language and day. No IDs, no cookies, no IP addresses are stored;
//! the app only sends what kind of level was finished in which course.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Counter {
    /// Finished levels (also repeated ones).
    pub levels: u64,
    /// Levels finished for the first time in a browser.
    pub first: u64,
    /// Lessons (theory levels) finished for the first time.
    pub lessons: u64,
    /// World exams and master exams passed for the first time.
    pub exams: u64,
    /// Courses started (first level of a course in a browser).
    pub starts: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Stats {
    /// Day of the first count (`YYYY-MM-DD`).
    pub since: String,
    pub total: Counter,
    pub courses: BTreeMap<String, Counter>,
    pub langs: BTreeMap<String, Counter>,
    pub days: BTreeMap<String, Counter>,
}

/// One finished level, as sent by the app: `{"c":"python","l":"de","k":"lesson","f":true,"s":false}`.
#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "c")]
    pub course: String,
    #[serde(rename = "l")]
    pub lang: String,
    #[serde(rename = "k")]
    pub kind: String,
    #[serde(rename = "f", default)]
    pub first: bool,
    #[serde(rename = "s", default)]
    pub start: bool,
}

pub const KINDS: [&str; 5] = ["lesson", "practice", "lab", "exam", "master"];

impl Event {
    pub fn is_valid(&self) -> bool {
        nrzlearn_model::course_info(&self.course).is_some()
            && nrzlearn_model::LANGUAGES.iter().any(|l| l.code == self.lang)
            && KINDS.contains(&self.kind.as_str())
    }
}

impl Counter {
    fn add(&mut self, e: &Event) {
        self.levels += 1;
        if e.first {
            self.first += 1;
            match e.kind.as_str() {
                "lesson" => self.lessons += 1,
                "exam" | "master" => self.exams += 1,
                _ => {}
            }
        }
        if e.start {
            self.starts += 1;
        }
    }
}

/// Today's date in UTC, `YYYY-MM-DD`.
pub fn today() -> String {
    day(today_number())
}

/// Today as days since 1970-01-01 (UTC).
pub fn today_number() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() / 86_400).unwrap_or(0) as i64
}

/// `YYYY-MM-DD` of a day given as days since 1970-01-01.
pub fn day(days: i64) -> String {
    // Civil date from days since 1970-01-01 (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

impl Stats {
    pub fn load(dir: Option<&Path>) -> Self {
        dir.and_then(|d| std::fs::read_to_string(d.join("stats.json")).ok()).and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
    }

    pub fn save(&self, dir: &Path) -> std::io::Result<()> {
        let tmp = dir.join("stats.json.tmp");
        std::fs::write(&tmp, serde_json::to_string(self).unwrap_or_default())?;
        std::fs::rename(tmp, dir.join("stats.json"))
    }

    pub fn add(&mut self, e: &Event, day: &str) {
        if self.since.is_empty() {
            self.since = day.to_string();
        }
        self.total.add(e);
        self.courses.entry(e.course.clone()).or_default().add(e);
        self.langs.entry(e.lang.clone()).or_default().add(e);
        self.days.entry(day.to_string()).or_default().add(e);
        // Keep about two years of days.
        while self.days.len() > 730 {
            let oldest = self.days.keys().next().cloned().unwrap_or_default();
            self.days.remove(&oldest);
        }
    }
}

/// Simple limit against floods: at most `per_client` events per client and
/// `global` events in total per window. Kept in memory only.
pub struct Limiter {
    window: std::time::Duration,
    per_client: u32,
    global: u32,
    start: Instant,
    total: u32,
    clients: HashMap<String, u32>,
}

impl Limiter {
    pub fn new(window: std::time::Duration, per_client: u32, global: u32) -> Self {
        Self { window, per_client, global, start: Instant::now(), total: 0, clients: HashMap::new() }
    }

    pub fn allow(&mut self, client: &str) -> bool {
        if self.start.elapsed() > self.window {
            self.start = Instant::now();
            self.total = 0;
            self.clients.clear();
        }
        let n = self.clients.entry(client.to_string()).or_insert(0);
        if *n >= self.per_client || self.total >= self.global {
            return false;
        }
        *n += 1;
        self.total += 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(course: &str, kind: &str, first: bool, start: bool) -> Event {
        Event { course: course.into(), lang: "de".into(), kind: kind.into(), first, start }
    }

    #[test]
    fn day_strings() {
        assert_eq!(day(0), "1970-01-01");
        assert_eq!(day(20_775), "2026-11-18");
        assert_eq!(day(19_782), "2024-02-29");
        assert_eq!(day(11_017), "2000-03-01");
    }

    #[test]
    fn counts_and_validates() {
        let mut s = Stats::default();
        s.add(&ev("python", "lesson", true, true), "2026-11-18");
        s.add(&ev("python", "lesson", false, false), "2026-11-18");
        s.add(&ev("rust", "exam", true, false), "2026-11-19");
        assert_eq!(s.total, Counter { levels: 3, first: 2, lessons: 1, exams: 1, starts: 1 });
        assert_eq!(s.courses["python"].levels, 2);
        assert_eq!(s.langs["de"].levels, 3);
        assert_eq!(s.days.len(), 2);
        assert_eq!(s.since, "2026-11-18");
        assert!(ev("python", "lesson", true, false).is_valid());
        assert!(!ev("cobol2", "lesson", true, false).is_valid());
        assert!(!ev("python", "hack", true, false).is_valid());
        assert_eq!(today().len(), 10);
    }

    #[test]
    fn limits_floods() {
        let mut l = Limiter::new(std::time::Duration::from_secs(60), 2, 3);
        assert!(l.allow("a") && l.allow("a"));
        assert!(!l.allow("a"));
        assert!(l.allow("b"));
        assert!(!l.allow("c"));
    }
}
