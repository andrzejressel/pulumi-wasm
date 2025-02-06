#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventConnectionAuthParametersInvocationHttpParameters {
    /// Contains additional body string parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into, default)]
    #[serde(rename = "bodies")]
    pub r#bodies: Box<Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersInvocationHttpParametersBody>>>,
    /// Contains additional header parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersInvocationHttpParametersHeader>>>,
    /// Contains additional query string parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into, default)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Box<Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersInvocationHttpParametersQueryString>>>,
}
