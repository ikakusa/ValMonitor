use reqwest::{header::HeaderMap, Client};
use serde_json::Value;
use std::{collections::BTreeMap, time::Instant};

use crate::{
    errors::{AppError, AppResult},
    http_log,
};

use super::settings::{load_settings, HenrikAuthMode};

#[derive(Clone)]
pub struct HenrikClient {
    http: Client,
}

impl HenrikClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
        }
    }

    pub async fn get(
        &self,
        path: impl AsRef<str>,
        query: impl IntoIterator<Item = (String, String)>,
    ) -> AppResult<Value> {
        let settings = load_settings()?;
        let api_key = settings
            .api_key
            .filter(|key| !key.trim().is_empty())
            .ok_or(AppError::HenrikApiKeyMissing)?;

        let base_url = settings.base_url.trim_end_matches('/');
        let path = path.as_ref().trim_start_matches('/');
        let mut query_params = query.into_iter().collect::<Vec<_>>();

        match settings.auth_mode {
            HenrikAuthMode::Header => {
                let mut headers = HeaderMap::new();
                headers.insert("Authorization", api_key.parse()?);
                let url = build_url(base_url, path, &query_params);
                let started = Instant::now();
                let response = match self.http.get(url).headers(headers).send().await {
                    Ok(response) => response,
                    Err(error) => {
                        http_log::transport_error("henrik", "GET", path, started.elapsed(), &error);
                        return Err(error.into());
                    }
                };
                return parse_response(response, path, started).await;
            }
            HenrikAuthMode::Query => {
                query_params.push(("api_key".to_string(), api_key));
            }
        }

        let started = Instant::now();
        let response = match self
            .http
            .get(build_url(base_url, path, &query_params))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                http_log::transport_error("henrik", "GET", path, started.elapsed(), &error);
                return Err(error.into());
            }
        };
        parse_response(response, path, started).await
    }
}

async fn parse_response(response: reqwest::Response, path: &str, started: Instant) -> AppResult<Value> {
    let status = response.status();
    let rate_limits = extract_rate_limit_headers(response.headers());
    let body = response.text().await?;

    if !status.is_success() {
        http_log::response_error("henrik", "GET", path, status.as_u16(), started.elapsed(), &body);
        return Err(AppError::HenrikRequestFailed {
            status: status.as_u16(),
            body: truncate_body(&body, 600),
        });
    }

    http_log::response("henrik", "GET", path, status.as_u16(), started.elapsed(), body.len());
    let mut value = serde_json::from_str::<Value>(&body).map_err(|error| {
        http_log::parse_error("henrik", "GET", path, started.elapsed(), &error.to_string(), &body);
        AppError::InvalidResponse(format!(
            "Henrik JSON parse failed: {error}; body starts with: {}",
            truncate_body(&body, 160)
        ))
    })?;

    if let Value::Object(object) = &mut value {
        object.insert(
            "_rate_limits".to_string(),
            serde_json::to_value(rate_limits)
                .map_err(|error| AppError::InvalidResponse(error.to_string()))?,
        );
    }

    Ok(value)
}

fn truncate_body(body: &str, max_chars: usize) -> String {
    let trimmed = body.trim();
    let mut output = trimmed.chars().take(max_chars).collect::<String>();
    if trimmed.chars().count() > max_chars {
        output.push_str("...");
    }
    output
}

pub fn segment(value: &str) -> String {
    urlencoding::encode(value).into_owned()
}

pub fn query(params: Vec<(&str, Option<String>)>) -> BTreeMap<String, String> {
    params
        .into_iter()
        .filter_map(|(key, value)| value.map(|value| (key.to_string(), value)))
        .filter(|(_, value)| !value.is_empty())
        .collect()
}

fn extract_rate_limit_headers(headers: &reqwest::header::HeaderMap) -> BTreeMap<String, String> {
    [
        "ratelimit",
        "ratelimit-policy",
        "x-ratelimit-limit",
        "x-ratelimit-remaining",
        "x-ratelimit-reset",
        "x-cache-status",
        "x-cache-ttl",
    ]
    .into_iter()
    .filter_map(|key| {
        headers
            .get(key)
            .and_then(|value| value.to_str().ok())
            .map(|value| (key.to_string(), value.to_string()))
    })
    .collect()
}

fn build_url(base_url: &str, path: &str, query_params: &[(String, String)]) -> String {
    if query_params.is_empty() {
        return format!("{base_url}/{path}");
    }

    let query = query_params
        .iter()
        .map(|(key, value)| format!("{}={}", segment(key), segment(value)))
        .collect::<Vec<_>>()
        .join("&");

    format!("{base_url}/{path}?{query}")
}
