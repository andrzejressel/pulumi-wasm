/// Manages an Azure App Configuration Key.
///
/// > **Note:** App Configuration Keys are provisioned using a Data Plane API which requires the role `App Configuration Data Owner` on either the App Configuration or a parent scope (such as the Resource Group/Subscription). [More information can be found in the Azure Documentation for App Configuration](https://docs.microsoft.com/azure/azure-app-configuration/concept-enable-rbac#azure-built-in-roles-for-azure-app-configuration).
///
/// ## Example Usage
///
/// ### `Kv` Type
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   appconf:
///     type: azure:appconfiguration:ConfigurationStore
///     properties:
///       name: appConf1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   appconfDataowner:
///     type: azure:authorization:Assignment
///     name: appconf_dataowner
///     properties:
///       scope: ${appconf.id}
///       roleDefinitionName: App Configuration Data Owner
///       principalId: ${current.objectId}
///   test:
///     type: azure:appconfiguration:ConfigurationKey
///     properties:
///       configurationStoreId: ${appconf.id}
///       key: appConfKey1
///       label: somelabel
///       value: a test
///     options:
///       dependsOn:
///         - ${appconfDataowner}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### `Vault` Type
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   appconf:
///     type: azure:appconfiguration:ConfigurationStore
///     properties:
///       name: appConf1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   kv:
///     type: azure:keyvault:KeyVault
///     properties:
///       name: kv
///       location: ${testAzurermResourceGroup.location}
///       resourceGroupName: ${testAzurermResourceGroup.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       softDeleteRetentionDays: 7
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Create
///             - Get
///           secretPermissions:
///             - Set
///             - Get
///             - Delete
///             - Purge
///             - Recover
///   kvs:
///     type: azure:keyvault:Secret
///     properties:
///       name: kvs
///       value: szechuan
///       keyVaultId: ${kv.id}
///   appconfDataowner:
///     type: azure:authorization:Assignment
///     name: appconf_dataowner
///     properties:
///       scope: ${appconf.id}
///       roleDefinitionName: App Configuration Data Owner
///       principalId: ${current.objectId}
///   test:
///     type: azure:appconfiguration:ConfigurationKey
///     properties:
///       configurationStoreId: ${testAzurermAppConfiguration.id}
///       key: key1
///       type: vault
///       label: label1
///       vaultKeyReference: ${kvs.versionlessId}
///     options:
///       dependsOn:
///         - ${appconfDataowner}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// App Configuration Keys can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appconfiguration/configurationKey:ConfigurationKey test https://appconfname1.azconfig.io/kv/keyName?label=labelName
/// ```
///
/// If you wish to import a key with an empty label then simply leave label's name blank:
///
/// ```sh
/// $ pulumi import azure:appconfiguration/configurationKey:ConfigurationKey test https://appconfname1.azconfig.io/kv/keyName?label=
/// ```
///
pub mod configuration_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationKeyArgs {
        /// Specifies the id of the App Configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub configuration_store_id: pulumi_wasm_rust::Output<String>,
        /// The content type of the App Configuration Key. This should only be set when type is set to `kv`.
        #[builder(into, default)]
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        /// (Optional) The ETag of the key.
        #[builder(into, default)]
        pub etag: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the App Configuration Key to create. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key: pulumi_wasm_rust::Output<String>,
        /// The label of the App Configuration Key. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub label: pulumi_wasm_rust::Output<Option<String>>,
        /// Should this App Configuration Key be Locked to prevent changes?
        #[builder(into, default)]
        pub locked: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the App Configuration Key. It can either be `kv` (simple [key/value](https://docs.microsoft.com/azure/azure-app-configuration/concept-key-value)) or `vault` (where the value is a reference to a [Key Vault Secret](https://azure.microsoft.com/en-gb/services/key-vault/). Defaults to `kv`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The value of the App Configuration Key. This should only be set when type is set to `kv`.
        ///
        /// > **NOTE:** `value` and `vault_key_reference` are mutually exclusive.
        #[builder(into, default)]
        pub value: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the vault secret this App Configuration Key refers to. This should only be set when `type` is set to `vault`.
        ///
        /// > **NOTE:** `vault_key_reference` and `value` are mutually exclusive.
        ///
        /// > **NOTE:** When setting the `vault_key_reference` using the `id` will pin the value to specific version of the secret, to reference latest secret value use `versionless_id`
        #[builder(into, default)]
        pub vault_key_reference: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationKeyResult {
        /// Specifies the id of the App Configuration. Changing this forces a new resource to be created.
        pub configuration_store_id: pulumi_wasm_rust::Output<String>,
        /// The content type of the App Configuration Key. This should only be set when type is set to `kv`.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// (Optional) The ETag of the key.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The name of the App Configuration Key to create. Changing this forces a new resource to be created.
        pub key: pulumi_wasm_rust::Output<String>,
        /// The label of the App Configuration Key. Changing this forces a new resource to be created.
        pub label: pulumi_wasm_rust::Output<Option<String>>,
        /// Should this App Configuration Key be Locked to prevent changes?
        pub locked: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the App Configuration Key. It can either be `kv` (simple [key/value](https://docs.microsoft.com/azure/azure-app-configuration/concept-key-value)) or `vault` (where the value is a reference to a [Key Vault Secret](https://azure.microsoft.com/en-gb/services/key-vault/). Defaults to `kv`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The value of the App Configuration Key. This should only be set when type is set to `kv`.
        ///
        /// > **NOTE:** `value` and `vault_key_reference` are mutually exclusive.
        pub value: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the vault secret this App Configuration Key refers to. This should only be set when `type` is set to `vault`.
        ///
        /// > **NOTE:** `vault_key_reference` and `value` are mutually exclusive.
        ///
        /// > **NOTE:** When setting the `vault_key_reference` using the `id` will pin the value to specific version of the secret, to reference latest secret value use `versionless_id`
        pub vault_key_reference: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConfigurationKeyArgs) -> ConfigurationKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_store_id_binding = args.configuration_store_id.get_inner();
        let content_type_binding = args.content_type.get_inner();
        let etag_binding = args.etag.get_inner();
        let key_binding = args.key.get_inner();
        let label_binding = args.label.get_inner();
        let locked_binding = args.locked.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let value_binding = args.value.get_inner();
        let vault_key_reference_binding = args.vault_key_reference.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appconfiguration/configurationKey:ConfigurationKey".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationStoreId".into(),
                    value: &configuration_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "label".into(),
                    value: &label_binding,
                },
                register_interface::ObjectField {
                    name: "locked".into(),
                    value: &locked_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
                register_interface::ObjectField {
                    name: "vaultKeyReference".into(),
                    value: &vault_key_reference_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configurationStoreId".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "label".into(),
                },
                register_interface::ResultField {
                    name: "locked".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
                register_interface::ResultField {
                    name: "vaultKeyReference".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationKeyResult {
            configuration_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationStoreId").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("label").unwrap(),
            ),
            locked: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locked").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
            vault_key_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultKeyReference").unwrap(),
            ),
        }
    }
}