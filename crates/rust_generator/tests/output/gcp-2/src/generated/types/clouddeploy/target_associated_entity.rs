#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TargetAssociatedEntity {
    /// Optional. Information specifying Anthos clusters as associated entities.
    #[builder(into, default)]
    #[serde(rename = "anthosClusters")]
    pub r#anthos_clusters: Box<Option<Vec<super::super::types::clouddeploy::TargetAssociatedEntityAnthosCluster>>>,
    /// The name for the key in the map for which this object is mapped to in the API
    #[builder(into)]
    #[serde(rename = "entityId")]
    pub r#entity_id: Box<String>,
    /// Optional. Information specifying GKE clusters as associated entities.
    #[builder(into, default)]
    #[serde(rename = "gkeClusters")]
    pub r#gke_clusters: Box<Option<Vec<super::super::types::clouddeploy::TargetAssociatedEntityGkeCluster>>>,
}
