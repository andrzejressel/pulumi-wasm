#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkPacketCoreControlPlaneLocalDiagnosticsAccess {
    /// How to authenticate users to access local diagnostics APIs. Possible values are `AAD` and `Password`.
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: Box<String>,
    /// The versionless certificate URL used to secure local access to packet core diagnostics over local APIs by the Kubernetes ingress.
    #[builder(into, default)]
    #[serde(rename = "httpsServerCertificateUrl")]
    pub r#https_server_certificate_url: Box<Option<String>>,
}