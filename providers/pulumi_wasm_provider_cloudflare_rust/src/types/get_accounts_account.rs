#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetAccountsAccount {
    /// Whether 2FA is enforced on the account.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enforceTwofactor")]
    pub r#enforce_twofactor: Box<Option<bool>>,
    /// Account ID.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Account name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Account subscription type.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
