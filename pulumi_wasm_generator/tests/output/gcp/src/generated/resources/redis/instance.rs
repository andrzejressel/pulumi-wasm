/// A Google Cloud Redis instance.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/memorystore/docs/redis/reference/rest/v1/projects.locations.instances)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/memorystore/docs/redis/)
///
/// ## Example Usage
///
/// ### Redis Instance Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cache = instance::create(
///         "cache",
///         InstanceArgs::builder().memory_size_gb(1).name("memory-cache").build_struct(),
///     );
/// }
/// ```
/// ### Redis Instance Full
///
///
/// ```yaml
/// resources:
///   cache:
///     type: gcp:redis:Instance
///     properties:
///       name: ha-memory-cache
///       tier: STANDARD_HA
///       memorySizeGb: 1
///       locationId: us-central1-a
///       alternativeLocationId: us-central1-f
///       authorizedNetwork: ${["redis-network"].id}
///       redisVersion: REDIS_4_0
///       displayName: Test Instance
///       reservedIpRange: 192.168.0.0/29
///       labels:
///         my_key: my_val
///         other_key: other_val
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: TUESDAY
///             startTime:
///               hours: 0
///               minutes: 30
///               seconds: 0
///               nanos: 0
/// variables:
///   # This example assumes this network already exists.
///   # // The API creates a tenant network per network authorized for a
///   # // Redis instance and that network is not deleted when the user-created
///   # // network (authorized_network) is deleted, so this prevents issues
///   # // with tenant network quota.
///   # // If this network hasn't been created and you are using this example in your
///   # // config, add an additional network resource or change
///   # // this from "data"to "resource"
///   redis-network:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: redis-test-network
/// ```
/// ### Redis Instance Full With Persistence Config
///
///
/// ```yaml
/// resources:
///   cache-persis:
///     type: gcp:redis:Instance
///     properties:
///       name: ha-memory-cache-persis
///       tier: STANDARD_HA
///       memorySizeGb: 1
///       locationId: us-central1-a
///       alternativeLocationId: us-central1-f
///       persistenceConfig:
///         persistenceMode: RDB
///         rdbSnapshotPeriod: TWELVE_HOURS
/// ```
/// ### Redis Instance Private Service
///
///
/// ```yaml
/// resources:
///   # This example assumes this network already exists.
///   # // The API creates a tenant network per network authorized for a
///   # // Redis instance and that network is not deleted when the user-created
///   # // network (authorized_network) is deleted, so this prevents issues
///   # // with tenant network quota.
///   # // If this network hasn't been created and you are using this example in your
///   # // config, add an additional network resource or change
///   # // this from "data"to "resource"
///   redis-network:
///     type: gcp:compute:Network
///     properties:
///       name: redis-test-network
///   serviceRange:
///     type: gcp:compute:GlobalAddress
///     name: service_range
///     properties:
///       name: address
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${["redis-network"].id}
///   privateServiceConnection:
///     type: gcp:servicenetworking:Connection
///     name: private_service_connection
///     properties:
///       network: ${["redis-network"].id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${serviceRange.name}
///   cache:
///     type: gcp:redis:Instance
///     properties:
///       name: private-cache
///       tier: STANDARD_HA
///       memorySizeGb: 1
///       locationId: us-central1-a
///       alternativeLocationId: us-central1-f
///       authorizedNetwork: ${["redis-network"].id}
///       connectMode: PRIVATE_SERVICE_ACCESS
///       redisVersion: REDIS_4_0
///       displayName: Test Instance
///     options:
///       dependsOn:
///         - ${privateServiceConnection}
/// ```
/// ### Redis Instance Mrr
///
///
/// ```yaml
/// resources:
///   cache:
///     type: gcp:redis:Instance
///     properties:
///       name: mrr-memory-cache
///       tier: STANDARD_HA
///       memorySizeGb: 5
///       locationId: us-central1-a
///       alternativeLocationId: us-central1-f
///       authorizedNetwork: ${["redis-network"].id}
///       redisVersion: REDIS_6_X
///       displayName: Terraform Test Instance
///       reservedIpRange: 192.168.0.0/28
///       replicaCount: 5
///       readReplicasMode: READ_REPLICAS_ENABLED
///       labels:
///         my_key: my_val
///         other_key: other_val
/// variables:
///   # This example assumes this network already exists.
///   # // The API creates a tenant network per network authorized for a
///   # // Redis instance and that network is not deleted when the user-created
///   # // network (authorized_network) is deleted, so this prevents issues
///   # // with tenant network quota.
///   # // If this network hasn't been created and you are using this example in your
///   # // config, add an additional network resource or change
///   # // this from "data"to "resource"
///   redis-network:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: redis-test-network
/// ```
/// ### Redis Instance Cmek
///
///
/// ```yaml
/// resources:
///   cache:
///     type: gcp:redis:Instance
///     properties:
///       name: cmek-memory-cache
///       tier: STANDARD_HA
///       memorySizeGb: 1
///       locationId: us-central1-a
///       alternativeLocationId: us-central1-f
///       authorizedNetwork: ${["redis-network"].id}
///       redisVersion: REDIS_6_X
///       displayName: Terraform Test Instance
///       reservedIpRange: 192.168.0.0/29
///       labels:
///         my_key: my_val
///         other_key: other_val
///       customerManagedKey: ${redisKey.id}
///   redisKeyring:
///     type: gcp:kms:KeyRing
///     name: redis_keyring
///     properties:
///       name: redis-keyring
///       location: us-central1
///   redisKey:
///     type: gcp:kms:CryptoKey
///     name: redis_key
///     properties:
///       name: redis-key
///       keyRing: ${redisKeyring.id}
/// variables:
///   # This example assumes this network already exists.
///   # // The API creates a tenant network per network authorized for a
///   # // Redis instance and that network is not deleted when the user-created
///   # // network (authorized_network) is deleted, so this prevents issues
///   # // with tenant network quota.
///   # // If this network hasn't been created and you are using this example in your
///   # // config, add an additional network resource or change
///   # // this from "data"to "resource"
///   redis-network:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: redis-test-network
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/instances/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:redis/instance:Instance default projects/{{project}}/locations/{{region}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:redis/instance:Instance default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:redis/instance:Instance default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:redis/instance:Instance default {{name}}
/// ```
///
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Only applicable to STANDARD_HA tier which protects the instance
        /// against zonal failures by provisioning it across two zones.
        /// If provided, it must be a different zone from the one provided in
        /// [locationId].
        #[builder(into, default)]
        pub alternative_location_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Indicates whether OSS Redis AUTH is enabled for the
        /// instance. If set to "true" AUTH is enabled on the instance.
        /// Default value is "false" meaning AUTH is disabled.
        #[builder(into, default)]
        pub auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The full name of the Google Compute Engine network to which the
        /// instance is connected. If left unspecified, the default network
        /// will be used.
        #[builder(into, default)]
        pub authorized_network: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection mode of the Redis instance.
        /// Default value is `DIRECT_PEERING`.
        /// Possible values are: `DIRECT_PEERING`, `PRIVATE_SERVICE_ACCESS`.
        #[builder(into, default)]
        pub connect_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. The KMS key reference that you want to use to encrypt the data at rest for this Redis
        /// instance. If this is provided, CMEK is enabled.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_wasm_rust::Output<Option<String>>,
        /// An arbitrary and optional user-provided name for the instance.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource labels to represent user provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The zone where the instance will be provisioned. If not provided,
        /// the service will choose a zone for the instance. For STANDARD_HA tier,
        /// instances will be created across two zones for protection against
        /// zonal failures. If [alternativeLocationId] is also provided, it must
        /// be different from [locationId].
        #[builder(into, default)]
        pub location_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Maintenance policy for an instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub maintenance_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::redis::InstanceMaintenancePolicy>,
        >,
        /// The self service update maintenance version.
        #[builder(into, default)]
        pub maintenance_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Redis memory size in GiB.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub memory_size_gb: pulumi_wasm_rust::Output<i32>,
        /// The ID of the instance or a fully qualified identifier for the instance.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Persistence configuration for an instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub persistence_config: pulumi_wasm_rust::Output<
            Option<super::super::types::redis::InstancePersistenceConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Read replica mode. Can only be specified when trying to create the instance.
        /// If not set, Memorystore Redis backend will default to READ_REPLICAS_DISABLED.
        /// - READ_REPLICAS_DISABLED: If disabled, read endpoint will not be provided and the
        /// instance cannot scale up or down the number of replicas.
        /// - READ_REPLICAS_ENABLED: If enabled, read endpoint will be provided and the instance
        /// can scale up and down the number of replicas.
        /// Possible values are: `READ_REPLICAS_DISABLED`, `READ_REPLICAS_ENABLED`.
        #[builder(into, default)]
        pub read_replicas_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Redis configuration parameters, according to http://redis.io/topics/config.
        /// Please check Memorystore documentation for the list of supported parameters:
        /// https://cloud.google.com/memorystore/docs/redis/reference/rest/v1/projects.locations.instances#Instance.FIELDS.redis_configs
        #[builder(into, default)]
        pub redis_configs: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of Redis software. If not provided, latest supported
        /// version will be used. Please check the API documentation linked
        /// at the top for the latest valid values.
        #[builder(into, default)]
        pub redis_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Redis region of the instance.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. The number of replica nodes. The valid range for the Standard Tier with
        /// read replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled
        /// for a Standard Tier instance, the only valid value is 1 and the default is 1.
        /// The valid value for basic tier is 0 and the default is also 0.
        #[builder(into, default)]
        pub replica_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The CIDR range of internal addresses that are reserved for this
        /// instance. If not provided, the service will choose an unused /29
        /// block, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be
        /// unique and non-overlapping with existing subnets in an authorized
        /// network.
        #[builder(into, default)]
        pub reserved_ip_range: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Additional IP range for node placement. Required when enabling read replicas on
        /// an existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or
        /// "auto". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address
        /// range associated with the private service access connection, or "auto".
        #[builder(into, default)]
        pub secondary_ip_range: pulumi_wasm_rust::Output<Option<String>>,
        /// The service tier of the instance. Must be one of these values:
        /// - BASIC: standalone instance
        /// - STANDARD_HA: highly available primary/replica instances
        /// Default value is `BASIC`.
        /// Possible values are: `BASIC`, `STANDARD_HA`.
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
        /// The TLS mode of the Redis instance, If not provided, TLS is disabled for the instance.
        /// - SERVER_AUTHENTICATION: Client to Server traffic encryption enabled with server authentication
        /// Default value is `DISABLED`.
        /// Possible values are: `SERVER_AUTHENTICATION`, `DISABLED`.
        #[builder(into, default)]
        pub transit_encryption_mode: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Only applicable to STANDARD_HA tier which protects the instance
        /// against zonal failures by provisioning it across two zones.
        /// If provided, it must be a different zone from the one provided in
        /// [locationId].
        pub alternative_location_id: pulumi_wasm_rust::Output<String>,
        /// Optional. Indicates whether OSS Redis AUTH is enabled for the
        /// instance. If set to "true" AUTH is enabled on the instance.
        /// Default value is "false" meaning AUTH is disabled.
        pub auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// AUTH String set on the instance. This field will only be populated if auth_enabled is true.
        pub auth_string: pulumi_wasm_rust::Output<String>,
        /// The full name of the Google Compute Engine network to which the
        /// instance is connected. If left unspecified, the default network
        /// will be used.
        pub authorized_network: pulumi_wasm_rust::Output<String>,
        /// The connection mode of the Redis instance.
        /// Default value is `DIRECT_PEERING`.
        /// Possible values are: `DIRECT_PEERING`, `PRIVATE_SERVICE_ACCESS`.
        pub connect_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// (Output)
        /// The time when the certificate was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The current zone where the Redis endpoint is placed.
        /// For Basic Tier instances, this will always be the same as the
        /// [locationId] provided by the user at creation time. For Standard Tier
        /// instances, this can be either [locationId] or [alternativeLocationId]
        /// and can change after a failover event.
        pub current_location_id: pulumi_wasm_rust::Output<String>,
        /// Optional. The KMS key reference that you want to use to encrypt the data at rest for this Redis
        /// instance. If this is provided, CMEK is enabled.
        pub customer_managed_key: pulumi_wasm_rust::Output<Option<String>>,
        /// An arbitrary and optional user-provided name for the instance.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Hostname or IP address of the exposed Redis endpoint used by clients
        /// to connect to the service.
        pub host: pulumi_wasm_rust::Output<String>,
        /// Resource labels to represent user provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The zone where the instance will be provisioned. If not provided,
        /// the service will choose a zone for the instance. For STANDARD_HA tier,
        /// instances will be created across two zones for protection against
        /// zonal failures. If [alternativeLocationId] is also provided, it must
        /// be different from [locationId].
        pub location_id: pulumi_wasm_rust::Output<String>,
        /// Maintenance policy for an instance.
        /// Structure is documented below.
        pub maintenance_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::redis::InstanceMaintenancePolicy>,
        >,
        /// Upcoming maintenance schedule.
        /// Structure is documented below.
        pub maintenance_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::types::redis::InstanceMaintenanceSchedule>,
        >,
        /// The self service update maintenance version.
        pub maintenance_version: pulumi_wasm_rust::Output<String>,
        /// Redis memory size in GiB.
        ///
        ///
        /// - - -
        pub memory_size_gb: pulumi_wasm_rust::Output<i32>,
        /// The ID of the instance or a fully qualified identifier for the instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Output only. Info per node.
        /// Structure is documented below.
        pub nodes: pulumi_wasm_rust::Output<
            Vec<super::super::types::redis::InstanceNode>,
        >,
        /// Persistence configuration for an instance.
        /// Structure is documented below.
        pub persistence_config: pulumi_wasm_rust::Output<
            super::super::types::redis::InstancePersistenceConfig,
        >,
        /// Output only. Cloud IAM identity used by import / export operations
        /// to transfer data to/from Cloud Storage. Format is "serviceAccount:".
        /// The value may change over time for a given instance so should be
        /// checked before each import/export operation.
        pub persistence_iam_identity: pulumi_wasm_rust::Output<String>,
        /// The port number of the exposed Redis endpoint.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Hostname or IP address of the exposed readonly Redis endpoint. Standard tier only.
        /// Targets all healthy replica nodes in instance. Replication is asynchronous and replica nodes
        /// will exhibit some lag behind the primary. Write requests must target 'host'.
        pub read_endpoint: pulumi_wasm_rust::Output<String>,
        /// Output only. The port number of the exposed readonly redis endpoint. Standard tier only.
        /// Write requests should target 'port'.
        pub read_endpoint_port: pulumi_wasm_rust::Output<i32>,
        /// Optional. Read replica mode. Can only be specified when trying to create the instance.
        /// If not set, Memorystore Redis backend will default to READ_REPLICAS_DISABLED.
        /// - READ_REPLICAS_DISABLED: If disabled, read endpoint will not be provided and the
        /// instance cannot scale up or down the number of replicas.
        /// - READ_REPLICAS_ENABLED: If enabled, read endpoint will be provided and the instance
        /// can scale up and down the number of replicas.
        /// Possible values are: `READ_REPLICAS_DISABLED`, `READ_REPLICAS_ENABLED`.
        pub read_replicas_mode: pulumi_wasm_rust::Output<String>,
        /// Redis configuration parameters, according to http://redis.io/topics/config.
        /// Please check Memorystore documentation for the list of supported parameters:
        /// https://cloud.google.com/memorystore/docs/redis/reference/rest/v1/projects.locations.instances#Instance.FIELDS.redis_configs
        pub redis_configs: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of Redis software. If not provided, latest supported
        /// version will be used. Please check the API documentation linked
        /// at the top for the latest valid values.
        pub redis_version: pulumi_wasm_rust::Output<String>,
        /// The name of the Redis region of the instance.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Optional. The number of replica nodes. The valid range for the Standard Tier with
        /// read replicas enabled is [1-5] and defaults to 2. If read replicas are not enabled
        /// for a Standard Tier instance, the only valid value is 1 and the default is 1.
        /// The valid value for basic tier is 0 and the default is also 0.
        pub replica_count: pulumi_wasm_rust::Output<i32>,
        /// The CIDR range of internal addresses that are reserved for this
        /// instance. If not provided, the service will choose an unused /29
        /// block, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be
        /// unique and non-overlapping with existing subnets in an authorized
        /// network.
        pub reserved_ip_range: pulumi_wasm_rust::Output<String>,
        /// Optional. Additional IP range for node placement. Required when enabling read replicas on
        /// an existing instance. For DIRECT_PEERING mode value must be a CIDR range of size /28, or
        /// "auto". For PRIVATE_SERVICE_ACCESS mode value must be the name of an allocated address
        /// range associated with the private service access connection, or "auto".
        pub secondary_ip_range: pulumi_wasm_rust::Output<String>,
        /// List of server CA certificates for the instance.
        /// Structure is documented below.
        pub server_ca_certs: pulumi_wasm_rust::Output<
            Vec<super::super::types::redis::InstanceServerCaCert>,
        >,
        /// The service tier of the instance. Must be one of these values:
        /// - BASIC: standalone instance
        /// - STANDARD_HA: highly available primary/replica instances
        /// Default value is `BASIC`.
        /// Possible values are: `BASIC`, `STANDARD_HA`.
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
        /// The TLS mode of the Redis instance, If not provided, TLS is disabled for the instance.
        /// - SERVER_AUTHENTICATION: Client to Server traffic encryption enabled with server authentication
        /// Default value is `DISABLED`.
        /// Possible values are: `SERVER_AUTHENTICATION`, `DISABLED`.
        pub transit_encryption_mode: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceArgs) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alternative_location_id_binding = args.alternative_location_id.get_inner();
        let auth_enabled_binding = args.auth_enabled.get_inner();
        let authorized_network_binding = args.authorized_network.get_inner();
        let connect_mode_binding = args.connect_mode.get_inner();
        let customer_managed_key_binding = args.customer_managed_key.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_id_binding = args.location_id.get_inner();
        let maintenance_policy_binding = args.maintenance_policy.get_inner();
        let maintenance_version_binding = args.maintenance_version.get_inner();
        let memory_size_gb_binding = args.memory_size_gb.get_inner();
        let name_binding = args.name.get_inner();
        let persistence_config_binding = args.persistence_config.get_inner();
        let project_binding = args.project.get_inner();
        let read_replicas_mode_binding = args.read_replicas_mode.get_inner();
        let redis_configs_binding = args.redis_configs.get_inner();
        let redis_version_binding = args.redis_version.get_inner();
        let region_binding = args.region.get_inner();
        let replica_count_binding = args.replica_count.get_inner();
        let reserved_ip_range_binding = args.reserved_ip_range.get_inner();
        let secondary_ip_range_binding = args.secondary_ip_range.get_inner();
        let tier_binding = args.tier.get_inner();
        let transit_encryption_mode_binding = args.transit_encryption_mode.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:redis/instance:Instance".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alternativeLocationId".into(),
                    value: &alternative_location_id_binding,
                },
                register_interface::ObjectField {
                    name: "authEnabled".into(),
                    value: &auth_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "authorizedNetwork".into(),
                    value: &authorized_network_binding,
                },
                register_interface::ObjectField {
                    name: "connectMode".into(),
                    value: &connect_mode_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKey".into(),
                    value: &customer_managed_key_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "locationId".into(),
                    value: &location_id_binding,
                },
                register_interface::ObjectField {
                    name: "maintenancePolicy".into(),
                    value: &maintenance_policy_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceVersion".into(),
                    value: &maintenance_version_binding,
                },
                register_interface::ObjectField {
                    name: "memorySizeGb".into(),
                    value: &memory_size_gb_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "persistenceConfig".into(),
                    value: &persistence_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "readReplicasMode".into(),
                    value: &read_replicas_mode_binding,
                },
                register_interface::ObjectField {
                    name: "redisConfigs".into(),
                    value: &redis_configs_binding,
                },
                register_interface::ObjectField {
                    name: "redisVersion".into(),
                    value: &redis_version_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "replicaCount".into(),
                    value: &replica_count_binding,
                },
                register_interface::ObjectField {
                    name: "reservedIpRange".into(),
                    value: &reserved_ip_range_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryIpRange".into(),
                    value: &secondary_ip_range_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "transitEncryptionMode".into(),
                    value: &transit_encryption_mode_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alternativeLocationId".into(),
                },
                register_interface::ResultField {
                    name: "authEnabled".into(),
                },
                register_interface::ResultField {
                    name: "authString".into(),
                },
                register_interface::ResultField {
                    name: "authorizedNetwork".into(),
                },
                register_interface::ResultField {
                    name: "connectMode".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "currentLocationId".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKey".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "host".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "locationId".into(),
                },
                register_interface::ResultField {
                    name: "maintenancePolicy".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceSchedules".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceVersion".into(),
                },
                register_interface::ResultField {
                    name: "memorySizeGb".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodes".into(),
                },
                register_interface::ResultField {
                    name: "persistenceConfig".into(),
                },
                register_interface::ResultField {
                    name: "persistenceIamIdentity".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "readEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "readEndpointPort".into(),
                },
                register_interface::ResultField {
                    name: "readReplicasMode".into(),
                },
                register_interface::ResultField {
                    name: "redisConfigs".into(),
                },
                register_interface::ResultField {
                    name: "redisVersion".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "replicaCount".into(),
                },
                register_interface::ResultField {
                    name: "reservedIpRange".into(),
                },
                register_interface::ResultField {
                    name: "secondaryIpRange".into(),
                },
                register_interface::ResultField {
                    name: "serverCaCerts".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "transitEncryptionMode".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceResult {
            alternative_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alternativeLocationId").unwrap(),
            ),
            auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authEnabled").unwrap(),
            ),
            auth_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authString").unwrap(),
            ),
            authorized_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedNetwork").unwrap(),
            ),
            connect_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectMode").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            current_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentLocationId").unwrap(),
            ),
            customer_managed_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKey").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("host").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationId").unwrap(),
            ),
            maintenance_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenancePolicy").unwrap(),
            ),
            maintenance_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceSchedules").unwrap(),
            ),
            maintenance_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceVersion").unwrap(),
            ),
            memory_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memorySizeGb").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodes").unwrap(),
            ),
            persistence_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("persistenceConfig").unwrap(),
            ),
            persistence_iam_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("persistenceIamIdentity").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            read_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readEndpoint").unwrap(),
            ),
            read_endpoint_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readEndpointPort").unwrap(),
            ),
            read_replicas_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readReplicasMode").unwrap(),
            ),
            redis_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisConfigs").unwrap(),
            ),
            redis_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisVersion").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            replica_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaCount").unwrap(),
            ),
            reserved_ip_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservedIpRange").unwrap(),
            ),
            secondary_ip_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryIpRange").unwrap(),
            ),
            server_ca_certs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverCaCerts").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            transit_encryption_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitEncryptionMode").unwrap(),
            ),
        }
    }
}
