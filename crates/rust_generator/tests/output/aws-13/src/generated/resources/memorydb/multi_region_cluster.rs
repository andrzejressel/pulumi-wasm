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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MultiRegionClusterArgs,
    ) -> MultiRegionClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let multi_region_cluster_name_suffix_binding = args
            .multi_region_cluster_name_suffix
            .get_output(context)
            .get_inner();
        let multi_region_parameter_group_name_binding = args
            .multi_region_parameter_group_name
            .get_output(context)
            .get_inner();
        let node_type_binding = args.node_type.get_output(context).get_inner();
        let num_shards_binding = args.num_shards.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let tls_enabled_binding = args.tls_enabled.get_output(context).get_inner();
        let update_strategy_binding = args
            .update_strategy
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:memorydb/multiRegionCluster:MultiRegionCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "multiRegionClusterNameSuffix".into(),
                    value: &multi_region_cluster_name_suffix_binding,
                },
                register_interface::ObjectField {
                    name: "multiRegionParameterGroupName".into(),
                    value: &multi_region_parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding,
                },
                register_interface::ObjectField {
                    name: "numShards".into(),
                    value: &num_shards_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "tlsEnabled".into(),
                    value: &tls_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "updateStrategy".into(),
                    value: &update_strategy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MultiRegionClusterResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            multi_region_cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiRegionClusterName"),
            ),
            multi_region_cluster_name_suffix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiRegionClusterNameSuffix"),
            ),
            multi_region_parameter_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiRegionParameterGroupName"),
            ),
            node_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeType"),
            ),
            num_shards: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numShards"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            tls_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsEnabled"),
            ),
            update_strategy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateStrategy"),
            ),
        }
    }
}
