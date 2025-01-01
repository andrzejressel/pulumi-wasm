/// Manages an Azure Cosmos DB for PostgreSQL Cluster.
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
///     let examplePostgresqlCluster = postgresql_cluster::create(
///         "examplePostgresqlCluster",
///         PostgresqlClusterArgs::builder()
///             .administrator_login_password("H@Sh1CoR3!")
///             .coordinator_storage_quota_in_mb(131072)
///             .coordinator_vcore_count(2)
///             .location("${example.location}")
///             .name("example-cluster")
///             .node_count(0)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Cosmos DB for PostgreSQL Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/postgresqlCluster:PostgresqlCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.DBforPostgreSQL/serverGroupsv2/cluster1
/// ```
///
pub mod postgresql_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostgresqlClusterArgs {
        /// The password of the administrator login. This is required when `source_resource_id` is not set.
        #[builder(into, default)]
        pub administrator_login_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The citus extension version on the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `8.3`, `9.0`, `9.1`, `9.2`, `9.3`, `9.4`, `9.5`, `10.0`, `10.1`, `10.2`, `11.0`, `11.1`, `11.2`, `11.3` and `12.1`.
        #[builder(into, default)]
        pub citus_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Is public access enabled on coordinator? Defaults to `true`.
        #[builder(into, default)]
        pub coordinator_public_ip_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The edition of the coordinator server. Possible values are `BurstableGeneralPurpose`, `BurstableMemoryOptimized`, `GeneralPurpose` and `MemoryOptimized`. Defaults to `GeneralPurpose`.
        #[builder(into, default)]
        pub coordinator_server_edition: pulumi_wasm_rust::Output<Option<String>>,
        /// The coordinator storage allowed for the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4194304`, `8388608`, `16777216`, and `33554432`.
        ///
        /// > **NOTE:** More information on [the types of compute resources available for CosmosDB can be found in the product documentation](https://learn.microsoft.com/azure/cosmos-db/postgresql/resources-compute)
        #[builder(into, default)]
        pub coordinator_storage_quota_in_mb: pulumi_wasm_rust::Output<Option<i32>>,
        /// The coordinator vCore count for the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `1`, `2`, `4`, `8`, `16`, `32`, `64` and `96`.
        #[builder(into, default)]
        pub coordinator_vcore_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Is high availability enabled for the Azure Cosmos DB for PostgreSQL cluster? Defaults to `false`.
        #[builder(into, default)]
        pub ha_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure Region where the Azure Cosmos DB for PostgreSQL Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `maintenance_window` block as defined below.
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::PostgresqlClusterMaintenanceWindow>,
        >,
        /// The name which should be used for this Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The worker node count of the Azure Cosmos DB for PostgreSQL Cluster. Possible value is between `0` and `20` except `1`.
        #[builder(into)]
        pub node_count: pulumi_wasm_rust::Output<i32>,
        /// Is public access enabled on worker nodes. Defaults to `false`.
        #[builder(into, default)]
        pub node_public_ip_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The edition of the node server. Possible values are `BurstableGeneralPurpose`, `BurstableMemoryOptimized`, `GeneralPurpose` and `MemoryOptimized`. Defaults to `MemoryOptimized`.
        #[builder(into, default)]
        pub node_server_edition: pulumi_wasm_rust::Output<Option<String>>,
        /// The storage quota in MB on each worker node. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4194304`, `8388608` and `16777216`.
        #[builder(into, default)]
        pub node_storage_quota_in_mb: pulumi_wasm_rust::Output<Option<i32>>,
        /// The vCores count on each worker node. Possible values are `1`, `2`, `4`, `8`, `16`, `32`, `64`, `96` and `104`.
        #[builder(into, default)]
        pub node_vcores: pulumi_wasm_rust::Output<Option<i32>>,
        /// The date and time in UTC (ISO8601 format) for the Azure Cosmos DB for PostgreSQL cluster restore. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub point_in_time_in_utc: pulumi_wasm_rust::Output<Option<String>>,
        /// The preferred primary availability zone for the Azure Cosmos DB for PostgreSQL cluster.
        #[builder(into, default)]
        pub preferred_primary_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Azure Cosmos DB for PostgreSQL Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Is shards on coordinator enabled for the Azure Cosmos DB for PostgreSQL cluster.
        #[builder(into, default)]
        pub shards_on_coordinator_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure region of the source Azure Cosmos DB for PostgreSQL cluster for read replica clusters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the source Azure Cosmos DB for PostgreSQL cluster for read replica clusters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The major PostgreSQL version on the Azure Cosmos DB for PostgreSQL cluster. Possible values are `11`, `12`, `13`, `14`, `15` and `16`.
        #[builder(into, default)]
        pub sql_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Azure Cosmos DB for PostgreSQL Cluster.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PostgresqlClusterResult {
        /// The password of the administrator login. This is required when `source_resource_id` is not set.
        pub administrator_login_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The citus extension version on the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `8.3`, `9.0`, `9.1`, `9.2`, `9.3`, `9.4`, `9.5`, `10.0`, `10.1`, `10.2`, `11.0`, `11.1`, `11.2`, `11.3` and `12.1`.
        pub citus_version: pulumi_wasm_rust::Output<String>,
        /// Is public access enabled on coordinator? Defaults to `true`.
        pub coordinator_public_ip_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The edition of the coordinator server. Possible values are `BurstableGeneralPurpose`, `BurstableMemoryOptimized`, `GeneralPurpose` and `MemoryOptimized`. Defaults to `GeneralPurpose`.
        pub coordinator_server_edition: pulumi_wasm_rust::Output<Option<String>>,
        /// The coordinator storage allowed for the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4194304`, `8388608`, `16777216`, and `33554432`.
        ///
        /// > **NOTE:** More information on [the types of compute resources available for CosmosDB can be found in the product documentation](https://learn.microsoft.com/azure/cosmos-db/postgresql/resources-compute)
        pub coordinator_storage_quota_in_mb: pulumi_wasm_rust::Output<Option<i32>>,
        /// The coordinator vCore count for the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `1`, `2`, `4`, `8`, `16`, `32`, `64` and `96`.
        pub coordinator_vcore_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The earliest restore point time (ISO8601 format) for the Azure Cosmos DB for PostgreSQL Cluster.
        pub earliest_restore_time: pulumi_wasm_rust::Output<String>,
        /// Is high availability enabled for the Azure Cosmos DB for PostgreSQL cluster? Defaults to `false`.
        pub ha_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure Region where the Azure Cosmos DB for PostgreSQL Cluster should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `maintenance_window` block as defined below.
        pub maintenance_window: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::PostgresqlClusterMaintenanceWindow>,
        >,
        /// The name which should be used for this Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The worker node count of the Azure Cosmos DB for PostgreSQL Cluster. Possible value is between `0` and `20` except `1`.
        pub node_count: pulumi_wasm_rust::Output<i32>,
        /// Is public access enabled on worker nodes. Defaults to `false`.
        pub node_public_ip_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The edition of the node server. Possible values are `BurstableGeneralPurpose`, `BurstableMemoryOptimized`, `GeneralPurpose` and `MemoryOptimized`. Defaults to `MemoryOptimized`.
        pub node_server_edition: pulumi_wasm_rust::Output<Option<String>>,
        /// The storage quota in MB on each worker node. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4194304`, `8388608` and `16777216`.
        pub node_storage_quota_in_mb: pulumi_wasm_rust::Output<i32>,
        /// The vCores count on each worker node. Possible values are `1`, `2`, `4`, `8`, `16`, `32`, `64`, `96` and `104`.
        pub node_vcores: pulumi_wasm_rust::Output<i32>,
        /// The date and time in UTC (ISO8601 format) for the Azure Cosmos DB for PostgreSQL cluster restore. Changing this forces a new resource to be created.
        pub point_in_time_in_utc: pulumi_wasm_rust::Output<Option<String>>,
        /// The preferred primary availability zone for the Azure Cosmos DB for PostgreSQL cluster.
        pub preferred_primary_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Azure Cosmos DB for PostgreSQL Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `servers` block as defined below.
        pub servers: pulumi_wasm_rust::Output<
            Vec<super::super::types::cosmosdb::PostgresqlClusterServer>,
        >,
        /// Is shards on coordinator enabled for the Azure Cosmos DB for PostgreSQL cluster.
        pub shards_on_coordinator_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure region of the source Azure Cosmos DB for PostgreSQL cluster for read replica clusters. Changing this forces a new resource to be created.
        pub source_location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the source Azure Cosmos DB for PostgreSQL cluster for read replica clusters. Changing this forces a new resource to be created.
        pub source_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The major PostgreSQL version on the Azure Cosmos DB for PostgreSQL cluster. Possible values are `11`, `12`, `13`, `14`, `15` and `16`.
        pub sql_version: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Cosmos DB for PostgreSQL Cluster.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PostgresqlClusterArgs) -> PostgresqlClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrator_login_password_binding = args
            .administrator_login_password
            .get_inner();
        let citus_version_binding = args.citus_version.get_inner();
        let coordinator_public_ip_access_enabled_binding = args
            .coordinator_public_ip_access_enabled
            .get_inner();
        let coordinator_server_edition_binding = args
            .coordinator_server_edition
            .get_inner();
        let coordinator_storage_quota_in_mb_binding = args
            .coordinator_storage_quota_in_mb
            .get_inner();
        let coordinator_vcore_count_binding = args.coordinator_vcore_count.get_inner();
        let ha_enabled_binding = args.ha_enabled.get_inner();
        let location_binding = args.location.get_inner();
        let maintenance_window_binding = args.maintenance_window.get_inner();
        let name_binding = args.name.get_inner();
        let node_count_binding = args.node_count.get_inner();
        let node_public_ip_access_enabled_binding = args
            .node_public_ip_access_enabled
            .get_inner();
        let node_server_edition_binding = args.node_server_edition.get_inner();
        let node_storage_quota_in_mb_binding = args.node_storage_quota_in_mb.get_inner();
        let node_vcores_binding = args.node_vcores.get_inner();
        let point_in_time_in_utc_binding = args.point_in_time_in_utc.get_inner();
        let preferred_primary_zone_binding = args.preferred_primary_zone.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let shards_on_coordinator_enabled_binding = args
            .shards_on_coordinator_enabled
            .get_inner();
        let source_location_binding = args.source_location.get_inner();
        let source_resource_id_binding = args.source_resource_id.get_inner();
        let sql_version_binding = args.sql_version.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/postgresqlCluster:PostgresqlCluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administratorLoginPassword".into(),
                    value: &administrator_login_password_binding,
                },
                register_interface::ObjectField {
                    name: "citusVersion".into(),
                    value: &citus_version_binding,
                },
                register_interface::ObjectField {
                    name: "coordinatorPublicIpAccessEnabled".into(),
                    value: &coordinator_public_ip_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "coordinatorServerEdition".into(),
                    value: &coordinator_server_edition_binding,
                },
                register_interface::ObjectField {
                    name: "coordinatorStorageQuotaInMb".into(),
                    value: &coordinator_storage_quota_in_mb_binding,
                },
                register_interface::ObjectField {
                    name: "coordinatorVcoreCount".into(),
                    value: &coordinator_vcore_count_binding,
                },
                register_interface::ObjectField {
                    name: "haEnabled".into(),
                    value: &ha_enabled_binding,
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
                    name: "nodeCount".into(),
                    value: &node_count_binding,
                },
                register_interface::ObjectField {
                    name: "nodePublicIpAccessEnabled".into(),
                    value: &node_public_ip_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "nodeServerEdition".into(),
                    value: &node_server_edition_binding,
                },
                register_interface::ObjectField {
                    name: "nodeStorageQuotaInMb".into(),
                    value: &node_storage_quota_in_mb_binding,
                },
                register_interface::ObjectField {
                    name: "nodeVcores".into(),
                    value: &node_vcores_binding,
                },
                register_interface::ObjectField {
                    name: "pointInTimeInUtc".into(),
                    value: &point_in_time_in_utc_binding,
                },
                register_interface::ObjectField {
                    name: "preferredPrimaryZone".into(),
                    value: &preferred_primary_zone_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "shardsOnCoordinatorEnabled".into(),
                    value: &shards_on_coordinator_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "sourceLocation".into(),
                    value: &source_location_binding,
                },
                register_interface::ObjectField {
                    name: "sourceResourceId".into(),
                    value: &source_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "sqlVersion".into(),
                    value: &sql_version_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorLoginPassword".into(),
                },
                register_interface::ResultField {
                    name: "citusVersion".into(),
                },
                register_interface::ResultField {
                    name: "coordinatorPublicIpAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "coordinatorServerEdition".into(),
                },
                register_interface::ResultField {
                    name: "coordinatorStorageQuotaInMb".into(),
                },
                register_interface::ResultField {
                    name: "coordinatorVcoreCount".into(),
                },
                register_interface::ResultField {
                    name: "earliestRestoreTime".into(),
                },
                register_interface::ResultField {
                    name: "haEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeCount".into(),
                },
                register_interface::ResultField {
                    name: "nodePublicIpAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "nodeServerEdition".into(),
                },
                register_interface::ResultField {
                    name: "nodeStorageQuotaInMb".into(),
                },
                register_interface::ResultField {
                    name: "nodeVcores".into(),
                },
                register_interface::ResultField {
                    name: "pointInTimeInUtc".into(),
                },
                register_interface::ResultField {
                    name: "preferredPrimaryZone".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "servers".into(),
                },
                register_interface::ResultField {
                    name: "shardsOnCoordinatorEnabled".into(),
                },
                register_interface::ResultField {
                    name: "sourceLocation".into(),
                },
                register_interface::ResultField {
                    name: "sourceResourceId".into(),
                },
                register_interface::ResultField {
                    name: "sqlVersion".into(),
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
        PostgresqlClusterResult {
            administrator_login_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLoginPassword").unwrap(),
            ),
            citus_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("citusVersion").unwrap(),
            ),
            coordinator_public_ip_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coordinatorPublicIpAccessEnabled").unwrap(),
            ),
            coordinator_server_edition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coordinatorServerEdition").unwrap(),
            ),
            coordinator_storage_quota_in_mb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coordinatorStorageQuotaInMb").unwrap(),
            ),
            coordinator_vcore_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coordinatorVcoreCount").unwrap(),
            ),
            earliest_restore_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("earliestRestoreTime").unwrap(),
            ),
            ha_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("haEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeCount").unwrap(),
            ),
            node_public_ip_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodePublicIpAccessEnabled").unwrap(),
            ),
            node_server_edition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeServerEdition").unwrap(),
            ),
            node_storage_quota_in_mb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeStorageQuotaInMb").unwrap(),
            ),
            node_vcores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeVcores").unwrap(),
            ),
            point_in_time_in_utc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pointInTimeInUtc").unwrap(),
            ),
            preferred_primary_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredPrimaryZone").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servers").unwrap(),
            ),
            shards_on_coordinator_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shardsOnCoordinatorEnabled").unwrap(),
            ),
            source_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceLocation").unwrap(),
            ),
            source_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceResourceId").unwrap(),
            ),
            sql_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlVersion").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
