/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appconf = configuration_store::create(
///         "appconf",
///         ConfigurationStoreArgs::builder()
///             .location("${example.location}")
///             .name("appConf1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Encryption)
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
///       name: example-identity
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: exampleKVt123
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       softDeleteRetentionDays: 7
///       purgeProtectionEnabled: true
///   server:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${exampleUserAssignedIdentity.principalId}
///       keyPermissions:
///         - Get
///         - UnwrapKey
///         - WrapKey
///       secretPermissions:
///         - Get
///   client:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Get
///         - Create
///         - Delete
///         - List
///         - Restore
///         - Recover
///         - UnwrapKey
///         - WrapKey
///         - Purge
///         - Encrypt
///         - Decrypt
///         - Sign
///         - Verify
///         - GetRotationPolicy
///       secretPermissions:
///         - Get
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: exampleKVkey
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
///     options:
///       dependsOn:
///         - ${client}
///         - ${server}
///   exampleConfigurationStore:
///     type: azure:appconfiguration:ConfigurationStore
///     name: example
///     properties:
///       name: appConf2
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: standard
///       localAuthEnabled: true
///       publicNetworkAccess: Enabled
///       purgeProtectionEnabled: false
///       softDeleteRetentionDays: 1
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       encryption:
///         keyVaultKeyIdentifier: ${exampleKey.id}
///         identityClientId: ${exampleUserAssignedIdentity.clientId}
///       replicas:
///         - name: replica1
///           location: West US
///       tags:
///         environment: development
///     options:
///       dependsOn:
///         - ${client}
///         - ${server}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// App Configurations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appconfiguration/configurationStore:ConfigurationStore appconf /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.AppConfiguration/configurationStores/appConf1
/// ```
///
pub mod configuration_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationStoreArgs {
        /// An `encryption` block as defined below.
        #[builder(into, default)]
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::appconfiguration::ConfigurationStoreEncryption>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appconfiguration::ConfigurationStoreIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the App Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Public Network Access setting of the App Configuration. Possible values are `Enabled` and `Disabled`.
        ///
        /// > **Note:** If `public_network_access` is not specified, the App Configuration will be created as  `Automatic`. However, once a different value is defined, can not be set again as automatic.
        #[builder(into, default)]
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether Purge Protection is enabled. This field only works for `standard` sku. Defaults to `false`.
        ///
        /// !> **Note:** Once Purge Protection has been enabled it's not possible to disable it. Deleting the App Configuration with Purge Protection enabled will schedule the App Configuration to be deleted (which will happen by Azure in the configured number of days).
        #[builder(into, default)]
        pub purge_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `replica` blocks as defined below.
        #[builder(into, default)]
        pub replicas: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appconfiguration::ConfigurationStoreReplica>>,
        >,
        /// The name of the resource group in which to create the App Configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU name of the App Configuration. Possible values are `free`, `standard` and `premium`. Defaults to `free`.
        ///
        /// > **Note:** Azure does not support downgrading `sku`. Downgrading from `premium` tier to `standard` or `free`, or from `standard` to `free`, forces a new resource to be created.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of days that items should be retained for once soft-deleted. This field only works for `standard` sku. This value can be between `1` and `7` days. Defaults to `7`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If Purge Protection is enabled, this field can only be configured one time and cannot be updated.
        #[builder(into, default)]
        pub soft_delete_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationStoreResult {
        /// An `encryption` block as defined below.
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::appconfiguration::ConfigurationStoreEncryption>,
        >,
        /// The URL of the App Configuration Replica.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appconfiguration::ConfigurationStoreIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`.
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the App Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `primary_read_key` block as defined below containing the primary read access key.
        pub primary_read_keys: pulumi_wasm_rust::Output<
            Vec<super::super::types::appconfiguration::ConfigurationStorePrimaryReadKey>,
        >,
        /// A `primary_write_key` block as defined below containing the primary write access key.
        pub primary_write_keys: pulumi_wasm_rust::Output<
            Vec<super::super::types::appconfiguration::ConfigurationStorePrimaryWriteKey>,
        >,
        /// The Public Network Access setting of the App Configuration. Possible values are `Enabled` and `Disabled`.
        ///
        /// > **Note:** If `public_network_access` is not specified, the App Configuration will be created as  `Automatic`. However, once a different value is defined, can not be set again as automatic.
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether Purge Protection is enabled. This field only works for `standard` sku. Defaults to `false`.
        ///
        /// !> **Note:** Once Purge Protection has been enabled it's not possible to disable it. Deleting the App Configuration with Purge Protection enabled will schedule the App Configuration to be deleted (which will happen by Azure in the configured number of days).
        pub purge_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `replica` blocks as defined below.
        pub replicas: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appconfiguration::ConfigurationStoreReplica>>,
        >,
        /// The name of the resource group in which to create the App Configuration. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `secondary_read_key` block as defined below containing the secondary read access key.
        pub secondary_read_keys: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::appconfiguration::ConfigurationStoreSecondaryReadKey,
            >,
        >,
        /// A `secondary_write_key` block as defined below containing the secondary write access key.
        pub secondary_write_keys: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::appconfiguration::ConfigurationStoreSecondaryWriteKey,
            >,
        >,
        /// The SKU name of the App Configuration. Possible values are `free`, `standard` and `premium`. Defaults to `free`.
        ///
        /// > **Note:** Azure does not support downgrading `sku`. Downgrading from `premium` tier to `standard` or `free`, or from `standard` to `free`, forces a new resource to be created.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of days that items should be retained for once soft-deleted. This field only works for `standard` sku. This value can be between `1` and `7` days. Defaults to `7`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If Purge Protection is enabled, this field can only be configured one time and cannot be updated.
        pub soft_delete_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConfigurationStoreArgs) -> ConfigurationStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encryption_binding = args.encryption.get_inner();
        let identity_binding = args.identity.get_inner();
        let local_auth_enabled_binding = args.local_auth_enabled.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_binding = args.public_network_access.get_inner();
        let purge_protection_enabled_binding = args.purge_protection_enabled.get_inner();
        let replicas_binding = args.replicas.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let soft_delete_retention_days_binding = args
            .soft_delete_retention_days
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appconfiguration/configurationStore:ConfigurationStore".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthEnabled".into(),
                    value: &local_auth_enabled_binding,
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
                    name: "publicNetworkAccess".into(),
                    value: &public_network_access_binding,
                },
                register_interface::ObjectField {
                    name: "purgeProtectionEnabled".into(),
                    value: &purge_protection_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "replicas".into(),
                    value: &replicas_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "softDeleteRetentionDays".into(),
                    value: &soft_delete_retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encryption".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "localAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryReadKeys".into(),
                },
                register_interface::ResultField {
                    name: "primaryWriteKeys".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccess".into(),
                },
                register_interface::ResultField {
                    name: "purgeProtectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "replicas".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryReadKeys".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWriteKeys".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "softDeleteRetentionDays".into(),
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
        ConfigurationStoreResult {
            encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryption").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_read_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryReadKeys").unwrap(),
            ),
            primary_write_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryWriteKeys").unwrap(),
            ),
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccess").unwrap(),
            ),
            purge_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purgeProtectionEnabled").unwrap(),
            ),
            replicas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicas").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_read_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryReadKeys").unwrap(),
            ),
            secondary_write_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWriteKeys").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            soft_delete_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("softDeleteRetentionDays").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}