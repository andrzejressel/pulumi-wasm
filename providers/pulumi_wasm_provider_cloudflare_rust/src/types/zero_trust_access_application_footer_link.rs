#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessApplicationFooterLink {
    /// The name of the footer link.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The URL of the footer link.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}
