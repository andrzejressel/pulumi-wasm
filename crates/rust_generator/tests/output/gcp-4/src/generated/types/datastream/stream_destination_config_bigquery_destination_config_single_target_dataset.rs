#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StreamDestinationConfigBigqueryDestinationConfigSingleTargetDataset {
    /// Dataset ID in the format projects/{project}/datasets/{dataset_id} or
    /// {project}:{dataset_id}
    #[builder(into)]
    #[serde(rename = "datasetId")]
    pub r#dataset_id: Box<String>,
}
