/// Provides an ElastiCache Global Replication Group resource, which manages replication between two or more Replication Groups in different regions. For more information, see the [ElastiCache User Guide](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Redis-Global-Datastore.html).
///
/// ## Example Usage
///
/// ### Global replication group with one secondary replication group
///
/// The global replication group depends on the primary group existing. Secondary replication groups depend on the global replication group. the provider dependency management will handle this transparently using resource value references.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_replication_group::create(
///         "example",
///         GlobalReplicationGroupArgs::builder()
///             .global_replication_group_id_suffix("example")
///             .primary_replication_group_id("${primary.id}")
///             .build_struct(),
///     );
///     let primary = replication_group::create(
///         "primary",
///         ReplicationGroupArgs::builder()
///             .description("primary replication group")
///             .engine("redis")
///             .engine_version("5.0.6")
///             .node_type("cache.m5.large")
///             .num_cache_clusters(1)
///             .replication_group_id("example-primary")
///             .build_struct(),
///     );
///     let secondary = replication_group::create(
///         "secondary",
///         ReplicationGroupArgs::builder()
///             .description("secondary replication group")
///             .global_replication_group_id("${example.globalReplicationGroupId}")
///             .num_cache_clusters(1)
///             .replication_group_id("example-secondary")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Managing Redis OOS/Valkey Engine Versions
///
/// The initial Redis version is determined by the version set on the primary replication group.
/// However, once it is part of a Global Replication Group,
/// the Global Replication Group manages the version of all member replication groups.
///
/// The member replication groups must have `lifecycle.ignore_changes[engine_version]` set,
/// or the provider will always return a diff.
///
/// In this example,
/// the primary replication group will be created with Redis 6.0,
/// and then upgraded to Redis 6.2 once added to the Global Replication Group.
/// The secondary replication group will be created with Redis 6.2.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_replication_group::create(
///         "example",
///         GlobalReplicationGroupArgs::builder()
///             .engine_version("6.2")
///             .global_replication_group_id_suffix("example")
///             .primary_replication_group_id("${primary.id}")
///             .build_struct(),
///     );
///     let primary = replication_group::create(
///         "primary",
///         ReplicationGroupArgs::builder()
///             .description("primary replication group")
///             .engine("redis")
///             .engine_version("6.0")
///             .node_type("cache.m5.large")
///             .num_cache_clusters(1)
///             .replication_group_id("example-primary")
///             .build_struct(),
///     );
///     let secondary = replication_group::create(
///         "secondary",
///         ReplicationGroupArgs::builder()
///             .description("secondary replication group")
///             .global_replication_group_id("${example.globalReplicationGroupId}")
///             .num_cache_clusters(1)
///             .replication_group_id("example-secondary")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache Global Replication Groups using the `global_replication_group_id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/globalReplicationGroup:GlobalReplicationGroup my_global_replication_group okuqm-global-replication-group-1
/// ```
pub mod global_replication_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalReplicationGroupArgs {
        /// Specifies whether read-only replicas will be automatically promoted to read/write primary if the existing primary fails.
        /// When creating, by default the Global Replication Group inherits the automatic failover setting of the primary replication group.
        #[builder(into, default)]
        pub automatic_failover_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The instance class used.
        /// See AWS documentation for information on [supported node types](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html)
        /// and [guidance on selecting node types](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/nodes-select-size.html).
        /// When creating, by default the Global Replication Group inherits the node type of the primary replication group.
        #[builder(into, default)]
        pub cache_node_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Redis version to use for the Global Replication Group.
        /// When creating, by default the Global Replication Group inherits the version of the primary replication group.
        /// If a version is specified, the Global Replication Group and all member replication groups will be upgraded to this version.
        /// Cannot be downgraded without replacing the Global Replication Group and all member replication groups.
        /// When the version is 7 or higher, the major and minor version should be set, e.g., `7.2`.
        /// When the version is 6, the major and minor version can be set, e.g., `6.2`,
        /// or the minor version can be unspecified which will use the latest version at creation time, e.g., `6.x`.
        /// The actual engine version used is returned in the attribute `engine_version_actual`, see Attribute Reference below.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A user-created description for the global replication group.
        #[builder(into, default)]
        pub global_replication_group_description: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The suffix name of a Global Datastore. If `global_replication_group_id_suffix` is changed, creates a new resource.
        #[builder(into)]
        pub global_replication_group_id_suffix: pulumi_wasm_rust::Output<String>,
        /// The number of node groups (shards) on the global replication group.
        #[builder(into, default)]
        pub num_node_groups: pulumi_wasm_rust::Output<Option<i32>>,
        /// An ElastiCache Parameter Group to use for the Global Replication Group.
        /// Required when upgrading a major engine version, but will be ignored if left configured after the upgrade is complete.
        /// Specifying without a major version upgrade will fail.
        /// Note that ElastiCache creates a copy of this parameter group for each member replication group.
        #[builder(into, default)]
        pub parameter_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the primary cluster that accepts writes and will replicate updates to the secondary cluster. If `primary_replication_group_id` is changed, creates a new resource.
        #[builder(into)]
        pub primary_replication_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GlobalReplicationGroupResult {
        /// The ARN of the ElastiCache Global Replication Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A flag that indicate whether the encryption at rest is enabled.
        pub at_rest_encryption_enabled: pulumi_wasm_rust::Output<bool>,
        /// A flag that indicate whether AuthToken (password) is enabled.
        pub auth_token_enabled: pulumi_wasm_rust::Output<bool>,
        /// Specifies whether read-only replicas will be automatically promoted to read/write primary if the existing primary fails.
        /// When creating, by default the Global Replication Group inherits the automatic failover setting of the primary replication group.
        pub automatic_failover_enabled: pulumi_wasm_rust::Output<bool>,
        /// The instance class used.
        /// See AWS documentation for information on [supported node types](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html)
        /// and [guidance on selecting node types](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/nodes-select-size.html).
        /// When creating, by default the Global Replication Group inherits the node type of the primary replication group.
        pub cache_node_type: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the Global Datastore is cluster enabled.
        pub cluster_enabled: pulumi_wasm_rust::Output<bool>,
        /// The name of the cache engine to be used for the clusters in this global replication group.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Redis version to use for the Global Replication Group.
        /// When creating, by default the Global Replication Group inherits the version of the primary replication group.
        /// If a version is specified, the Global Replication Group and all member replication groups will be upgraded to this version.
        /// Cannot be downgraded without replacing the Global Replication Group and all member replication groups.
        /// When the version is 7 or higher, the major and minor version should be set, e.g., `7.2`.
        /// When the version is 6, the major and minor version can be set, e.g., `6.2`,
        /// or the minor version can be unspecified which will use the latest version at creation time, e.g., `6.x`.
        /// The actual engine version used is returned in the attribute `engine_version_actual`, see Attribute Reference below.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The full version number of the cache engine running on the members of this global replication group.
        pub engine_version_actual: pulumi_wasm_rust::Output<String>,
        /// Set of node groups (shards) on the global replication group.
        /// Has the values:
        pub global_node_groups: pulumi_wasm_rust::Output<
            Vec<super::super::types::elasticache::GlobalReplicationGroupGlobalNodeGroup>,
        >,
        /// A user-created description for the global replication group.
        pub global_replication_group_description: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The full ID of the global replication group.
        pub global_replication_group_id: pulumi_wasm_rust::Output<String>,
        /// The suffix name of a Global Datastore. If `global_replication_group_id_suffix` is changed, creates a new resource.
        pub global_replication_group_id_suffix: pulumi_wasm_rust::Output<String>,
        /// The number of node groups (shards) on the global replication group.
        pub num_node_groups: pulumi_wasm_rust::Output<i32>,
        /// An ElastiCache Parameter Group to use for the Global Replication Group.
        /// Required when upgrading a major engine version, but will be ignored if left configured after the upgrade is complete.
        /// Specifying without a major version upgrade will fail.
        /// Note that ElastiCache creates a copy of this parameter group for each member replication group.
        pub parameter_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the primary cluster that accepts writes and will replicate updates to the secondary cluster. If `primary_replication_group_id` is changed, creates a new resource.
        pub primary_replication_group_id: pulumi_wasm_rust::Output<String>,
        /// A flag that indicates whether the encryption in transit is enabled.
        pub transit_encryption_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: GlobalReplicationGroupArgs,
    ) -> GlobalReplicationGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automatic_failover_enabled_binding = args
            .automatic_failover_enabled
            .get_inner();
        let cache_node_type_binding = args.cache_node_type.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let global_replication_group_description_binding = args
            .global_replication_group_description
            .get_inner();
        let global_replication_group_id_suffix_binding = args
            .global_replication_group_id_suffix
            .get_inner();
        let num_node_groups_binding = args.num_node_groups.get_inner();
        let parameter_group_name_binding = args.parameter_group_name.get_inner();
        let primary_replication_group_id_binding = args
            .primary_replication_group_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/globalReplicationGroup:GlobalReplicationGroup"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automaticFailoverEnabled".into(),
                    value: &automatic_failover_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "cacheNodeType".into(),
                    value: &cache_node_type_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "globalReplicationGroupDescription".into(),
                    value: &global_replication_group_description_binding,
                },
                register_interface::ObjectField {
                    name: "globalReplicationGroupIdSuffix".into(),
                    value: &global_replication_group_id_suffix_binding,
                },
                register_interface::ObjectField {
                    name: "numNodeGroups".into(),
                    value: &num_node_groups_binding,
                },
                register_interface::ObjectField {
                    name: "parameterGroupName".into(),
                    value: &parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "primaryReplicationGroupId".into(),
                    value: &primary_replication_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "atRestEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "authTokenEnabled".into(),
                },
                register_interface::ResultField {
                    name: "automaticFailoverEnabled".into(),
                },
                register_interface::ResultField {
                    name: "cacheNodeType".into(),
                },
                register_interface::ResultField {
                    name: "clusterEnabled".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "engineVersionActual".into(),
                },
                register_interface::ResultField {
                    name: "globalNodeGroups".into(),
                },
                register_interface::ResultField {
                    name: "globalReplicationGroupDescription".into(),
                },
                register_interface::ResultField {
                    name: "globalReplicationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "globalReplicationGroupIdSuffix".into(),
                },
                register_interface::ResultField {
                    name: "numNodeGroups".into(),
                },
                register_interface::ResultField {
                    name: "parameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "primaryReplicationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "transitEncryptionEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GlobalReplicationGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            at_rest_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("atRestEncryptionEnabled").unwrap(),
            ),
            auth_token_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authTokenEnabled").unwrap(),
            ),
            automatic_failover_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticFailoverEnabled").unwrap(),
            ),
            cache_node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheNodeType").unwrap(),
            ),
            cluster_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterEnabled").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            engine_version_actual: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersionActual").unwrap(),
            ),
            global_node_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalNodeGroups").unwrap(),
            ),
            global_replication_group_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalReplicationGroupDescription").unwrap(),
            ),
            global_replication_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalReplicationGroupId").unwrap(),
            ),
            global_replication_group_id_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalReplicationGroupIdSuffix").unwrap(),
            ),
            num_node_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numNodeGroups").unwrap(),
            ),
            parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterGroupName").unwrap(),
            ),
            primary_replication_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryReplicationGroupId").unwrap(),
            ),
            transit_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitEncryptionEnabled").unwrap(),
            ),
        }
    }
}