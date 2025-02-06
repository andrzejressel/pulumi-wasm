#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineWorkloadDataflowFlexTemplateRequestLaunchParameter {
    /// Cloud Storage path to a file with a JSON-serialized ContainerSpec as content.
    #[builder(into, default)]
    #[serde(rename = "containerSpecGcsPath")]
    pub r#container_spec_gcs_path: Box<Option<String>>,
    /// The runtime environment for the Flex Template job.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexTemplateRuntimeEnvironment
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequestLaunchParameterEnvironment>>,
    /// The job name to use for the created job. For an update job request, the job name should be the same as the existing running job.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: Box<String>,
    /// Launch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into, default)]
    #[serde(rename = "launchOptions")]
    pub r#launch_options: Box<Option<std::collections::HashMap<String, String>>>,
    /// 'The parameters for the Flex Template. Example: {"numWorkers":"5"}'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// 'Use this to pass transform name mappings for streaming update jobs. Example: {"oldTransformName":"newTransformName",...}'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into, default)]
    #[serde(rename = "transformNameMappings")]
    pub r#transform_name_mappings: Box<Option<std::collections::HashMap<String, String>>>,
    /// Set this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job.
    #[builder(into, default)]
    #[serde(rename = "update")]
    pub r#update: Box<Option<bool>>,
}
