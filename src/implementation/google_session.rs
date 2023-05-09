use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::json;
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use super::google_drive::GoogleDrive;

#[derive(Debug, Clone)]
pub struct GoogleSession {
    pub token: String,
    pub expiration_unix_seconds: u64,
}

impl GoogleSession {
    pub fn new<T: Into<String>>(
        client_email: T,
        private_key: T,
    ) -> Result<GoogleSession, Box<dyn Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expiration_unix_seconds = now + 3600;
        let claims = json!({
            "iss": client_email.into(),
            "scope": "https://www.googleapis.com/auth/drive", // change this to the scope you need
            "aud": "https://oauth2.googleapis.com/token",
            "exp": now + 3600,
            "iat": now
        });
        let header = Header::new(Algorithm::RS256);
        let key = EncodingKey::from_rsa_pem(private_key.into().as_bytes())?;
        let jwt = encode(&header, &claims, &key)?;

        // Send a POST request to get the access token
        let response = ureq::post("https://oauth2.googleapis.com/token").send_form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])?;
        let json = response.into_json::<serde_json::Value>()?;
        let access_token = match json["access_token"].as_str() {
            Some(value) => value,
            None => return Err("Access token is empty".into()),
        };
        Ok(GoogleSession {
            token: access_token.to_string(),
            expiration_unix_seconds,
        })
    }
    pub fn drive(&self) -> GoogleDrive {
        return GoogleDrive::new(self.clone());
    }
}
