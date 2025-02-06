#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationCheckCertificateRevocationStatus {
    #[builder(into, default)]
    #[serde(rename = "revokedStatusAction")]
    pub r#revoked_status_action: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "unknownStatusAction")]
    pub r#unknown_status_action: Box<Option<String>>,
}
