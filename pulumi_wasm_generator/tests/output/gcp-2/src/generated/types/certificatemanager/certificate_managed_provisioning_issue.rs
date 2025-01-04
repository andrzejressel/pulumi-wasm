#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateManagedProvisioningIssue {
    /// Human readable explanation about the issue. Provided to help address
    /// the configuration issues.
    /// Not guaranteed to be stable. For programmatic access use 'reason' field.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<String>>,
    /// Reason for provisioning failures.
    #[builder(into, default)]
    #[serde(rename = "reason")]
    pub r#reason: Box<Option<String>>,
}
