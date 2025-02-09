/// Manages an Azure Cosmos DB for PostgreSQL Cluster.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod postgresql_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostgresqlClusterArgs {
        /// The password of the administrator login. This is required when `source_resource_id` is not set.
        #[builder(into, default)]
        pub administrator_login_password: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The citus extension version on the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `8.3`, `9.0`, `9.1`, `9.2`, `9.3`, `9.4`, `9.5`, `10.0`, `10.1`, `10.2`, `11.0`, `11.1`, `11.2`, `11.3` and `12.1`.
        #[builder(into, default)]
        pub citus_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is public access enabled on coordinator? Defaults to `true`.
        #[builder(into, default)]
        pub coordinator_public_ip_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The edition of the coordinator server. Possible values are `BurstableGeneralPurpose`, `BurstableMemoryOptimized`, `GeneralPurpose` and `MemoryOptimized`. Defaults to `GeneralPurpose`.
        #[builder(into, default)]
        pub coordinator_server_edition: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The coordinator storage allowed for the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4194304`, `8388608`, `16777216`, and `33554432`.
        ///
        /// > **NOTE:** More information on [the types of compute resources available for CosmosDB can be found in the product documentation](https://learn.microsoft.com/azure/cosmos-db/postgresql/resources-compute)
        #[builder(into, default)]
        pub coordinator_storage_quota_in_mb: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The coordinator vCore count for the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `1`, `2`, `4`, `8`, `16`, `32`, `64` and `96`.
        #[builder(into, default)]
        pub coordinator_vcore_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Is high availability enabled for the Azure Cosmos DB for PostgreSQL cluster? Defaults to `false`.
        #[builder(into, default)]
        pub ha_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Azure Region where the Azure Cosmos DB for PostgreSQL Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `maintenance_window` block as defined below.
        #[builder(into, default)]
        pub maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::PostgresqlClusterMaintenanceWindow>,
        >,
        /// The name which should be used for this Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The worker node count of the Azure Cosmos DB for PostgreSQL Cluster. Possible value is between `0` and `20` except `1`.
        #[builder(into)]
        pub node_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Is public access enabled on worker nodes. Defaults to `false`.
        #[builder(into, default)]
        pub node_public_ip_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The edition of the node server. Possible values are `BurstableGeneralPurpose`, `BurstableMemoryOptimized`, `GeneralPurpose` and `MemoryOptimized`. Defaults to `MemoryOptimized`.
        #[builder(into, default)]
        pub node_server_edition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The storage quota in MB on each worker node. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4194304`, `8388608` and `16777216`.
        #[builder(into, default)]
        pub node_storage_quota_in_mb: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The vCores count on each worker node. Possible values are `1`, `2`, `4`, `8`, `16`, `32`, `64`, `96` and `104`.
        #[builder(into, default)]
        pub node_vcores: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The date and time in UTC (ISO8601 format) for the Azure Cosmos DB for PostgreSQL cluster restore. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub point_in_time_in_utc: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The preferred primary availability zone for the Azure Cosmos DB for PostgreSQL cluster.
        #[builder(into, default)]
        pub preferred_primary_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure Cosmos DB for PostgreSQL Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is shards on coordinator enabled for the Azure Cosmos DB for PostgreSQL cluster.
        #[builder(into, default)]
        pub shards_on_coordinator_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Azure region of the source Azure Cosmos DB for PostgreSQL cluster for read replica clusters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the source Azure Cosmos DB for PostgreSQL cluster for read replica clusters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_resource_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The major PostgreSQL version on the Azure Cosmos DB for PostgreSQL cluster. Possible values are `11`, `12`, `13`, `14`, `15` and `16`.
        #[builder(into, default)]
        pub sql_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Azure Cosmos DB for PostgreSQL Cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PostgresqlClusterResult {
        /// The password of the administrator login. This is required when `source_resource_id` is not set.
        pub administrator_login_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The citus extension version on the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `8.3`, `9.0`, `9.1`, `9.2`, `9.3`, `9.4`, `9.5`, `10.0`, `10.1`, `10.2`, `11.0`, `11.1`, `11.2`, `11.3` and `12.1`.
        pub citus_version: pulumi_gestalt_rust::Output<String>,
        /// Is public access enabled on coordinator? Defaults to `true`.
        pub coordinator_public_ip_access_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The edition of the coordinator server. Possible values are `BurstableGeneralPurpose`, `BurstableMemoryOptimized`, `GeneralPurpose` and `MemoryOptimized`. Defaults to `GeneralPurpose`.
        pub coordinator_server_edition: pulumi_gestalt_rust::Output<Option<String>>,
        /// The coordinator storage allowed for the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4194304`, `8388608`, `16777216`, and `33554432`.
        ///
        /// > **NOTE:** More information on [the types of compute resources available for CosmosDB can be found in the product documentation](https://learn.microsoft.com/azure/cosmos-db/postgresql/resources-compute)
        pub coordinator_storage_quota_in_mb: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The coordinator vCore count for the Azure Cosmos DB for PostgreSQL Cluster. Possible values are `1`, `2`, `4`, `8`, `16`, `32`, `64` and `96`.
        pub coordinator_vcore_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The earliest restore point time (ISO8601 format) for the Azure Cosmos DB for PostgreSQL Cluster.
        pub earliest_restore_time: pulumi_gestalt_rust::Output<String>,
        /// Is high availability enabled for the Azure Cosmos DB for PostgreSQL cluster? Defaults to `false`.
        pub ha_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region where the Azure Cosmos DB for PostgreSQL Cluster should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `maintenance_window` block as defined below.
        pub maintenance_window: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::PostgresqlClusterMaintenanceWindow>,
        >,
        /// The name which should be used for this Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The worker node count of the Azure Cosmos DB for PostgreSQL Cluster. Possible value is between `0` and `20` except `1`.
        pub node_count: pulumi_gestalt_rust::Output<i32>,
        /// Is public access enabled on worker nodes. Defaults to `false`.
        pub node_public_ip_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The edition of the node server. Possible values are `BurstableGeneralPurpose`, `BurstableMemoryOptimized`, `GeneralPurpose` and `MemoryOptimized`. Defaults to `MemoryOptimized`.
        pub node_server_edition: pulumi_gestalt_rust::Output<Option<String>>,
        /// The storage quota in MB on each worker node. Possible values are `32768`, `65536`, `131072`, `262144`, `524288`, `1048576`, `2097152`, `4194304`, `8388608` and `16777216`.
        pub node_storage_quota_in_mb: pulumi_gestalt_rust::Output<i32>,
        /// The vCores count on each worker node. Possible values are `1`, `2`, `4`, `8`, `16`, `32`, `64`, `96` and `104`.
        pub node_vcores: pulumi_gestalt_rust::Output<i32>,
        /// The date and time in UTC (ISO8601 format) for the Azure Cosmos DB for PostgreSQL cluster restore. Changing this forces a new resource to be created.
        pub point_in_time_in_utc: pulumi_gestalt_rust::Output<Option<String>>,
        /// The preferred primary availability zone for the Azure Cosmos DB for PostgreSQL cluster.
        pub preferred_primary_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Azure Cosmos DB for PostgreSQL Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `servers` block as defined below.
        pub servers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cosmosdb::PostgresqlClusterServer>,
        >,
        /// Is shards on coordinator enabled for the Azure Cosmos DB for PostgreSQL cluster.
        pub shards_on_coordinator_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure region of the source Azure Cosmos DB for PostgreSQL cluster for read replica clusters. Changing this forces a new resource to be created.
        pub source_location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the source Azure Cosmos DB for PostgreSQL cluster for read replica clusters. Changing this forces a new resource to be created.
        pub source_resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The major PostgreSQL version on the Azure Cosmos DB for PostgreSQL cluster. Possible values are `11`, `12`, `13`, `14`, `15` and `16`.
        pub sql_version: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Cosmos DB for PostgreSQL Cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PostgresqlClusterArgs,
    ) -> PostgresqlClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let administrator_login_password_binding = args
            .administrator_login_password
            .get_output(context);
        let citus_version_binding = args.citus_version.get_output(context);
        let coordinator_public_ip_access_enabled_binding = args
            .coordinator_public_ip_access_enabled
            .get_output(context);
        let coordinator_server_edition_binding = args
            .coordinator_server_edition
            .get_output(context);
        let coordinator_storage_quota_in_mb_binding = args
            .coordinator_storage_quota_in_mb
            .get_output(context);
        let coordinator_vcore_count_binding = args
            .coordinator_vcore_count
            .get_output(context);
        let ha_enabled_binding = args.ha_enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let maintenance_window_binding = args.maintenance_window.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_count_binding = args.node_count.get_output(context);
        let node_public_ip_access_enabled_binding = args
            .node_public_ip_access_enabled
            .get_output(context);
        let node_server_edition_binding = args.node_server_edition.get_output(context);
        let node_storage_quota_in_mb_binding = args
            .node_storage_quota_in_mb
            .get_output(context);
        let node_vcores_binding = args.node_vcores.get_output(context);
        let point_in_time_in_utc_binding = args.point_in_time_in_utc.get_output(context);
        let preferred_primary_zone_binding = args
            .preferred_primary_zone
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let shards_on_coordinator_enabled_binding = args
            .shards_on_coordinator_enabled
            .get_output(context);
        let source_location_binding = args.source_location.get_output(context);
        let source_resource_id_binding = args.source_resource_id.get_output(context);
        let sql_version_binding = args.sql_version.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/postgresqlCluster:PostgresqlCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "administratorLoginPassword".into(),
                    value: administrator_login_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "citusVersion".into(),
                    value: citus_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coordinatorPublicIpAccessEnabled".into(),
                    value: coordinator_public_ip_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coordinatorServerEdition".into(),
                    value: coordinator_server_edition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coordinatorStorageQuotaInMb".into(),
                    value: coordinator_storage_quota_in_mb_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coordinatorVcoreCount".into(),
                    value: coordinator_vcore_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "haEnabled".into(),
                    value: ha_enabled_binding.get_id(),
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
                    name: "nodeCount".into(),
                    value: node_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodePublicIpAccessEnabled".into(),
                    value: node_public_ip_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeServerEdition".into(),
                    value: node_server_edition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeStorageQuotaInMb".into(),
                    value: node_storage_quota_in_mb_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeVcores".into(),
                    value: node_vcores_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pointInTimeInUtc".into(),
                    value: point_in_time_in_utc_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredPrimaryZone".into(),
                    value: preferred_primary_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardsOnCoordinatorEnabled".into(),
                    value: shards_on_coordinator_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceLocation".into(),
                    value: source_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceResourceId".into(),
                    value: source_resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlVersion".into(),
                    value: sql_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PostgresqlClusterResult {
            administrator_login_password: o.get_field("administratorLoginPassword"),
            citus_version: o.get_field("citusVersion"),
            coordinator_public_ip_access_enabled: o
                .get_field("coordinatorPublicIpAccessEnabled"),
            coordinator_server_edition: o.get_field("coordinatorServerEdition"),
            coordinator_storage_quota_in_mb: o.get_field("coordinatorStorageQuotaInMb"),
            coordinator_vcore_count: o.get_field("coordinatorVcoreCount"),
            earliest_restore_time: o.get_field("earliestRestoreTime"),
            ha_enabled: o.get_field("haEnabled"),
            location: o.get_field("location"),
            maintenance_window: o.get_field("maintenanceWindow"),
            name: o.get_field("name"),
            node_count: o.get_field("nodeCount"),
            node_public_ip_access_enabled: o.get_field("nodePublicIpAccessEnabled"),
            node_server_edition: o.get_field("nodeServerEdition"),
            node_storage_quota_in_mb: o.get_field("nodeStorageQuotaInMb"),
            node_vcores: o.get_field("nodeVcores"),
            point_in_time_in_utc: o.get_field("pointInTimeInUtc"),
            preferred_primary_zone: o.get_field("preferredPrimaryZone"),
            resource_group_name: o.get_field("resourceGroupName"),
            servers: o.get_field("servers"),
            shards_on_coordinator_enabled: o.get_field("shardsOnCoordinatorEnabled"),
            source_location: o.get_field("sourceLocation"),
            source_resource_id: o.get_field("sourceResourceId"),
            sql_version: o.get_field("sqlVersion"),
            tags: o.get_field("tags"),
        }
    }
}
