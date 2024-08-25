#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetAccountsAccount {
    /// Whether 2FA is enforced on the account.
    #[serde(rename = "enforceTwofactor")]
    pub r#enforce_twofactor: Box<Option<bool>>,
    /// Account ID.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Account name.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Account subscription type.
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
