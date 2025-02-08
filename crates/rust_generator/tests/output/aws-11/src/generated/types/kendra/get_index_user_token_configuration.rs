#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetIndexUserTokenConfiguration {
    /// A block that specifies the information about the JSON token type configuration.
    #[builder(into)]
    #[serde(rename = "jsonTokenTypeConfigurations")]
    pub r#json_token_type_configurations: Box<Vec<super::super::types::kendra::GetIndexUserTokenConfigurationJsonTokenTypeConfiguration>>,
    /// A block that specifies the information about the JWT token type configuration.
    #[builder(into)]
    #[serde(rename = "jwtTokenTypeConfigurations")]
    pub r#jwt_token_type_configurations: Box<Vec<super::super::types::kendra::GetIndexUserTokenConfigurationJwtTokenTypeConfiguration>>,
}
