/// Manages a Key Vault Managed Storage Account.
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
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: keyvaultname
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           secretPermissions:
///             - Get
///             - Delete
///           storagePermissions:
///             - Get
///             - List
///             - Set
///             - SetSAS
///             - GetSAS
///             - DeleteSAS
///             - Update
///             - RegenerateKey
///   exampleManagedStorageAccount:
///     type: azure:keyvault:ManagedStorageAccount
///     name: example
///     properties:
///       name: examplemanagedstorage
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       storageAccountKey: key1
///       regenerateKeyAutomatically: false
///       regenerationPeriod: P1D
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### Automatically Regenerate Storage Account Access Key)
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
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: keyvaultname
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           secretPermissions:
///             - Get
///             - Delete
///           storagePermissions:
///             - Get
///             - List
///             - Set
///             - SetSAS
///             - GetSAS
///             - DeleteSAS
///             - Update
///             - RegenerateKey
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Storage Account Key Operator Service Role
///       principalId: ${test.id}
///   exampleManagedStorageAccount:
///     type: azure:keyvault:ManagedStorageAccount
///     name: example
///     properties:
///       name: examplemanagedstorage
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       storageAccountKey: key1
///       regenerateKeyAutomatically: true
///       regenerationPeriod: P1D
///     options:
///       dependsOn:
///         - ${exampleAssignment}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   test:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         applicationId: cfa8b339-82a2-471a-a3c9-0fc0be7a4093
/// ```
///
/// ## Import
///
/// Key Vault Managed Storage Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/managedStorageAccount:ManagedStorageAccount example https://example-keyvault.vault.azure.net/storage/exampleStorageAcc01
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_storage_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedStorageAccountArgs {
        /// The ID of the Key Vault where the Managed Storage Account should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Key Vault Managed Storage Account. Changing this forces a new Key Vault Managed Storage Account to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should Storage Account access key be regenerated periodically?
        ///
        /// > **NOTE:** Azure Key Vault application needs to have access to Storage Account for auto regeneration to work. Example can be found above.
        #[builder(into, default)]
        pub regenerate_key_automatically: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// How often Storage Account access key should be regenerated. Value needs to be in [ISO 8601 duration format](https://en.wikipedia.org/wiki/ISO_8601#Durations).
        #[builder(into, default)]
        pub regeneration_period: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Which Storage Account access key that is managed by Key Vault. Possible values are `key1` and `key2`.
        #[builder(into)]
        pub storage_account_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Key Vault Managed Storage Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedStorageAccountResult {
        /// The ID of the Key Vault where the Managed Storage Account should be created. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Key Vault Managed Storage Account. Changing this forces a new Key Vault Managed Storage Account to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Should Storage Account access key be regenerated periodically?
        ///
        /// > **NOTE:** Azure Key Vault application needs to have access to Storage Account for auto regeneration to work. Example can be found above.
        pub regenerate_key_automatically: pulumi_gestalt_rust::Output<Option<bool>>,
        /// How often Storage Account access key should be regenerated. Value needs to be in [ISO 8601 duration format](https://en.wikipedia.org/wiki/ISO_8601#Durations).
        pub regeneration_period: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Storage Account.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// Which Storage Account access key that is managed by Key Vault. Possible values are `key1` and `key2`.
        pub storage_account_key: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Key Vault Managed Storage Account. Changing this forces a new resource to be created.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedStorageAccountArgs,
    ) -> ManagedStorageAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let regenerate_key_automatically_binding = args
            .regenerate_key_automatically
            .get_output(context);
        let regeneration_period_binding = args.regeneration_period.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let storage_account_key_binding = args.storage_account_key.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:keyvault/managedStorageAccount:ManagedStorageAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regenerateKeyAutomatically".into(),
                    value: &regenerate_key_automatically_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regenerationPeriod".into(),
                    value: &regeneration_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountKey".into(),
                    value: &storage_account_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedStorageAccountResult {
            key_vault_id: o.get_field("keyVaultId"),
            name: o.get_field("name"),
            regenerate_key_automatically: o.get_field("regenerateKeyAutomatically"),
            regeneration_period: o.get_field("regenerationPeriod"),
            storage_account_id: o.get_field("storageAccountId"),
            storage_account_key: o.get_field("storageAccountKey"),
            tags: o.get_field("tags"),
        }
    }
}
