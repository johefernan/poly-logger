use rand::prelude::*;
use std::{env, thread, time::Duration};
use tracing::{debug, error, info, trace, warn};

const LEVELS: [&str; 7] = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR", "CRITICAL", "FATAL"];
const ACTIONS: [&str; 7] = [
    "processing request",
    "connecting to database",
    "fetching user data",
    "updating cache",
    "validating input",
    "generating report",
    "sending notification",
];
const USERS: [&str; 5] = ["user123", "admin", "guest", "service_account", "api_client"];
const ERRORS: [&str; 5] = [
    "connection timeout",
    "invalid credentials",
    "resource not found",
    "permission denied",
    "internal server error",
];

fn parse_interval(value: Option<String>) -> Duration {
    let Some(v) = value else {
        return Duration::from_secs(1);
    };
    let raw = v.trim().to_lowercase();
    if raw.ends_with("ms") {
        return raw[..raw.len() - 2]
            .trim()
            .parse::<u64>()
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(1));
    }
    if raw.ends_with('s') {
        return raw[..raw.len() - 1]
            .trim()
            .parse::<u64>()
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(1));
    }
    if raw.ends_with('m') {
        return raw[..raw.len() - 1]
            .trim()
            .parse::<u64>()
            .map(|n| Duration::from_secs(n * 60))
            .unwrap_or(Duration::from_secs(1));
    }
    if raw.ends_with('h') {
        return raw[..raw.len() - 1]
            .trim()
            .parse::<u64>()
            .map(|n| Duration::from_secs(n * 3600))
            .unwrap_or(Duration::from_secs(1));
    }
    Duration::from_secs(1)
}

fn parse_total(value: Option<String>) -> i64 {
    value
        .and_then(|v| v.trim().parse::<i64>().ok())
        .unwrap_or(-1)
}

fn build_message(level: &str, rng: &mut ThreadRng) -> String {
    let action = ACTIONS.choose(rng).unwrap();
    let user = USERS.choose(rng).unwrap();
    let error = ERRORS.choose(rng).unwrap();

    match level {
        "TRACE" => format!("trace {}", action),
        "DEBUG" => format!("debug {}", action),
        "INFO" => format!("info {} user={}", action, user),
        "WARN" => format!("warn possible issue action={}", action),
        _ => format!("error {} user={}", error, user),
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    let interval = parse_interval(env::var("LOG_INTERVAL").ok());
    let total = parse_total(env::var("TOTAL_LOGS").ok());
    let mut rng = rand::thread_rng();

    let mut seq: i64 = 1;
    while total == -1 || seq <= total {
        let level = LEVELS.choose(&mut rng).unwrap();
        let message = build_message(level, &mut rng);
        let mut request_id: Option<i32> = None;
        let mut duration_ms: Option<f64> = None;

        if rng.gen_bool(0.5) {
            request_id = Some(rng.gen_range(0..10000));
        }
        if rng.gen_bool(0.3) {
            duration_ms = Some(rng.gen_range(0.0..1000.0));
        }

        match *level {
            "TRACE" => trace!(language = "rust", seq, ?request_id, ?duration_ms, "{message}"),
            "DEBUG" => debug!(language = "rust", seq, ?request_id, ?duration_ms, "{message}"),
            "INFO" => info!(language = "rust", seq, ?request_id, ?duration_ms, "{message}"),
            "WARN" => warn!(language = "rust", seq, ?request_id, ?duration_ms, "{message}"),
            _ => error!(language = "rust", seq, ?request_id, ?duration_ms, "{message}"),
        }
        seq += 1;
        thread::sleep(interval);
    }
}

