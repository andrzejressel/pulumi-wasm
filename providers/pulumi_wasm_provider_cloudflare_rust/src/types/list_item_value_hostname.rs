#[derive(serde::Serialize)]
pub struct ListItemValueHostname {
    #[serde(rename = "urlHostname")]
    pub r#url_hostname: Box<String>,
}
