#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigMetastoreConfig {
    /// Required. Resource name of an existing Dataproc Metastore service. Example: * `projects/`
    #[builder(into)]
    #[serde(rename = "dataprocMetastoreService")]
    pub r#dataproc_metastore_service: Box<String>,
}
