#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigAutoscalingConfig {
    /// The autoscaling policy used by the cluster.
    /// 
    /// Only resource names including projectid and location (region) are valid. Examples:
    /// 
    /// `https://www.googleapis.com/compute/v1/projects/[projectId]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]`
    /// `projects/[projectId]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]`
    /// Note that the policy must be in the same project and Cloud Dataproc region.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "policyUri")]
    pub r#policy_uri: Box<String>,
}
