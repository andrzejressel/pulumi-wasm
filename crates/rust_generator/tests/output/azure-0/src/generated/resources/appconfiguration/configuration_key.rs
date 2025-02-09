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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationKeyArgs {
        /// Specifies the id of the App Configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub configuration_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The content type of the App Configuration Key. This should only be set when type is set to `kv`.
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// (Optional) The ETag of the key.
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the App Configuration Key to create. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The label of the App Configuration Key. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should this App Configuration Key be Locked to prevent changes?
        #[builder(into, default)]
        pub locked: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the App Configuration Key. It can either be `kv` (simple [key/value](https://docs.microsoft.com/azure/azure-app-configuration/concept-key-value)) or `vault` (where the value is a reference to a [Key Vault Secret](https://azure.microsoft.com/en-gb/services/key-vault/). Defaults to `kv`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value of the App Configuration Key. This should only be set when type is set to `kv`.
        ///
        /// > **NOTE:** `value` and `vault_key_reference` are mutually exclusive.
        #[builder(into, default)]
        pub value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the vault secret this App Configuration Key refers to. This should only be set when `type` is set to `vault`.
        ///
        /// > **NOTE:** `vault_key_reference` and `value` are mutually exclusive.
        ///
        /// > **NOTE:** When setting the `vault_key_reference` using the `id` will pin the value to specific version of the secret, to reference latest secret value use `versionless_id`
        #[builder(into, default)]
        pub vault_key_reference: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationKeyResult {
        /// Specifies the id of the App Configuration. Changing this forces a new resource to be created.
        pub configuration_store_id: pulumi_gestalt_rust::Output<String>,
        /// The content type of the App Configuration Key. This should only be set when type is set to `kv`.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// (Optional) The ETag of the key.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The name of the App Configuration Key to create. Changing this forces a new resource to be created.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The label of the App Configuration Key. Changing this forces a new resource to be created.
        pub label: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should this App Configuration Key be Locked to prevent changes?
        pub locked: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the App Configuration Key. It can either be `kv` (simple [key/value](https://docs.microsoft.com/azure/azure-app-configuration/concept-key-value)) or `vault` (where the value is a reference to a [Key Vault Secret](https://azure.microsoft.com/en-gb/services/key-vault/). Defaults to `kv`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// The value of the App Configuration Key. This should only be set when type is set to `kv`.
        ///
        /// > **NOTE:** `value` and `vault_key_reference` are mutually exclusive.
        pub value: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the vault secret this App Configuration Key refers to. This should only be set when `type` is set to `vault`.
        ///
        /// > **NOTE:** `vault_key_reference` and `value` are mutually exclusive.
        ///
        /// > **NOTE:** When setting the `vault_key_reference` using the `id` will pin the value to specific version of the secret, to reference latest secret value use `versionless_id`
        pub vault_key_reference: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationKeyArgs,
    ) -> ConfigurationKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_store_id_binding = args
            .configuration_store_id
            .get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let etag_binding = args.etag.get_output(context);
        let key_binding = args.key.get_output(context);
        let label_binding = args.label.get_output(context);
        let locked_binding = args.locked.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let value_binding = args.value.get_output(context);
        let vault_key_reference_binding = args.vault_key_reference.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appconfiguration/configurationKey:ConfigurationKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationStoreId".into(),
                    value: configuration_store_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: content_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "etag".into(),
                    value: etag_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "label".into(),
                    value: label_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locked".into(),
                    value: locked_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultKeyReference".into(),
                    value: vault_key_reference_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationKeyResult {
            configuration_store_id: o.get_field("configurationStoreId"),
            content_type: o.get_field("contentType"),
            etag: o.get_field("etag"),
            key: o.get_field("key"),
            label: o.get_field("label"),
            locked: o.get_field("locked"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            value: o.get_field("value"),
            vault_key_reference: o.get_field("vaultKeyReference"),
        }
    }
}
