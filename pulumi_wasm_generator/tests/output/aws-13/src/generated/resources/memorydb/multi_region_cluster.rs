/// Provides a MemoryDB Multi Region Cluster.
///
/// More information about MemoryDB can be found in the [Developer Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/what-is-memorydb-for-redis.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiRegionClusterArgs {
        /// description for the multi-region cluster.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the engine to be used for the multi-region cluster. Valid values are `redis` and `valkey`.
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of the engine to be used for the multi-region cluster. Downgrades are not supported.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A suffix to be added to the multi-region cluster name. An AWS generated prefix is automatically applied to the multi-region cluster name when it is created.
        #[builder(into)]
        pub multi_region_cluster_name_suffix: pulumi_wasm_rust::Output<String>,
        /// The name of the multi-region parameter group to be associated with the cluster.
        #[builder(into, default)]
        pub multi_region_parameter_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The node type to be used for the multi-region cluster.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// The number of shards for the multi-region cluster.
        #[builder(into, default)]
        pub num_shards: pulumi_wasm_rust::Output<Option<i32>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::memorydb::MultiRegionClusterTimeouts>,
        >,
        /// A flag to enable in-transit encryption on the cluster.
        #[builder(into, default)]
        pub tls_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub update_strategy: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MultiRegionClusterResult {
        /// The ARN of the multi-region cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// description for the multi-region cluster.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the engine to be used for the multi-region cluster. Valid values are `redis` and `valkey`.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// The version of the engine to be used for the multi-region cluster. Downgrades are not supported.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The name of the multi-region cluster.
        pub multi_region_cluster_name: pulumi_wasm_rust::Output<String>,
        /// A suffix to be added to the multi-region cluster name. An AWS generated prefix is automatically applied to the multi-region cluster name when it is created.
        pub multi_region_cluster_name_suffix: pulumi_wasm_rust::Output<String>,
        /// The name of the multi-region parameter group to be associated with the cluster.
        pub multi_region_parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// The node type to be used for the multi-region cluster.
        ///
        /// The following arguments are optional:
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// The number of shards for the multi-region cluster.
        pub num_shards: pulumi_wasm_rust::Output<i32>,
        pub status: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::memorydb::MultiRegionClusterTimeouts>,
        >,
        /// A flag to enable in-transit encryption on the cluster.
        pub tls_enabled: pulumi_wasm_rust::Output<bool>,
        pub update_strategy: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MultiRegionClusterArgs) -> MultiRegionClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let engine_binding = args.engine.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let multi_region_cluster_name_suffix_binding = args
            .multi_region_cluster_name_suffix
            .get_inner();
        let multi_region_parameter_group_name_binding = args
            .multi_region_parameter_group_name
            .get_inner();
        let node_type_binding = args.node_type.get_inner();
        let num_shards_binding = args.num_shards.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let tls_enabled_binding = args.tls_enabled.get_inner();
        let update_strategy_binding = args.update_strategy.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "multiRegionClusterName".into(),
                },
                register_interface::ResultField {
                    name: "multiRegionClusterNameSuffix".into(),
                },
                register_interface::ResultField {
                    name: "multiRegionParameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "numShards".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "tlsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "updateStrategy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MultiRegionClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            multi_region_cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiRegionClusterName").unwrap(),
            ),
            multi_region_cluster_name_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiRegionClusterNameSuffix").unwrap(),
            ),
            multi_region_parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiRegionParameterGroupName").unwrap(),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            num_shards: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numShards").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            tls_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsEnabled").unwrap(),
            ),
            update_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateStrategy").unwrap(),
            ),
        }
    }
}
