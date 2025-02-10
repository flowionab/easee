use std::time::Duration;
use serde::Deserialize;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "accessClaims")]
    pub access_claims: Vec<String>,

    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "expiresIn", deserialize_with = "deserialize_expires_in")]
    pub expires_in: Duration,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,

    #[serde(rename = "tokenType")]
    pub token_type: String,
}

fn deserialize_expires_in<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let seconds = u64::deserialize(deserializer)?; // Expecting a number in seconds
    Ok(Duration::from_secs(seconds))
}