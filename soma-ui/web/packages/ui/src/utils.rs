//! Dependency-free formatting helpers: dates, relative time, and user-agent prettification.

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

// ── Internal parsers ──────────────────────────────────────────────────────────

/// Parses the `YYYY-MM-DD` prefix of an ISO-8601 string.
/// Returns `(year, month_index_0based, day)` or `None` on malformed input.
fn parse_date(iso: &str) -> Option<(u32, usize, u32)> {
    if iso.len() < 10 {
        return None;
    }
    let year: u32 = iso.get(0..4)?.parse().ok()?;
    let month: usize = iso.get(5..7)?.parse::<usize>().ok()?.checked_sub(1)?;
    let day: u32 = iso.get(8..10)?.parse().ok()?;
    if month >= 12 || day == 0 || day > 31 {
        return None;
    }
    Some((year, month, day))
}

/// Parses the `THH:MM` time segment of an ISO-8601 string.
/// Returns `(hour, minute)` or `None` on absent or malformed input.
fn parse_time(iso: &str) -> Option<(u32, u32)> {
    if iso.len() < 16 {
        return None;
    }
    let sep = *iso.as_bytes().get(10)?;
    if sep != b'T' && sep != b' ' {
        return None;
    }
    let hour: u32 = iso.get(11..13)?.parse().ok()?;
    let minute: u32 = iso.get(14..16)?.parse().ok()?;
    if hour > 23 || minute > 59 {
        return None;
    }
    Some((hour, minute))
}

