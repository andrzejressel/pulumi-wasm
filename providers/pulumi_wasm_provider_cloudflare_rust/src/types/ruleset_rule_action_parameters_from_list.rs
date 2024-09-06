#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RulesetRuleActionParametersFromList {
    /// Expression to use for the list lookup.
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
