use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};

use crate::types::config::Config;

use self::provider_uris::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
}

#[derive(Debug, Clone)]
pub enum Provider {
    Github,
    Microsoft,
    Google,
    Apple,
    Other,
}

mod provider_uris {
    pub const GITHUB_AUTH_URI: &str = "https://github.com/login/oauth/authorize";
    pub const GITHUB_TOKEN_URI: &str = "https://github.com/login/oauth/access_token";
    pub const MICROSOFT_AUTH_URI: &str =
        "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";
    pub const MICROSOFT_TOKEN_URI: &str =
        "https://login.microsoftonline.com/common/oauth2/v2.0/token";
    pub const GOOGLE_AUTH_URI: &str = "https://accounts.google.com/o/oauth2/v2/auth";
    pub const GOOGLE_TOKEN_URI: &str = "https://www.googleapis.com/oauth2/v3/token";
    pub const APPLE_AUTH_URI: &str = "https://example.com";
    pub const APPLE_TOKEN_URI: &str = "https://example.com";
}

fn provider_uri(provider: &Provider) -> (String, String) {
    let (auth, token) = match provider {
        Provider::Github => (GITHUB_AUTH_URI, GITHUB_TOKEN_URI),
        Provider::Microsoft => (MICROSOFT_AUTH_URI, MICROSOFT_TOKEN_URI),
        Provider::Google => (GOOGLE_AUTH_URI, GOOGLE_TOKEN_URI),
        Provider::Apple => (APPLE_AUTH_URI, APPLE_TOKEN_URI),
        Provider::Other => ("", ""),
    };

    (auth.to_string(), token.to_string())
}

#[derive(Debug, Clone)]
pub struct Oauth2 {
    pub provider: Provider,
    pub authorize_url: String,
    pub csrf_state: CsrfToken,
}

impl Oauth2 {
    pub fn new(
        config: Config,
        provider: Provider,
        client_id: String,
        client_secret: String,
    ) -> Result<Self, AuthError> {
        let (auth_url, token_url) = provider_uri(&provider);

        let token_url = TokenUrl::new(token_url)?;
        let auth_url = AuthUrl::new(auth_url)?;
        let redirect_url = RedirectUrl::new(config.oauth2.redirect_url)?;

        let id = ClientId::new(client_id);
        let secret = ClientSecret::new(client_secret);

        let client = BasicClient::new(id, Some(secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("".to_string()))
            .url();

        Ok(Self {
            provider,
            authorize_url: authorize_url.to_string(),
            csrf_state,
        })
    }
}
