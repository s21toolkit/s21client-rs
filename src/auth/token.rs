use super::constants;

use chrono::Utc;
use uuid::Uuid;

pub struct LoginError {
	pub body: String,
	pub reason: String,
	pub code: u16
}

impl LoginError {
	pub fn new(reason: &str, body: &str, code: u16) -> LoginError {
		return LoginError {
			reason: reason.to_owned(),
			body: body.to_owned(),
			code: code
		}
	}
}

pub struct Token {
	access_token: String,
	code: String,

	username: String,
	password: String,

	issue_time:  u64,
	expiry_time: u64,
}

impl Token {
	pub fn new(username: &str, password: &str) -> Token {
		return Token{
			access_token: String::from(""),
			code: String::from(""),

			username: username.to_owned(),
			password: password.to_owned(),

			expiry_time: 0,
			issue_time: 0,
		}
	}

	pub fn is_expired(self) -> bool {
		return self.code == "" || (u64::try_from(Utc::now().timestamp()).unwrap() - self.issue_time) > self.expiry_time
	}

	pub async fn refresh(self) -> Result<(), LoginError> {
		if !self.is_expired() {
			return Ok(());
		}

		Ok(())
	}

	async fn get_auth_data(self) -> Result<&str, LoginError> {
		let state = Uuid::new_v4();
		let nonce = Uuid::new_v4();

		let client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build().unwrap();

		let cookie_url = constants::get_cookie_url(state.to_string().as_str(), nonce.to_string().as_str());

		let cookie: &str = match client.get(cookie_url).send().await {
			Err(err) => return Err(LoginError::new("Can't connect?", "", err.status().unwrap_or_default().as_u16())),
			Ok(res) => res.headers()["Cookie"].to_str().unwrap()
		};

		Ok("")
	}
}

#[cfg(test)]
mod tests {

	#[tokio::test]
	async fn test_login() {
	
	}
}
