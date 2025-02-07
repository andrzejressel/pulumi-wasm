#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DefinitionAuthorization {
    /// Specifies a role definition identifier for the provider. This role will define all the permissions that the provider must have on the managed application's container resource group. This role definition cannot have permission to delete the resource group.
    #[builder(into)]
    #[serde(rename = "roleDefinitionId")]
    pub r#role_definition_id: Box<String>,
    /// Specifies a service principal identifier for the provider. This is the identity that the provider will use to call ARM to manage the managed application resources.
    #[builder(into)]
    #[serde(rename = "servicePrincipalId")]
    pub r#service_principal_id: Box<String>,
}
