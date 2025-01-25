pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Group identifier.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Tags assigned to the resource
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Availability Zone for the cache cluster.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// List of node objects including `id`, `address`, `port`, `availability_zone` and `outpost_arn`.
        /// Referenceable e.g., as `${data.aws_elasticache_cluster.bar.cache_nodes.0.address}`
        pub cache_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticache::GetClusterCacheNode>,
        >,
        /// (Memcached only) DNS name of the cache cluster without the port appended.
        pub cluster_address: pulumi_wasm_rust::Output<String>,
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// (Memcached only) Configuration endpoint to allow host discovery.
        pub configuration_endpoint: pulumi_wasm_rust::Output<String>,
        /// Name of the cache engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Version number of the cache engine.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The IP version advertised in the discovery protocol.
        pub ip_discovery: pulumi_wasm_rust::Output<String>,
        /// Redis [SLOWLOG](https://redis.io/commands/slowlog) or Redis [Engine Log](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log) delivery settings.
        pub log_delivery_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::elasticache::GetClusterLogDeliveryConfiguration,
            >,
        >,
        /// Specifies the weekly time range for when maintenance
        /// on the cache cluster is performed.
        pub maintenance_window: pulumi_wasm_rust::Output<String>,
        /// The IP versions for cache cluster connections.
        pub network_type: pulumi_wasm_rust::Output<String>,
        /// The cluster node type.
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// An ARN of an
        /// SNS topic that ElastiCache notifications get sent to.
        pub notification_topic_arn: pulumi_wasm_rust::Output<String>,
        /// The number of cache nodes that the cache cluster has.
        pub num_cache_nodes: pulumi_wasm_rust::Output<i32>,
        /// Name of the parameter group associated with this cache cluster.
        pub parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// The port number on which each of the cache nodes will
        /// accept connections.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The outpost ARN in which the cache cluster was created if created in outpost.
        pub preferred_outpost_arn: pulumi_wasm_rust::Output<String>,
        /// The replication group to which this cache cluster belongs.
        pub replication_group_id: pulumi_wasm_rust::Output<String>,
        /// List VPC security groups associated with the cache cluster.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The number of days for which ElastiCache will
        /// retain automatic cache cluster snapshots before deleting them.
        pub snapshot_retention_limit: pulumi_wasm_rust::Output<i32>,
        /// Daily time range (in UTC) during which ElastiCache will
        /// begin taking a daily snapshot of the cache cluster.
        pub snapshot_window: pulumi_wasm_rust::Output<String>,
        /// Name of the subnet group associated to the cache cluster.
        pub subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// Tags assigned to the resource
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticache/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "cacheNodes".into(),
                },
                register_interface::ResultField {
                    name: "clusterAddress".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "configurationEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipDiscovery".into(),
                },
                register_interface::ResultField {
                    name: "logDeliveryConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "networkType".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "notificationTopicArn".into(),
                },
                register_interface::ResultField {
                    name: "numCacheNodes".into(),
                },
                register_interface::ResultField {
                    name: "parameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "preferredOutpostArn".into(),
                },
                register_interface::ResultField {
                    name: "replicationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "snapshotRetentionLimit".into(),
                },
                register_interface::ResultField {
                    name: "snapshotWindow".into(),
                },
                register_interface::ResultField {
                    name: "subnetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            cache_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheNodes").unwrap(),
            ),
            cluster_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterAddress").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            configuration_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationEndpoint").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_discovery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipDiscovery").unwrap(),
            ),
            log_delivery_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logDeliveryConfigurations").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkType").unwrap(),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            notification_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationTopicArn").unwrap(),
            ),
            num_cache_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numCacheNodes").unwrap(),
            ),
            parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterGroupName").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            preferred_outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredOutpostArn").unwrap(),
            ),
            replication_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationGroupId").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            snapshot_retention_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotRetentionLimit").unwrap(),
            ),
            snapshot_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotWindow").unwrap(),
            ),
            subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
