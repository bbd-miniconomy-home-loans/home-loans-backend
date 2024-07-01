use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl};
pub use oauth2::AccessToken;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::auth_config;
use crate::oauth::error::Result;

#[derive(Clone)]
pub struct OAuthClient {
	client: BasicClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	pub id: String,
	avatar: Option<String>,
	username: String,
	discriminator: String,
}

impl OAuthClient {
	pub fn new() -> Result<OAuthClient> {
		Ok(OAuthClient {
			client: create_oauth_client()?
		})
	}
	pub fn create_redirect_url(&self) -> String {
		let (token, _csrf) = self.client
			.authorize_url(CsrfToken::new_random)
			.add_scope(Scope::new("identify".to_owned()))
			.add_scope(Scope::new("email".to_owned()))
			.url();
		token.to_string()
	}

	pub async fn exchange_token(&self, code: String) -> Result<AccessToken> {
		let token = self.client
			.exchange_code(AuthorizationCode::new(code))
			.request_async(async_http_client)
			.await?;
		Ok(token.access_token().to_owned())
	}

	pub async fn request_client_data(&self, request_client: &Client, token: AccessToken) -> Result<User> {
		let profile_response = request_client.get("https://discordapp.com/api/users/@me")
			.bearer_auth(token.secret())
			.send()
			.await?;
		let profile = profile_response.json::<User>().await?;
		Ok(profile)
	}
}

fn create_oauth_client() -> Result<BasicClient> {
	let config = auth_config();
	// TODO: env vars ???
	let auth_url = "https://discord.com/oauth2/authorize".to_string();
	let token_url = "https://discord.com/api/oauth2/token".to_string();

	let token_url = TokenUrl::new(token_url).expect("Invalid token endpoint URL");
	let auth_url = AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL");
	let redirect_url = RedirectUrl::new(config.REDIRECT_URL.clone()).expect("Invalid redirect endpoint URL");

	Ok(BasicClient::new(
		ClientId::new(config.CLIENT_ID.clone()),
		Some(ClientSecret::new(config.CLIENT_SECRET.clone())),
		auth_url,
		Some(token_url),
	).set_redirect_uri(
		redirect_url,
	))
}
