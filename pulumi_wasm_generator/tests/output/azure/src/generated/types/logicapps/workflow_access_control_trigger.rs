#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowAccessControlTrigger {
    /// A list of the allowed caller IP address ranges.
    #[builder(into)]
    #[serde(rename = "allowedCallerIpAddressRanges")]
    pub r#allowed_caller_ip_address_ranges: Box<Vec<String>>,
    /// A `open_authentication_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "openAuthenticationPolicies")]
    pub r#open_authentication_policies: Box<Option<Vec<super::super::types::logicapps::WorkflowAccessControlTriggerOpenAuthenticationPolicy>>>,
}