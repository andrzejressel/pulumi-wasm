#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDefaultUserSettingsJupyterLabAppSettingsCodeRepository {
    /// The URL of the Git repository.
    #[builder(into)]
    #[serde(rename = "repositoryUrl")]
    pub r#repository_url: Box<String>,
}
