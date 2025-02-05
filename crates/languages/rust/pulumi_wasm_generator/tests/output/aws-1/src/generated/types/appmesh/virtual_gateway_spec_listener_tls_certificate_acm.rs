#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualGatewaySpecListenerTlsCertificateAcm {
    /// ARN for the certificate.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: Box<String>,
}
