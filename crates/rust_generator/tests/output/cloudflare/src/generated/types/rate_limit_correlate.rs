#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RateLimitCorrelate {
    /// If set to 'nat', NAT support will be enabled for rate limiting. Available values: `nat`.
    #[builder(into, default)]
    #[serde(rename = "by")]
    pub r#by: Box<Option<String>>,
}
