#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClientCertificateThumbprint {
    /// Does the Client Certificate have Admin Access to the cluster? Non-admin clients can only perform read only operations on the cluster.
    #[builder(into)]
    #[serde(rename = "isAdmin")]
    pub r#is_admin: Box<bool>,
    /// The Thumbprint associated with the Client Certificate.
    #[builder(into)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<String>,
}
