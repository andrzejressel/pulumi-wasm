#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserPoolUserPoolAddOns {
    /// Mode for advanced security, must be one of `OFF`, `AUDIT` or `ENFORCED`.
    #[builder(into)]
    #[serde(rename = "advancedSecurityMode")]
    pub r#advanced_security_mode: Box<String>,
}