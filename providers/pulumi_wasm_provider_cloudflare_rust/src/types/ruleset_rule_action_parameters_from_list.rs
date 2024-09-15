#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersFromList {
    /// Expression to use for the list lookup.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
