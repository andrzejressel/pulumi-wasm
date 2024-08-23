#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFields {
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsCookie>>,
    #[serde(rename = "header")]
    pub r#header: Box<Option<crate::types::PageRuleActionsCacheKeyFieldsHeader>>,
    #[serde(rename = "host")]
    pub r#host: Box<crate::types::PageRuleActionsCacheKeyFieldsHost>,
    #[serde(rename = "queryString")]
    pub r#query_string: Box<crate::types::PageRuleActionsCacheKeyFieldsQueryString>,
    #[serde(rename = "user")]
    pub r#user: Box<crate::types::PageRuleActionsCacheKeyFieldsUser>,
}
