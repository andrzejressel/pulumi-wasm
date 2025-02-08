#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipelineWorkloadDataflowLaunchTemplateRequest {
    /// A Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with 'gs://'.
    #[builder(into, default)]
    #[serde(rename = "gcsPath")]
    pub r#gcs_path: Box<Option<String>>,
    /// The parameters of the template to launch. This should be part of the body of the POST request.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchtemplateparameters
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "launchParameters")]
    pub r#launch_parameters: Box<Option<super::super::types::dataflow::PipelineWorkloadDataflowLaunchTemplateRequestLaunchParameters>>,
    /// The regional endpoint to which to direct the request.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The ID of the Cloud Platform project that the job belongs to.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
    /// (Optional)
    #[builder(into, default)]
    #[serde(rename = "validateOnly")]
    pub r#validate_only: Box<Option<bool>>,
}
