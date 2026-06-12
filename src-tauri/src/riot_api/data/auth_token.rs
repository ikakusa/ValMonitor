#[derive(Clone, Debug, Default)]
pub struct AuthToken {
    pub entitlements_token: String,
    pub access_token: String,
    pub lockfile_port: String,
    pub lockfile_password: String,
}
