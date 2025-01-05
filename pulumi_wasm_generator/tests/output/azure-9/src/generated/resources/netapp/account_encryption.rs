/// Manages a NetApp Account Encryption Resource.
///
/// For more information about Azure NetApp Files Customer-Managed Keys feature, please refer to [Configure customer-managed keys for Azure NetApp Files volume encryption](https://learn.microsoft.com/en-us/azure/azure-netapp-files/configure-customer-managed-keys)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: anf-user-assigned-identity
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: anfcmkakv
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       enabledForDiskEncryption: true
///       enabledForDeployment: true
///       enabledForTemplateDeployment: true
///       purgeProtectionEnabled: true
///       tenantId: 00000000-0000-0000-0000-000000000000
///       skuName: standard
///       accessPolicies:
///         - tenantId: 00000000-0000-0000-0000-000000000000
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Get
///             - Create
///             - Delete
///             - WrapKey
///             - UnwrapKey
///             - GetRotationPolicy
///             - SetRotationPolicy
///         - tenantId: 00000000-0000-0000-0000-000000000000
///           objectId: ${exampleUserAssignedIdentity.principalId}
///           keyPermissions:
///             - Get
///             - Encrypt
///             - Decrypt
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: anfencryptionkey
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - decrypt
///         - encrypt
///         - sign
///         - unwrapKey
///         - verify
///         - wrapKey
///   exampleAccount:
///     type: azure:netapp:Account
///     name: example
///     properties:
///       name: netappaccount
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///   exampleAccountEncryption:
///     type: azure:netapp:AccountEncryption
///     name: example
///     properties:
///       netappAccountId: ${exampleAccount.id}
///       userAssignedIdentityId: ${exampleUserAssignedIdentity.id}
///       encryptionKey: ${exampleKey.versionlessId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Account Encryption Resources can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/accountEncryption:AccountEncryption example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.NetApp/netAppAccounts/account1
/// ```
///
pub mod account_encryption {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountEncryptionArgs {
        /// Specify the versionless ID of the encryption key.
        #[builder(into)]
        pub encryption_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the NetApp account where volume under it will have customer managed keys-based encryption enabled.
        #[builder(into)]
        pub netapp_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the System Assigned Manged Identity. Conflicts with `user_assigned_identity_id`.
        #[builder(into, default)]
        pub system_assigned_identity_principal_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ID of the User Assigned Managed Identity. Conflicts with `system_assigned_identity_principal_id`.
        #[builder(into, default)]
        pub user_assigned_identity_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountEncryptionResult {
        /// Specify the versionless ID of the encryption key.
        pub encryption_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the NetApp account where volume under it will have customer managed keys-based encryption enabled.
        pub netapp_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the System Assigned Manged Identity. Conflicts with `user_assigned_identity_id`.
        pub system_assigned_identity_principal_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ID of the User Assigned Managed Identity. Conflicts with `system_assigned_identity_principal_id`.
        pub user_assigned_identity_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountEncryptionArgs) -> AccountEncryptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encryption_key_binding = args.encryption_key.get_inner();
        let netapp_account_id_binding = args.netapp_account_id.get_inner();
        let system_assigned_identity_principal_id_binding = args
            .system_assigned_identity_principal_id
            .get_inner();
        let user_assigned_identity_id_binding = args
            .user_assigned_identity_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/accountEncryption:AccountEncryption".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryptionKey".into(),
                    value: &encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "netappAccountId".into(),
                    value: &netapp_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "systemAssignedIdentityPrincipalId".into(),
                    value: &system_assigned_identity_principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "userAssignedIdentityId".into(),
                    value: &user_assigned_identity_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encryptionKey".into(),
                },
                register_interface::ResultField {
                    name: "netappAccountId".into(),
                },
                register_interface::ResultField {
                    name: "systemAssignedIdentityPrincipalId".into(),
                },
                register_interface::ResultField {
                    name: "userAssignedIdentityId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountEncryptionResult {
            encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionKey").unwrap(),
            ),
            netapp_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netappAccountId").unwrap(),
            ),
            system_assigned_identity_principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("systemAssignedIdentityPrincipalId").unwrap(),
            ),
            user_assigned_identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userAssignedIdentityId").unwrap(),
            ),
        }
    }
}
