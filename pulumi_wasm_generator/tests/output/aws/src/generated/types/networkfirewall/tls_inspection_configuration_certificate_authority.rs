#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TlsInspectionConfigurationCertificateAuthority {
    /// ARN of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: Box<String>,
    /// Serial number of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateSerial")]
    pub r#certificate_serial: Box<String>,
    /// Status of the certificate.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// Details about the certificate status, including information about certificate errors.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: Box<String>,
}