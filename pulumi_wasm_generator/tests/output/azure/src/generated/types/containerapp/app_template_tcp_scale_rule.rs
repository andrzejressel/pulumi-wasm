#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppTemplateTcpScaleRule {
    /// Zero or more `authentication` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "authentications")]
    pub r#authentications: Box<Option<Vec<super::super::types::containerapp::AppTemplateTcpScaleRuleAuthentication>>>,
    /// The number of concurrent requests to trigger scaling.
    #[builder(into)]
    #[serde(rename = "concurrentRequests")]
    pub r#concurrent_requests: Box<String>,
    /// The name of the Scaling Rule
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}