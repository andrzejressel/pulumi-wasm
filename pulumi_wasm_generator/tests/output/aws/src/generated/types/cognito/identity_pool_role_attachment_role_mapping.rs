#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IdentityPoolRoleAttachmentRoleMapping {
    /// Specifies the action to be taken if either no rules match the claim value for the Rules type, or there is no cognito:preferred_role claim and there are multiple cognito:roles matches for the Token type. `Required` if you specify Token or Rules as the Type.
    #[builder(into, default)]
    #[serde(rename = "ambiguousRoleResolution")]
    pub r#ambiguous_role_resolution: Box<Option<String>>,
    /// A string identifying the identity provider, for example, "graph.facebook.com" or "cognito-idp.us-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id". Depends on `cognito_identity_providers` set on `aws.cognito.IdentityPool` resource or a `aws.cognito.IdentityProvider` resource.
    #[builder(into)]
    #[serde(rename = "identityProvider")]
    pub r#identity_provider: Box<String>,
    /// The Rules Configuration to be used for mapping users to roles. You can specify up to 25 rules per identity provider. Rules are evaluated in order. The first one to match specifies the role.
    #[builder(into, default)]
    #[serde(rename = "mappingRules")]
    pub r#mapping_rules: Box<Option<Vec<super::super::types::cognito::IdentityPoolRoleAttachmentRoleMappingMappingRule>>>,
    /// The role mapping type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
