#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedHardwareSecurityModuleRoleDefinitionPermission {
    /// One or more Allowed Actions, such as `*`, `Microsoft.Resources/subscriptions/resourceGroups/read`. See ['Azure Resource Manager resource provider operations'](https://docs.microsoft.com/azure/role-based-access-control/resource-provider-operations) for details.
    #[builder(into, default)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<String>>>,
    /// Specifies a list of data action permission to grant. Possible values are `Microsoft.KeyVault/managedHsm/keys/read/action`, `Microsoft.KeyVault/managedHsm/keys/write/action`, `Microsoft.KeyVault/managedHsm/keys/deletedKeys/read/action`, `Microsoft.KeyVault/managedHsm/keys/deletedKeys/recover/action`, `Microsoft.KeyVault/managedHsm/keys/backup/action`, `Microsoft.KeyVault/managedHsm/keys/restore/action`, `Microsoft.KeyVault/managedHsm/roleAssignments/delete/action`, `Microsoft.KeyVault/managedHsm/roleAssignments/read/action`, `Microsoft.KeyVault/managedHsm/roleAssignments/write/action`, `Microsoft.KeyVault/managedHsm/roleDefinitions/read/action`, `Microsoft.KeyVault/managedHsm/roleDefinitions/write/action`, `Microsoft.KeyVault/managedHsm/roleDefinitions/delete/action`, `Microsoft.KeyVault/managedHsm/keys/encrypt/action`, `Microsoft.KeyVault/managedHsm/keys/decrypt/action`, `Microsoft.KeyVault/managedHsm/keys/wrap/action`, `Microsoft.KeyVault/managedHsm/keys/unwrap/action`, `Microsoft.KeyVault/managedHsm/keys/sign/action`, `Microsoft.KeyVault/managedHsm/keys/verify/action`, `Microsoft.KeyVault/managedHsm/keys/create`, `Microsoft.KeyVault/managedHsm/keys/delete`, `Microsoft.KeyVault/managedHsm/keys/export/action`, `Microsoft.KeyVault/managedHsm/keys/release/action`, `Microsoft.KeyVault/managedHsm/keys/import/action`, `Microsoft.KeyVault/managedHsm/keys/deletedKeys/delete`, `Microsoft.KeyVault/managedHsm/securitydomain/download/action`, `Microsoft.KeyVault/managedHsm/securitydomain/download/read`, `Microsoft.KeyVault/managedHsm/securitydomain/upload/action`, `Microsoft.KeyVault/managedHsm/securitydomain/upload/read`, `Microsoft.KeyVault/managedHsm/securitydomain/transferkey/read`, `Microsoft.KeyVault/managedHsm/backup/start/action`, `Microsoft.KeyVault/managedHsm/restore/start/action`, `Microsoft.KeyVault/managedHsm/backup/status/action`, `Microsoft.KeyVault/managedHsm/restore/status/action` and `Microsoft.KeyVault/managedHsm/rng/action`.
    #[builder(into, default)]
    #[serde(rename = "dataActions")]
    pub r#data_actions: Box<Option<Vec<String>>>,
    /// One or more Disallowed Actions, such as `*`, `Microsoft.Resources/subscriptions/resourceGroups/read`. See ['Azure Resource Manager resource provider operations'](https://docs.microsoft.com/azure/role-based-access-control/resource-provider-operations) for details.
    #[builder(into, default)]
    #[serde(rename = "notActions")]
    pub r#not_actions: Box<Option<Vec<String>>>,
    /// Specifies a list of data action permission not to grant. Possible values are `Microsoft.KeyVault/managedHsm/keys/read/action`, `Microsoft.KeyVault/managedHsm/keys/write/action`, `Microsoft.KeyVault/managedHsm/keys/deletedKeys/read/action`, `Microsoft.KeyVault/managedHsm/keys/deletedKeys/recover/action`, `Microsoft.KeyVault/managedHsm/keys/backup/action`, `Microsoft.KeyVault/managedHsm/keys/restore/action`, `Microsoft.KeyVault/managedHsm/roleAssignments/delete/action`, `Microsoft.KeyVault/managedHsm/roleAssignments/read/action`, `Microsoft.KeyVault/managedHsm/roleAssignments/write/action`, `Microsoft.KeyVault/managedHsm/roleDefinitions/read/action`, `Microsoft.KeyVault/managedHsm/roleDefinitions/write/action`, `Microsoft.KeyVault/managedHsm/roleDefinitions/delete/action`, `Microsoft.KeyVault/managedHsm/keys/encrypt/action`, `Microsoft.KeyVault/managedHsm/keys/decrypt/action`, `Microsoft.KeyVault/managedHsm/keys/wrap/action`, `Microsoft.KeyVault/managedHsm/keys/unwrap/action`, `Microsoft.KeyVault/managedHsm/keys/sign/action`, `Microsoft.KeyVault/managedHsm/keys/verify/action`, `Microsoft.KeyVault/managedHsm/keys/create`, `Microsoft.KeyVault/managedHsm/keys/delete`, `Microsoft.KeyVault/managedHsm/keys/export/action`, `Microsoft.KeyVault/managedHsm/keys/release/action`, `Microsoft.KeyVault/managedHsm/keys/import/action`, `Microsoft.KeyVault/managedHsm/keys/deletedKeys/delete`, `Microsoft.KeyVault/managedHsm/securitydomain/download/action`, `Microsoft.KeyVault/managedHsm/securitydomain/download/read`, `Microsoft.KeyVault/managedHsm/securitydomain/upload/action`, `Microsoft.KeyVault/managedHsm/securitydomain/upload/read`, `Microsoft.KeyVault/managedHsm/securitydomain/transferkey/read`, `Microsoft.KeyVault/managedHsm/backup/start/action`, `Microsoft.KeyVault/managedHsm/restore/start/action`, `Microsoft.KeyVault/managedHsm/backup/status/action`, `Microsoft.KeyVault/managedHsm/restore/status/action` and `Microsoft.KeyVault/managedHsm/rng/action`.
    #[builder(into, default)]
    #[serde(rename = "notDataActions")]
    pub r#not_data_actions: Box<Option<Vec<String>>>,
}