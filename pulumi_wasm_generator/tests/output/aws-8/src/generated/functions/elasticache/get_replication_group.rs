pub mod get_replication_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationGroupArgs {
        /// Identifier for the replication group.
        #[builder(into)]
        pub replication_group_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReplicationGroupResult {
        /// ARN of the created ElastiCache Replication Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether an AuthToken (password) is enabled.
        pub auth_token_enabled: pulumi_wasm_rust::Output<bool>,
        /// A flag whether a read-only replica will be automatically promoted to read/write primary if the existing primary fails.
        pub automatic_failover_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether cluster mode is enabled or disabled.
        pub cluster_mode: pulumi_wasm_rust::Output<String>,
        /// The configuration endpoint address to allow host discovery.
        pub configuration_endpoint_address: pulumi_wasm_rust::Output<String>,
        /// Description of the replication group.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Redis [SLOWLOG](https://redis.io/commands/slowlog) or Redis [Engine Log](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log) delivery settings.
        pub log_delivery_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::elasticache::GetReplicationGroupLogDeliveryConfiguration,
            >,
        >,
        /// Identifiers of all the nodes that are part of this replication group.
        pub member_clusters: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether Multi-AZ Support is enabled for the replication group.
        pub multi_az_enabled: pulumi_wasm_rust::Output<bool>,
        /// The cluster node type.
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// The number of cache clusters that the replication group has.
        pub num_cache_clusters: pulumi_wasm_rust::Output<i32>,
        /// Number of node groups (shards) for the replication group.
        pub num_node_groups: pulumi_wasm_rust::Output<i32>,
        /// The port number on which the configuration endpoint will accept connections.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The endpoint of the primary node in this node group (shard).
        pub primary_endpoint_address: pulumi_wasm_rust::Output<String>,
        /// The endpoint of the reader node in this node group (shard).
        pub reader_endpoint_address: pulumi_wasm_rust::Output<String>,
        /// Number of replica nodes in each node group.
        pub replicas_per_node_group: pulumi_wasm_rust::Output<i32>,
        pub replication_group_id: pulumi_wasm_rust::Output<String>,
        /// The number of days for which ElastiCache retains automatic cache cluster snapshots before deleting them.
        pub snapshot_retention_limit: pulumi_wasm_rust::Output<i32>,
        /// Daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).
        pub snapshot_window: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetReplicationGroupArgs,
    ) -> GetReplicationGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let replication_group_id_binding = args
            .replication_group_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticache/getReplicationGroup:getReplicationGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "replicationGroupId".into(),
                    value: &replication_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authTokenEnabled".into(),
                },
                register_interface::ResultField {
                    name: "automaticFailoverEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clusterMode".into(),
                },
                register_interface::ResultField {
                    name: "configurationEndpointAddress".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "logDeliveryConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "memberClusters".into(),
                },
                register_interface::ResultField {
                    name: "multiAzEnabled".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "numCacheClusters".into(),
                },
                register_interface::ResultField {
                    name: "numNodeGroups".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "primaryEndpointAddress".into(),
                },
                register_interface::ResultField {
                    name: "readerEndpointAddress".into(),
                },
                register_interface::ResultField {
                    name: "replicasPerNodeGroup".into(),
                },
                register_interface::ResultField {
                    name: "replicationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "snapshotRetentionLimit".into(),
                },
                register_interface::ResultField {
                    name: "snapshotWindow".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReplicationGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auth_token_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authTokenEnabled").unwrap(),
            ),
            automatic_failover_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticFailoverEnabled").unwrap(),
            ),
            cluster_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterMode").unwrap(),
            ),
            configuration_endpoint_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationEndpointAddress").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            log_delivery_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logDeliveryConfigurations").unwrap(),
            ),
            member_clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memberClusters").unwrap(),
            ),
            multi_az_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAzEnabled").unwrap(),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            num_cache_clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numCacheClusters").unwrap(),
            ),
            num_node_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numNodeGroups").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            primary_endpoint_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryEndpointAddress").unwrap(),
            ),
            reader_endpoint_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readerEndpointAddress").unwrap(),
            ),
            replicas_per_node_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicasPerNodeGroup").unwrap(),
            ),
            replication_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationGroupId").unwrap(),
            ),
            snapshot_retention_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotRetentionLimit").unwrap(),
            ),
            snapshot_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotWindow").unwrap(),
            ),
        }
    }
}
