#[derive(serde::Serialize)]
pub struct ListItemHostname {
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
