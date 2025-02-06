/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationStoreArgs {
        /// An `encryption` block as defined below.
        #[builder(into, default)]
        pub encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appconfiguration::ConfigurationStoreEncryption>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appconfiguration::ConfigurationStoreIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the App Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Public Network Access setting of the App Configuration. Possible values are `Enabled` and `Disabled`.
        ///
        /// > **Note:** If `public_network_access` is not specified, the App Configuration will be created as  `Automatic`. However, once a different value is defined, can not be set again as automatic.
        #[builder(into, default)]
        pub public_network_access: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether Purge Protection is enabled. This field only works for `standard` sku. Defaults to `false`.
        ///
        /// !> **Note:** Once Purge Protection has been enabled it's not possible to disable it. Deleting the App Configuration with Purge Protection enabled will schedule the App Configuration to be deleted (which will happen by Azure in the configured number of days).
        #[builder(into, default)]
        pub purge_protection_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more `replica` blocks as defined below.
        #[builder(into, default)]
        pub replicas: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appconfiguration::ConfigurationStoreReplica>>,
        >,
        /// The name of the resource group in which to create the App Configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU name of the App Configuration. Possible values are `free`, `standard` and `premium`. Defaults to `free`.
        ///
        /// > **Note:** Azure does not support downgrading `sku`. Downgrading from `premium` tier to `standard` or `free`, or from `standard` to `free`, forces a new resource to be created.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of days that items should be retained for once soft-deleted. This field only works for `standard` sku. This value can be between `1` and `7` days. Defaults to `7`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If Purge Protection is enabled, this field can only be configured one time and cannot be updated.
        #[builder(into, default)]
        pub soft_delete_retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationStoreResult {
        /// An `encryption` block as defined below.
        pub encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::appconfiguration::ConfigurationStoreEncryption>,
        >,
        /// The URL of the App Configuration Replica.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::appconfiguration::ConfigurationStoreIdentity>,
        >,
        /// Whether local authentication methods is enabled. Defaults to `true`.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the App Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `primary_read_key` block as defined below containing the primary read access key.
        pub primary_read_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appconfiguration::ConfigurationStorePrimaryReadKey>,
        >,
        /// A `primary_write_key` block as defined below containing the primary write access key.
        pub primary_write_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appconfiguration::ConfigurationStorePrimaryWriteKey>,
        >,
        /// The Public Network Access setting of the App Configuration. Possible values are `Enabled` and `Disabled`.
        ///
        /// > **Note:** If `public_network_access` is not specified, the App Configuration will be created as  `Automatic`. However, once a different value is defined, can not be set again as automatic.
        pub public_network_access: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether Purge Protection is enabled. This field only works for `standard` sku. Defaults to `false`.
        ///
        /// !> **Note:** Once Purge Protection has been enabled it's not possible to disable it. Deleting the App Configuration with Purge Protection enabled will schedule the App Configuration to be deleted (which will happen by Azure in the configured number of days).
        pub purge_protection_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `replica` blocks as defined below.
        pub replicas: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appconfiguration::ConfigurationStoreReplica>>,
        >,
        /// The name of the resource group in which to create the App Configuration. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `secondary_read_key` block as defined below containing the secondary read access key.
        pub secondary_read_keys: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::appconfiguration::ConfigurationStoreSecondaryReadKey,
            >,
        >,
        /// A `secondary_write_key` block as defined below containing the secondary write access key.
        pub secondary_write_keys: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::appconfiguration::ConfigurationStoreSecondaryWriteKey,
            >,
        >,
        /// The SKU name of the App Configuration. Possible values are `free`, `standard` and `premium`. Defaults to `free`.
        ///
        /// > **Note:** Azure does not support downgrading `sku`. Downgrading from `premium` tier to `standard` or `free`, or from `standard` to `free`, forces a new resource to be created.
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of days that items should be retained for once soft-deleted. This field only works for `standard` sku. This value can be between `1` and `7` days. Defaults to `7`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If Purge Protection is enabled, this field can only be configured one time and cannot be updated.
        pub soft_delete_retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ConfigurationStoreArgs,
    ) -> ConfigurationStoreResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let encryption_binding = args.encryption.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let local_auth_enabled_binding = args
            .local_auth_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_binding = args
            .public_network_access
            .get_output(context)
            .get_inner();
        let purge_protection_enabled_binding = args
            .purge_protection_enabled
            .get_output(context)
            .get_inner();
        let replicas_binding = args.replicas.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let soft_delete_retention_days_binding = args
            .soft_delete_retention_days
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appconfiguration/configurationStore:ConfigurationStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConfigurationStoreResult {
            encryption: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryption"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            local_auth_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localAuthEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            primary_read_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryReadKeys"),
            ),
            primary_write_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryWriteKeys"),
            ),
            public_network_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccess"),
            ),
            purge_protection_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("purgeProtectionEnabled"),
            ),
            replicas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicas"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_read_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryReadKeys"),
            ),
            secondary_write_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryWriteKeys"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            soft_delete_retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("softDeleteRetentionDays"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
