#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DefinitionAuthorization {
    /// The set of role definition ids which define all the permissions that the principal id can assign.
    #[builder(into, default)]
    #[serde(rename = "delegatedRoleDefinitionIds")]
    pub r#delegated_role_definition_ids: Box<Option<Vec<String>>>,
    /// The display name of the security group/service principal/user that would be assigned permissions to the projected subscription.
    #[builder(into, default)]
    #[serde(rename = "principalDisplayName")]
    pub r#principal_display_name: Box<Option<String>>,
    /// Principal ID of the security group/service principal/user that would be assigned permissions to the projected subscription.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The role definition identifier. This role will define the permissions that are granted to the principal. This cannot be an `Owner` role.
    #[builder(into)]
    #[serde(rename = "roleDefinitionId")]
    pub r#role_definition_id: Box<String>,
}
