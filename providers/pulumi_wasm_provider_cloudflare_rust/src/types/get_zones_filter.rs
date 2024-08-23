#[derive(serde::Serialize)]
pub struct GetZonesFilter {
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    #[serde(rename = "lookupType")]
    pub r#lookup_type: Box<Option<String>>,
    #[serde(rename = "match")]
    pub r#match: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "paused")]
    pub r#paused: Box<Option<bool>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
