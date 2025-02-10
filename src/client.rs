use std::error::Error;
use std::sync::Arc;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use crate::response::LoginResponse;

const BASE_URL: &str = "https://api.easee.com/api";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct Client {
    access_token: Arc<String>,
    client: reqwest::Client,
}

impl Client {
    pub fn new_with_access_token(access_token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            access_token: Arc::new(access_token.to_string()),
        }
    }

    pub async fn setup_with_username_and_password(username: &str, password: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let response = Self::login(username, password).await?;

        Ok(Self {
            client: reqwest::Client::new(),
            access_token: Arc::new(response.access_token.to_string()),
        })
    }

    pub async fn login(username: &str, password: &str) -> Result<LoginResponse, Box<dyn Error + Send + Sync>> {
        let client = reqwest::Client::new();
        let payload = json!({
            "userName": username,
            "password": password,
        })
        .to_string();
        let response = client
            .post(format!("{}/accounts/login", BASE_URL))
            .header("Content-Type", "application/json")
            .header("User-Agent", format!("Flowion/EaseeClient/{}", VERSION))
            .body(payload)
            .send()
            .await?;
        if response.status().is_success() {
            let data = response.json::<LoginResponse>().await?;
            dbg!(&data);
            Ok(data)
        } else {
            let status = response.status();
            let text = response.text().await?;
            Err(format!("{} - {}", status, text).into())
        }
    }

    async fn post<T: DeserializeOwned + std::fmt::Debug>(&self, path: &str, payload: Value) -> Result<T, Box<dyn Error + Send + Sync>> {
        let payload = payload.to_string();
        let response = self.client
            .post(format!("{}/{}", BASE_URL, path))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("User-Agent", format!("Flowion/EaseeClient/{}", VERSION))
            .body(payload)
            .send()
            .await?;
        if response.status().is_success() {
            let data = response.json::<T>().await?;
            dbg!(&data);
            Ok(data)
        } else {
            let status = response.status();
            let text = response.text().await?;
            Err(format!("{} - {}", status, text).into())
        }
    }

    pub async fn pair_charger(&self, serial_number: &str, pin_code: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.post(&format!("chargers/{}/pair?pinCode={}", serial_number, pin_code), json!({})).await
    }

}