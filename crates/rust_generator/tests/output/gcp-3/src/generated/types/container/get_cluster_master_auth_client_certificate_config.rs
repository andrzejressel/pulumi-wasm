#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterMasterAuthClientCertificateConfig {
    /// Whether client certificate authorization is enabled for this cluster.
    #[builder(into)]
    #[serde(rename = "issueClientCertificate")]
    pub r#issue_client_certificate: Box<bool>,
}
