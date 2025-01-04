/// Manages an Azure Batch account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: testbatch
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: teststorage
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleAccount2:
///     type: azure:batch:Account
///     name: example
///     properties:
///       name: testbatchaccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       poolAllocationMode: BatchService
///       storageAccountId: ${exampleAccount.id}
///       storageAccountAuthenticationMode: StorageKeys
///       tags:
///         env: test
/// ```
///
/// ## Import
///
/// Batch Account can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:batch/account:Account example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Batch/batchAccounts/account1
/// ```
///
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// Specifies the allowed authentication mode for the Batch account. Possible values include `AAD`, `SharedKey` or `TaskAuthenticationToken`.
        #[builder(into, default)]
        pub allowed_authentication_modes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies if customer managed key encryption should be used to encrypt batch account data. One `encryption` block as defined below.
        #[builder(into, default)]
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::AccountEncryption>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::AccountIdentity>,
        >,
        /// A `key_vault_reference` block, as defined below, that describes the Azure KeyVault reference to use when deploying the Azure Batch account using the `UserSubscription` pool allocation mode.
        #[builder(into, default)]
        pub key_vault_reference: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::AccountKeyVaultReference>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Batch account. Only lowercase Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network_profile` block as defined below.
        #[builder(into, default)]
        pub network_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::AccountNetworkProfile>,
        >,
        /// Specifies the mode to use for pool allocation. Possible values are `BatchService` or `UserSubscription`. Defaults to `BatchService`.
        #[builder(into, default)]
        pub pool_allocation_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether public network access is allowed for this server. Defaults to `true`.
        ///
        /// > **NOTE:** When using `UserSubscription` mode, an Azure KeyVault reference has to be specified. See `key_vault_reference` below.
        ///
        /// > **NOTE:** When using `UserSubscription` mode, the `Microsoft Azure Batch` service principal has to have `Contributor` role on your subscription scope, as documented [here](https://docs.microsoft.com/azure/batch/batch-account-create-portal#additional-configuration-for-user-subscription-mode).
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Batch account. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** To work around [a bug in the Azure API](https://github.com/Azure/azure-rest-api-specs/issues/5574) this property is currently treated as case-insensitive. A future version of this provider will require that the casing is correct.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage account authentication mode. Possible values include `StorageKeys`, `BatchAccountManagedIdentity`.
        ///
        /// > **NOTE:** When using `BatchAccountManagedIdentity` mod, the `identity.type` must set to `UserAssigned` or `SystemAssigned`.
        #[builder(into, default)]
        pub storage_account_authentication_mode: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Specifies the storage account to use for the Batch account. If not specified, Azure Batch will manage the storage.
        ///
        /// > **NOTE:** When using `storage_account_id`, the `storage_account_authentication_mode` must be specified as well.
        #[builder(into, default)]
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the user assigned identity for the storage account.
        #[builder(into, default)]
        pub storage_account_node_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// The account endpoint used to interact with the Batch service.
        pub account_endpoint: pulumi_wasm_rust::Output<String>,
        /// Specifies the allowed authentication mode for the Batch account. Possible values include `AAD`, `SharedKey` or `TaskAuthenticationToken`.
        pub allowed_authentication_modes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies if customer managed key encryption should be used to encrypt batch account data. One `encryption` block as defined below.
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::AccountEncryption>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::AccountIdentity>,
        >,
        /// A `key_vault_reference` block, as defined below, that describes the Azure KeyVault reference to use when deploying the Azure Batch account using the `UserSubscription` pool allocation mode.
        pub key_vault_reference: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::AccountKeyVaultReference>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Batch account. Only lowercase Alphanumeric characters allowed. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_profile` block as defined below.
        pub network_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::AccountNetworkProfile>,
        >,
        /// Specifies the mode to use for pool allocation. Possible values are `BatchService` or `UserSubscription`. Defaults to `BatchService`.
        pub pool_allocation_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The Batch account primary access key.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// Whether public network access is allowed for this server. Defaults to `true`.
        ///
        /// > **NOTE:** When using `UserSubscription` mode, an Azure KeyVault reference has to be specified. See `key_vault_reference` below.
        ///
        /// > **NOTE:** When using `UserSubscription` mode, the `Microsoft Azure Batch` service principal has to have `Contributor` role on your subscription scope, as documented [here](https://docs.microsoft.com/azure/batch/batch-account-create-portal#additional-configuration-for-user-subscription-mode).
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Batch account. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** To work around [a bug in the Azure API](https://github.com/Azure/azure-rest-api-specs/issues/5574) this property is currently treated as case-insensitive. A future version of this provider will require that the casing is correct.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Batch account secondary access key.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage account authentication mode. Possible values include `StorageKeys`, `BatchAccountManagedIdentity`.
        ///
        /// > **NOTE:** When using `BatchAccountManagedIdentity` mod, the `identity.type` must set to `UserAssigned` or `SystemAssigned`.
        pub storage_account_authentication_mode: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Specifies the storage account to use for the Batch account. If not specified, Azure Batch will manage the storage.
        ///
        /// > **NOTE:** When using `storage_account_id`, the `storage_account_authentication_mode` must be specified as well.
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the user assigned identity for the storage account.
        pub storage_account_node_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountArgs) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_authentication_modes_binding = args
            .allowed_authentication_modes
            .get_inner();
        let encryption_binding = args.encryption.get_inner();
        let identity_binding = args.identity.get_inner();
        let key_vault_reference_binding = args.key_vault_reference.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_profile_binding = args.network_profile.get_inner();
        let pool_allocation_mode_binding = args.pool_allocation_mode.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let storage_account_authentication_mode_binding = args
            .storage_account_authentication_mode
            .get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let storage_account_node_identity_binding = args
            .storage_account_node_identity
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:batch/account:Account".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedAuthenticationModes".into(),
                    value: &allowed_authentication_modes_binding,
                },
                register_interface::ObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultReference".into(),
                    value: &key_vault_reference_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "poolAllocationMode".into(),
                    value: &pool_allocation_mode_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAuthenticationMode".into(),
                    value: &storage_account_authentication_mode_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountNodeIdentity".into(),
                    value: &storage_account_node_identity_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "allowedAuthenticationModes".into(),
                },
                register_interface::ResultField {
                    name: "encryption".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultReference".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkProfile".into(),
                },
                register_interface::ResultField {
                    name: "poolAllocationMode".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountAuthenticationMode".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountNodeIdentity".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            account_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountEndpoint").unwrap(),
            ),
            allowed_authentication_modes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedAuthenticationModes").unwrap(),
            ),
            encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryption").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            key_vault_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultReference").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkProfile").unwrap(),
            ),
            pool_allocation_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolAllocationMode").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            storage_account_authentication_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountAuthenticationMode").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            storage_account_node_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountNodeIdentity").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
