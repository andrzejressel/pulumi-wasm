#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterIdentity {
    /// Nested attribute containing [OpenID Connect](https://openid.net/connect/) identity provider information for the cluster.
    #[builder(into)]
    #[serde(rename = "oidcs")]
    pub r#oidcs: Box<Vec<super::super::types::eks::GetClusterIdentityOidc>>,
}
