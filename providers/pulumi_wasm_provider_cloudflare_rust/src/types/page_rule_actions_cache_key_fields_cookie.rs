#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PageRuleActionsCacheKeyFieldsCookie {
    /// Check for presence of specified HTTP headers, without including their actual values.
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Only use values of specified query string parameters in Cache Key.
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
