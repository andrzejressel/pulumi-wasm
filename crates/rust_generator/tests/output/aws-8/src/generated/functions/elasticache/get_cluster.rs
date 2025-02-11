#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_id_binding = args.cluster_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:elasticache/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClusterResult {
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            cache_nodes: o.get_field("cacheNodes"),
            cluster_address: o.get_field("clusterAddress"),
            cluster_id: o.get_field("clusterId"),
            configuration_endpoint: o.get_field("configurationEndpoint"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            id: o.get_field("id"),
            ip_discovery: o.get_field("ipDiscovery"),
            log_delivery_configurations: o.get_field("logDeliveryConfigurations"),
            maintenance_window: o.get_field("maintenanceWindow"),
            network_type: o.get_field("networkType"),
            node_type: o.get_field("nodeType"),
            notification_topic_arn: o.get_field("notificationTopicArn"),
            num_cache_nodes: o.get_field("numCacheNodes"),
            parameter_group_name: o.get_field("parameterGroupName"),
            port: o.get_field("port"),
            preferred_outpost_arn: o.get_field("preferredOutpostArn"),
            replication_group_id: o.get_field("replicationGroupId"),
            security_group_ids: o.get_field("securityGroupIds"),
            snapshot_retention_limit: o.get_field("snapshotRetentionLimit"),
            snapshot_window: o.get_field("snapshotWindow"),
            subnet_group_name: o.get_field("subnetGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
