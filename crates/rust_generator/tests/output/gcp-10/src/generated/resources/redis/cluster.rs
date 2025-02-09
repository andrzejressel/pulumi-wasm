/// ## Example Usage
///
/// ### Redis Cluster Ha
///
///
/// ```yaml
/// resources:
///   cluster-ha:
///     type: gcp:redis:Cluster
///     properties:
///       name: ha-cluster
///       shardCount: 3
///       pscConfigs:
///         - network: ${producerNet.id}
///       region: us-central1
///       replicaCount: 1
///       nodeType: REDIS_SHARED_CORE_NANO
///       transitEncryptionMode: TRANSIT_ENCRYPTION_MODE_DISABLED
///       authorizationMode: AUTH_MODE_DISABLED
///       redisConfigs:
///         maxmemory-policy: volatile-ttl
///       deletionProtectionEnabled: true
///       zoneDistributionConfig:
///         mode: MULTI_ZONE
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: MONDAY
///             startTime:
///               hours: 1
///               minutes: 0
///               seconds: 0
///               nanos: 0
///     options:
///       dependsOn:
///         - ${default}
///   default:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     properties:
///       name: mypolicy
///       location: us-central1
///       serviceClass: gcp-memorystore-redis
///       description: my basic service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${producerSubnet.id}
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: mysubnet
///       ipCidrRange: 10.0.0.248/29
///       region: us-central1
///       network: ${producerNet.id}
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: mynetwork
///       autoCreateSubnetworks: false
/// ```
/// ### Redis Cluster Ha Single Zone
///
///
/// ```yaml
/// resources:
///   cluster-ha-single-zone:
///     type: gcp:redis:Cluster
///     properties:
///       name: ha-cluster-single-zone
///       shardCount: 3
///       pscConfigs:
///         - network: ${producerNet.id}
///       region: us-central1
///       zoneDistributionConfig:
///         mode: SINGLE_ZONE
///         zone: us-central1-f
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: MONDAY
///             startTime:
///               hours: 1
///               minutes: 0
///               seconds: 0
///               nanos: 0
///       deletionProtectionEnabled: true
///     options:
///       dependsOn:
///         - ${default}
///   default:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     properties:
///       name: mypolicy
///       location: us-central1
///       serviceClass: gcp-memorystore-redis
///       description: my basic service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${producerSubnet.id}
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: mysubnet
///       ipCidrRange: 10.0.0.248/29
///       region: us-central1
///       network: ${producerNet.id}
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: mynetwork
///       autoCreateSubnetworks: false
/// ```
/// ### Redis Cluster Secondary
///
///
/// ```yaml
/// resources:
///   # Primary cluster
///   primaryCluster:
///     type: gcp:redis:Cluster
///     name: primary_cluster
///     properties:
///       name: my-primary-cluster
///       region: us-east1
///       pscConfigs:
///         - network: ${producerNet.id}
///       authorizationMode: AUTH_MODE_DISABLED
///       transitEncryptionMode: TRANSIT_ENCRYPTION_MODE_DISABLED
///       shardCount: 3
///       redisConfigs:
///         maxmemory-policy: volatile-ttl
///       nodeType: REDIS_HIGHMEM_MEDIUM
///       persistenceConfig:
///         mode: RDB
///         rdbConfig:
///           rdbSnapshotPeriod: ONE_HOUR
///           rdbSnapshotStartTime: 2024-10-02T15:01:23Z
///       zoneDistributionConfig:
///         mode: MULTI_ZONE
///       replicaCount: 1
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: MONDAY
///             startTime:
///               hours: 1
///               minutes: 0
///               seconds: 0
///               nanos: 0
///       deletionProtectionEnabled: true
///     options:
///       dependsOn:
///         - ${primaryClusterRegionScp}
///   # Secondary cluster
///   secondaryCluster:
///     type: gcp:redis:Cluster
///     name: secondary_cluster
///     properties:
///       name: my-secondary-cluster
///       region: europe-west1
///       pscConfigs:
///         - network: ${producerNet.id}
///       authorizationMode: AUTH_MODE_DISABLED
///       transitEncryptionMode: TRANSIT_ENCRYPTION_MODE_DISABLED
///       shardCount: 3
///       redisConfigs:
///         maxmemory-policy: volatile-ttl
///       nodeType: REDIS_HIGHMEM_MEDIUM
///       persistenceConfig:
///         mode: RDB
///         rdbConfig:
///           rdbSnapshotPeriod: ONE_HOUR
///           rdbSnapshotStartTime: 2024-10-02T15:01:23Z
///       zoneDistributionConfig:
///         mode: MULTI_ZONE
///       replicaCount: 2
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: WEDNESDAY
///             startTime:
///               hours: 1
///               minutes: 0
///               seconds: 0
///               nanos: 0
///       deletionProtectionEnabled: true # Cross cluster replication config
///       crossClusterReplicationConfig:
///         clusterRole: SECONDARY
///         primaryCluster:
///           cluster: ${primaryCluster.id}
///     options:
///       dependsOn:
///         - ${secondaryClusterRegionScp}
///   primaryClusterRegionScp:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     name: primary_cluster_region_scp
///     properties:
///       name: mypolicy-primary-cluster
///       location: us-east1
///       serviceClass: gcp-memorystore-redis
///       description: Primary cluster service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${primaryClusterProducerSubnet.id}
///   primaryClusterProducerSubnet:
///     type: gcp:compute:Subnetwork
///     name: primary_cluster_producer_subnet
///     properties:
///       name: mysubnet-primary-cluster
///       ipCidrRange: 10.0.1.0/29
///       region: us-east1
///       network: ${producerNet.id}
///   secondaryClusterRegionScp:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     name: secondary_cluster_region_scp
///     properties:
///       name: mypolicy-secondary-cluster
///       location: europe-west1
///       serviceClass: gcp-memorystore-redis
///       description: Secondary cluster service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${secondaryClusterProducerSubnet.id}
///   secondaryClusterProducerSubnet:
///     type: gcp:compute:Subnetwork
///     name: secondary_cluster_producer_subnet
///     properties:
///       name: mysubnet-secondary-cluster
///       ipCidrRange: 10.0.2.0/29
///       region: europe-west1
///       network: ${producerNet.id}
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: mynetwork
///       autoCreateSubnetworks: false
/// ```
/// ### Redis Cluster Rdb
///
///
/// ```yaml
/// resources:
///   cluster-rdb:
///     type: gcp:redis:Cluster
///     properties:
///       name: rdb-cluster
///       shardCount: 3
///       pscConfigs:
///         - network: ${producerNet.id}
///       region: us-central1
///       replicaCount: 0
///       nodeType: REDIS_SHARED_CORE_NANO
///       transitEncryptionMode: TRANSIT_ENCRYPTION_MODE_DISABLED
///       authorizationMode: AUTH_MODE_DISABLED
///       redisConfigs:
///         maxmemory-policy: volatile-ttl
///       deletionProtectionEnabled: true
///       zoneDistributionConfig:
///         mode: MULTI_ZONE
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: MONDAY
///             startTime:
///               hours: 1
///               minutes: 0
///               seconds: 0
///               nanos: 0
///       persistenceConfig:
///         mode: RDB
///         rdbConfig:
///           rdbSnapshotPeriod: ONE_HOUR
///           rdbSnapshotStartTime: 2024-10-02T15:01:23Z
///     options:
///       dependsOn:
///         - ${default}
///   default:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     properties:
///       name: mypolicy
///       location: us-central1
///       serviceClass: gcp-memorystore-redis
///       description: my basic service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${producerSubnet.id}
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: mysubnet
///       ipCidrRange: 10.0.0.248/29
///       region: us-central1
///       network: ${producerNet.id}
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: mynetwork
///       autoCreateSubnetworks: false
/// ```
/// ### Redis Cluster Aof
///
///
/// ```yaml
/// resources:
///   cluster-aof:
///     type: gcp:redis:Cluster
///     properties:
///       name: aof-cluster
///       shardCount: 3
///       pscConfigs:
///         - network: ${producerNet.id}
///       region: us-central1
///       replicaCount: 0
///       nodeType: REDIS_SHARED_CORE_NANO
///       transitEncryptionMode: TRANSIT_ENCRYPTION_MODE_DISABLED
///       authorizationMode: AUTH_MODE_DISABLED
///       redisConfigs:
///         maxmemory-policy: volatile-ttl
///       deletionProtectionEnabled: true
///       zoneDistributionConfig:
///         mode: MULTI_ZONE
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: MONDAY
///             startTime:
///               hours: 1
///               minutes: 0
///               seconds: 0
///               nanos: 0
///       persistenceConfig:
///         mode: AOF
///         aofConfig:
///           appendFsync: EVERYSEC
///     options:
///       dependsOn:
///         - ${default}
///   default:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     properties:
///       name: mypolicy
///       location: us-central1
///       serviceClass: gcp-memorystore-redis
///       description: my basic service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${producerSubnet.id}
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: mysubnet
///       ipCidrRange: 10.0.0.248/29
///       region: us-central1
///       network: ${producerNet.id}
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: mynetwork
///       autoCreateSubnetworks: false
/// ```
///
/// ## Import
///
/// Cluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/clusters/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:redis/cluster:Cluster default projects/{{project}}/locations/{{region}}/clusters/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:redis/cluster:Cluster default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:redis/cluster:Cluster default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:redis/cluster:Cluster default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Optional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster.
        /// Default value: "AUTH_MODE_DISABLED" Possible values: ["AUTH_MODE_UNSPECIFIED", "AUTH_MODE_IAM_AUTH",
        /// "AUTH_MODE_DISABLED"]
        #[builder(into, default)]
        pub authorization_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Cross cluster replication config
        #[builder(into, default)]
        pub cross_cluster_replication_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::redis::ClusterCrossClusterReplicationConfig>,
        >,
        /// Optional. Indicates if the cluster is deletion protected or not. If the value if set to true, any delete cluster
        /// operation will fail. Default value is true.
        #[builder(into, default)]
        pub deletion_protection_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Maintenance policy for a cluster
        #[builder(into, default)]
        pub maintenance_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::redis::ClusterMaintenancePolicy>,
        >,
        /// Unique name of the resource in this scope including project and location using the form:
        /// projects/{projectId}/locations/{locationId}/clusters/{clusterId}
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The nodeType for the Redis cluster. If not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values:
        /// ["REDIS_SHARED_CORE_NANO", "REDIS_HIGHMEM_MEDIUM", "REDIS_HIGHMEM_XLARGE", "REDIS_STANDARD_SMALL"]
        #[builder(into, default)]
        pub node_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Persistence config (RDB, AOF) for the cluster.
        #[builder(into, default)]
        pub persistence_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::redis::ClusterPersistenceConfig>,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Each PscConfig configures the consumer network where two
        /// network addresses will be designated to the cluster for client access.
        /// Currently, only one PscConfig is supported.
        /// Structure is documented below.
        #[builder(into)]
        pub psc_configs: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::redis::ClusterPscConfig>,
        >,
        /// Configure Redis Cluster behavior using a subset of native Redis configuration parameters. Please check Memorystore
        /// documentation for the list of supported parameters:
        /// https://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations
        #[builder(into, default)]
        pub redis_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the region of the Redis cluster.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. The number of replica nodes per shard.
        #[builder(into, default)]
        pub replica_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Required. Number of shards for the Redis cluster.
        #[builder(into)]
        pub shard_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Optional. The in-transit encryption for the Redis cluster. If not provided, encryption is disabled for the cluster.
        /// Default value: "TRANSIT_ENCRYPTION_MODE_DISABLED" Possible values: ["TRANSIT_ENCRYPTION_MODE_UNSPECIFIED",
        /// "TRANSIT_ENCRYPTION_MODE_DISABLED", "TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
        #[builder(into, default)]
        pub transit_encryption_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Immutable. Zone distribution config for Memorystore Redis cluster.
        #[builder(into, default)]
        pub zone_distribution_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::redis::ClusterZoneDistributionConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Optional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster.
        /// Default value: "AUTH_MODE_DISABLED" Possible values: ["AUTH_MODE_UNSPECIFIED", "AUTH_MODE_IAM_AUTH",
        /// "AUTH_MODE_DISABLED"]
        pub authorization_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp associated with the cluster creation request. A timestamp in
        /// RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional
        /// digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Cross cluster replication config
        pub cross_cluster_replication_config: pulumi_gestalt_rust::Output<
            super::super::types::redis::ClusterCrossClusterReplicationConfig,
        >,
        /// Optional. Indicates if the cluster is deletion protected or not. If the value if set to true, any delete cluster
        /// operation will fail. Default value is true.
        pub deletion_protection_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Output only. Endpoints created on each given network,
        /// for Redis clients to connect to the cluster.
        /// Currently only one endpoint is supported.
        /// Structure is documented below.
        pub discovery_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redis::ClusterDiscoveryEndpoint>,
        >,
        /// Maintenance policy for a cluster
        pub maintenance_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::redis::ClusterMaintenancePolicy>,
        >,
        /// Upcoming maintenance schedule.
        /// Structure is documented below.
        pub maintenance_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redis::ClusterMaintenanceSchedule>,
        >,
        /// Unique name of the resource in this scope including project and location using the form:
        /// projects/{projectId}/locations/{locationId}/clusters/{clusterId}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The nodeType for the Redis cluster. If not provided, REDIS_HIGHMEM_MEDIUM will be used as default Possible values:
        /// ["REDIS_SHARED_CORE_NANO", "REDIS_HIGHMEM_MEDIUM", "REDIS_HIGHMEM_XLARGE", "REDIS_STANDARD_SMALL"]
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// Persistence config (RDB, AOF) for the cluster.
        pub persistence_config: pulumi_gestalt_rust::Output<
            super::super::types::redis::ClusterPersistenceConfig,
        >,
        /// Output only. Redis memory precise size in GB for the entire cluster.
        pub precise_size_gb: pulumi_gestalt_rust::Output<f64>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Required. Each PscConfig configures the consumer network where two
        /// network addresses will be designated to the cluster for client access.
        /// Currently, only one PscConfig is supported.
        /// Structure is documented below.
        pub psc_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redis::ClusterPscConfig>,
        >,
        /// Output only. PSC connections for discovery of the cluster topology and accessing the cluster.
        /// Structure is documented below.
        pub psc_connections: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redis::ClusterPscConnection>,
        >,
        /// Configure Redis Cluster behavior using a subset of native Redis configuration parameters. Please check Memorystore
        /// documentation for the list of supported parameters:
        /// https://cloud.google.com/memorystore/docs/cluster/supported-instance-configurations
        pub redis_configs: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the region of the Redis cluster.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Optional. The number of replica nodes per shard.
        pub replica_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Required. Number of shards for the Redis cluster.
        pub shard_count: pulumi_gestalt_rust::Output<i32>,
        /// Output only. Redis memory size in GB for the entire cluster.
        pub size_gb: pulumi_gestalt_rust::Output<i32>,
        /// The current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. Additional information about the current state of the cluster.
        /// Structure is documented below.
        pub state_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redis::ClusterStateInfo>,
        >,
        /// Optional. The in-transit encryption for the Redis cluster. If not provided, encryption is disabled for the cluster.
        /// Default value: "TRANSIT_ENCRYPTION_MODE_DISABLED" Possible values: ["TRANSIT_ENCRYPTION_MODE_UNSPECIFIED",
        /// "TRANSIT_ENCRYPTION_MODE_DISABLED", "TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"]
        pub transit_encryption_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// System assigned, unique identifier for the cluster.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Immutable. Zone distribution config for Memorystore Redis cluster.
        pub zone_distribution_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::redis::ClusterZoneDistributionConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorization_mode_binding = args.authorization_mode.get_output(context);
        let cross_cluster_replication_config_binding = args
            .cross_cluster_replication_config
            .get_output(context);
        let deletion_protection_enabled_binding = args
            .deletion_protection_enabled
            .get_output(context);
        let maintenance_policy_binding = args.maintenance_policy.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_type_binding = args.node_type.get_output(context);
        let persistence_config_binding = args.persistence_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let psc_configs_binding = args.psc_configs.get_output(context);
        let redis_configs_binding = args.redis_configs.get_output(context);
        let region_binding = args.region.get_output(context);
        let replica_count_binding = args.replica_count.get_output(context);
        let shard_count_binding = args.shard_count.get_output(context);
        let transit_encryption_mode_binding = args
            .transit_encryption_mode
            .get_output(context);
        let zone_distribution_config_binding = args
            .zone_distribution_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:redis/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationMode".into(),
                    value: authorization_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "crossClusterReplicationConfig".into(),
                    value: cross_cluster_replication_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtectionEnabled".into(),
                    value: deletion_protection_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenancePolicy".into(),
                    value: maintenance_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeType".into(),
                    value: node_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "persistenceConfig".into(),
                    value: persistence_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pscConfigs".into(),
                    value: psc_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redisConfigs".into(),
                    value: redis_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicaCount".into(),
                    value: replica_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardCount".into(),
                    value: shard_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitEncryptionMode".into(),
                    value: transit_encryption_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneDistributionConfig".into(),
                    value: zone_distribution_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            authorization_mode: o.get_field("authorizationMode"),
            create_time: o.get_field("createTime"),
            cross_cluster_replication_config: o
                .get_field("crossClusterReplicationConfig"),
            deletion_protection_enabled: o.get_field("deletionProtectionEnabled"),
            discovery_endpoints: o.get_field("discoveryEndpoints"),
            maintenance_policy: o.get_field("maintenancePolicy"),
            maintenance_schedules: o.get_field("maintenanceSchedules"),
            name: o.get_field("name"),
            node_type: o.get_field("nodeType"),
            persistence_config: o.get_field("persistenceConfig"),
            precise_size_gb: o.get_field("preciseSizeGb"),
            project: o.get_field("project"),
            psc_configs: o.get_field("pscConfigs"),
            psc_connections: o.get_field("pscConnections"),
            redis_configs: o.get_field("redisConfigs"),
            region: o.get_field("region"),
            replica_count: o.get_field("replicaCount"),
            shard_count: o.get_field("shardCount"),
            size_gb: o.get_field("sizeGb"),
            state: o.get_field("state"),
            state_infos: o.get_field("stateInfos"),
            transit_encryption_mode: o.get_field("transitEncryptionMode"),
            uid: o.get_field("uid"),
            zone_distribution_config: o.get_field("zoneDistributionConfig"),
        }
    }
}
