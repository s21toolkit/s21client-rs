use regex::Regex;
use lazy_static::lazy_static;

#[macro_export]
macro_rules! KC_BASE_URL {
	() => ( "https://auth.sberclass.ru/auth/realms/EduPowerKeycloak" )
}

#[macro_export]
macro_rules! TOKEN_URL {
	() => ( concat!(KC_BASE_URL!(), "/protocol/openid-connect/token") )
}

lazy_static! {
	pub static ref LOGIN_ACTION_REGEX: Regex = Regex::new("(?P<LoginActionURL>https:\\/\\/.+?)\"").unwrap();
	pub static ref OAUTH_CODE_REGEX: Regex = Regex::new("code=(?P<OAuthCode>.+)[&$]?").unwrap();
}

pub fn get_cookie_url(state: &str, nonce: &str) -> String {
	return String::from(KC_BASE_URL!()) + "/protocol/openid-connect/auth?client_id=school21&redirect_uri=https%%3A%%2F%%2Fedu.21-school.ru%%2F&state=" + state + "&response_mode=fragment&response_type=code&scope=openid&nonce=" + nonce;
}
