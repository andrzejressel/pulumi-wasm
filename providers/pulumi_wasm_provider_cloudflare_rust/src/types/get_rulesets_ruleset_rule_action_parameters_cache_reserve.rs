#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersCacheReserve {
    /// Determines whether Cloudflare will write the eligible resource to cache reserve.
    #[builder(into)]
    #[serde(rename = "eligible")]
    pub r#eligible: Box<bool>,
    /// The minimum file size, in bytes, eligible for storage in cache reserve. If omitted and "eligible" is true, Cloudflare will use 0 bytes by default.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "minimumFileSize")]
    pub r#minimum_file_size: Box<Option<i32>>,
}
