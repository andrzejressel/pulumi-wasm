#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfiguration {
    /// The name of the schema entity type that's mapped to the user pool group. Defaults to `AWS::CognitoGroup`.
    #[builder(into)]
    #[serde(rename = "groupEntityType")]
    pub r#group_entity_type: Box<String>,
}
