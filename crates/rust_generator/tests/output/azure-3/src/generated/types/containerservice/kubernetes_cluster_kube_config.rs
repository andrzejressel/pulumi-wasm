#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterKubeConfig {
    /// Base64 encoded public certificate used by clients to authenticate to the Kubernetes cluster.
    #[builder(into, default)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Box<Option<String>>,
    /// Base64 encoded private key used by clients to authenticate to the Kubernetes cluster.
    #[builder(into, default)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<String>>,
    /// Base64 encoded public CA certificate used as the root of trust for the Kubernetes cluster.
    #[builder(into, default)]
    #[serde(rename = "clusterCaCertificate")]
    pub r#cluster_ca_certificate: Box<Option<String>>,
    /// The Kubernetes cluster server host.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// A password or token used to authenticate to the Kubernetes cluster.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// A username used to authenticate to the Kubernetes cluster.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
