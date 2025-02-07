#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineWorkloadDataflowFlexTemplateRequest {
    /// Parameter to launch a job from a Flex Template.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchflextemplateparameter
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "launchParameter")]
    pub r#launch_parameter: Box<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter>,
    /// The regional endpoint to which to direct the request. For example, us-central1, us-west1.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The ID of the Cloud Platform project that the job belongs to.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
    /// If true, the request is validated but not actually executed. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "validateOnly")]
    pub r#validate_only: Box<Option<bool>>,
}
