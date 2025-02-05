/// Manages a PostgreSQL Flexible Server.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
///             .delegated_subnet_id("${exampleSubnet.id}")
///             .location("${example.location}")
///             .name("example-psqlflexibleserver")
///             .private_dns_zone_id("${exampleZone.id}")
///             .public_network_access_enabled(false)
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Standard_D4s_v3")
///             .storage_mb(32768)
///             .storage_tier("P30")
///             .version("12")
///             .zone("1")
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
///                     .name("Microsoft.DBforPostgreSQL/flexibleServers").build_struct())
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
///             .name("example.postgres.database.azure.com")
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
/// ## `storage_tier` defaults based on `storage_mb`
///
/// | `storage_mb` | GiB   | TiB | Default | Supported `storage_tier`'s           | Provisioned `IOPS`  |
/// |:------------:|:-----:|:---:|:-------:|:------------------------------------:|:-------------------:|
/// | 32768        | 32    |  -  | P4      | P4, P6, P10, P15, P20, P30, P40, P50 | 120                 |
/// | 65536        | 64    |  -  | P6      | P6, P10, P15, P20, P30, P40, P50     | 240                 |
/// | 131072       | 128   |  -  | P10     | P10, P15, P20, P30, P40, P50         | 500                 |
/// | 262144       | 256   |  -  | P15     | P15, P20, P30, P40, P50              | 1,100               |
/// | 524288       | 512   |  -  | P20     | P20, P30, P40, P50                   | 2,300               |
/// | 1048576      | 1024  |  1  | P30     | P30, P40, P50                        | 5,000               |
/// | 2097152      | 2048  |  2  | P40     | P40, P50                             | 7,500               |
/// | 4193280      | 4095  |  4  | P50     | P50                                  | 7,500               |
/// | 4194304      | 4096  |  4  | P50     | P50                                  | 7,500               |
/// | 8388608      | 8192  |  8  | P60     | P60, P70                             | 16,000              |
/// | 16777216     | 16384 |  16 | P70     | P70, P80                             | 18,000              |
/// | 33553408     | 32767 |  32 | P80     | P80                                  | 20,000              |
///
/// > **Note:** Host Caching (ReadOnly and Read/Write) is supported on disk sizes less than 4194304 MiB. This means any disk that is provisioned up to 4193280 MiB can take advantage of Host Caching. Host caching is not supported for disk sizes larger than 4193280 MiB. For example, a P50 premium disk provisioned at 4193280 GiB can take advantage of Host caching while a P50 disk provisioned at 4194304 MiB cannot. Moving from a smaller disk size to a larger disk size, greater than 4193280 MiB, will cause the disk to lose the disk caching ability.
///
/// ---
///
/// ## Import
///
/// PostgreSQL Flexible Servers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/flexibleServer:FlexibleServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.DBforPostgreSQL/flexibleServers/server1
/// ```
///
pub mod flexible_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerArgs {
        /// The Administrator login for the PostgreSQL Flexible Server. Required when `create_mode` is `Default` and `authentication.password_auth_enabled` is `true`.
        ///
        /// > **Note:** Once `administrator_login` is specified, changing this forces a new PostgreSQL Flexible Server to be created.
        ///
        /// > **Note:** To create with `administrator_login` specified or update with it first specified , `authentication.password_auth_enabled` must be set to `true`.
        #[builder(into, default)]
        pub administrator_login: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Password associated with the `administrator_login` for the PostgreSQL Flexible Server. Required when `create_mode` is `Default` and `authentication.password_auth_enabled` is `true`.
        #[builder(into, default)]
        pub administrator_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An `authentication` block as defined below.
        #[builder(into, default)]
        pub authentication: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::postgresql::FlexibleServerAuthentication>,
        >,
        /// Is the storage auto grow for PostgreSQL Flexible Server enabled? Defaults to `false`.
        #[builder(into, default)]
        pub auto_grow_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The backup retention days for the PostgreSQL Flexible Server. Possible values are between `7` and `35` days.
        #[builder(into, default)]
        pub backup_retention_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The creation mode which can be used to restore or replicate existing servers. Possible values are `Default`, `GeoRestore`, `PointInTimeRestore`, `Replica` and `Update`. Changing this forces a new PostgreSQL Flexible Server to be created.
        ///
        /// > **Note:** `create_mode` cannot be changed once it's set since it's a parameter at creation.
        ///
        /// > **Note:** While creating the resource, `create_mode` cannot be set to `Update`.
        #[builder(into, default)]
        pub create_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `customer_managed_key` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::postgresql::FlexibleServerCustomerManagedKey>,
        >,
        /// The ID of the virtual network subnet to create the PostgreSQL Flexible Server. The provided subnet should not have any other resource deployed in it and this subnet will be delegated to the PostgreSQL Flexible Server, if not already delegated. Changing this forces a new PostgreSQL Flexible Server to be created.
        #[builder(into, default)]
        pub delegated_subnet_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Is Geo-Redundant backup enabled on the PostgreSQL Flexible Server. Defaults to `false`. Changing this forces a new PostgreSQL Flexible Server to be created.
        #[builder(into, default)]
        pub geo_redundant_backup_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A `high_availability` block as defined below.
        #[builder(into, default)]
        pub high_availability: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::postgresql::FlexibleServerHighAvailability>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::postgresql::FlexibleServerIdentity>,
        >,
        /// The Azure Region where the PostgreSQL Flexible Server should exist. Changing this forces a new PostgreSQL Flexible Server to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `maintenance_window` block as defined below.
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::postgresql::FlexibleServerMaintenanceWindow>,
        >,
        /// The name which should be used for this PostgreSQL Flexible Server. Changing this forces a new PostgreSQL Flexible Server to be created.
        ///
        /// > **Note** This must be unique across the entire Azure service, not just within the resource group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The point in time to restore from `source_server_id` when `create_mode` is `GeoRestore`, `PointInTimeRestore`. Changing this forces a new PostgreSQL Flexible Server to be created.
        #[builder(into, default)]
        pub point_in_time_restore_time_in_utc: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the private DNS zone to create the PostgreSQL Flexible Server.
        ///
        /// > **Note:** There will be a breaking change from upstream service at 15th July 2021, the `private_dns_zone_id` will be required when setting a `delegated_subnet_id`. For existing flexible servers who don't want to be recreated, you need to provide the `private_dns_zone_id` to the service team to manually migrate to the specified private DNS zone. The `azure.privatedns.Zone` should end with suffix `.postgres.database.azure.com`.
        #[builder(into, default)]
        pub private_dns_zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies whether this PostgreSQL Flexible Server is publicly accessible. Defaults to `true`.
        ///
        /// > **Note:** `public_network_access_enabled` must be set to `false` when `delegated_subnet_id` and `private_dns_zone_id` have a value.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The replication role for the PostgreSQL Flexible Server. Possible value is `None`.
        ///
        /// > **Note:** The `replication_role` cannot be set while creating and only can be updated to `None` for replica server.
        #[builder(into, default)]
        pub replication_role: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the PostgreSQL Flexible Server should exist. Changing this forces a new PostgreSQL Flexible Server to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The SKU Name for the PostgreSQL Flexible Server. The name of the SKU, follows the `tier` + `name` pattern (e.g. `B_Standard_B1ms`, `GP_Standard_D2s_v3`, `MO_Standard_E4s_v3`).
        #[builder(into, default)]
        pub sku_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the source PostgreSQL Flexible Server to be restored. Required when `create_mode` is `GeoRestore`, `PointInTimeRestore` or `Replica`. Changing this forces a new PostgreSQL Flexible Server to be created.
        #[builder(into, default)]
        pub source_server_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The max storage allowed for the PostgreSQL Flexible Server. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4193280`, `4194304`, `8388608`, `16777216` and `33553408`.
        ///
        /// > **Note:** If the `storage_mb` field is undefined on the initial deployment of the PostgreSQL Flexible Server resource it will default to `32768`. If the `storage_mb` field has been defined and then removed, the `storage_mb` field will retain the previously defined value.
        ///
        /// > **Note:** The `storage_mb` can only be scaled up, for example, you can scale the `storage_mb` from `32768` to `65536`, but not from `65536` to `32768`.
        #[builder(into, default)]
        pub storage_mb: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The name of storage performance tier for IOPS of the PostgreSQL Flexible Server. Possible values are `P4`, `P6`, `P10`, `P15`,`P20`, `P30`,`P40`, `P50`,`P60`, `P70` or `P80`. Default value is dependant on the `storage_mb` value. Please see the `storage_tier` defaults based on `storage_mb` table below.
        ///
        /// > **Note:** The `storage_tier` can be scaled once every 12 hours, this restriction is in place to ensure stability and performance after any changes to your PostgreSQL Flexible Server's configuration.
        #[builder(into, default)]
        pub storage_tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the PostgreSQL Flexible Server.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of PostgreSQL Flexible Server to use. Possible values are `11`,`12`, `13`, `14`, `15` and `16`. Required when `create_mode` is `Default`.
        ///
        /// > **Note:** When `create_mode` is `Update`, upgrading version wouldn't force a new resource to be created.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerResult {
        /// The Administrator login for the PostgreSQL Flexible Server. Required when `create_mode` is `Default` and `authentication.password_auth_enabled` is `true`.
        ///
        /// > **Note:** Once `administrator_login` is specified, changing this forces a new PostgreSQL Flexible Server to be created.
        ///
        /// > **Note:** To create with `administrator_login` specified or update with it first specified , `authentication.password_auth_enabled` must be set to `true`.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// The Password associated with the `administrator_login` for the PostgreSQL Flexible Server. Required when `create_mode` is `Default` and `authentication.password_auth_enabled` is `true`.
        pub administrator_password: pulumi_wasm_rust::Output<Option<String>>,
        /// An `authentication` block as defined below.
        pub authentication: pulumi_wasm_rust::Output<
            super::super::types::postgresql::FlexibleServerAuthentication,
        >,
        /// Is the storage auto grow for PostgreSQL Flexible Server enabled? Defaults to `false`.
        pub auto_grow_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The backup retention days for the PostgreSQL Flexible Server. Possible values are between `7` and `35` days.
        pub backup_retention_days: pulumi_wasm_rust::Output<i32>,
        /// The creation mode which can be used to restore or replicate existing servers. Possible values are `Default`, `GeoRestore`, `PointInTimeRestore`, `Replica` and `Update`. Changing this forces a new PostgreSQL Flexible Server to be created.
        ///
        /// > **Note:** `create_mode` cannot be changed once it's set since it's a parameter at creation.
        ///
        /// > **Note:** While creating the resource, `create_mode` cannot be set to `Update`.
        pub create_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// A `customer_managed_key` block as defined below. Changing this forces a new resource to be created.
        pub customer_managed_key: pulumi_wasm_rust::Output<
            Option<super::super::types::postgresql::FlexibleServerCustomerManagedKey>,
        >,
        /// The ID of the virtual network subnet to create the PostgreSQL Flexible Server. The provided subnet should not have any other resource deployed in it and this subnet will be delegated to the PostgreSQL Flexible Server, if not already delegated. Changing this forces a new PostgreSQL Flexible Server to be created.
        pub delegated_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The FQDN of the PostgreSQL Flexible Server.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// Is Geo-Redundant backup enabled on the PostgreSQL Flexible Server. Defaults to `false`. Changing this forces a new PostgreSQL Flexible Server to be created.
        pub geo_redundant_backup_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `high_availability` block as defined below.
        pub high_availability: pulumi_wasm_rust::Output<
            Option<super::super::types::postgresql::FlexibleServerHighAvailability>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::postgresql::FlexibleServerIdentity>,
        >,
        /// The Azure Region where the PostgreSQL Flexible Server should exist. Changing this forces a new PostgreSQL Flexible Server to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `maintenance_window` block as defined below.
        pub maintenance_window: pulumi_wasm_rust::Output<
            Option<super::super::types::postgresql::FlexibleServerMaintenanceWindow>,
        >,
        /// The name which should be used for this PostgreSQL Flexible Server. Changing this forces a new PostgreSQL Flexible Server to be created.
        ///
        /// > **Note** This must be unique across the entire Azure service, not just within the resource group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The point in time to restore from `source_server_id` when `create_mode` is `GeoRestore`, `PointInTimeRestore`. Changing this forces a new PostgreSQL Flexible Server to be created.
        pub point_in_time_restore_time_in_utc: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the private DNS zone to create the PostgreSQL Flexible Server.
        ///
        /// > **Note:** There will be a breaking change from upstream service at 15th July 2021, the `private_dns_zone_id` will be required when setting a `delegated_subnet_id`. For existing flexible servers who don't want to be recreated, you need to provide the `private_dns_zone_id` to the service team to manually migrate to the specified private DNS zone. The `azure.privatedns.Zone` should end with suffix `.postgres.database.azure.com`.
        pub private_dns_zone_id: pulumi_wasm_rust::Output<String>,
        /// Specifies whether this PostgreSQL Flexible Server is publicly accessible. Defaults to `true`.
        ///
        /// > **Note:** `public_network_access_enabled` must be set to `false` when `delegated_subnet_id` and `private_dns_zone_id` have a value.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The replication role for the PostgreSQL Flexible Server. Possible value is `None`.
        ///
        /// > **Note:** The `replication_role` cannot be set while creating and only can be updated to `None` for replica server.
        pub replication_role: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the PostgreSQL Flexible Server should exist. Changing this forces a new PostgreSQL Flexible Server to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU Name for the PostgreSQL Flexible Server. The name of the SKU, follows the `tier` + `name` pattern (e.g. `B_Standard_B1ms`, `GP_Standard_D2s_v3`, `MO_Standard_E4s_v3`).
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the source PostgreSQL Flexible Server to be restored. Required when `create_mode` is `GeoRestore`, `PointInTimeRestore` or `Replica`. Changing this forces a new PostgreSQL Flexible Server to be created.
        pub source_server_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The max storage allowed for the PostgreSQL Flexible Server. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4193280`, `4194304`, `8388608`, `16777216` and `33553408`.
        ///
        /// > **Note:** If the `storage_mb` field is undefined on the initial deployment of the PostgreSQL Flexible Server resource it will default to `32768`. If the `storage_mb` field has been defined and then removed, the `storage_mb` field will retain the previously defined value.
        ///
        /// > **Note:** The `storage_mb` can only be scaled up, for example, you can scale the `storage_mb` from `32768` to `65536`, but not from `65536` to `32768`.
        pub storage_mb: pulumi_wasm_rust::Output<i32>,
        /// The name of storage performance tier for IOPS of the PostgreSQL Flexible Server. Possible values are `P4`, `P6`, `P10`, `P15`,`P20`, `P30`,`P40`, `P50`,`P60`, `P70` or `P80`. Default value is dependant on the `storage_mb` value. Please see the `storage_tier` defaults based on `storage_mb` table below.
        ///
        /// > **Note:** The `storage_tier` can be scaled once every 12 hours, this restriction is in place to ensure stability and performance after any changes to your PostgreSQL Flexible Server's configuration.
        pub storage_tier: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the PostgreSQL Flexible Server.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of PostgreSQL Flexible Server to use. Possible values are `11`,`12`, `13`, `14`, `15` and `16`. Required when `create_mode` is `Default`.
        ///
        /// > **Note:** When `create_mode` is `Update`, upgrading version wouldn't force a new resource to be created.
        pub version: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FlexibleServerArgs,
    ) -> FlexibleServerResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrator_login_binding = args
            .administrator_login
            .get_output(context)
            .get_inner();
        let administrator_password_binding = args
            .administrator_password
            .get_output(context)
            .get_inner();
        let authentication_binding = args.authentication.get_output(context).get_inner();
        let auto_grow_enabled_binding = args
            .auto_grow_enabled
            .get_output(context)
            .get_inner();
        let backup_retention_days_binding = args
            .backup_retention_days
            .get_output(context)
            .get_inner();
        let create_mode_binding = args.create_mode.get_output(context).get_inner();
        let customer_managed_key_binding = args
            .customer_managed_key
            .get_output(context)
            .get_inner();
        let delegated_subnet_id_binding = args
            .delegated_subnet_id
            .get_output(context)
            .get_inner();
        let geo_redundant_backup_enabled_binding = args
            .geo_redundant_backup_enabled
            .get_output(context)
            .get_inner();
        let high_availability_binding = args
            .high_availability
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let maintenance_window_binding = args
            .maintenance_window
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let point_in_time_restore_time_in_utc_binding = args
            .point_in_time_restore_time_in_utc
            .get_output(context)
            .get_inner();
        let private_dns_zone_id_binding = args
            .private_dns_zone_id
            .get_output(context)
            .get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let replication_role_binding = args
            .replication_role
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let source_server_id_binding = args
            .source_server_id
            .get_output(context)
            .get_inner();
        let storage_mb_binding = args.storage_mb.get_output(context).get_inner();
        let storage_tier_binding = args.storage_tier.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/flexibleServer:FlexibleServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administratorLogin".into(),
                    value: &administrator_login_binding,
                },
                register_interface::ObjectField {
                    name: "administratorPassword".into(),
                    value: &administrator_password_binding,
                },
                register_interface::ObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding,
                },
                register_interface::ObjectField {
                    name: "autoGrowEnabled".into(),
                    value: &auto_grow_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "backupRetentionDays".into(),
                    value: &backup_retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "createMode".into(),
                    value: &create_mode_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKey".into(),
                    value: &customer_managed_key_binding,
                },
                register_interface::ObjectField {
                    name: "delegatedSubnetId".into(),
                    value: &delegated_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "geoRedundantBackupEnabled".into(),
                    value: &geo_redundant_backup_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "highAvailability".into(),
                    value: &high_availability_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pointInTimeRestoreTimeInUtc".into(),
                    value: &point_in_time_restore_time_in_utc_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsZoneId".into(),
                    value: &private_dns_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "replicationRole".into(),
                    value: &replication_role_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceServerId".into(),
                    value: &source_server_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageMb".into(),
                    value: &storage_mb_binding,
                },
                register_interface::ObjectField {
                    name: "storageTier".into(),
                    value: &storage_tier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FlexibleServerResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("administratorLogin"),
            ),
            administrator_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("administratorPassword"),
            ),
            authentication: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authentication"),
            ),
            auto_grow_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoGrowEnabled"),
            ),
            backup_retention_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupRetentionDays"),
            ),
            create_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createMode"),
            ),
            customer_managed_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerManagedKey"),
            ),
            delegated_subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("delegatedSubnetId"),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(o.extract_field("fqdn")),
            geo_redundant_backup_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("geoRedundantBackupEnabled"),
            ),
            high_availability: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("highAvailability"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenanceWindow"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            point_in_time_restore_time_in_utc: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pointInTimeRestoreTimeInUtc"),
            ),
            private_dns_zone_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateDnsZoneId"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            replication_role: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationRole"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            source_server_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceServerId"),
            ),
            storage_mb: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageMb"),
            ),
            storage_tier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageTier"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
