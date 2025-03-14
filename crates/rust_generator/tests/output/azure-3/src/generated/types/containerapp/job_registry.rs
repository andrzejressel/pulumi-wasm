#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobRegistry {
    /// A Managed Identity to use to authenticate with Azure Container Registry.
    #[builder(into, default)]
    #[serde(rename = "identity")]
    pub r#identity: Box<Option<String>>,
    /// The name of the Secret that contains the registry login password.
    #[builder(into, default)]
    #[serde(rename = "passwordSecretName")]
    pub r#password_secret_name: Box<Option<String>>,
    /// The URL of the Azure Container Registry server.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: Box<String>,
    /// The username to use to authenticate with Azure Container Registry.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
