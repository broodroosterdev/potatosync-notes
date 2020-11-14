/// Claims for access_token
#[derive(Deserialize)]
pub struct Token {
    pub(crate) sub: String,
    pub role: String,
    #[serde(rename = "type")]
    pub(crate) token_type: String,
    pub iat: u64,
    pub exp: u64,
}