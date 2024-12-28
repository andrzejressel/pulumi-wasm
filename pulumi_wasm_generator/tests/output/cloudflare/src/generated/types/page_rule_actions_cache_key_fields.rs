#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PageRuleActionsCacheKeyFields {
    /// Controls what cookies go into Cache Key:
    #[builder(into, default)]
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<super::types::PageRuleActionsCacheKeyFieldsCookie>>,
    /// Controls what HTTP headers go into Cache Key:
    #[builder(into, default)]
    #[serde(rename = "header")]
    pub r#header: Box<Option<super::types::PageRuleActionsCacheKeyFieldsHeader>>,
    /// Controls which Host header goes into Cache Key:
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<super::types::PageRuleActionsCacheKeyFieldsHost>,
    /// Controls which URL query string parameters go into the Cache Key.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<super::types::PageRuleActionsCacheKeyFieldsQueryString>,
    /// Controls which end user-related features go into the Cache Key.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Box<super::types::PageRuleActionsCacheKeyFieldsUser>,
}
