#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineWorkloadDataflowLaunchTemplateRequestLaunchParameters {
    /// The runtime environment for the job.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#RuntimeEnvironment
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<super::super::types::dataflow::PipelineWorkloadDataflowLaunchTemplateRequestLaunchParametersEnvironment>>,
    /// The job name to use for the created job.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: Box<String>,
    /// The runtime parameters to pass to the job.
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// Map of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job. Only applicable when updating a pipeline.
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into, default)]
    #[serde(rename = "transformNameMapping")]
    pub r#transform_name_mapping: Box<Option<std::collections::HashMap<String, String>>>,
    /// If set, replace the existing pipeline with the name specified by jobName with this pipeline, preserving state.
    #[builder(into, default)]
    #[serde(rename = "update")]
    pub r#update: Box<Option<bool>>,
}
