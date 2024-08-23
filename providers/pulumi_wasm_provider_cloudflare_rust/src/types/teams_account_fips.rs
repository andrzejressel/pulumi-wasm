#[derive(serde::Serialize)]
pub struct TeamsAccountFips {
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<bool>>,
}
