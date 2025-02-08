#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDomainSettingsRStudioServerProDomainSettings {
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block above.
    #[builder(into, default)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::DomainDomainSettingsRStudioServerProDomainSettingsDefaultResourceSpec>>,
    /// The ARN of the execution role for the RStudioServerPro Domain-level app.
    #[builder(into)]
    #[serde(rename = "domainExecutionRoleArn")]
    pub r#domain_execution_role_arn: Box<String>,
    /// A URL pointing to an RStudio Connect server.
    #[builder(into, default)]
    #[serde(rename = "rStudioConnectUrl")]
    pub r#r_studio_connect_url: Box<Option<String>>,
    /// A URL pointing to an RStudio Package Manager server.
    #[builder(into, default)]
    #[serde(rename = "rStudioPackageManagerUrl")]
    pub r#r_studio_package_manager_url: Box<Option<String>>,
}
