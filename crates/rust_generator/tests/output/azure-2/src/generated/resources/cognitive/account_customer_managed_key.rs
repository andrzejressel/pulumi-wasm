/// Manages a Customer Managed Key for a Cognitive Services Account.
///
/// > **NOTE:** It's possible to define a Customer Managed Key both within the `azure.cognitive.Account` resource via the `customer_managed_key` block and by using the `azure.cognitive.AccountCustomerManagedKey` resource. However it's not possible to use both methods to manage a Customer Managed Key for a Cognitive Account, since there'll be conflicts.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West US
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       name: example-identity
///   exampleAccount:
///     type: azure:cognitive:Account
///     name: example
///     properties:
///       name: example-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       kind: Face
///       skuName: E0
///       customSubdomainName: example-account
///       identity:
///         type: SystemAssigned, UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example-vault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       purgeProtectionEnabled: true
///       accessPolicies:
///         - tenantId: ${exampleAccount.identity.tenantId}
///           objectId: ${exampleAccount.identity.principalId}
///           keyPermissions:
///             - Get
///             - Create
///             - List
///             - Restore
///             - Recover
///             - UnwrapKey
///             - WrapKey
///             - Purge
///             - Encrypt
///             - Decrypt
///             - Sign
///             - Verify
///           secretPermissions:
///             - Get
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Get
///             - Create
///             - Delete
///             - List
///             - Restore
///             - Recover
///             - UnwrapKey
///             - WrapKey
///             - Purge
///             - Encrypt
///             - Decrypt
///             - Sign
///             - Verify
///             - GetRotationPolicy
///           secretPermissions:
///             - Get
///         - tenantId: ${exampleUserAssignedIdentity.tenantId}
///           objectId: ${exampleUserAssignedIdentity.principalId}
///           keyPermissions:
///             - Get
///             - Create
///             - Delete
///             - List
///             - Restore
///             - Recover
///             - UnwrapKey
///             - WrapKey
///             - Purge
///             - Encrypt
///             - Decrypt
///             - Sign
///             - Verify
///           secretPermissions:
///             - Get
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: example-key
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
///   exampleAccountCustomerManagedKey:
///     type: azure:cognitive:AccountCustomerManagedKey
///     name: example
///     properties:
///       cognitiveAccountId: ${exampleAccount.id}
///       keyVaultKeyId: ${exampleKey.id}
///       identityClientId: ${exampleUserAssignedIdentity.clientId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Customer Managed Keys for a Cognitive Account can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cognitive/accountCustomerManagedKey:AccountCustomerManagedKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.CognitiveServices/accounts/account1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_customer_managed_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountCustomerManagedKeyArgs {
        /// The ID of the Cognitive Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cognitive_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Client ID of the User Assigned Identity that has access to the key. This property only needs to be specified when there're multiple identities attached to the Cognitive Account.
        #[builder(into, default)]
        pub identity_client_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Key Vault Key which should be used to Encrypt the data in this Cognitive Account.
        #[builder(into)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccountCustomerManagedKeyResult {
        /// The ID of the Cognitive Account. Changing this forces a new resource to be created.
        pub cognitive_account_id: pulumi_gestalt_rust::Output<String>,
        /// The Client ID of the User Assigned Identity that has access to the key. This property only needs to be specified when there're multiple identities attached to the Cognitive Account.
        pub identity_client_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Key Vault Key which should be used to Encrypt the data in this Cognitive Account.
        pub key_vault_key_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountCustomerManagedKeyArgs,
    ) -> AccountCustomerManagedKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cognitive_account_id_binding = args.cognitive_account_id.get_output(context);
        let identity_client_id_binding = args.identity_client_id.get_output(context);
        let key_vault_key_id_binding = args.key_vault_key_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cognitive/accountCustomerManagedKey:AccountCustomerManagedKey"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cognitiveAccountId".into(),
                    value: cognitive_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityClientId".into(),
                    value: identity_client_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultKeyId".into(),
                    value: key_vault_key_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountCustomerManagedKeyResult {
            cognitive_account_id: o.get_field("cognitiveAccountId"),
            identity_client_id: o.get_field("identityClientId"),
            key_vault_key_id: o.get_field("keyVaultKeyId"),
        }
    }
}
