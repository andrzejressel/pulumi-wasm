#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionJobTriggerInspectJobStorageConfigDatastoreOptionsPartitionId {
    /// If not empty, the ID of the namespace to which the entities belong.
    #[builder(into, default)]
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: Box<Option<String>>,
    /// The ID of the project to which the entities belong.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
}
