#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigAutoscalingConfig {
    /// The autoscaling policy used by the cluster. Only resource names including projectid and location (region) are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/` Note that the policy must be in the same project and Dataproc region.
    #[builder(into, default)]
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<String>>,
}
