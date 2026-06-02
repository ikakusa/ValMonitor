use std::{env, fs, sync::Arc, time::Duration};

use super::MutexExt;
use crate::{
    errors::{AppError, AppResult},
    models::{auth::AuthSnapshot, lockfile::LockfileData},
};
use std::sync::atomic::Ordering;

impl super::ValorantApi {
    pub fn monitor_lockfile(self: Arc<Self>) {
        tauri::async_runtime::spawn(async move {
            loop {
                let wait = match self.monitor_tick().await {
                    Ok(()) => Duration::from_secs(1),
                    Err(AppError::LockfileNotFound) => {
                        // VALORANT / Riot Client が起動していない通常状態では lockfile が存在しない。
                        // ここで warn を出し続けると debug.log が実害のないログで埋まるため、低頻度の debug に落とす。
                        if let Err(error) = self.clear_session_state() {
                            tracing::warn!(%error, "failed to clear Riot session state");
                        }
                        tracing::debug!("Riot lockfile is not available; waiting for Riot Client");
                        Duration::from_secs(5)
                    }
                    Err(AppError::RiotSessionNotReady(reason)) => {
                        // lockfile 作成直後は認証 API や PVP セッションがまだ揃わないことがある。
                        // セッション確立まで少し間隔を空け、API への空振りとログ量を抑える。
                        tracing::debug!(%reason, "Riot session is not ready; delaying next probe");
                        Duration::from_secs(2)
                    }
                    Err(error) => {
                        tracing::warn!(%error, "failed to update Riot session from lockfile");
                        Duration::from_secs(2)
                    }
                };

                tokio::time::sleep(wait).await;
            }
        });
    }

    async fn monitor_tick(&self) -> AppResult<()> {
        let content = self.read_lockfile()?;
        let lockfile = LockfileData::parse(content)?;
        let current_puuid = if self.is_initialized() {
            self.get_entitlements_token()
                .await
                .map(|entitlements| entitlements.subject)
                .unwrap_or_default()
        } else {
            String::new()
        };

        if *self.raw_lockfile.lock_app("raw_lockfile")? == lockfile.raw
            && self.puuid()? == current_puuid
        {
            return Ok(());
        }

        self.set_local_session(lockfile)?;

        let entitlements = match self.get_entitlements_token().await {
            Ok(entitlements) => entitlements,
            Err(error) if is_session_not_ready(&error) => {
                tracing::debug!(%error, "Riot entitlements are not ready yet");
                return Ok(());
            }
            Err(error) => return Err(error),
        };
        self.set_auth_session(entitlements.access_token.as_str())?;

        let userinfo = match self.get_userinfo().await {
            Ok(userinfo) => userinfo,
            Err(error) if is_session_not_ready(&error) => {
                tracing::debug!(%error, "Riot userinfo is not ready yet");
                return Ok(());
            }
            Err(error) => return Err(error),
        };
        *self.auth_snapshot.lock_app("auth_snapshot")? = AuthSnapshot {
            access_token: entitlements.access_token,
            entitlements_token: entitlements.token,
            puuid: entitlements.subject,
            game_name: userinfo.acct.game_name,
            tag_line: userinfo.acct.tag_line,
        };

        if let Err(error) = self.build_pvp_client().await {
            if matches!(error, AppError::RiotSessionNotReady(_)) {
                tracing::debug!(%error, "PVP client is not ready yet");
                return Ok(());
            }

            return Err(error);
        }
        tracing::info!("Riot session updated");

        Ok(())
    }

    fn read_lockfile(&self) -> AppResult<String> {
        let local_appdata = env::var("LOCALAPPDATA")?;
        let path = format!(r"{local_appdata}\Riot Games\Riot Client\Config\lockfile");

        fs::read_to_string(&path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                AppError::LockfileNotFound
            } else {
                error.into()
            }
        })
    }

    fn clear_session_state(&self) -> AppResult<()> {
        if !self.is_initialized() && self.raw_lockfile.lock_app("raw_lockfile")?.is_empty() {
            return Ok(());
        }

        // lockfile が消えた後も古い token/client を保持すると、UI が接続中だと誤認して
        // PVP API を叩き続ける。未起動時は明示的に未初期化へ戻す。
        *self.raw_lockfile.lock_app("raw_lockfile")? = String::new();
        *self.local_session.lock_app("local_session")? = None;
        *self.auth_session.lock_app("auth_session")? = None;
        *self.pvp_client.lock_app("pvp_client")? = None;
        *self.region.lock_app("region")? = None;
        *self.auth_snapshot.lock_app("auth_snapshot")? = AuthSnapshot::default();
        self.local_initialized.store(false, Ordering::Release);
        self.auth_initialized.store(false, Ordering::Release);
        self.pvp_initialized.store(false, Ordering::Release);

        Ok(())
    }
}

fn is_session_not_ready(error: &AppError) -> bool {
    matches!(error, AppError::RiotSessionNotReady(_))
}
