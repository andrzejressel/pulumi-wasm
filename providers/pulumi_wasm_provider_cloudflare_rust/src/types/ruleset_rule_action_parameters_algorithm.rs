#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersAlgorithm {
    /// Name of the compression algorithm to use. Available values: `zstd`, `gzip`, `brotli`, `auto`, `default`, `none`
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
