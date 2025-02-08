#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServerTlsPolicyServerCertificateGrpcEndpoint {
    /// The target URI of the gRPC endpoint. Only UDS path is supported, and should start with "unix:".
    #[builder(into)]
    #[serde(rename = "targetUri")]
    pub r#target_uri: Box<String>,
}
