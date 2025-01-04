#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecurityPolicyRuleHeaderAction {
    /// The list of request headers to add or overwrite if they're already present.
    #[builder(into)]
    #[serde(rename = "requestHeadersToAdds")]
    pub r#request_headers_to_adds: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleHeaderActionRequestHeadersToAdd>>,
}
