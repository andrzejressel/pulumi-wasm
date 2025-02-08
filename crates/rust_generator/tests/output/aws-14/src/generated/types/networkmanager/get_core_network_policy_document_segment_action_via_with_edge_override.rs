#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCoreNetworkPolicyDocumentSegmentActionViaWithEdgeOverride {
    /// A list of a list of strings. The list of edges associated with the network function group.
    #[builder(into, default)]
    #[serde(rename = "edgeSets")]
    pub r#edge_sets: Box<Option<Vec<Vec<String>>>>,
    /// The preferred edge to use.
    #[builder(into, default)]
    #[serde(rename = "useEdge")]
    pub r#use_edge: Box<Option<String>>,
    /// The preferred edge to use.
    #[builder(into, default)]
    #[serde(rename = "useEdgeLocation")]
    pub r#use_edge_location: Box<Option<String>>,
}
