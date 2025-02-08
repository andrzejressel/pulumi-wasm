#[allow(clippy::doc_lazy_continuation)]
pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Group identifier.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags assigned to the resource
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Availability Zone for the cache cluster.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// List of node objects including `id`, `address`, `port`, `availability_zone` and `outpost_arn`.
        /// Referenceable e.g., as `${data.aws_elasticache_cluster.bar.cache_nodes.0.address}`
        pub cache_nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticache::GetClusterCacheNode>,
        >,
        /// (Memcached only) DNS name of the cache cluster without the port appended.
        pub cluster_address: pulumi_gestalt_rust::Output<String>,
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// (Memcached only) Configuration endpoint to allow host discovery.
        pub configuration_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Name of the cache engine.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Version number of the cache engine.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The IP version advertised in the discovery protocol.
        pub ip_discovery: pulumi_gestalt_rust::Output<String>,
        /// Redis [SLOWLOG](https://redis.io/commands/slowlog) or Redis [Engine Log](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log) delivery settings.
        pub log_delivery_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::elasticache::GetClusterLogDeliveryConfiguration,
            >,
        >,
        /// Specifies the weekly time range for when maintenance
        /// on the cache cluster is performed.
        pub maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// The IP versions for cache cluster connections.
        pub network_type: pulumi_gestalt_rust::Output<String>,
        /// The cluster node type.
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// An ARN of an
        /// SNS topic that ElastiCache notifications get sent to.
        pub notification_topic_arn: pulumi_gestalt_rust::Output<String>,
        /// The number of cache nodes that the cache cluster has.
        pub num_cache_nodes: pulumi_gestalt_rust::Output<i32>,
        /// Name of the parameter group associated with this cache cluster.
        pub parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// The port number on which each of the cache nodes will
        /// accept connections.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The outpost ARN in which the cache cluster was created if created in outpost.
        pub preferred_outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// The replication group to which this cache cluster belongs.
        pub replication_group_id: pulumi_gestalt_rust::Output<String>,
        /// List VPC security groups associated with the cache cluster.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The number of days for which ElastiCache will
        /// retain automatic cache cluster snapshots before deleting them.
        pub snapshot_retention_limit: pulumi_gestalt_rust::Output<i32>,
        /// Daily time range (in UTC) during which ElastiCache will
        /// begin taking a daily snapshot of the cache cluster.
        pub snapshot_window: pulumi_gestalt_rust::Output<String>,
        /// Name of the subnet group associated to the cache cluster.
        pub subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// Tags assigned to the resource
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            cache_nodes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheNodes"),
            ),
            cluster_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterAddress"),
            ),
            cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            configuration_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationEndpoint"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_discovery: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipDiscovery"),
            ),
            log_delivery_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logDeliveryConfigurations"),
            ),
            maintenance_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceWindow"),
            ),
            network_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkType"),
            ),
            node_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeType"),
            ),
            notification_topic_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notificationTopicArn"),
            ),
            num_cache_nodes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numCacheNodes"),
            ),
            parameter_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameterGroupName"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            preferred_outpost_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredOutpostArn"),
            ),
            replication_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationGroupId"),
            ),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            snapshot_retention_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotRetentionLimit"),
            ),
            snapshot_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotWindow"),
            ),
            subnet_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
