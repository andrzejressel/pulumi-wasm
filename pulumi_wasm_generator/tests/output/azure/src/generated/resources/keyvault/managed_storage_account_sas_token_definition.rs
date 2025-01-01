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
pub mod managed_storage_account_sas_token_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedStorageAccountSasTokenDefinitionArgs {
        /// The ID of the Managed Storage Account.
        #[builder(into)]
        pub managed_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this SAS Definition.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The SAS definition token template signed with an arbitrary key. Tokens created according to the SAS definition will have the same properties as the template, but regenerated with a new validity period.
        #[builder(into)]
        pub sas_template_uri: pulumi_wasm_rust::Output<String>,
        /// The type of SAS token the SAS definition will create. Possible values are `account` and `service`.
        #[builder(into)]
        pub sas_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SAS Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Validity period of SAS token. Value needs to be in [ISO 8601 duration format](https://en.wikipedia.org/wiki/ISO_8601#Durations).
        #[builder(into)]
        pub validity_period: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedStorageAccountSasTokenDefinitionResult {
        /// The ID of the Managed Storage Account.
        pub managed_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this SAS Definition.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The SAS definition token template signed with an arbitrary key. Tokens created according to the SAS definition will have the same properties as the template, but regenerated with a new validity period.
        pub sas_template_uri: pulumi_wasm_rust::Output<String>,
        /// The type of SAS token the SAS definition will create. Possible values are `account` and `service`.
        pub sas_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the Secret that is created by Managed Storage Account SAS Definition.
        pub secret_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SAS Definition. Changing this forces a new resource to be created.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Validity period of SAS token. Value needs to be in [ISO 8601 duration format](https://en.wikipedia.org/wiki/ISO_8601#Durations).
        pub validity_period: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedStorageAccountSasTokenDefinitionArgs,
    ) -> ManagedStorageAccountSasTokenDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let managed_storage_account_id_binding = args
            .managed_storage_account_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let sas_template_uri_binding = args.sas_template_uri.get_inner();
        let sas_type_binding = args.sas_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let validity_period_binding = args.validity_period.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/managedStorageAccountSasTokenDefinition:ManagedStorageAccountSasTokenDefinition"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedStorageAccountId".into(),
                    value: &managed_storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sasTemplateUri".into(),
                    value: &sas_template_uri_binding,
                },
                register_interface::ObjectField {
                    name: "sasType".into(),
                    value: &sas_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "validityPeriod".into(),
                    value: &validity_period_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "managedStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sasTemplateUri".into(),
                },
                register_interface::ResultField {
                    name: "sasType".into(),
                },
                register_interface::ResultField {
                    name: "secretId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "validityPeriod".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedStorageAccountSasTokenDefinitionResult {
            managed_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedStorageAccountId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sas_template_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sasTemplateUri").unwrap(),
            ),
            sas_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sasType").unwrap(),
            ),
            secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            validity_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validityPeriod").unwrap(),
            ),
        }
    }
}
