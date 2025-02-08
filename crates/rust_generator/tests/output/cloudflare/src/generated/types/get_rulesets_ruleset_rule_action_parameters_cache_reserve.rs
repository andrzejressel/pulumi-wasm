#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRulesetsRulesetRuleActionParametersCacheReserve {
    /// Determines whether Cloudflare will write the eligible resource to cache reserve.
    #[builder(into)]
    #[serde(rename = "eligible")]
    pub r#eligible: Box<bool>,
    /// The minimum file size, in bytes, eligible for storage in cache reserve. If omitted and "eligible" is true, Cloudflare will use 0 bytes by default.
    #[builder(into, default)]
    #[serde(rename = "minimumFileSize")]
    pub r#minimum_file_size: Box<Option<i32>>,
}
