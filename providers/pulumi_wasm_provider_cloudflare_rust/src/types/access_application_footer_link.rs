#[derive(serde::Serialize)]
pub struct AccessApplicationFooterLink {
    /// The name of the footer link.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The URL of the footer link.
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}
