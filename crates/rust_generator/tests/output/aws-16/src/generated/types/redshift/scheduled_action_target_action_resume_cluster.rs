#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduledActionTargetActionResumeCluster {
    /// The identifier of the cluster to be resumed.
    #[builder(into)]
    #[serde(rename = "clusterIdentifier")]
    pub r#cluster_identifier: Box<String>,
}
