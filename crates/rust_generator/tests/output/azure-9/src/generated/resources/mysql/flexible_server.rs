/// Manages a MySQL Flexible Server.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .administrator_login("psqladmin")
///             .administrator_password("H@Sh1CoR3!")
///             .backup_retention_days(7)
///             .delegated_subnet_id("${exampleSubnet.id}")
///             .location("${example.location}")
///             .name("example-fs")
///             .private_dns_zone_id("${exampleZone.id}")
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Standard_D2ds_v4")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("fs")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/virtualNetworks/subnets/join/action",])
///                     .name("Microsoft.DBforMySQL/flexibleServers").build_struct())
///                     .build_struct(),
///                 ],
///             )
///             .name("example-sn")
///             .resource_group_name("${example.name}")
///             .service_endpoints(vec!["Microsoft.Storage",])
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-vn")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("example.mysql.database.azure.com")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleZoneVirtualNetworkLink = zone_virtual_network_link::create(
///         "exampleZoneVirtualNetworkLink",
///         ZoneVirtualNetworkLinkArgs::builder()
///             .name("exampleVnetZone.com")
///             .private_dns_zone_name("${exampleZone.name}")
///             .resource_group_name("${example.name}")
///             .virtual_network_id("${exampleVirtualNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MySQL Flexible Servers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mysql/flexibleServer:FlexibleServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DBforMySQL/flexibleServers/flexibleServer1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flexible_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerArgs {
        /// The Administrator login for the MySQL Flexible Server. Required when `create_mode` is `Default`. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into, default)]
        pub administrator_login: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Password associated with the `administrator_login` for the MySQL Flexible Server. Required when `create_mode` is `Default`.
        #[builder(into, default)]
        pub administrator_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The backup retention days for the MySQL Flexible Server. Possible values are between `1` and `35` days. Defaults to `7`.
        #[builder(into, default)]
        pub backup_retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The creation mode which can be used to restore or replicate existing servers. Possible values are `Default`, `PointInTimeRestore`, `GeoRestore`, and `Replica`. Changing this forces a new MySQL Flexible Server to be created.
        ///
        /// > **NOTE:** Creating a `GeoRestore` server requires the source server with `geo_redundant_backup_enabled` enabled.
        ///
        /// > **NOTE:** When a server is first created it may not be immediately available for `geo restore` or `replica`. It may take a few minutes to several hours for the necessary metadata to be populated. Please see the [Geo Restore](https://learn.microsoft.com/azure/mysql/single-server/how-to-restore-server-portal#geo-restore) and the [Replica](https://learn.microsoft.com/azure/mysql/flexible-server/concepts-read-replicas#create-a-replica) for more information.
        #[builder(into, default)]
        pub create_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `customer_managed_key` block as defined below.
        ///
        /// > **NOTE:** `identity` is required when `customer_managed_key` is specified.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mysql::FlexibleServerCustomerManagedKey>,
        >,
        /// The ID of the virtual network subnet to create the MySQL Flexible Server. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into, default)]
        pub delegated_subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should geo redundant backup enabled? Defaults to `false`. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into, default)]
        pub geo_redundant_backup_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `high_availability` block as defined below.
        #[builder(into, default)]
        pub high_availability: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mysql::FlexibleServerHighAvailability>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mysql::FlexibleServerIdentity>,
        >,
        /// The Azure Region where the MySQL Flexible Server should exist. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `maintenance_window` block as defined below.
        #[builder(into, default)]
        pub maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mysql::FlexibleServerMaintenanceWindow>,
        >,
        /// The name which should be used for this MySQL Flexible Server. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The point in time to restore from `creation_source_server_id` when `create_mode` is `PointInTimeRestore`. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into, default)]
        pub point_in_time_restore_time_in_utc: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the private DNS zone to create the MySQL Flexible Server. Changing this forces a new MySQL Flexible Server to be created.
        ///
        /// > **NOTE:** The `private_dns_zone_id` is required when setting a `delegated_subnet_id`. The `azure.privatedns.Zone` should end with suffix `.mysql.database.azure.com`.
        #[builder(into, default)]
        pub private_dns_zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The replication role. Possible value is `None`.
        ///
        /// > **NOTE:** The `replication_role` cannot be set while creating and only can be updated from `Replica` to `None`.
        #[builder(into, default)]
        pub replication_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the MySQL Flexible Server should exist. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU Name for the MySQL Flexible Server.
        ///
        /// > **NOTE:** `sku_name` should start with SKU tier `B (Burstable)`, `GP (General Purpose)`, `MO (Memory Optimized)` like `B_Standard_B1s`.
        #[builder(into, default)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the source MySQL Flexible Server to be restored. Required when `create_mode` is `PointInTimeRestore`, `GeoRestore`, and `Replica`. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into, default)]
        pub source_server_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `storage` block as defined below.
        #[builder(into, default)]
        pub storage: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mysql::FlexibleServerStorage>,
        >,
        /// A mapping of tags which should be assigned to the MySQL Flexible Server.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of the MySQL Flexible Server to use. Possible values are `5.7`, and `8.0.21`. Changing this forces a new MySQL Flexible Server to be created.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerResult {
        /// The Administrator login for the MySQL Flexible Server. Required when `create_mode` is `Default`. Changing this forces a new MySQL Flexible Server to be created.
        pub administrator_login: pulumi_gestalt_rust::Output<String>,
        /// The Password associated with the `administrator_login` for the MySQL Flexible Server. Required when `create_mode` is `Default`.
        pub administrator_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The backup retention days for the MySQL Flexible Server. Possible values are between `1` and `35` days. Defaults to `7`.
        pub backup_retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The creation mode which can be used to restore or replicate existing servers. Possible values are `Default`, `PointInTimeRestore`, `GeoRestore`, and `Replica`. Changing this forces a new MySQL Flexible Server to be created.
        ///
        /// > **NOTE:** Creating a `GeoRestore` server requires the source server with `geo_redundant_backup_enabled` enabled.
        ///
        /// > **NOTE:** When a server is first created it may not be immediately available for `geo restore` or `replica`. It may take a few minutes to several hours for the necessary metadata to be populated. Please see the [Geo Restore](https://learn.microsoft.com/azure/mysql/single-server/how-to-restore-server-portal#geo-restore) and the [Replica](https://learn.microsoft.com/azure/mysql/flexible-server/concepts-read-replicas#create-a-replica) for more information.
        pub create_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `customer_managed_key` block as defined below.
        ///
        /// > **NOTE:** `identity` is required when `customer_managed_key` is specified.
        pub customer_managed_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::mysql::FlexibleServerCustomerManagedKey>,
        >,
        /// The ID of the virtual network subnet to create the MySQL Flexible Server. Changing this forces a new MySQL Flexible Server to be created.
        pub delegated_subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fully qualified domain name of the MySQL Flexible Server.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// Should geo redundant backup enabled? Defaults to `false`. Changing this forces a new MySQL Flexible Server to be created.
        pub geo_redundant_backup_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `high_availability` block as defined below.
        pub high_availability: pulumi_gestalt_rust::Output<
            Option<super::super::types::mysql::FlexibleServerHighAvailability>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::mysql::FlexibleServerIdentity>,
        >,
        /// The Azure Region where the MySQL Flexible Server should exist. Changing this forces a new MySQL Flexible Server to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `maintenance_window` block as defined below.
        pub maintenance_window: pulumi_gestalt_rust::Output<
            Option<super::super::types::mysql::FlexibleServerMaintenanceWindow>,
        >,
        /// The name which should be used for this MySQL Flexible Server. Changing this forces a new MySQL Flexible Server to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The point in time to restore from `creation_source_server_id` when `create_mode` is `PointInTimeRestore`. Changing this forces a new MySQL Flexible Server to be created.
        pub point_in_time_restore_time_in_utc: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of the private DNS zone to create the MySQL Flexible Server. Changing this forces a new MySQL Flexible Server to be created.
        ///
        /// > **NOTE:** The `private_dns_zone_id` is required when setting a `delegated_subnet_id`. The `azure.privatedns.Zone` should end with suffix `.mysql.database.azure.com`.
        pub private_dns_zone_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is the public network access enabled?
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The maximum number of replicas that a primary MySQL Flexible Server can have.
        pub replica_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The replication role. Possible value is `None`.
        ///
        /// > **NOTE:** The `replication_role` cannot be set while creating and only can be updated from `Replica` to `None`.
        pub replication_role: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the MySQL Flexible Server should exist. Changing this forces a new MySQL Flexible Server to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU Name for the MySQL Flexible Server.
        ///
        /// > **NOTE:** `sku_name` should start with SKU tier `B (Burstable)`, `GP (General Purpose)`, `MO (Memory Optimized)` like `B_Standard_B1s`.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the source MySQL Flexible Server to be restored. Required when `create_mode` is `PointInTimeRestore`, `GeoRestore`, and `Replica`. Changing this forces a new MySQL Flexible Server to be created.
        pub source_server_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `storage` block as defined below.
        pub storage: pulumi_gestalt_rust::Output<
            super::super::types::mysql::FlexibleServerStorage,
        >,
        /// A mapping of tags which should be assigned to the MySQL Flexible Server.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of the MySQL Flexible Server to use. Possible values are `5.7`, and `8.0.21`. Changing this forces a new MySQL Flexible Server to be created.
        pub version: pulumi_gestalt_rust::Output<String>,
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlexibleServerArgs,
    ) -> FlexibleServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let administrator_login_binding = args.administrator_login.get_output(context);
        let administrator_password_binding = args
            .administrator_password
            .get_output(context);
        let backup_retention_days_binding = args
            .backup_retention_days
            .get_output(context);
        let create_mode_binding = args.create_mode.get_output(context);
        let customer_managed_key_binding = args.customer_managed_key.get_output(context);
        let delegated_subnet_id_binding = args.delegated_subnet_id.get_output(context);
        let geo_redundant_backup_enabled_binding = args
            .geo_redundant_backup_enabled
            .get_output(context);
        let high_availability_binding = args.high_availability.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let maintenance_window_binding = args.maintenance_window.get_output(context);
        let name_binding = args.name.get_output(context);
        let point_in_time_restore_time_in_utc_binding = args
            .point_in_time_restore_time_in_utc
            .get_output(context);
        let private_dns_zone_id_binding = args.private_dns_zone_id.get_output(context);
        let replication_role_binding = args.replication_role.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let source_server_id_binding = args.source_server_id.get_output(context);
        let storage_binding = args.storage.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let version_binding = args.version.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mysql/flexibleServer:FlexibleServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "administratorLogin".into(),
                    value: administrator_login_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "administratorPassword".into(),
                    value: administrator_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRetentionDays".into(),
                    value: backup_retention_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createMode".into(),
                    value: create_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKey".into(),
                    value: customer_managed_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegatedSubnetId".into(),
                    value: delegated_subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "geoRedundantBackupEnabled".into(),
                    value: geo_redundant_backup_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "highAvailability".into(),
                    value: high_availability_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceWindow".into(),
                    value: maintenance_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pointInTimeRestoreTimeInUtc".into(),
                    value: point_in_time_restore_time_in_utc_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateDnsZoneId".into(),
                    value: private_dns_zone_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationRole".into(),
                    value: replication_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceServerId".into(),
                    value: source_server_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storage".into(),
                    value: storage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlexibleServerResult {
            administrator_login: o.get_field("administratorLogin"),
            administrator_password: o.get_field("administratorPassword"),
            backup_retention_days: o.get_field("backupRetentionDays"),
            create_mode: o.get_field("createMode"),
            customer_managed_key: o.get_field("customerManagedKey"),
            delegated_subnet_id: o.get_field("delegatedSubnetId"),
            fqdn: o.get_field("fqdn"),
            geo_redundant_backup_enabled: o.get_field("geoRedundantBackupEnabled"),
            high_availability: o.get_field("highAvailability"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            maintenance_window: o.get_field("maintenanceWindow"),
            name: o.get_field("name"),
            point_in_time_restore_time_in_utc: o
                .get_field("pointInTimeRestoreTimeInUtc"),
            private_dns_zone_id: o.get_field("privateDnsZoneId"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            replica_capacity: o.get_field("replicaCapacity"),
            replication_role: o.get_field("replicationRole"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            source_server_id: o.get_field("sourceServerId"),
            storage: o.get_field("storage"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
            zone: o.get_field("zone"),
        }
    }
}
