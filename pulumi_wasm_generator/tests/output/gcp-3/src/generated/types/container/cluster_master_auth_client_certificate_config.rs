#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMasterAuthClientCertificateConfig {
    /// Whether client certificate authorization is enabled for this cluster.
    #[builder(into)]
    #[serde(rename = "issueClientCertificate")]
    pub r#issue_client_certificate: Box<bool>,
}
