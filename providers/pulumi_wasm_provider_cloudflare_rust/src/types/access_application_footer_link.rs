#[derive(serde::Serialize)]
pub struct AccessApplicationFooterLink {
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}
