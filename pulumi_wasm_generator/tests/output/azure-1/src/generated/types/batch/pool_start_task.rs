#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolStartTask {
    /// The command line executed by the start task.
    #[builder(into)]
    #[serde(rename = "commandLine")]
    pub r#command_line: Box<String>,
    /// A map of strings (key,value) that represents the environment variables to set in the start task.
    #[builder(into, default)]
    #[serde(rename = "commonEnvironmentProperties")]
    pub r#common_environment_properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// A `container` block is the settings for the container under which the start task runs as defined below. When this is specified, all directories recursively below the `AZ_BATCH_NODE_ROOT_DIR` (the root of Azure Batch directories on the node) are mapped into the container, all task environment variables are mapped into the container, and the task command line is executed in the container.
    #[builder(into, default)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Option<Vec<super::super::types::batch::PoolStartTaskContainer>>>,
    /// One or more `resource_file` blocks that describe the files to be downloaded to a compute node as defined below.
    #[builder(into, default)]
    #[serde(rename = "resourceFiles")]
    pub r#resource_files: Box<Option<Vec<super::super::types::batch::PoolStartTaskResourceFile>>>,
    /// The number of retry count.
    #[builder(into, default)]
    #[serde(rename = "taskRetryMaximum")]
    pub r#task_retry_maximum: Box<Option<i32>>,
    /// A `user_identity` block that describes the user identity under which the start task runs as defined below.
    #[builder(into)]
    #[serde(rename = "userIdentity")]
    pub r#user_identity: Box<super::super::types::batch::PoolStartTaskUserIdentity>,
    /// A flag that indicates if the Batch pool should wait for the start task to be completed. Default to `false`.
    #[builder(into, default)]
    #[serde(rename = "waitForSuccess")]
    pub r#wait_for_success: Box<Option<bool>>,
}
