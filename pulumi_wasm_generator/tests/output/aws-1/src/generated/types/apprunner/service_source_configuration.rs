#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceSourceConfiguration {
    /// Describes resources needed to authenticate access to some source repositories. See Authentication Configuration below for more details.
    #[builder(into, default)]
    #[serde(rename = "authenticationConfiguration")]
    pub r#authentication_configuration: Box<Option<super::super::types::apprunner::ServiceSourceConfigurationAuthenticationConfiguration>>,
    /// Whether continuous integration from the source repository is enabled for the App Runner service. If set to `true`, each repository change (source code commit or new image version) starts a deployment. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "autoDeploymentsEnabled")]
    pub r#auto_deployments_enabled: Box<Option<bool>>,
    /// Description of a source code repository. See Code Repository below for more details.
    #[builder(into, default)]
    #[serde(rename = "codeRepository")]
    pub r#code_repository: Box<Option<super::super::types::apprunner::ServiceSourceConfigurationCodeRepository>>,
    /// Description of a source image repository. See Image Repository below for more details.
    #[builder(into, default)]
    #[serde(rename = "imageRepository")]
    pub r#image_repository: Box<Option<super::super::types::apprunner::ServiceSourceConfigurationImageRepository>>,
}
