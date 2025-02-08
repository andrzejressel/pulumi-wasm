#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPoolStartTask {
    /// The command line executed by the start task.
    #[builder(into)]
    #[serde(rename = "commandLine")]
    pub r#command_line: Box<String>,
    /// A map of strings (key,value) that represents the environment variables to set in the start task.
    #[builder(into, default)]
    #[serde(rename = "commonEnvironmentProperties")]
    pub r#common_environment_properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// The settings for the container under which the start task runs.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Vec<super::super::types::batch::GetPoolStartTaskContainer>>,
    /// One or more `resource_file` blocks that describe the files to be downloaded to a compute node.
    #[builder(into)]
    #[serde(rename = "resourceFiles")]
    pub r#resource_files: Box<Vec<super::super::types::batch::GetPoolStartTaskResourceFile>>,
    /// The number of retry count
    #[builder(into)]
    #[serde(rename = "taskRetryMaximum")]
    pub r#task_retry_maximum: Box<i32>,
    /// A `user_identity` block that describes the user identity under which the start task runs.
    #[builder(into)]
    #[serde(rename = "userIdentities")]
    pub r#user_identities: Box<Vec<super::super::types::batch::GetPoolStartTaskUserIdentity>>,
    /// A flag that indicates if the Batch pool should wait for the start task to be completed.
    #[builder(into)]
    #[serde(rename = "waitForSuccess")]
    pub r#wait_for_success: Box<bool>,
}