/// Days since Unix epoch (1970-01-01 = 0). Requires `year >= 1970` and 1-based `month`.
fn epoch_day(year: u32, month: u32, day: u32) -> i64 {
    const MONTH_DAYS: [i64; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let is_leap = |y: u32| (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mut d: i64 = 0;
    for y in 1970..year {
        d += if is_leap(y) { 366 } else { 365 };
    }
    for m in 1..month {
        d += MONTH_DAYS[m as usize];
        if m == 2 && is_leap(year) {
            d += 1;
        }
    }
    d + day as i64 - 1
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Formats an ISO-8601 timestamp as `"Jul 27, 2026"`.
///
/// Returns a short truncation on malformed input rather than panicking.
pub fn format_date(iso: &str) -> String {
    if let Some((year, month, day)) = parse_date(iso) {
        format!("{} {}, {}", MONTHS[month], day, year)
    } else {
        iso.chars().take(10).collect()
    }
}

/// Formats an ISO-8601 timestamp as `"Jul 27, 2026 at 14:32"`.
///
/// Falls back to [`format_date`] when the time component is absent or malformed.
pub fn format_datetime(iso: &str) -> String {
    match (parse_date(iso), parse_time(iso)) {
        (Some((year, month, day)), Some((hour, minute))) => {
            format!(
                "{} {}, {} at {:02}:{:02}",
                MONTHS[month], day, year, hour, minute
            )
        }
        _ => format_date(iso),
    }
}

/// Formats an ISO-8601 timestamp as a human-readable relative string.
///
/// Returns strings like `"just now"`, `"2 hours ago"`, `"3 days ago"`.
/// Falls back to [`format_date`] for timestamps more than ~1 year old or in the future.
///
/// `now_ms` is milliseconds since the Unix epoch. In a WASM context pass
/// `js_sys::Date::now()`; in tests supply a fixed value.
pub fn format_relative(iso: &str, now_ms: f64) -> String {
    let (year, month, day) = match parse_date(iso) {
        Some(v) => v,
        None => return format_date(iso),
    };
    if year < 1970 {
        return format_date(iso);
    }
    let (hour, minute) = parse_time(iso).unwrap_or((0, 0));
    let second: u32 = iso.get(17..19).and_then(|s| s.parse().ok()).unwrap_or(0);
    let ts_ms = (epoch_day(year, month as u32 + 1, day) * 86400
        + hour as i64 * 3600
        + minute as i64 * 60
        + second as i64) as f64
        * 1000.0;

    let diff_secs = ((now_ms - ts_ms) / 1000.0) as i64;
    match diff_secs {
        s if s < 0 => format_date(iso),
        0..=59 => "just now".to_string(),
        60..=3599 => {
            let m = diff_secs / 60;
            if m == 1 {
                "1 minute ago".to_string()
            } else {
                format!("{} minutes ago", m)
            }
        }
        3600..=86399 => {
            let h = diff_secs / 3600;
            if h == 1 {
                "1 hour ago".to_string()
            } else {
                format!("{} hours ago", h)
            }
        }
        86400..=2591999 => {
            let d = diff_secs / 86400;
            if d == 1 {
                "1 day ago".to_string()
            } else {
                format!("{} days ago", d)
            }
        }
        2592000..=31535999 => {
            let mo = diff_secs / 2592000;
            if mo == 1 {
                "1 month ago".to_string()
            } else {
                format!("{} months ago", mo)
            }
        }
        _ => format_date(iso),
    }
}

/// Prettifies a raw User-Agent string into a short human label like `"Chrome 150 · macOS"`.
///
/// Matches common browser and OS families. Falls back to a 40-character truncation of the
/// original string rather than returning empty or panicking.
pub fn prettify_ua(ua: &str) -> String {
    if ua.trim().is_empty() {
        return "(unknown)".to_string();
    }
    match (detect_browser(ua), detect_os(ua)) {
        (Some(b), Some(o)) => format!("{} · {}", b, o),
        (Some(b), None) => b,
        (None, Some(o)) => o.to_string(),
        (None, None) => {
            let s = ua.trim();
            let end = s.char_indices().nth(40).map(|(i, _)| i).unwrap_or(s.len());
            s[..end].to_string()
        }
    }
}

fn detect_browser(ua: &str) -> Option<String> {
    // Check more specific tokens before generic ones so Edge/Opera are not
    // mis-identified as Chrome (their UAs also contain "Chrome/").
    if let Some(v) = extract_major(ua, "Edg/").or_else(|| extract_major(ua, "Edge/")) {
        return Some(format!("Edge {}", v));
    }
    if let Some(v) = extract_major(ua, "OPR/").or_else(|| extract_major(ua, "Opera/")) {
        return Some(format!("Opera {}", v));
    }
    if let Some(v) = extract_major(ua, "Firefox/") {
        return Some(format!("Firefox {}", v));
    }
    if let Some(v) = extract_major(ua, "SamsungBrowser/") {
        return Some(format!("Samsung Browser {}", v));
    }
    if ua.contains("Chrome/") {
        return extract_major(ua, "Chrome/").map(|v| format!("Chrome {}", v));
    }
    if ua.contains("Version/") && ua.contains("Safari") {
        return extract_major(ua, "Version/").map(|v| format!("Safari {}", v));
    }
    if ua.contains("Safari") {
        return Some("Safari".to_string());
    }
    None
}

fn detect_os(ua: &str) -> Option<&'static str> {
    if ua.contains("Windows NT") {
        return Some("Windows");
    }
    if ua.contains("CrOS") {
        return Some("Chrome OS");
    }
    // iOS UAs contain "Mac OS X" too — check iPhone/iPad/iPod first.
    if ua.contains("iPhone") || ua.contains("iPad") || ua.contains("iPod") {
        return Some("iOS");
    }
    if ua.contains("Android") {
        return Some("Android");
    }
    if ua.contains("Macintosh") || ua.contains("Mac OS X") {
        return Some("macOS");
    }
    if ua.contains("Linux") {
        return Some("Linux");
    }
    None
}

/// Extracts the major version number after `token` in `ua`.
fn extract_major(ua: &str, token: &str) -> Option<String> {
    let start = ua.find(token)? + token.len();
    let rest = &ua[start..];
    let end = rest
        .find(|c: char| !c.is_ascii_digit() && c != '.')
        .unwrap_or(rest.len());
    let major = rest[..end].split('.').next()?;
    if major.is_empty() {
        None
    } else {
        Some(major.to_string())
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // --- epoch_day ---

    #[test]
    fn epoch_day_at_unix_epoch() {
        assert_eq!(epoch_day(1970, 1, 1), 0);
    }

    #[test]
    fn epoch_day_known_date() {
        // 2026-07-27 verified against unix timestamp 1785110400 / 86400 = 20661
        assert_eq!(epoch_day(2026, 7, 27), 20661);
    }

    // --- format_date ---

    #[test]
    fn format_date_full_timestamp() {
        assert_eq!(format_date("2026-07-27T14:32:00Z"), "Jul 27, 2026");
    }

    #[test]
    fn format_date_date_only() {
        assert_eq!(format_date("2026-01-01"), "Jan 1, 2026");
    }

    #[test]
    fn format_date_empty() {
        assert_eq!(format_date(""), "");
    }

    #[test]
    fn format_date_malformed_does_not_panic() {
        let result = format_date("not-a-date");
        assert!(result.chars().count() <= 10);
    }

    #[test]
    fn format_date_invalid_month_does_not_panic() {
        // month 13 is out of range
        let result = format_date("2026-13-01");
        assert!(!result.is_empty());
    }

    // --- format_datetime ---

    #[test]
    fn format_datetime_full() {
        assert_eq!(
            format_datetime("2026-07-27T14:32:00Z"),
            "Jul 27, 2026 at 14:32"
        );
    }

    #[test]
    fn format_datetime_midnight() {
        assert_eq!(
            format_datetime("2026-07-27T00:00:00Z"),
            "Jul 27, 2026 at 00:00"
        );
    }

    #[test]
    fn format_datetime_no_time_falls_back() {
        assert_eq!(format_datetime("2026-07-27"), "Jul 27, 2026");
    }

    #[test]
    fn format_datetime_empty() {
        assert_eq!(format_datetime(""), "");
    }

    // --- format_relative ---
    //
    // 2026-07-27T14:32:00Z in epoch ms:
    //   epoch_day(2026, 7, 27) = 20661 days × 86400 s = 1_785_110_400 s
    //   + 14×3600 + 32×60 = 52_320 s
    //   total = 1_785_162_720 s = 1_785_162_720_000 ms

    const SAMPLE_MS: f64 = 1_785_162_720_000.0;

    #[test]
    fn format_relative_just_now() {
        assert_eq!(
            format_relative("2026-07-27T14:32:00Z", SAMPLE_MS + 30_000.0),
            "just now"
        );
    }

    #[test]
    fn format_relative_one_minute() {
        assert_eq!(
            format_relative("2026-07-27T14:32:00Z", SAMPLE_MS + 90_000.0),
            "1 minute ago"
        );
    }

    #[test]
    fn format_relative_plural_minutes() {
        assert_eq!(
            format_relative("2026-07-27T14:32:00Z", SAMPLE_MS + 180_000.0),
            "3 minutes ago"
        );
    }

    #[test]
    fn format_relative_two_hours() {
        assert_eq!(
            format_relative("2026-07-27T14:32:00Z", SAMPLE_MS + 7_200_000.0),
            "2 hours ago"
        );
    }

    #[test]
    fn format_relative_one_day() {
        // 25 hours later → 1 day ago
        assert_eq!(
            format_relative("2026-07-27T14:32:00Z", SAMPLE_MS + 90_000_000.0),
            "1 day ago"
        );
    }

    #[test]
    fn format_relative_future_falls_back() {
        // now is BEFORE the timestamp — should fall back to format_date
        assert_eq!(
            format_relative("2026-07-27T14:32:00Z", SAMPLE_MS - 1_000.0),
            "Jul 27, 2026"
        );
    }

    #[test]
    fn format_relative_malformed_does_not_panic() {
        let result = format_relative("not-a-date", 0.0);
        assert!(!result.is_empty());
    }

    // --- prettify_ua ---

    #[test]
    fn prettify_ua_chrome_macos() {
        let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
                  (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36";
        assert_eq!(prettify_ua(ua), "Chrome 150 · macOS");
    }

    #[test]
    fn prettify_ua_firefox_windows() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) \
                  Gecko/20100101 Firefox/120.0";
        assert_eq!(prettify_ua(ua), "Firefox 120 · Windows");
    }

    #[test]
    fn prettify_ua_safari_ios() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) \
                  AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 \
                  Mobile/15E148 Safari/604.1";
        assert_eq!(prettify_ua(ua), "Safari 17 · iOS");
    }

    #[test]
    fn prettify_ua_edge_windows() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                  (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.2210.91";
        assert_eq!(prettify_ua(ua), "Edge 120 · Windows");
    }

    #[test]
    fn prettify_ua_empty() {
        assert_eq!(prettify_ua(""), "(unknown)");
    }

    #[test]
    fn prettify_ua_whitespace_only() {
        assert_eq!(prettify_ua("   "), "(unknown)");
    }

    #[test]
    fn prettify_ua_unknown_truncates() {
        let ua = "SomeCrazyBotAgent/1.0 (DefinitelyNotABrowser; UnknownOS 99.9)";
        let result = prettify_ua(ua);
        assert!(!result.is_empty());
        assert!(result.chars().count() <= 40);
    }
}
