#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefaultShieldedInstanceConfig {
    /// Defines whether the instance has integrity monitoring enabled.
    #[builder(into)]
    #[serde(rename = "enableIntegrityMonitoring")]
    pub r#enable_integrity_monitoring: Box<bool>,
    /// Defines whether the instance has Secure Boot enabled.
    #[builder(into)]
    #[serde(rename = "enableSecureBoot")]
    pub r#enable_secure_boot: Box<bool>,
}
