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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EncryptionScopeArgs,
    ) -> EncryptionScopeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let infrastructure_encryption_required_binding = args
            .infrastructure_encryption_required
            .get_output(context)
            .get_inner();
        let key_vault_key_id_binding = args
            .key_vault_key_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/encryptionScope:EncryptionScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "infrastructureEncryptionRequired".into(),
                    value: &infrastructure_encryption_required_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EncryptionScopeResult {
            infrastructure_encryption_required: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("infrastructureEncryptionRequired"),
            ),
            key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
        }
    }
}
