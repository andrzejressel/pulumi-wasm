#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkPacketCoreControlPlaneLocalDiagnosticsAccess {
    /// How to authenticate users who access local diagnostics APIs.
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: Box<String>,
    /// The versionless certificate URL used to secure local access to packet core diagnostics over local APIs by the Kubernetes ingress.
    #[builder(into)]
    #[serde(rename = "httpsServerCertificateUrl")]
    pub r#https_server_certificate_url: Box<String>,
}
