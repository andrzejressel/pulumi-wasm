#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExperienceConfiguration {
    /// The identifiers of your data sources and FAQs. Or, you can specify that you want to use documents indexed via the `BatchPutDocument API`. The provider will only perform drift detection of its value when present in a configuration. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "contentSourceConfiguration")]
    pub r#content_source_configuration: Box<Option<super::super::types::kendra::ExperienceConfigurationContentSourceConfiguration>>,
    /// The AWS SSO field name that contains the identifiers of your users, such as their emails. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "userIdentityConfiguration")]
    pub r#user_identity_configuration: Box<Option<super::super::types::kendra::ExperienceConfigurationUserIdentityConfiguration>>,
}
