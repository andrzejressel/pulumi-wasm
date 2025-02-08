#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegistryTaskEncodedStep {
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
    /// The (optionally base64 encoded) content of the build template.
    #[builder(into)]
    #[serde(rename = "taskContent")]
    pub r#task_content: Box<String>,
    /// The (optionally base64 encoded) content of the build parameters.
    #[builder(into, default)]
    #[serde(rename = "valueContent")]
    pub r#value_content: Box<Option<String>>,
    /// Specifies a map of values that can be passed when running a task.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<std::collections::HashMap<String, String>>>,
}
