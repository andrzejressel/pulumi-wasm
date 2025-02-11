#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_replication_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationGroupArgs {
        /// Identifier for the replication group.
        #[builder(into)]
        pub replication_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReplicationGroupResult {
        /// ARN of the created ElastiCache Replication Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether an AuthToken (password) is enabled.
        pub auth_token_enabled: pulumi_gestalt_rust::Output<bool>,
        /// A flag whether a read-only replica will be automatically promoted to read/write primary if the existing primary fails.
        pub automatic_failover_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether cluster mode is enabled or disabled.
        pub cluster_mode: pulumi_gestalt_rust::Output<String>,
        /// The configuration endpoint address to allow host discovery.
        pub configuration_endpoint_address: pulumi_gestalt_rust::Output<String>,
        /// Description of the replication group.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Redis [SLOWLOG](https://redis.io/commands/slowlog) or Redis [Engine Log](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log) delivery settings.
        pub log_delivery_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::elasticache::GetReplicationGroupLogDeliveryConfiguration,
            >,
        >,
        /// Identifiers of all the nodes that are part of this replication group.
        pub member_clusters: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether Multi-AZ Support is enabled for the replication group.
        pub multi_az_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The cluster node type.
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// The number of cache clusters that the replication group has.
        pub num_cache_clusters: pulumi_gestalt_rust::Output<i32>,
        /// Number of node groups (shards) for the replication group.
        pub num_node_groups: pulumi_gestalt_rust::Output<i32>,
        /// The port number on which the configuration endpoint will accept connections.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The endpoint of the primary node in this node group (shard).
        pub primary_endpoint_address: pulumi_gestalt_rust::Output<String>,
        /// The endpoint of the reader node in this node group (shard).
        pub reader_endpoint_address: pulumi_gestalt_rust::Output<String>,
        /// Number of replica nodes in each node group.
        pub replicas_per_node_group: pulumi_gestalt_rust::Output<i32>,
        pub replication_group_id: pulumi_gestalt_rust::Output<String>,
        /// The number of days for which ElastiCache retains automatic cache cluster snapshots before deleting them.
        pub snapshot_retention_limit: pulumi_gestalt_rust::Output<i32>,
        /// Daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).
        pub snapshot_window: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReplicationGroupArgs,
    ) -> GetReplicationGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let replication_group_id_binding = args.replication_group_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:elasticache/getReplicationGroup:getReplicationGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationGroupId".into(),
                    value: &replication_group_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReplicationGroupResult {
            arn: o.get_field("arn"),
            auth_token_enabled: o.get_field("authTokenEnabled"),
            automatic_failover_enabled: o.get_field("automaticFailoverEnabled"),
            cluster_mode: o.get_field("clusterMode"),
            configuration_endpoint_address: o.get_field("configurationEndpointAddress"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            log_delivery_configurations: o.get_field("logDeliveryConfigurations"),
            member_clusters: o.get_field("memberClusters"),
            multi_az_enabled: o.get_field("multiAzEnabled"),
            node_type: o.get_field("nodeType"),
            num_cache_clusters: o.get_field("numCacheClusters"),
            num_node_groups: o.get_field("numNodeGroups"),
            port: o.get_field("port"),
            primary_endpoint_address: o.get_field("primaryEndpointAddress"),
            reader_endpoint_address: o.get_field("readerEndpointAddress"),
            replicas_per_node_group: o.get_field("replicasPerNodeGroup"),
            replication_group_id: o.get_field("replicationGroupId"),
            snapshot_retention_limit: o.get_field("snapshotRetentionLimit"),
            snapshot_window: o.get_field("snapshotWindow"),
        }
    }
}
