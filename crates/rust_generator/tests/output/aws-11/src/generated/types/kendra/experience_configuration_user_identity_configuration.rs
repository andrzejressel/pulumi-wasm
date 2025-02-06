#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExperienceConfigurationUserIdentityConfiguration {
    /// The AWS SSO field name that contains the identifiers of your users, such as their emails.
    #[builder(into)]
    #[serde(rename = "identityAttributeName")]
    pub r#identity_attribute_name: Box<String>,
}
