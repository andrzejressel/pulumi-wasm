#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipelineWorkload {
    /// Template information and additional parameters needed to launch a Dataflow job using the flex launch API.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchflextemplaterequest
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dataflowFlexTemplateRequest")]
    pub r#dataflow_flex_template_request: Box<Option<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequest>>,
    /// Template information and additional parameters needed to launch a Dataflow job using the standard launch API.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchtemplaterequest
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dataflowLaunchTemplateRequest")]
    pub r#dataflow_launch_template_request: Box<Option<super::super::types::dataflow::PipelineWorkloadDataflowLaunchTemplateRequest>>,
}
