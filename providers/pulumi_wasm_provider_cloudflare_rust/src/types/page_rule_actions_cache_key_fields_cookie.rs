#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleActionsCacheKeyFieldsCookie {
    /// Check for presence of specified HTTP headers, without including their actual values.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Only use values of specified query string parameters in Cache Key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
