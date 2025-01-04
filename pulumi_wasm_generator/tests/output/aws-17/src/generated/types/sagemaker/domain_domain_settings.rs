#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDomainSettings {
    /// A collection of settings that configure the domain’s Docker interaction. see `docker_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "dockerSettings")]
    pub r#docker_settings: Box<Option<super::super::types::sagemaker::DomainDomainSettingsDockerSettings>>,
    /// The configuration for attaching a SageMaker user profile name to the execution role as a sts:SourceIdentity key [AWS Docs](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html). Valid values are `USER_PROFILE_NAME` and `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "executionRoleIdentityConfig")]
    pub r#execution_role_identity_config: Box<Option<String>>,
    /// A collection of settings that configure the RStudioServerPro Domain-level app. see `r_studio_server_pro_domain_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "rStudioServerProDomainSettings")]
    pub r#r_studio_server_pro_domain_settings: Box<Option<super::super::types::sagemaker::DomainDomainSettingsRStudioServerProDomainSettings>>,
    /// The security groups for the Amazon Virtual Private Cloud that the Domain uses for communication between Domain-level apps and user apps.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
}
