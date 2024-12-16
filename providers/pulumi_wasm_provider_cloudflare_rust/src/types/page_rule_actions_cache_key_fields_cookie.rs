#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleActionsCacheKeyFieldsCookie {
    /// Check for presence of specified cookies, without including their actual values.
    #[builder(into, default)]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Use values of specified cookies in Cache Key.
    #[builder(into, default)]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
