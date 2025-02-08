#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigWorkerConfigManagedGroupConfig {
    /// Output only. The name of the Instance Group Manager for this group.
    #[builder(into, default)]
    #[serde(rename = "instanceGroupManagerName")]
    pub r#instance_group_manager_name: Box<Option<String>>,
    /// Output only. The name of the Instance Template used for the Managed Instance Group.
    #[builder(into, default)]
    #[serde(rename = "instanceTemplateName")]
    pub r#instance_template_name: Box<Option<String>>,
}
