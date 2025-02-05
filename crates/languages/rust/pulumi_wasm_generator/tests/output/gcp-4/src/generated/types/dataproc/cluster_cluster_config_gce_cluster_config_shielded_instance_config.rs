#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigGceClusterConfigShieldedInstanceConfig {
    /// Defines whether instances have integrity monitoring enabled.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "enableIntegrityMonitoring")]
    pub r#enable_integrity_monitoring: Box<Option<bool>>,
    /// Defines whether instances have Secure Boot enabled.
    #[builder(into, default)]
    #[serde(rename = "enableSecureBoot")]
    pub r#enable_secure_boot: Box<Option<bool>>,
    /// Defines whether instances have the [vTPM](https://cloud.google.com/security/shielded-cloud/shielded-vm#vtpm) enabled.
    #[builder(into, default)]
    #[serde(rename = "enableVtpm")]
    pub r#enable_vtpm: Box<Option<bool>>,
}
