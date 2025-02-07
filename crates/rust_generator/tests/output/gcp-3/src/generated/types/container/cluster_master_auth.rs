#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMasterAuth {
    /// Base64 encoded public certificate
    /// used by clients to authenticate to the cluster endpoint.
    #[builder(into, default)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Box<Option<String>>,
    /// Whether client certificate authorization is enabled for this cluster.  For example:
    /// 
    #[builder(into)]
    #[serde(rename = "clientCertificateConfig")]
    pub r#client_certificate_config: Box<super::super::types::container::ClusterMasterAuthClientCertificateConfig>,
    /// Base64 encoded private key used by clients
    /// to authenticate to the cluster endpoint.
    #[builder(into, default)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<String>>,
    /// Base64 encoded public certificate
    /// that is the root certificate of the cluster.
    #[builder(into, default)]
    #[serde(rename = "clusterCaCertificate")]
    pub r#cluster_ca_certificate: Box<Option<String>>,
}
