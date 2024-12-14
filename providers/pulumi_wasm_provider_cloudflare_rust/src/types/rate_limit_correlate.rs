#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RateLimitCorrelate {
    /// If set to 'nat', NAT support will be enabled for rate limiting. Available values: `nat`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "by")]
    pub r#by: Box<Option<String>>,
}
