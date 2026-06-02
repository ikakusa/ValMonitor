use std::sync::Mutex;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use serde::Deserialize;

use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordRpcActivityInput {
    pub client_id: String,
    pub details: Option<String>,
    pub state: Option<String>,
}

pub struct DiscordRpcService {
    client: Mutex<Option<DiscordIpcClient>>,
}

impl DiscordRpcService {
    pub fn new() -> Self {
        Self {
            client: Mutex::new(None),
        }
    }

    pub fn set_activity(&self, input: DiscordRpcActivityInput) -> AppResult<()> {
        let client_id = input.client_id.trim();
        if client_id.is_empty() {
            return Err(AppError::InvalidResponse(
                "Discord RPC client id is missing".to_string(),
            ));
        }

        let mut guard = self
            .client
            .lock()
            .map_err(|_| AppError::StatePoisoned("discord_rpc"))?;
        if guard.is_none() {
            let mut client = DiscordIpcClient::new(client_id);
            // Discord が起動していない環境ではここで失敗する。
            // アプリ本体を巻き込まないため command error に落として UI へ返す。
            client.connect().map_err(discord_error)?;
            *guard = Some(client);
        }

        let details = input
            .details
            .unwrap_or_else(|| "Tracking VALORANT".to_string());
        let state = input.state.unwrap_or_else(|| "ValMonitor".to_string());
        let activity = activity::Activity::new().details(&details).state(&state);
        guard
            .as_mut()
            .ok_or_else(|| AppError::InvalidResponse("Discord RPC client is missing".to_string()))?
            .set_activity(activity)
            .map_err(discord_error)
    }

    pub fn clear(&self) -> AppResult<()> {
        let mut guard = self
            .client
            .lock()
            .map_err(|_| AppError::StatePoisoned("discord_rpc"))?;
        if let Some(client) = guard.as_mut() {
            if let Err(error) = client.clear_activity() {
                tracing::debug!(error = %error, "failed to clear Discord RPC activity");
            }
            if let Err(error) = client.close() {
                tracing::debug!(error = %error, "failed to close Discord RPC client");
            }
        }
        *guard = None;
        Ok(())
    }
}

fn discord_error(error: discord_rich_presence::error::Error) -> AppError {
    AppError::InvalidResponse(format!("Discord RPC failed: {error}"))
}
