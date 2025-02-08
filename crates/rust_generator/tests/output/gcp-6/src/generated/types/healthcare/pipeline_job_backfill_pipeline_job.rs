#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineJobBackfillPipelineJob {
    /// Specifies the mapping pipeline job to backfill, the name format
    /// should follow: projects/{projectId}/locations/{locationId}/datasets/{datasetId}/pipelineJobs/{pipelineJobId}.
    #[builder(into, default)]
    #[serde(rename = "mappingPipelineJob")]
    pub r#mapping_pipeline_job: Box<Option<String>>,
}
