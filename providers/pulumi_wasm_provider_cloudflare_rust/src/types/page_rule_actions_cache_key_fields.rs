#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleActionsCacheKeyFields {
    /// Controls what cookies go into Cache Key:
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsCookie>>,
    /// Controls what HTTP headers go into Cache Key:
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsHeader>>,
    /// Controls which Host header goes into Cache Key:
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<crate::types::PageRuleActionsCacheKeyFieldsHost>,
    /// Controls which URL query string parameters go into the Cache Key.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<crate::types::PageRuleActionsCacheKeyFieldsQueryString>,
    /// Controls which end user-related features go into the Cache Key.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Box<crate::types::PageRuleActionsCacheKeyFieldsUser>,
}
