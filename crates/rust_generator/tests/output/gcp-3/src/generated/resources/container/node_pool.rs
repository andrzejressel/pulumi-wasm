/// Manages a node pool in a Google Kubernetes Engine (GKE) cluster separately from
/// the cluster control plane. For more information see [the official documentation](https://cloud.google.com/container-engine/docs/node-pools)
/// and [the API reference](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters.nodePools).
///
/// ## Example Usage
///
/// ### Using A Separately Managed Node Pool (Recommended)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod node_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodePoolArgs {
        /// Configuration required by cluster autoscaler to adjust
        /// the size of the node pool to the current cluster usage. Structure is documented below.
        #[builder(into, default)]
        pub autoscaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::NodePoolAutoscaling>,
        >,
        /// The cluster to create the node pool for. Cluster must be present in `location` provided for clusters. May be specified in the format `projects/{{project}}/locations/{{location}}/clusters/{{cluster}}` or as just the name of the cluster.
        ///
        /// - - -
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The initial number of nodes for the pool. In
        /// regional or multi-zonal clusters, this is the number of nodes per zone. Changing
        /// this will force recreation of the resource. WARNING: Resizing your node pool manually
        /// may change this value in your existing cluster, which will trigger destruction
        /// and recreation on the next provider run (to rectify the discrepancy).  If you don't
        /// need this value, don't set it.  If you do need it, you can use a lifecycle block to
        /// ignore subsqeuent changes to this field.
        #[builder(into, default)]
        pub initial_node_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The location (region or zone) of the cluster.
        ///
        /// - - -
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Node management configuration, wherein auto-repair and
        /// auto-upgrade is configured. Structure is documented below.
        #[builder(into, default)]
        pub management: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::NodePoolManagement>,
        >,
        /// The maximum number of pods per node in this node pool.
        /// Note that this does not work on node pools which are "route-based" - that is, node
        /// pools belonging to clusters that do not have IP Aliasing enabled.
        /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/flexible-pod-cidr)
        /// for more information.
        #[builder(into, default)]
        pub max_pods_per_node: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the node pool. If left blank, the provider will
        /// auto-generate a unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name for the node pool beginning
        /// with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network configuration of the pool. Such as
        /// configuration for [Adding Pod IP address ranges](https://cloud.google.com/kubernetes-engine/docs/how-to/multi-pod-cidr)) to the node pool. Or enabling private nodes. Structure is
        /// documented below
        #[builder(into, default)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::NodePoolNetworkConfig>,
        >,
        /// Parameters used in creating the node pool. See
        /// gcp.container.Cluster for schema.
        #[builder(into, default)]
        pub node_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::NodePoolNodeConfig>,
        >,
        /// The number of nodes per instance group. This field can be used to
        /// update the number of nodes per instance group but should not be used alongside `autoscaling`.
        #[builder(into, default)]
        pub node_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The list of zones in which the node pool's nodes should be located. Nodes must
        /// be in the region of their regional cluster or in the same region as their
        /// cluster's zone for zonal clusters. If unspecified, the cluster-level
        /// `node_locations` will be used.
        ///
        /// > Note: `node_locations` will not revert to the cluster's default set of zones
        /// upon being unset. You must manually reconcile the list of zones with your
        /// cluster.
        #[builder(into, default)]
        pub node_locations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies a custom placement policy for the
        /// nodes.
        #[builder(into, default)]
        pub placement_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::NodePoolPlacementPolicy>,
        >,
        /// The ID of the project in which to create the node pool. If blank,
        /// the provider-configured project will be used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies node pool-level settings of queued provisioning.
        /// Structure is documented below.
        #[builder(into, default)]
        pub queued_provisioning: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::NodePoolQueuedProvisioning>,
        >,
        /// Specify node upgrade settings to change how GKE upgrades nodes.
        /// The maximum number of nodes upgraded simultaneously is limited to 20. Structure is documented below.
        #[builder(into, default)]
        pub upgrade_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::NodePoolUpgradeSettings>,
        >,
        /// The Kubernetes version for the nodes in this pool. Note that if this field
        /// and `auto_upgrade` are both specified, they will fight each other for what the node version should
        /// be, so setting both is highly discouraged. While a fuzzy version can be specified, it's
        /// recommended that you specify explicit versions as the provider will see spurious diffs
        /// when fuzzy versions are used. See the `gcp.container.getEngineVersions` data source's
        /// `version_prefix` field to approximate fuzzy versions in a provider-compatible way.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NodePoolResult {
        /// Configuration required by cluster autoscaler to adjust
        /// the size of the node pool to the current cluster usage. Structure is documented below.
        pub autoscaling: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::NodePoolAutoscaling>,
        >,
        /// The cluster to create the node pool for. Cluster must be present in `location` provided for clusters. May be specified in the format `projects/{{project}}/locations/{{location}}/clusters/{{cluster}}` or as just the name of the cluster.
        ///
        /// - - -
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// The initial number of nodes for the pool. In
        /// regional or multi-zonal clusters, this is the number of nodes per zone. Changing
        /// this will force recreation of the resource. WARNING: Resizing your node pool manually
        /// may change this value in your existing cluster, which will trigger destruction
        /// and recreation on the next provider run (to rectify the discrepancy).  If you don't
        /// need this value, don't set it.  If you do need it, you can use a lifecycle block to
        /// ignore subsqeuent changes to this field.
        pub initial_node_count: pulumi_gestalt_rust::Output<i32>,
        /// The resource URLs of the managed instance groups associated with this node pool.
        pub instance_group_urls: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The location (region or zone) of the cluster.
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// List of instance group URLs which have been assigned to this node pool.
        pub managed_instance_group_urls: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Node management configuration, wherein auto-repair and
        /// auto-upgrade is configured. Structure is documented below.
        pub management: pulumi_gestalt_rust::Output<
            super::super::types::container::NodePoolManagement,
        >,
        /// The maximum number of pods per node in this node pool.
        /// Note that this does not work on node pools which are "route-based" - that is, node
        /// pools belonging to clusters that do not have IP Aliasing enabled.
        /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/flexible-pod-cidr)
        /// for more information.
        pub max_pods_per_node: pulumi_gestalt_rust::Output<i32>,
        /// The name of the node pool. If left blank, the provider will
        /// auto-generate a unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name for the node pool beginning
        /// with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The network configuration of the pool. Such as
        /// configuration for [Adding Pod IP address ranges](https://cloud.google.com/kubernetes-engine/docs/how-to/multi-pod-cidr)) to the node pool. Or enabling private nodes. Structure is
        /// documented below
        pub network_config: pulumi_gestalt_rust::Output<
            super::super::types::container::NodePoolNetworkConfig,
        >,
        /// Parameters used in creating the node pool. See
        /// gcp.container.Cluster for schema.
        pub node_config: pulumi_gestalt_rust::Output<
            super::super::types::container::NodePoolNodeConfig,
        >,
        /// The number of nodes per instance group. This field can be used to
        /// update the number of nodes per instance group but should not be used alongside `autoscaling`.
        pub node_count: pulumi_gestalt_rust::Output<i32>,
        /// The list of zones in which the node pool's nodes should be located. Nodes must
        /// be in the region of their regional cluster or in the same region as their
        /// cluster's zone for zonal clusters. If unspecified, the cluster-level
        /// `node_locations` will be used.
        ///
        /// > Note: `node_locations` will not revert to the cluster's default set of zones
        /// upon being unset. You must manually reconcile the list of zones with your
        /// cluster.
        pub node_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        pub operation: pulumi_gestalt_rust::Output<String>,
        /// Specifies a custom placement policy for the
        /// nodes.
        pub placement_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::NodePoolPlacementPolicy>,
        >,
        /// The ID of the project in which to create the node pool. If blank,
        /// the provider-configured project will be used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Specifies node pool-level settings of queued provisioning.
        /// Structure is documented below.
        pub queued_provisioning: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::NodePoolQueuedProvisioning>,
        >,
        /// Specify node upgrade settings to change how GKE upgrades nodes.
        /// The maximum number of nodes upgraded simultaneously is limited to 20. Structure is documented below.
        pub upgrade_settings: pulumi_gestalt_rust::Output<
            super::super::types::container::NodePoolUpgradeSettings,
        >,
        /// The Kubernetes version for the nodes in this pool. Note that if this field
        /// and `auto_upgrade` are both specified, they will fight each other for what the node version should
        /// be, so setting both is highly discouraged. While a fuzzy version can be specified, it's
        /// recommended that you specify explicit versions as the provider will see spurious diffs
        /// when fuzzy versions are used. See the `gcp.container.getEngineVersions` data source's
        /// `version_prefix` field to approximate fuzzy versions in a provider-compatible way.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NodePoolArgs,
    ) -> NodePoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_binding = args.autoscaling.get_output(context);
        let cluster_binding = args.cluster.get_output(context);
        let initial_node_count_binding = args.initial_node_count.get_output(context);
        let location_binding = args.location.get_output(context);
        let management_binding = args.management.get_output(context);
        let max_pods_per_node_binding = args.max_pods_per_node.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let network_config_binding = args.network_config.get_output(context);
        let node_config_binding = args.node_config.get_output(context);
        let node_count_binding = args.node_count.get_output(context);
        let node_locations_binding = args.node_locations.get_output(context);
        let placement_policy_binding = args.placement_policy.get_output(context);
        let project_binding = args.project.get_output(context);
        let queued_provisioning_binding = args.queued_provisioning.get_output(context);
        let upgrade_settings_binding = args.upgrade_settings.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:container/nodePool:NodePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscaling".into(),
                    value: autoscaling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cluster".into(),
                    value: cluster_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialNodeCount".into(),
                    value: initial_node_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "management".into(),
                    value: management_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxPodsPerNode".into(),
                    value: max_pods_per_node_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfig".into(),
                    value: network_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeConfig".into(),
                    value: node_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeCount".into(),
                    value: node_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeLocations".into(),
                    value: node_locations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "placementPolicy".into(),
                    value: placement_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queuedProvisioning".into(),
                    value: queued_provisioning_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upgradeSettings".into(),
                    value: upgrade_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NodePoolResult {
            autoscaling: o.get_field("autoscaling"),
            cluster: o.get_field("cluster"),
            initial_node_count: o.get_field("initialNodeCount"),
            instance_group_urls: o.get_field("instanceGroupUrls"),
            location: o.get_field("location"),
            managed_instance_group_urls: o.get_field("managedInstanceGroupUrls"),
            management: o.get_field("management"),
            max_pods_per_node: o.get_field("maxPodsPerNode"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            network_config: o.get_field("networkConfig"),
            node_config: o.get_field("nodeConfig"),
            node_count: o.get_field("nodeCount"),
            node_locations: o.get_field("nodeLocations"),
            operation: o.get_field("operation"),
            placement_policy: o.get_field("placementPolicy"),
            project: o.get_field("project"),
            queued_provisioning: o.get_field("queuedProvisioning"),
            upgrade_settings: o.get_field("upgradeSettings"),
            version: o.get_field("version"),
        }
    }
}
