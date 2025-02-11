/// Manages a CosmosDB (formally DocumentDB) Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   rg:
///     type: azure:core:ResourceGroup
///     properties:
///       name: sample-rg
///       location: westus
///   ri:
///     type: random:RandomInteger
///     properties:
///       min: 10000
///       max: 99999
///   db:
///     type: azure:cosmosdb:Account
///     properties:
///       name: tfex-cosmos-db-${ri.result}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       offerType: Standard
///       kind: MongoDB
///       automaticFailoverEnabled: true
///       capabilities:
///         - name: EnableAggregationPipeline
///         - name: mongoEnableDocLevelTTL
///         - name: MongoDBv3.4
///         - name: EnableMongo
///       consistencyPolicy:
///         consistencyLevel: BoundedStaleness
///         maxIntervalInSeconds: 300
///         maxStalenessPrefix: 100000
///       geoLocations:
///         - location: eastus
///           failoverPriority: 1
///         - location: westus
///           failoverPriority: 0
/// ```
///
/// ## User Assigned Identity Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:authorization:UserAssignedIdentity
///     properties:
///       resourceGroupName: ${exampleAzurermResourceGroup.name}
///       location: ${exampleAzurermResourceGroup.location}
///       name: example-resource
///   exampleAccount:
///     type: azure:cosmosdb:Account
///     name: example
///     properties:
///       name: example-resource
///       location: ${exampleAzurermResourceGroup.location}
///       resourceGroupName: ${exampleAzurermResourceGroup.name}
///       defaultIdentityType:
///         fn::invoke:
///           function: std:join
///           arguments:
///             separator: =
///             input:
///               - UserAssignedIdentity
///               - ${example.id}
///           return: result
///       offerType: Standard
///       kind: MongoDB
///       capabilities:
///         - name: EnableMongo
///       consistencyPolicy:
///         consistencyLevel: Strong
///       geoLocations:
///         - location: westus
///           failoverPriority: 0
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${example.id}
/// ```
///
/// ## Import
///
/// CosmosDB Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/account:Account account1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        #[builder(into, default)]
        pub access_key_metadata_writes_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `analytical_storage` block as defined below.
        #[builder(into, default)]
        pub analytical_storage: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::AccountAnalyticalStorage>,
        >,
        #[builder(into, default)]
        pub analytical_storage_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub automatic_failover_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub backup: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::AccountBackup>,
        >,
        #[builder(into, default)]
        pub burst_capacity_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub capabilities: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cosmosdb::AccountCapability>>,
        >,
        /// A `capacity` block as defined below.
        #[builder(into, default)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::AccountCapacity>,
        >,
        #[builder(into)]
        pub consistency_policy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cosmosdb::AccountConsistencyPolicy,
        >,
        #[builder(into, default)]
        pub cors_rule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::AccountCorsRule>,
        >,
        /// The creation mode for the CosmosDB Account. Possible values are `Default` and `Restore`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `create_mode` can only be defined when the `backup.type` is set to `Continuous`.
        #[builder(into, default)]
        pub create_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default identity for accessing Key Vault. Possible values are `FirstPartyIdentity`, `SystemAssignedIdentity` or `UserAssignedIdentity`. Defaults to `FirstPartyIdentity`.
        #[builder(into, default)]
        pub default_identity_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub free_tier_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into)]
        pub geo_locations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::cosmosdb::AccountGeoLocation>,
        >,
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::AccountIdentity>,
        >,
        #[builder(into, default)]
        pub ip_range_filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub is_virtual_network_filter_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub local_authentication_disabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub managed_hsm_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the minimal TLS version for the CosmosDB account. Possible values are: `Tls`, `Tls11`, and `Tls12`. Defaults to `Tls12`.
        ///
        /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more details.
        #[builder(into, default)]
        pub minimal_tls_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub mongo_server_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub multiple_write_locations_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the name of the CosmosDB Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub network_acl_bypass_for_azure_services: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub network_acl_bypass_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the Offer Type to use for this CosmosDB Account; currently, this can only be set to `Standard`.
        #[builder(into)]
        pub offer_type: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub partition_merge_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which the CosmosDB Account is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub restore: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::AccountRestore>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub virtual_network_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cosmosdb::AccountVirtualNetworkRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        pub access_key_metadata_writes_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// An `analytical_storage` block as defined below.
        pub analytical_storage: pulumi_gestalt_rust::Output<
            super::super::types::cosmosdb::AccountAnalyticalStorage,
        >,
        pub analytical_storage_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub automatic_failover_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub backup: pulumi_gestalt_rust::Output<
            super::super::types::cosmosdb::AccountBackup,
        >,
        pub burst_capacity_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub capabilities: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cosmosdb::AccountCapability>,
        >,
        /// A `capacity` block as defined below.
        pub capacity: pulumi_gestalt_rust::Output<
            super::super::types::cosmosdb::AccountCapacity,
        >,
        pub consistency_policy: pulumi_gestalt_rust::Output<
            super::super::types::cosmosdb::AccountConsistencyPolicy,
        >,
        pub cors_rule: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::AccountCorsRule>,
        >,
        /// The creation mode for the CosmosDB Account. Possible values are `Default` and `Restore`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `create_mode` can only be defined when the `backup.type` is set to `Continuous`.
        pub create_mode: pulumi_gestalt_rust::Output<String>,
        /// The default identity for accessing Key Vault. Possible values are `FirstPartyIdentity`, `SystemAssignedIdentity` or `UserAssignedIdentity`. Defaults to `FirstPartyIdentity`.
        pub default_identity_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The endpoint used to connect to the CosmosDB account.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        pub free_tier_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub geo_locations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cosmosdb::AccountGeoLocation>,
        >,
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::AccountIdentity>,
        >,
        pub ip_range_filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub is_virtual_network_filter_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub key_vault_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub kind: pulumi_gestalt_rust::Output<Option<String>>,
        pub local_authentication_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub managed_hsm_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the minimal TLS version for the CosmosDB account. Possible values are: `Tls`, `Tls11`, and `Tls12`. Defaults to `Tls12`.
        ///
        /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more details.
        pub minimal_tls_version: pulumi_gestalt_rust::Output<Option<String>>,
        pub mongo_server_version: pulumi_gestalt_rust::Output<String>,
        pub multiple_write_locations_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the CosmosDB Account. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_acl_bypass_for_azure_services: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        pub network_acl_bypass_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the Offer Type to use for this CosmosDB Account; currently, this can only be set to `Standard`.
        pub offer_type: pulumi_gestalt_rust::Output<String>,
        pub partition_merge_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Primary key for the CosmosDB Account.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// Primary Mongodb connection string for the CosmosDB Account.
        pub primary_mongodb_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Primary read-only Key for the CosmosDB Account.
        pub primary_readonly_key: pulumi_gestalt_rust::Output<String>,
        /// Primary readonly Mongodb connection string for the CosmosDB Account.
        pub primary_readonly_mongodb_connection_string: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Primary readonly SQL connection string for the CosmosDB Account.
        pub primary_readonly_sql_connection_string: pulumi_gestalt_rust::Output<String>,
        /// Primary SQL connection string for the CosmosDB Account.
        pub primary_sql_connection_string: pulumi_gestalt_rust::Output<String>,
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of read endpoints available for this CosmosDB account.
        pub read_endpoints: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the resource group in which the CosmosDB Account is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub restore: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::AccountRestore>,
        >,
        /// The Secondary key for the CosmosDB Account.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Secondary Mongodb connection string for the CosmosDB Account.
        pub secondary_mongodb_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Secondary read-only key for the CosmosDB Account.
        pub secondary_readonly_key: pulumi_gestalt_rust::Output<String>,
        /// Secondary readonly Mongodb connection string for the CosmosDB Account.
        pub secondary_readonly_mongodb_connection_string: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Secondary readonly SQL connection string for the CosmosDB Account.
        pub secondary_readonly_sql_connection_string: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Secondary SQL connection string for the CosmosDB Account.
        pub secondary_sql_connection_string: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub virtual_network_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cosmosdb::AccountVirtualNetworkRule>>,
        >,
        /// A list of write endpoints available for this CosmosDB account.
        pub write_endpoints: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_key_metadata_writes_enabled_binding = args
            .access_key_metadata_writes_enabled
            .get_output(context);
        let analytical_storage_binding = args.analytical_storage.get_output(context);
        let analytical_storage_enabled_binding = args
            .analytical_storage_enabled
            .get_output(context);
        let automatic_failover_enabled_binding = args
            .automatic_failover_enabled
            .get_output(context);
        let backup_binding = args.backup.get_output(context);
        let burst_capacity_enabled_binding = args
            .burst_capacity_enabled
            .get_output(context);
        let capabilities_binding = args.capabilities.get_output(context);
        let capacity_binding = args.capacity.get_output(context);
        let consistency_policy_binding = args.consistency_policy.get_output(context);
        let cors_rule_binding = args.cors_rule.get_output(context);
        let create_mode_binding = args.create_mode.get_output(context);
        let default_identity_type_binding = args
            .default_identity_type
            .get_output(context);
        let free_tier_enabled_binding = args.free_tier_enabled.get_output(context);
        let geo_locations_binding = args.geo_locations.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let ip_range_filters_binding = args.ip_range_filters.get_output(context);
        let is_virtual_network_filter_enabled_binding = args
            .is_virtual_network_filter_enabled
            .get_output(context);
        let key_vault_key_id_binding = args.key_vault_key_id.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let local_authentication_disabled_binding = args
            .local_authentication_disabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_hsm_key_id_binding = args.managed_hsm_key_id.get_output(context);
        let minimal_tls_version_binding = args.minimal_tls_version.get_output(context);
        let mongo_server_version_binding = args.mongo_server_version.get_output(context);
        let multiple_write_locations_enabled_binding = args
            .multiple_write_locations_enabled
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let network_acl_bypass_for_azure_services_binding = args
            .network_acl_bypass_for_azure_services
            .get_output(context);
        let network_acl_bypass_ids_binding = args
            .network_acl_bypass_ids
            .get_output(context);
        let offer_type_binding = args.offer_type.get_output(context);
        let partition_merge_enabled_binding = args
            .partition_merge_enabled
            .get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let restore_binding = args.restore.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_network_rules_binding = args
            .virtual_network_rules
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessKeyMetadataWritesEnabled".into(),
                    value: &access_key_metadata_writes_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analyticalStorage".into(),
                    value: &analytical_storage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analyticalStorageEnabled".into(),
                    value: &analytical_storage_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticFailoverEnabled".into(),
                    value: &automatic_failover_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backup".into(),
                    value: &backup_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "burstCapacityEnabled".into(),
                    value: &burst_capacity_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capabilities".into(),
                    value: &capabilities_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consistencyPolicy".into(),
                    value: &consistency_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "corsRule".into(),
                    value: &cors_rule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createMode".into(),
                    value: &create_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultIdentityType".into(),
                    value: &default_identity_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "freeTierEnabled".into(),
                    value: &free_tier_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "geoLocations".into(),
                    value: &geo_locations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipRangeFilters".into(),
                    value: &ip_range_filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isVirtualNetworkFilterEnabled".into(),
                    value: &is_virtual_network_filter_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: &kind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthenticationDisabled".into(),
                    value: &local_authentication_disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedHsmKeyId".into(),
                    value: &managed_hsm_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimalTlsVersion".into(),
                    value: &minimal_tls_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mongoServerVersion".into(),
                    value: &mongo_server_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multipleWriteLocationsEnabled".into(),
                    value: &multiple_write_locations_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkAclBypassForAzureServices".into(),
                    value: &network_acl_bypass_for_azure_services_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkAclBypassIds".into(),
                    value: &network_acl_bypass_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "offerType".into(),
                    value: &offer_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionMergeEnabled".into(),
                    value: &partition_merge_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restore".into(),
                    value: &restore_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkRules".into(),
                    value: &virtual_network_rules_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountResult {
            access_key_metadata_writes_enabled: o
                .get_field("accessKeyMetadataWritesEnabled"),
            analytical_storage: o.get_field("analyticalStorage"),
            analytical_storage_enabled: o.get_field("analyticalStorageEnabled"),
            automatic_failover_enabled: o.get_field("automaticFailoverEnabled"),
            backup: o.get_field("backup"),
            burst_capacity_enabled: o.get_field("burstCapacityEnabled"),
            capabilities: o.get_field("capabilities"),
            capacity: o.get_field("capacity"),
            consistency_policy: o.get_field("consistencyPolicy"),
            cors_rule: o.get_field("corsRule"),
            create_mode: o.get_field("createMode"),
            default_identity_type: o.get_field("defaultIdentityType"),
            endpoint: o.get_field("endpoint"),
            free_tier_enabled: o.get_field("freeTierEnabled"),
            geo_locations: o.get_field("geoLocations"),
            identity: o.get_field("identity"),
            ip_range_filters: o.get_field("ipRangeFilters"),
            is_virtual_network_filter_enabled: o
                .get_field("isVirtualNetworkFilterEnabled"),
            key_vault_key_id: o.get_field("keyVaultKeyId"),
            kind: o.get_field("kind"),
            local_authentication_disabled: o.get_field("localAuthenticationDisabled"),
            location: o.get_field("location"),
            managed_hsm_key_id: o.get_field("managedHsmKeyId"),
            minimal_tls_version: o.get_field("minimalTlsVersion"),
            mongo_server_version: o.get_field("mongoServerVersion"),
            multiple_write_locations_enabled: o
                .get_field("multipleWriteLocationsEnabled"),
            name: o.get_field("name"),
            network_acl_bypass_for_azure_services: o
                .get_field("networkAclBypassForAzureServices"),
            network_acl_bypass_ids: o.get_field("networkAclBypassIds"),
            offer_type: o.get_field("offerType"),
            partition_merge_enabled: o.get_field("partitionMergeEnabled"),
            primary_key: o.get_field("primaryKey"),
            primary_mongodb_connection_string: o
                .get_field("primaryMongodbConnectionString"),
            primary_readonly_key: o.get_field("primaryReadonlyKey"),
            primary_readonly_mongodb_connection_string: o
                .get_field("primaryReadonlyMongodbConnectionString"),
            primary_readonly_sql_connection_string: o
                .get_field("primaryReadonlySqlConnectionString"),
            primary_sql_connection_string: o.get_field("primarySqlConnectionString"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            read_endpoints: o.get_field("readEndpoints"),
            resource_group_name: o.get_field("resourceGroupName"),
            restore: o.get_field("restore"),
            secondary_key: o.get_field("secondaryKey"),
            secondary_mongodb_connection_string: o
                .get_field("secondaryMongodbConnectionString"),
            secondary_readonly_key: o.get_field("secondaryReadonlyKey"),
            secondary_readonly_mongodb_connection_string: o
                .get_field("secondaryReadonlyMongodbConnectionString"),
            secondary_readonly_sql_connection_string: o
                .get_field("secondaryReadonlySqlConnectionString"),
            secondary_sql_connection_string: o.get_field("secondarySqlConnectionString"),
            tags: o.get_field("tags"),
            virtual_network_rules: o.get_field("virtualNetworkRules"),
            write_endpoints: o.get_field("writeEndpoints"),
        }
    }
}
