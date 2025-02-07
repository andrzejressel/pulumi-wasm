#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetExperienceConfiguration {
    /// The identifiers of your data sources and FAQs. This is the content you want to use for your Amazon Kendra Experience. Documented below.
    #[builder(into)]
    #[serde(rename = "contentSourceConfigurations")]
    pub r#content_source_configurations: Box<Vec<super::super::types::kendra::GetExperienceConfigurationContentSourceConfiguration>>,
    /// The AWS SSO field name that contains the identifiers of your users, such as their emails. Documented below.
    #[builder(into)]
    #[serde(rename = "userIdentityConfigurations")]
    pub r#user_identity_configurations: Box<Vec<super::super::types::kendra::GetExperienceConfigurationUserIdentityConfiguration>>,
}
