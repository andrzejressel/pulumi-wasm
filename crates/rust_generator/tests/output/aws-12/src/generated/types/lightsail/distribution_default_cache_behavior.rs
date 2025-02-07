#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionDefaultCacheBehavior {
    /// The cache behavior of the distribution. Valid values: `cache` and `dont-cache`.
    #[builder(into)]
    #[serde(rename = "behavior")]
    pub r#behavior: Box<String>,
}
