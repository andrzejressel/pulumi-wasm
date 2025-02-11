/// Manages a Key Vault Managed Storage Account SAS Definition.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example-keyvault
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       tenantId: ${example.tenantId}
///       skuName: standard
///       accessPolicies:
///         - tenantId: ${example.tenantId}
///           objectId: ${example.objectId}
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
///   exampleManagedStorageAccountSasTokenDefinition:
///     type: azure:keyvault:ManagedStorageAccountSasTokenDefinition
///     name: example
///     properties:
///       name: examplesasdefinition
///       validityPeriod: P1D
///       managedStorageAccountId: ${exampleManagedStorageAccount.id}
///       sasTemplateUri: ${exampleGetAccountSAS.sas}
///       sasType: account
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   exampleGetAccountSAS:
///     fn::invoke:
///       function: azure:storage:getAccountSAS
///       arguments:
///         connectionString: ${exampleAccount.primaryConnectionString}
///         httpsOnly: true
///         resourceTypes:
///           service: true
///           container: false
///           object: false
///         services:
///           blob: true
///           queue: false
///           table: false
///           file: false
///         start: 2021-04-30T00:00:00Z
///         expiry: 2023-04-30T00:00:00Z
///         permissions:
///           read: true
///           write: true
///           delete: false
///           list: false
///           add: true
///           create: true
///           update: false
///           process: false
///           tag: false
///           filter: false
/// ```
///
/// ## Import
///
/// Key Vaults can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/managedStorageAccountSasTokenDefinition:ManagedStorageAccountSasTokenDefinition example https://example-keyvault.vault.azure.net/storage/exampleStorageAcc01/sas/exampleSasDefinition01
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_storage_account_sas_token_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedStorageAccountSasTokenDefinitionArgs {
        /// The ID of the Managed Storage Account.
        #[builder(into)]
        pub managed_storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this SAS Definition.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SAS definition token template signed with an arbitrary key. Tokens created according to the SAS definition will have the same properties as the template, but regenerated with a new validity period.
        #[builder(into)]
        pub sas_template_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of SAS token the SAS definition will create. Possible values are `account` and `service`.
        #[builder(into)]
        pub sas_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the SAS Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Validity period of SAS token. Value needs to be in [ISO 8601 duration format](https://en.wikipedia.org/wiki/ISO_8601#Durations).
        #[builder(into)]
        pub validity_period: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedStorageAccountSasTokenDefinitionResult {
        /// The ID of the Managed Storage Account.
        pub managed_storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this SAS Definition.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The SAS definition token template signed with an arbitrary key. Tokens created according to the SAS definition will have the same properties as the template, but regenerated with a new validity period.
        pub sas_template_uri: pulumi_gestalt_rust::Output<String>,
        /// The type of SAS token the SAS definition will create. Possible values are `account` and `service`.
        pub sas_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Secret that is created by Managed Storage Account SAS Definition.
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SAS Definition. Changing this forces a new resource to be created.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Validity period of SAS token. Value needs to be in [ISO 8601 duration format](https://en.wikipedia.org/wiki/ISO_8601#Durations).
        pub validity_period: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedStorageAccountSasTokenDefinitionArgs,
    ) -> ManagedStorageAccountSasTokenDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let managed_storage_account_id_binding = args
            .managed_storage_account_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let sas_template_uri_binding = args.sas_template_uri.get_output(context);
        let sas_type_binding = args.sas_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let validity_period_binding = args.validity_period.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:keyvault/managedStorageAccountSasTokenDefinition:ManagedStorageAccountSasTokenDefinition"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedStorageAccountId".into(),
                    value: &managed_storage_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sasTemplateUri".into(),
                    value: &sas_template_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sasType".into(),
                    value: &sas_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validityPeriod".into(),
                    value: &validity_period_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedStorageAccountSasTokenDefinitionResult {
            managed_storage_account_id: o.get_field("managedStorageAccountId"),
            name: o.get_field("name"),
            sas_template_uri: o.get_field("sasTemplateUri"),
            sas_type: o.get_field("sasType"),
            secret_id: o.get_field("secretId"),
            tags: o.get_field("tags"),
            validity_period: o.get_field("validityPeriod"),
        }
    }
}
