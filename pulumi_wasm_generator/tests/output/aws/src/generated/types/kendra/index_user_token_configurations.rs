#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexUserTokenConfigurations {
    /// A block that specifies the information about the JSON token type configuration. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "jsonTokenTypeConfiguration")]
    pub r#json_token_type_configuration: Box<Option<super::super::types::kendra::IndexUserTokenConfigurationsJsonTokenTypeConfiguration>>,
    /// A block that specifies the information about the JWT token type configuration. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "jwtTokenTypeConfiguration")]
    pub r#jwt_token_type_configuration: Box<Option<super::super::types::kendra::IndexUserTokenConfigurationsJwtTokenTypeConfiguration>>,
}