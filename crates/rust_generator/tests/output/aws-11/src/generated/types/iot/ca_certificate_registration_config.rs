#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CaCertificateRegistrationConfig {
    /// The ARN of the role.
    #[builder(into, default)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<Option<String>>,
    /// The template body.
    #[builder(into, default)]
    #[serde(rename = "templateBody")]
    pub r#template_body: Box<Option<String>>,
    /// The name of the provisioning template.
    #[builder(into, default)]
    #[serde(rename = "templateName")]
    pub r#template_name: Box<Option<String>>,
}
