#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateJobTemplateDataJobDriverSparkSubmitJobDriver {
    /// The entry point of job application.
    #[builder(into)]
    #[serde(rename = "entryPoint")]
    pub r#entry_point: Box<String>,
    /// The arguments for job application.
    #[builder(into, default)]
    #[serde(rename = "entryPointArguments")]
    pub r#entry_point_arguments: Box<Option<Vec<String>>>,
    /// The Spark submit parameters that are used for job runs.
    #[builder(into, default)]
    #[serde(rename = "sparkSubmitParameters")]
    pub r#spark_submit_parameters: Box<Option<String>>,
}
