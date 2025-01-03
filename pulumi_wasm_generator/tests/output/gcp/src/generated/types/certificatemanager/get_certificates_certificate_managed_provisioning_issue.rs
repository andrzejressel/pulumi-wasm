#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificatesCertificateManagedProvisioningIssue {
    /// Human readable explanation about the issue. Provided to help address
    /// the configuration issues.
    /// Not guaranteed to be stable. For programmatic access use 'reason' field.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Box<String>,
    /// Reason for provisioning failures.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Box<String>,
}
