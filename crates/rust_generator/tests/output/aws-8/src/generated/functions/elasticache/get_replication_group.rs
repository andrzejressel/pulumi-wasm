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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetReplicationGroupArgs,
    ) -> GetReplicationGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetReplicationGroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auth_token_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authTokenEnabled"),
            ),
            automatic_failover_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticFailoverEnabled"),
            ),
            cluster_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterMode"),
            ),
            configuration_endpoint_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationEndpointAddress"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            log_delivery_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logDeliveryConfigurations"),
            ),
            member_clusters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("memberClusters"),
            ),
            multi_az_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiAzEnabled"),
            ),
            node_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeType"),
            ),
            num_cache_clusters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numCacheClusters"),
            ),
            num_node_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numNodeGroups"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            primary_endpoint_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryEndpointAddress"),
            ),
            reader_endpoint_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readerEndpointAddress"),
            ),
            replicas_per_node_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicasPerNodeGroup"),
            ),
            replication_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationGroupId"),
            ),
            snapshot_retention_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotRetentionLimit"),
            ),
            snapshot_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotWindow"),
            ),
        }
    }
}
