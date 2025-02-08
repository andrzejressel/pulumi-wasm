#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterPscConnection {
    /// Output only. The IP allocated on the consumer network for the PSC forwarding rule.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// Output only. The URI of the consumer side forwarding rule. Example: projects/{projectNumOrId}/regions/us-east1/forwardingRules/{resourceId}.
    #[builder(into, default)]
    #[serde(rename = "forwardingRule")]
    pub r#forwarding_rule: Box<Option<String>>,
    /// The consumer network where the IP address resides, in the form of projects/{projectId}/global/networks/{network_id}.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// Output only. The consumer projectId where the forwarding rule is created from.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
    /// Output only. The PSC connection id of the forwarding rule connected to the service attachment.
    #[builder(into, default)]
    #[serde(rename = "pscConnectionId")]
    pub r#psc_connection_id: Box<Option<String>>,
}
