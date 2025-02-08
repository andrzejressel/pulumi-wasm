#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetKubernetesClusterKubeAdminConfig {
    /// Base64 encoded public certificate used by clients to authenticate to the Kubernetes cluster.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Box<String>,
    /// Base64 encoded private key used by clients to authenticate to the Kubernetes cluster.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<String>,
    /// Base64 encoded public CA certificate used as the root of trust for the Kubernetes cluster.
    #[builder(into)]
    #[serde(rename = "clusterCaCertificate")]
    pub r#cluster_ca_certificate: Box<String>,
    /// The Kubernetes cluster server host.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// A password or token used to authenticate to the Kubernetes cluster.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// A username used to authenticate to the Kubernetes cluster.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
