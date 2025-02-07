#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryTaskDockerStep {
    /// Specifies a map of arguments to be used when executing this step.
    #[builder(into, default)]
    #[serde(rename = "arguments")]
    pub r#arguments: Box<Option<std::collections::HashMap<String, String>>>,
    /// Should the image cache be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "cacheEnabled")]
    pub r#cache_enabled: Box<Option<bool>>,
    /// The token (Git PAT or SAS token of storage account blob) associated with the context for this step.
    #[builder(into)]
    #[serde(rename = "contextAccessToken")]
    pub r#context_access_token: Box<String>,
    /// The URL (absolute or relative) of the source context for this step. If the context is an url you can reference a specific branch or folder via `#branch:folder`.
    #[builder(into)]
    #[serde(rename = "contextPath")]
    pub r#context_path: Box<String>,
    /// The Dockerfile path relative to the source context.
    #[builder(into)]
    #[serde(rename = "dockerfilePath")]
    pub r#dockerfile_path: Box<String>,
    /// Specifies a list of fully qualified image names including the repository and tag.
    #[builder(into, default)]
    #[serde(rename = "imageNames")]
    pub r#image_names: Box<Option<Vec<String>>>,
    /// Should the image built be pushed to the registry or not? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "pushEnabled")]
    pub r#push_enabled: Box<Option<bool>>,
    /// Specifies a map of *secret* arguments to be used when executing this step.
    #[builder(into, default)]
    #[serde(rename = "secretArguments")]
    pub r#secret_arguments: Box<Option<std::collections::HashMap<String, String>>>,
    /// The name of the target build stage for the docker build.
    #[builder(into, default)]
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
}
