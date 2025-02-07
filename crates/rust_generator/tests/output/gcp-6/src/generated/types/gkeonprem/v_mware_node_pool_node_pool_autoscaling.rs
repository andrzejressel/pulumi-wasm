#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareNodePoolNodePoolAutoscaling {
    /// Maximum number of replicas in the NodePool.
    #[builder(into)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<i32>,
    /// Minimum number of replicas in the NodePool.
    #[builder(into)]
    #[serde(rename = "minReplicas")]
    pub r#min_replicas: Box<i32>,
}
