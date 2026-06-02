use std::time::Duration;

pub fn response(scope: &str, method: &str, path: &str, status: u16, elapsed: Duration, body_len: usize) {
    log::debug!(
        "HTTP {scope} {method} {path} -> {status} in {}ms body={}B",
        elapsed.as_millis(),
        body_len
    );
}

pub fn response_error(
    scope: &str,
    method: &str,
    path: &str,
    status: u16,
    elapsed: Duration,
    body: &str,
) {
    log::debug!(
        "HTTP {scope} {method} {path} -> {status} in {}ms body={}B preview=\"{}\"",
        elapsed.as_millis(),
        body.len(),
        preview(body, 160)
    );
}

pub fn transport_error(scope: &str, method: &str, path: &str, elapsed: Duration, error: &reqwest::Error) {
    log::debug!(
        "HTTP {scope} {method} {path} -> transport_error in {}ms error=\"{}\"",
        elapsed.as_millis(),
        error
    );
}

pub fn parse_error(scope: &str, method: &str, path: &str, elapsed: Duration, error: &str, body: &str) {
    log::debug!(
        "HTTP {scope} {method} {path} -> parse_error in {}ms error=\"{}\" body={}B preview=\"{}\"",
        elapsed.as_millis(),
        error,
        body.len(),
        preview(body, 160)
    );
}

fn preview(body: &str, max_chars: usize) -> String {
    let compact = body
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let mut output = compact.chars().take(max_chars).collect::<String>();
    if compact.chars().count() > max_chars {
        output.push_str("...");
    }
    output.replace('"', "'")
}
