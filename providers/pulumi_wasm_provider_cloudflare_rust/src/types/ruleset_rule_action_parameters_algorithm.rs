#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersAlgorithm {
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
