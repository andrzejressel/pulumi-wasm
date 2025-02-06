#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexUserTokenConfigurationsJsonTokenTypeConfiguration {
    /// The group attribute field. Minimum length of 1. Maximum length of 2048.
    #[builder(into)]
    #[serde(rename = "groupAttributeField")]
    pub r#group_attribute_field: Box<String>,
    /// The user name attribute field. Minimum length of 1. Maximum length of 2048.
    #[builder(into)]
    #[serde(rename = "userNameAttributeField")]
    pub r#user_name_attribute_field: Box<String>,
}
