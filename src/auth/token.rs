use super::constants::{self, get_cookie_url};

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
	access_token: Option<String>,
	code: Option<String>,

	username: String,
	password: String,

	issue_time:  u64,
	expiry_time: u64,
}

impl Token {
	pub fn new(username: &str, password: &str) -> Token {
		return Token{
			access_token: None,
			code: None,

			username: username.to_owned(),
			password: password.to_owned(),

			expiry_time: 0,
			issue_time: 0,
		}
	}

	pub fn is_expired(self) -> bool {
		return match self.code {
			Some(val) => val == "" || (u64::try_from(Utc::now().timestamp()).unwrap() - self.issue_time) > self.expiry_time,
			None => true,
		}
	}

	pub async fn refresh(self) -> Result<(), LoginError> {
		if !self.is_expired() {
			return Ok(());
		}

		Ok(())
	}

	async fn get_cookies(self) -> Result<String, LoginError> {
		let state = Uuid::new_v4();
		let nonce = Uuid::new_v4();

		let client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build().unwrap();
		let cookie_url = constants::get_cookie_url(state.to_string().as_str(), nonce.to_string().as_str());

		return match client.get(cookie_url).send().await {
			Err(err) => Err(LoginError::new("Can't connect?", "", err.status().unwrap_or_default().as_u16())),
			Ok(res) => {
				let headers = res.headers();
				let status = res.status();
				match headers.get("Cookie") {
						Some(v) => Ok(String::from(v.to_str().unwrap())),
						None => Err(LoginError::new("No cookies header", res.text().await.unwrap_or_default().as_str(), status.as_u16()))
				}
			}
		};
	} 

	async fn get_auth_data(self) -> Result<String, LoginError> {
		let var = self.get_cookies().await?;

		Ok(var)
	}
}

#[cfg(test)]
mod tests {

	#[tokio::test]
	async fn test_login() {
	
	}
}
