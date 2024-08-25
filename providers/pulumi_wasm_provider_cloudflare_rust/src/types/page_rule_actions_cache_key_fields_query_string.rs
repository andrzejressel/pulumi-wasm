#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PageRuleActionsCacheKeyFieldsQueryString {
    /// Exclude these query string parameters from Cache Key.
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// `false` (default) - all query string parameters are used for Cache Key, unless explicitly excluded; `true` - all query string parameters are ignored; value should be `false` if any of `exclude` or `include` is non-empty.
    #[serde(rename = "ignore")]
    pub r#ignore: Box<Option<bool>>,
    /// Only use values of specified query string parameters in Cache Key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
