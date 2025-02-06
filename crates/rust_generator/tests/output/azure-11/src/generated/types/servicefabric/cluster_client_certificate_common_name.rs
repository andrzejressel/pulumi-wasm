#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClientCertificateCommonName {
    /// The common or subject name of the certificate.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<String>,
    /// Does the Client Certificate have Admin Access to the cluster? Non-admin clients can only perform read only operations on the cluster.
    #[builder(into)]
    #[serde(rename = "isAdmin")]
    pub r#is_admin: Box<bool>,
    /// The Issuer Thumbprint of the Certificate.
    /// 
    /// > **NOTE:** Certificate Issuer Thumbprint may become required in the future, `https://docs.microsoft.com/azure/service-fabric/service-fabric-create-cluster-using-cert-cn#download-and-update-a-sample-template`.
    #[builder(into, default)]
    #[serde(rename = "issuerThumbprint")]
    pub r#issuer_thumbprint: Box<Option<String>>,
}
