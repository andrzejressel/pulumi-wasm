#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterRemoteNetworkConfigRemotePodNetwork {
    /// List of network CIDRs that can contain pods that run Kubernetes webhooks on hybrid nodes.
    #[builder(into)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Box<Vec<String>>,
}
