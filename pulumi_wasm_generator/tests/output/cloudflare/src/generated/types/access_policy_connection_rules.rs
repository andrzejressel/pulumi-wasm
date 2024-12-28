#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessPolicyConnectionRules {
    /// The SSH-specific rules that define how users may connect to the targets secured by your application.
    #[builder(into)]
    #[serde(rename = "ssh")]
    pub r#ssh: Box<super::types::AccessPolicyConnectionRulesSsh>,
}
