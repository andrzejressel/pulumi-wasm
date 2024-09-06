#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PageRuleActionsCacheKeyFields {
    /// Controls what cookies go into Cache Key:
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsCookie>>,
    /// Controls what HTTP headers go into Cache Key:
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsHeader>>,
    /// Controls which Host header goes into Cache Key:
    #[serde(rename = "host")]
    pub r#host: Box<crate::types::PageRuleActionsCacheKeyFieldsHost>,
    /// Controls which URL query string parameters go into the Cache Key.
    #[serde(rename = "queryString")]
    pub r#query_string: Box<crate::types::PageRuleActionsCacheKeyFieldsQueryString>,
    /// Controls which end user-related features go into the Cache Key.
    #[serde(rename = "user")]
    pub r#user: Box<crate::types::PageRuleActionsCacheKeyFieldsUser>,
}
