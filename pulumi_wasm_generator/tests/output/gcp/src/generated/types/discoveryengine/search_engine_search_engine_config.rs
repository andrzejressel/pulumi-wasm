#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SearchEngineSearchEngineConfig {
    /// The add-on that this search engine enables.
    /// Each value may be one of: `SEARCH_ADD_ON_LLM`.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "searchAddOns")]
    pub r#search_add_ons: Box<Option<Vec<String>>>,
    /// The search feature tier of this engine. Defaults to SearchTier.SEARCH_TIER_STANDARD if not specified.
    /// Default value is `SEARCH_TIER_STANDARD`.
    /// Possible values are: `SEARCH_TIER_STANDARD`, `SEARCH_TIER_ENTERPRISE`.
    #[builder(into, default)]
    #[serde(rename = "searchTier")]
    pub r#search_tier: Box<Option<String>>,
}
