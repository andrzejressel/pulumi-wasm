#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryTaskFileStep {
    /// The token (Git PAT or SAS token of storage account blob) associated with the context for this step.
    #[builder(into, default)]
    #[serde(rename = "contextAccessToken")]
    pub r#context_access_token: Box<Option<String>>,
    /// The URL (absolute or relative) of the source context for this step.
    #[builder(into, default)]
    #[serde(rename = "contextPath")]
    pub r#context_path: Box<Option<String>>,
    /// Specifies a map of secret values that can be passed when running a task.
    #[builder(into, default)]
    #[serde(rename = "secretValues")]
    pub r#secret_values: Box<Option<std::collections::HashMap<String, String>>>,
    /// The task template file path relative to the source context.
    #[builder(into)]
    #[serde(rename = "taskFilePath")]
    pub r#task_file_path: Box<String>,
    /// The parameters file path relative to the source context.
    #[builder(into, default)]
    #[serde(rename = "valueFilePath")]
    pub r#value_file_path: Box<Option<String>>,
    /// Specifies a map of values that can be passed when running a task.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<std::collections::HashMap<String, String>>>,
}
