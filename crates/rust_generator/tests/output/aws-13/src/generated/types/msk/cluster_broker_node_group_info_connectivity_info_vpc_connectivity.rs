#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity {
    /// Configuration block for specifying a client authentication. See below.
    #[builder(into, default)]
    #[serde(rename = "clientAuthentication")]
    pub r#client_authentication: Box<Option<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication>>,
}
