#[derive(serde::Serialize)]
pub struct GetAccountsAccount {
    #[serde(rename = "enforceTwofactor")]
    pub r#enforce_twofactor: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
