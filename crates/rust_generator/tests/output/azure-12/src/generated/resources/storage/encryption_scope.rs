/// Manages a Storage Encryption Scope.
///
/// > **Note:** Storage Encryption Scopes are in Preview [more information can be found here](https://docs.microsoft.com/azure/storage/blobs/encryption-scope-manage).
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       identity:
///         type: SystemAssigned
///   exampleEncryptionScope:
///     type: azure:storage:EncryptionScope
///     name: example
///     properties:
///       name: microsoftmanaged
///       storageAccountId: ${exampleAccount.id}
///       source: Microsoft.Storage
/// ```
///
/// ## Import
///
/// Storage Encryption Scopes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/encryptionScope:EncryptionScope example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Storage/storageAccounts/account1/encryptionScopes/scope1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod encryption_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EncryptionScopeArgs {
        /// Is a secondary layer of encryption with Platform Managed Keys for data applied? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub infrastructure_encryption_required: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the Key Vault Key. Required when `source` is `Microsoft.KeyVault`.
        #[builder(into, default)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Storage Encryption Scope. Changing this forces a new Storage Encryption Scope to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source of the Storage Encryption Scope. Possible values are `Microsoft.KeyVault` and `Microsoft.Storage`.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Storage Account where this Storage Encryption Scope is created. Changing this forces a new Storage Encryption Scope to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EncryptionScopeResult {
        /// Is a secondary layer of encryption with Platform Managed Keys for data applied? Changing this forces a new resource to be created.
        pub infrastructure_encryption_required: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The ID of the Key Vault Key. Required when `source` is `Microsoft.KeyVault`.
        pub key_vault_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Storage Encryption Scope. Changing this forces a new Storage Encryption Scope to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The source of the Storage Encryption Scope. Possible values are `Microsoft.KeyVault` and `Microsoft.Storage`.
        pub source: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account where this Storage Encryption Scope is created. Changing this forces a new Storage Encryption Scope to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EncryptionScopeArgs,
    ) -> EncryptionScopeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let infrastructure_encryption_required_binding = args
            .infrastructure_encryption_required
            .get_output(context);
        let key_vault_key_id_binding = args.key_vault_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let source_binding = args.source.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/encryptionScope:EncryptionScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "infrastructureEncryptionRequired".into(),
                    value: infrastructure_encryption_required_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultKeyId".into(),
                    value: key_vault_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EncryptionScopeResult {
            infrastructure_encryption_required: o
                .get_field("infrastructureEncryptionRequired"),
            key_vault_key_id: o.get_field("keyVaultKeyId"),
            name: o.get_field("name"),
            source: o.get_field("source"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
