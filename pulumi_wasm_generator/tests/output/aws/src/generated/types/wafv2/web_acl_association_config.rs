#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclAssociationConfig {
    /// Customizes the request body that your protected resource forward to AWS WAF for inspection. See `request_body` below for details.
    #[builder(into, default)]
    #[serde(rename = "requestBodies")]
    pub r#request_bodies: Box<Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBody>>>,
}