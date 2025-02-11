/// A Google Cloud Memorystore instance.
///
///
///
/// ## Example Usage
///
/// ### Memorystore Instance Basic
///
///
/// ```yaml
/// resources:
///   instance-basic:
///     type: gcp:memorystore:Instance
///     properties:
///       instanceId: basic-instance
///       shardCount: 3
///       desiredPscAutoConnections:
///         - network: ${producerNet.id}
///           projectId: ${project.projectId}
///       location: us-central1
///       deletionProtectionEnabled: false
///     options:
///       dependsOn:
///         - ${default}
///   default:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     properties:
///       name: my-policy
///       location: us-central1
///       serviceClass: gcp-memorystore
///       description: my basic service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${producerSubnet.id}
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: my-subnet
///       ipCidrRange: 10.0.0.248/29
///       region: us-central1
///       network: ${producerNet.id}
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Memorystore Instance Full
///
///
/// ```yaml
/// resources:
///   instance-full:
///     type: gcp:memorystore:Instance
///     properties:
///       instanceId: full-instance
///       shardCount: 3
///       desiredPscAutoConnections:
///         - network: ${producerNet.id}
///           projectId: ${project.projectId}
///       location: us-central1
///       replicaCount: 2
///       nodeType: SHARED_CORE_NANO
///       transitEncryptionMode: TRANSIT_ENCRYPTION_DISABLED
///       authorizationMode: AUTH_DISABLED
///       engineConfigs:
///         maxmemory-policy: volatile-ttl
///       zoneDistributionConfig:
///         mode: SINGLE_ZONE
///         zone: us-central1-b
///       engineVersion: VALKEY_7_2
///       deletionProtectionEnabled: false
///       mode: CLUSTER
///       persistenceConfig:
///         mode: RDB
///         rdbConfig:
///           rdbSnapshotPeriod: ONE_HOUR
///           rdbSnapshotStartTime: 2024-10-02T15:01:23Z
///       labels:
///         abc: xyz
///     options:
///       dependsOn:
///         - ${default}
///   default:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     properties:
///       name: my-policy
///       location: us-central1
///       serviceClass: gcp-memorystore
///       description: my basic service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${producerSubnet.id}
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: my-subnet
///       ipCidrRange: 10.0.0.248/29
///       region: us-central1
///       network: ${producerNet.id}
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Memorystore Instance Persistence Aof
///
///
/// ```yaml
/// resources:
///   instance-persistence-aof:
///     type: gcp:memorystore:Instance
///     properties:
///       instanceId: aof-instance
///       shardCount: 3
///       desiredPscAutoConnections:
///         - network: ${producerNet.id}
///           projectId: ${project.projectId}
///       location: us-central1
///       persistenceConfig:
///         mode: AOF
///         aofConfig:
///           appendFsync: EVERY_SEC
///       deletionProtectionEnabled: false
///     options:
///       dependsOn:
///         - ${default}
///   default:
///     type: gcp:networkconnectivity:ServiceConnectionPolicy
///     properties:
///       name: my-policy
///       location: us-central1
///       serviceClass: gcp-memorystore
///       description: my basic service connection policy
///       network: ${producerNet.id}
///       pscConfig:
///         subnetworks:
///           - ${producerSubnet.id}
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: my-subnet
///       ipCidrRange: 10.0.0.248/29
///       region: us-central1
///       network: ${producerNet.id}
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/instances/{{instance_id}}`
///
/// * `{{project}}/{{location}}/{{instance_id}}`
///
/// * `{{location}}/{{instance_id}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:memorystore/instance:Instance default projects/{{project}}/locations/{{location}}/instances/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:memorystore/instance:Instance default {{project}}/{{location}}/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:memorystore/instance:Instance default {{location}}/{{instance_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Optional. Immutable. Authorization mode of the instance. Possible values:
        /// AUTH_DISABLED
        /// IAM_AUTH
        #[builder(into, default)]
        pub authorization_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. If set to true deletion of the instance will fail.
        #[builder(into, default)]
        pub deletion_protection_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Required. Immutable. User inputs for the auto-created PSC connections.
        #[builder(into)]
        pub desired_psc_auto_connections: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::memorystore::InstanceDesiredPscAutoConnection>,
        >,
        /// Optional. User-provided engine configurations for the instance.
        #[builder(into, default)]
        pub engine_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Immutable. Engine version of the instance.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. The ID to use for the instance, which will become the final component of
        /// the instance's resource name.
        /// This value is subject to the following restrictions:
        /// * Must be 4-63 characters in length
        /// * Must begin with a letter or digit
        /// * Must contain only lowercase letters, digits, and hyphens
        /// * Must not end with a hyphen
        /// * Must be unique within a location
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Labels to represent user-provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `memorystore.googleapis.com/CertificateAuthority`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Standalone or cluster.
        /// Possible values:
        /// CLUSTER
        /// STANDALONE
        /// Possible values are: `CLUSTER`, `STANDALONE`.
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Immutable. Machine type for individual nodes of the instance.
        /// Possible values:
        /// SHARED_CORE_NANO
        /// HIGHMEM_MEDIUM
        /// HIGHMEM_XLARGE
        /// STANDARD_SMALL
        #[builder(into, default)]
        pub node_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Represents persistence configuration for a instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub persistence_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::memorystore::InstancePersistenceConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Number of replica nodes per shard. If omitted the default is 0 replicas.
        #[builder(into, default)]
        pub replica_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Required. Number of shards for the instance.
        #[builder(into)]
        pub shard_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Optional. Immutable. In-transit encryption mode of the instance.
        /// Possible values:
        /// TRANSIT_ENCRYPTION_DISABLED
        /// SERVER_AUTHENTICATION
        #[builder(into, default)]
        pub transit_encryption_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Zone distribution configuration for allocation of instance resources.
        /// Structure is documented below.
        #[builder(into, default)]
        pub zone_distribution_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::memorystore::InstanceZoneDistributionConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Optional. Immutable. Authorization mode of the instance. Possible values:
        /// AUTH_DISABLED
        /// IAM_AUTH
        pub authorization_mode: pulumi_gestalt_rust::Output<String>,
        /// Output only. Creation timestamp of the instance.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. If set to true deletion of the instance will fail.
        pub deletion_protection_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Required. Immutable. User inputs for the auto-created PSC connections.
        pub desired_psc_auto_connections: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memorystore::InstanceDesiredPscAutoConnection>,
        >,
        /// Output only. Endpoints clients can connect to the instance through. Currently only one
        /// discovery endpoint is supported.
        /// Structure is documented below.
        pub discovery_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memorystore::InstanceDiscoveryEndpoint>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Endpoints for the instance.
        pub endpoints: pulumi_gestalt_rust::Output<Vec<Vec<String>>>,
        /// Optional. User-provided engine configurations for the instance.
        pub engine_configs: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Immutable. Engine version of the instance.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Required. The ID to use for the instance, which will become the final component of
        /// the instance's resource name.
        /// This value is subject to the following restrictions:
        /// * Must be 4-63 characters in length
        /// * Must begin with a letter or digit
        /// * Must contain only lowercase letters, digits, and hyphens
        /// * Must not end with a hyphen
        /// * Must be unique within a location
        ///
        ///
        /// - - -
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Optional. Labels to represent user-provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `memorystore.googleapis.com/CertificateAuthority`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Optional. Standalone or cluster.
        /// Possible values:
        /// CLUSTER
        /// STANDALONE
        /// Possible values are: `CLUSTER`, `STANDALONE`.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// Identifier. Unique name of the instance.
        /// Format: projects/{project}/locations/{location}/instances/{instance}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Represents configuration for nodes of the instance.
        /// Structure is documented below.
        pub node_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memorystore::InstanceNodeConfig>,
        >,
        /// Optional. Immutable. Machine type for individual nodes of the instance.
        /// Possible values:
        /// SHARED_CORE_NANO
        /// HIGHMEM_MEDIUM
        /// HIGHMEM_XLARGE
        /// STANDARD_SMALL
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// Represents persistence configuration for a instance.
        /// Structure is documented below.
        pub persistence_config: pulumi_gestalt_rust::Output<
            super::super::types::memorystore::InstancePersistenceConfig,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. User inputs and resource details of the auto-created PSC connections.
        /// Structure is documented below.
        pub psc_auto_connections: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memorystore::InstancePscAutoConnection>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Number of replica nodes per shard. If omitted the default is 0 replicas.
        pub replica_count: pulumi_gestalt_rust::Output<i32>,
        /// Required. Number of shards for the instance.
        pub shard_count: pulumi_gestalt_rust::Output<i32>,
        /// Output only. Current state of the instance.
        /// Possible values:
        /// CREATING
        /// ACTIVE
        /// UPDATING
        /// DELETING
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Additional information about the state of the instance.
        /// Structure is documented below.
        pub state_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memorystore::InstanceStateInfo>,
        >,
        /// Optional. Immutable. In-transit encryption mode of the instance.
        /// Possible values:
        /// TRANSIT_ENCRYPTION_DISABLED
        /// SERVER_AUTHENTICATION
        pub transit_encryption_mode: pulumi_gestalt_rust::Output<String>,
        /// Output only. System assigned, unique identifier for the instance.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. Latest update timestamp of the instance.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Zone distribution configuration for allocation of instance resources.
        /// Structure is documented below.
        pub zone_distribution_config: pulumi_gestalt_rust::Output<
            super::super::types::memorystore::InstanceZoneDistributionConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorization_mode_binding = args.authorization_mode.get_output(context);
        let deletion_protection_enabled_binding = args
            .deletion_protection_enabled
            .get_output(context);
        let desired_psc_auto_connections_binding = args
            .desired_psc_auto_connections
            .get_output(context);
        let engine_configs_binding = args.engine_configs.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let node_type_binding = args.node_type.get_output(context);
        let persistence_config_binding = args.persistence_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let replica_count_binding = args.replica_count.get_output(context);
        let shard_count_binding = args.shard_count.get_output(context);
        let transit_encryption_mode_binding = args
            .transit_encryption_mode
            .get_output(context);
        let zone_distribution_config_binding = args
            .zone_distribution_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:memorystore/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationMode".into(),
                    value: &authorization_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtectionEnabled".into(),
                    value: &deletion_protection_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "desiredPscAutoConnections".into(),
                    value: &desired_psc_auto_connections_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineConfigs".into(),
                    value: &engine_configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: &mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "persistenceConfig".into(),
                    value: &persistence_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicaCount".into(),
                    value: &replica_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardCount".into(),
                    value: &shard_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitEncryptionMode".into(),
                    value: &transit_encryption_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneDistributionConfig".into(),
                    value: &zone_distribution_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceResult {
            authorization_mode: o.get_field("authorizationMode"),
            create_time: o.get_field("createTime"),
            deletion_protection_enabled: o.get_field("deletionProtectionEnabled"),
            desired_psc_auto_connections: o.get_field("desiredPscAutoConnections"),
            discovery_endpoints: o.get_field("discoveryEndpoints"),
            effective_labels: o.get_field("effectiveLabels"),
            endpoints: o.get_field("endpoints"),
            engine_configs: o.get_field("engineConfigs"),
            engine_version: o.get_field("engineVersion"),
            instance_id: o.get_field("instanceId"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            node_configs: o.get_field("nodeConfigs"),
            node_type: o.get_field("nodeType"),
            persistence_config: o.get_field("persistenceConfig"),
            project: o.get_field("project"),
            psc_auto_connections: o.get_field("pscAutoConnections"),
            pulumi_labels: o.get_field("pulumiLabels"),
            replica_count: o.get_field("replicaCount"),
            shard_count: o.get_field("shardCount"),
            state: o.get_field("state"),
            state_infos: o.get_field("stateInfos"),
            transit_encryption_mode: o.get_field("transitEncryptionMode"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            zone_distribution_config: o.get_field("zoneDistributionConfig"),
        }
    }
}
