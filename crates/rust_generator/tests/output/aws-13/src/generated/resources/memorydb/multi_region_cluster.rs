/// Provides a MemoryDB Multi Region Cluster.
///
/// More information about MemoryDB can be found in the [Developer Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/what-is-memorydb-for-redis.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = multi_region_cluster::create(
///         "example",
///         MultiRegionClusterArgs::builder()
///             .multi_region_cluster_name_suffix("example")
///             .node_type("db.r7g.xlarge")
///             .build_struct(),
///     );
///     let exampleCluster = cluster::create(
///         "exampleCluster",
///         ClusterArgs::builder()
///             .acl_name("${exampleAwsMemorydbAcl.id}")
///             .auto_minor_version_upgrade(false)
///             .multi_region_cluster_name("${example.multiRegionClusterName}")
///             .name("example")
///             .node_type("db.t4g.small")
///             .num_shards(2)
///             .security_group_ids(vec!["${exampleAwsSecurityGroup.id}",])
///             .snapshot_retention_limit(7)
///             .subnet_group_name("${exampleAwsMemorydbSubnetGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a cluster using the `multi_region_cluster_name`. For example:
///
/// ```sh
/// $ pulumi import aws:memorydb/multiRegionCluster:MultiRegionCluster example virxk-example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod multi_region_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiRegionClusterArgs {
        /// description for the multi-region cluster.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the engine to be used for the multi-region cluster. Valid values are `redis` and `valkey`.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version of the engine to be used for the multi-region cluster. Downgrades are not supported.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A suffix to be added to the multi-region cluster name. An AWS generated prefix is automatically applied to the multi-region cluster name when it is created.
        #[builder(into)]
        pub multi_region_cluster_name_suffix: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the multi-region parameter group to be associated with the cluster.
        #[builder(into, default)]
        pub multi_region_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The node type to be used for the multi-region cluster.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub node_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of shards for the multi-region cluster.
        #[builder(into, default)]
        pub num_shards: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::memorydb::MultiRegionClusterTimeouts>,
        >,
        /// A flag to enable in-transit encryption on the cluster.
        #[builder(into, default)]
        pub tls_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub update_strategy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MultiRegionClusterResult {
        /// The ARN of the multi-region cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// description for the multi-region cluster.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the engine to be used for the multi-region cluster. Valid values are `redis` and `valkey`.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// The version of the engine to be used for the multi-region cluster. Downgrades are not supported.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The name of the multi-region cluster.
        pub multi_region_cluster_name: pulumi_gestalt_rust::Output<String>,
        /// A suffix to be added to the multi-region cluster name. An AWS generated prefix is automatically applied to the multi-region cluster name when it is created.
        pub multi_region_cluster_name_suffix: pulumi_gestalt_rust::Output<String>,
        /// The name of the multi-region parameter group to be associated with the cluster.
        pub multi_region_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// The node type to be used for the multi-region cluster.
        ///
        /// The following arguments are optional:
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// The number of shards for the multi-region cluster.
        pub num_shards: pulumi_gestalt_rust::Output<i32>,
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::memorydb::MultiRegionClusterTimeouts>,
        >,
        /// A flag to enable in-transit encryption on the cluster.
        pub tls_enabled: pulumi_gestalt_rust::Output<bool>,
        pub update_strategy: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MultiRegionClusterArgs,
    ) -> MultiRegionClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let engine_binding = args.engine.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let multi_region_cluster_name_suffix_binding = args
            .multi_region_cluster_name_suffix
            .get_output(context);
        let multi_region_parameter_group_name_binding = args
            .multi_region_parameter_group_name
            .get_output(context);
        let node_type_binding = args.node_type.get_output(context);
        let num_shards_binding = args.num_shards.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let tls_enabled_binding = args.tls_enabled.get_output(context);
        let update_strategy_binding = args.update_strategy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:memorydb/multiRegionCluster:MultiRegionCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: engine_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiRegionClusterNameSuffix".into(),
                    value: multi_region_cluster_name_suffix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiRegionParameterGroupName".into(),
                    value: multi_region_parameter_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeType".into(),
                    value: node_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numShards".into(),
                    value: num_shards_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsEnabled".into(),
                    value: tls_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "updateStrategy".into(),
                    value: update_strategy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MultiRegionClusterResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            multi_region_cluster_name: o.get_field("multiRegionClusterName"),
            multi_region_cluster_name_suffix: o
                .get_field("multiRegionClusterNameSuffix"),
            multi_region_parameter_group_name: o
                .get_field("multiRegionParameterGroupName"),
            node_type: o.get_field("nodeType"),
            num_shards: o.get_field("numShards"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            tls_enabled: o.get_field("tlsEnabled"),
            update_strategy: o.get_field("updateStrategy"),
        }
    }
}
