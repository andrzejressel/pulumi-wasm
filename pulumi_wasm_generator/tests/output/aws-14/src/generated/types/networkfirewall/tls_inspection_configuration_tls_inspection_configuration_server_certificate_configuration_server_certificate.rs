#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationServerCertificate {
    /// ARN of the Certificate Manager SSL/TLS server certificate that's used for inbound SSL/TLS inspection.
    #[builder(into, default)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Box<Option<String>>,
}
