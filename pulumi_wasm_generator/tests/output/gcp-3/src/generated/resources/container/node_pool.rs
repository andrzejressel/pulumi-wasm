/// Manages a node pool in a Google Kubernetes Engine (GKE) cluster separately from
/// the cluster control plane. For more information see [the official documentation](https://cloud.google.com/container-engine/docs/node-pools)
/// and [the API reference](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters.nodePools).
///
/// ## Example Usage
///
/// ### Using A Separately Managed Node Pool (Recommended)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = account::create(
///         "default",
///         AccountArgs::builder()
///             .account_id("service-account-id")
///             .display_name("Service Account")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("my-gke-cluster")
///             .remove_default_node_pool(true)
///             .build_struct(),
///     );
///     let primaryPreemptibleNodes = node_pool::create(
///         "primaryPreemptibleNodes",
///         NodePoolArgs::builder()
///             .cluster("${primary.id}")
///             .name("my-node-pool")
///             .node_config(
///                 NodePoolNodeConfig::builder()
///                     .machineType("e2-medium")
///                     .oauthScopes(vec!["https://www.googleapis.com/auth/cloud-platform",])
///                     .preemptible(true)
///                     .serviceAccount("${default.email}")
///                     .build_struct(),
///             )
///             .node_count(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### 2 Node Pools, 1 Separately Managed + The Default Node Pool
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = account::create(
///         "default",
///         AccountArgs::builder()
///             .account_id("service-account-id")
///             .display_name("Service Account")
///             .build_struct(),
///     );
///     let np = node_pool::create(
///         "np",
///         NodePoolArgs::builder()
///             .cluster("${primary.id}")
///             .name("my-node-pool")
///             .node_config(
///                 NodePoolNodeConfig::builder()
///                     .machineType("e2-medium")
///                     .oauthScopes(vec!["https://www.googleapis.com/auth/cloud-platform",])
///                     .serviceAccount("${default.email}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .initial_node_count(3)
///             .location("us-central1-a")
///             .name("marcellus-wallace")
///             .node_config(
///                 ClusterNodeConfig::builder()
///                     .guestAccelerators(
///                         vec![
///                             ClusterNodeConfigGuestAccelerator::builder().count(1). type
///                             ("nvidia-tesla-k80").build_struct(),
///                         ],
///                     )
///                     .oauthScopes(vec!["https://www.googleapis.com/auth/cloud-platform",])
///                     .serviceAccount("${default.email}")
///                     .build_struct(),
///             )
///             .node_locations(vec!["us-central1-c",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Node pools can be imported using the `project`, `location`, `cluster` and `name`. If
///
/// the project is omitted, the project value in the provider configuration will be used. Examples:
///
/// * `{{project_id}}/{{location}}/{{cluster_id}}/{{pool_id}}`
///
/// * `{{location}}/{{cluster_id}}/{{pool_id}}`
///
/// When using the `pulumi import` command, node pools can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/nodePool:NodePool default {{project_id}}/{{location}}/{{cluster_id}}/{{pool_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/nodePool:NodePool default {{location}}/{{cluster_id}}/{{pool_id}}
/// ```
///
pub mod node_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodePoolArgs {
        /// Configuration required by cluster autoscaler to adjust
        /// the size of the node pool to the current cluster usage. Structure is documented below.
        #[builder(into, default)]
        pub autoscaling: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolAutoscaling>,
        >,
        /// The cluster to create the node pool for. Cluster must be present in `location` provided for clusters. May be specified in the format `projects/{{project}}/locations/{{location}}/clusters/{{cluster}}` or as just the name of the cluster.
        ///
        /// - - -
        #[builder(into)]
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// The initial number of nodes for the pool. In
        /// regional or multi-zonal clusters, this is the number of nodes per zone. Changing
        /// this will force recreation of the resource. WARNING: Resizing your node pool manually
        /// may change this value in your existing cluster, which will trigger destruction
        /// and recreation on the next provider run (to rectify the discrepancy).  If you don't
        /// need this value, don't set it.  If you do need it, you can use a lifecycle block to
        /// ignore subsqeuent changes to this field.
        #[builder(into, default)]
        pub initial_node_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The location (region or zone) of the cluster.
        ///
        /// - - -
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Node management configuration, wherein auto-repair and
        /// auto-upgrade is configured. Structure is documented below.
        #[builder(into, default)]
        pub management: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolManagement>,
        >,
        /// The maximum number of pods per node in this node pool.
        /// Note that this does not work on node pools which are "route-based" - that is, node
        /// pools belonging to clusters that do not have IP Aliasing enabled.
        /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/flexible-pod-cidr)
        /// for more information.
        #[builder(into, default)]
        pub max_pods_per_node: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the node pool. If left blank, the provider will
        /// auto-generate a unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name for the node pool beginning
        /// with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The network configuration of the pool. Such as
        /// configuration for [Adding Pod IP address ranges](https://cloud.google.com/kubernetes-engine/docs/how-to/multi-pod-cidr)) to the node pool. Or enabling private nodes. Structure is
        /// documented below
        #[builder(into, default)]
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolNetworkConfig>,
        >,
        /// Parameters used in creating the node pool. See
        /// gcp.container.Cluster for schema.
        #[builder(into, default)]
        pub node_config: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolNodeConfig>,
        >,
        /// The number of nodes per instance group. This field can be used to
        /// update the number of nodes per instance group but should not be used alongside `autoscaling`.
        #[builder(into, default)]
        pub node_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The list of zones in which the node pool's nodes should be located. Nodes must
        /// be in the region of their regional cluster or in the same region as their
        /// cluster's zone for zonal clusters. If unspecified, the cluster-level
        /// `node_locations` will be used.
        ///
        /// > Note: `node_locations` will not revert to the cluster's default set of zones
        /// upon being unset. You must manually reconcile the list of zones with your
        /// cluster.
        #[builder(into, default)]
        pub node_locations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a custom placement policy for the
        /// nodes.
        #[builder(into, default)]
        pub placement_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolPlacementPolicy>,
        >,
        /// The ID of the project in which to create the node pool. If blank,
        /// the provider-configured project will be used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies node pool-level settings of queued provisioning.
        /// Structure is documented below.
        #[builder(into, default)]
        pub queued_provisioning: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolQueuedProvisioning>,
        >,
        /// Specify node upgrade settings to change how GKE upgrades nodes.
        /// The maximum number of nodes upgraded simultaneously is limited to 20. Structure is documented below.
        #[builder(into, default)]
        pub upgrade_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolUpgradeSettings>,
        >,
        /// The Kubernetes version for the nodes in this pool. Note that if this field
        /// and `auto_upgrade` are both specified, they will fight each other for what the node version should
        /// be, so setting both is highly discouraged. While a fuzzy version can be specified, it's
        /// recommended that you specify explicit versions as the provider will see spurious diffs
        /// when fuzzy versions are used. See the `gcp.container.getEngineVersions` data source's
        /// `version_prefix` field to approximate fuzzy versions in a provider-compatible way.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NodePoolResult {
        /// Configuration required by cluster autoscaler to adjust
        /// the size of the node pool to the current cluster usage. Structure is documented below.
        pub autoscaling: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolAutoscaling>,
        >,
        /// The cluster to create the node pool for. Cluster must be present in `location` provided for clusters. May be specified in the format `projects/{{project}}/locations/{{location}}/clusters/{{cluster}}` or as just the name of the cluster.
        ///
        /// - - -
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// The initial number of nodes for the pool. In
        /// regional or multi-zonal clusters, this is the number of nodes per zone. Changing
        /// this will force recreation of the resource. WARNING: Resizing your node pool manually
        /// may change this value in your existing cluster, which will trigger destruction
        /// and recreation on the next provider run (to rectify the discrepancy).  If you don't
        /// need this value, don't set it.  If you do need it, you can use a lifecycle block to
        /// ignore subsqeuent changes to this field.
        pub initial_node_count: pulumi_wasm_rust::Output<i32>,
        /// The resource URLs of the managed instance groups associated with this node pool.
        pub instance_group_urls: pulumi_wasm_rust::Output<Vec<String>>,
        /// The location (region or zone) of the cluster.
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// List of instance group URLs which have been assigned to this node pool.
        pub managed_instance_group_urls: pulumi_wasm_rust::Output<Vec<String>>,
        /// Node management configuration, wherein auto-repair and
        /// auto-upgrade is configured. Structure is documented below.
        pub management: pulumi_wasm_rust::Output<
            super::super::types::container::NodePoolManagement,
        >,
        /// The maximum number of pods per node in this node pool.
        /// Note that this does not work on node pools which are "route-based" - that is, node
        /// pools belonging to clusters that do not have IP Aliasing enabled.
        /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/flexible-pod-cidr)
        /// for more information.
        pub max_pods_per_node: pulumi_wasm_rust::Output<i32>,
        /// The name of the node pool. If left blank, the provider will
        /// auto-generate a unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name for the node pool beginning
        /// with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The network configuration of the pool. Such as
        /// configuration for [Adding Pod IP address ranges](https://cloud.google.com/kubernetes-engine/docs/how-to/multi-pod-cidr)) to the node pool. Or enabling private nodes. Structure is
        /// documented below
        pub network_config: pulumi_wasm_rust::Output<
            super::super::types::container::NodePoolNetworkConfig,
        >,
        /// Parameters used in creating the node pool. See
        /// gcp.container.Cluster for schema.
        pub node_config: pulumi_wasm_rust::Output<
            super::super::types::container::NodePoolNodeConfig,
        >,
        /// The number of nodes per instance group. This field can be used to
        /// update the number of nodes per instance group but should not be used alongside `autoscaling`.
        pub node_count: pulumi_wasm_rust::Output<i32>,
        /// The list of zones in which the node pool's nodes should be located. Nodes must
        /// be in the region of their regional cluster or in the same region as their
        /// cluster's zone for zonal clusters. If unspecified, the cluster-level
        /// `node_locations` will be used.
        ///
        /// > Note: `node_locations` will not revert to the cluster's default set of zones
        /// upon being unset. You must manually reconcile the list of zones with your
        /// cluster.
        pub node_locations: pulumi_wasm_rust::Output<Vec<String>>,
        pub operation: pulumi_wasm_rust::Output<String>,
        /// Specifies a custom placement policy for the
        /// nodes.
        pub placement_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolPlacementPolicy>,
        >,
        /// The ID of the project in which to create the node pool. If blank,
        /// the provider-configured project will be used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Specifies node pool-level settings of queued provisioning.
        /// Structure is documented below.
        pub queued_provisioning: pulumi_wasm_rust::Output<
            Option<super::super::types::container::NodePoolQueuedProvisioning>,
        >,
        /// Specify node upgrade settings to change how GKE upgrades nodes.
        /// The maximum number of nodes upgraded simultaneously is limited to 20. Structure is documented below.
        pub upgrade_settings: pulumi_wasm_rust::Output<
            super::super::types::container::NodePoolUpgradeSettings,
        >,
        /// The Kubernetes version for the nodes in this pool. Note that if this field
        /// and `auto_upgrade` are both specified, they will fight each other for what the node version should
        /// be, so setting both is highly discouraged. While a fuzzy version can be specified, it's
        /// recommended that you specify explicit versions as the provider will see spurious diffs
        /// when fuzzy versions are used. See the `gcp.container.getEngineVersions` data source's
        /// `version_prefix` field to approximate fuzzy versions in a provider-compatible way.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NodePoolArgs) -> NodePoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autoscaling_binding = args.autoscaling.get_inner();
        let cluster_binding = args.cluster.get_inner();
        let initial_node_count_binding = args.initial_node_count.get_inner();
        let location_binding = args.location.get_inner();
        let management_binding = args.management.get_inner();
        let max_pods_per_node_binding = args.max_pods_per_node.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let network_config_binding = args.network_config.get_inner();
        let node_config_binding = args.node_config.get_inner();
        let node_count_binding = args.node_count.get_inner();
        let node_locations_binding = args.node_locations.get_inner();
        let placement_policy_binding = args.placement_policy.get_inner();
        let project_binding = args.project.get_inner();
        let queued_provisioning_binding = args.queued_provisioning.get_inner();
        let upgrade_settings_binding = args.upgrade_settings.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:container/nodePool:NodePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscaling".into(),
                    value: &autoscaling_binding,
                },
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "initialNodeCount".into(),
                    value: &initial_node_count_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "management".into(),
                    value: &management_binding,
                },
                register_interface::ObjectField {
                    name: "maxPodsPerNode".into(),
                    value: &max_pods_per_node_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding,
                },
                register_interface::ObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding,
                },
                register_interface::ObjectField {
                    name: "nodeCount".into(),
                    value: &node_count_binding,
                },
                register_interface::ObjectField {
                    name: "nodeLocations".into(),
                    value: &node_locations_binding,
                },
                register_interface::ObjectField {
                    name: "placementPolicy".into(),
                    value: &placement_policy_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "queuedProvisioning".into(),
                    value: &queued_provisioning_binding,
                },
                register_interface::ObjectField {
                    name: "upgradeSettings".into(),
                    value: &upgrade_settings_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoscaling".into(),
                },
                register_interface::ResultField {
                    name: "cluster".into(),
                },
                register_interface::ResultField {
                    name: "initialNodeCount".into(),
                },
                register_interface::ResultField {
                    name: "instanceGroupUrls".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedInstanceGroupUrls".into(),
                },
                register_interface::ResultField {
                    name: "management".into(),
                },
                register_interface::ResultField {
                    name: "maxPodsPerNode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "networkConfig".into(),
                },
                register_interface::ResultField {
                    name: "nodeConfig".into(),
                },
                register_interface::ResultField {
                    name: "nodeCount".into(),
                },
                register_interface::ResultField {
                    name: "nodeLocations".into(),
                },
                register_interface::ResultField {
                    name: "operation".into(),
                },
                register_interface::ResultField {
                    name: "placementPolicy".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "queuedProvisioning".into(),
                },
                register_interface::ResultField {
                    name: "upgradeSettings".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NodePoolResult {
            autoscaling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaling").unwrap(),
            ),
            cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cluster").unwrap(),
            ),
            initial_node_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initialNodeCount").unwrap(),
            ),
            instance_group_urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceGroupUrls").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_instance_group_urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedInstanceGroupUrls").unwrap(),
            ),
            management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("management").unwrap(),
            ),
            max_pods_per_node: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxPodsPerNode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            network_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfig").unwrap(),
            ),
            node_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeConfig").unwrap(),
            ),
            node_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeCount").unwrap(),
            ),
            node_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeLocations").unwrap(),
            ),
            operation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operation").unwrap(),
            ),
            placement_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementPolicy").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            queued_provisioning: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queuedProvisioning").unwrap(),
            ),
            upgrade_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upgradeSettings").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
