#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateJobTemplateData {
    /// The configuration settings that are used to override defaults configuration.
    #[builder(into, default)]
    #[serde(rename = "configurationOverrides")]
    pub r#configuration_overrides: Box<Option<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverrides>>,
    /// The execution role ARN of the job run.
    #[builder(into)]
    #[serde(rename = "executionRoleArn")]
    pub r#execution_role_arn: Box<String>,
    /// Specify the driver that the job runs on. Exactly one of the two available job drivers is required, either sparkSqlJobDriver or sparkSubmitJobDriver.
    #[builder(into)]
    #[serde(rename = "jobDriver")]
    pub r#job_driver: Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataJobDriver>,
    /// The tags assigned to jobs started using the job template.
    #[builder(into, default)]
    #[serde(rename = "jobTags")]
    pub r#job_tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The release version of Amazon EMR.
    #[builder(into)]
    #[serde(rename = "releaseLabel")]
    pub r#release_label: Box<String>,
}
