#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsQueryString {
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "ignore")]
    pub r#ignore: Box<Option<bool>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
