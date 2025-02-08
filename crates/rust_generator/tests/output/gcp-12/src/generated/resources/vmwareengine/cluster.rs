/// A cluster in a private cloud.
///
///
/// To get more information about Cluster, see:
///
/// * [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.clusters)
///
/// ## Example Usage
///
/// ### Vmware Engine Cluster Basic
///
///
/// ```yaml
/// resources:
///   vmw-engine-ext-cluster:
///     type: gcp:vmwareengine:Cluster
///     properties:
///       name: ext-cluster
///       parent: ${["cluster-pc"].id}
///       nodeTypeConfigs:
///         - nodeTypeId: standard-72
///           nodeCount: 3
///   cluster-pc:
///     type: gcp:vmwareengine:PrivateCloud
///     properties:
///       location: us-west1-a
///       name: sample-pc
///       description: Sample test PC.
///       networkConfig:
///         managementCidr: 192.168.30.0/24
///         vmwareEngineNetwork: ${["cluster-nw"].id}
///       managementCluster:
///         clusterId: sample-mgmt-cluster
///         nodeTypeConfigs:
///           - nodeTypeId: standard-72
///             nodeCount: 3
///   cluster-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: pc-nw
///       type: STANDARD
///       location: global
///       description: PC network description.
/// ```
/// ### Vmware Engine Cluster Full
///
///
/// ```yaml
/// resources:
///   vmw-ext-cluster:
///     type: gcp:vmwareengine:Cluster
///     properties:
///       name: ext-cluster
///       parent: ${["cluster-pc"].id}
///       nodeTypeConfigs:
///         - nodeTypeId: standard-72
///           nodeCount: 3
///           customCoreCount: 32
///       autoscalingSettings:
///         autoscalingPolicies:
///           - autoscalePolicyId: autoscaling-policy
///             nodeTypeId: standard-72
///             scaleOutSize: 1
///             cpuThresholds:
///               scaleOut: 80
///               scaleIn: 15
///             consumedMemoryThresholds:
///               scaleOut: 75
///               scaleIn: 20
///             storageThresholds:
///               scaleOut: 80
///               scaleIn: 20
///         minClusterNodeCount: 3
///         maxClusterNodeCount: 8
///         coolDownPeriod: 1800s
///   cluster-pc:
///     type: gcp:vmwareengine:PrivateCloud
///     properties:
///       location: us-west1-a
///       name: sample-pc
///       description: Sample test PC.
///       networkConfig:
///         managementCidr: 192.168.30.0/24
///         vmwareEngineNetwork: ${["cluster-nw"].id}
///       managementCluster:
///         clusterId: sample-mgmt-cluster
///         nodeTypeConfigs:
///           - nodeTypeId: standard-72
///             nodeCount: 3
///             customCoreCount: 32
///   cluster-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: pc-nw
///       type: STANDARD
///       location: global
///       description: PC network description.
/// ```
///
/// ## Import
///
/// Cluster can be imported using any of these accepted formats:
///
/// * `{{parent}}/clusters/{{name}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/cluster:Cluster default {{parent}}/clusters/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Configuration of the autoscaling applied to this cluster
        /// Structure is documented below.
        #[builder(into, default)]
        pub autoscaling_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vmwareengine::ClusterAutoscalingSettings>,
        >,
        /// The ID of the Cluster.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The map of cluster node types in this cluster,
        /// where the key is canonical identifier of the node type (corresponds to the NodeType).
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_type_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::vmwareengine::ClusterNodeTypeConfig>>,
        >,
        /// The resource name of the private cloud to create a new cluster in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Configuration of the autoscaling applied to this cluster
        /// Structure is documented below.
        pub autoscaling_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::vmwareengine::ClusterAutoscalingSettings>,
        >,
        /// True if the cluster is a management cluster; false otherwise.
        /// There can only be one management cluster in a private cloud and it has to be the first one.
        pub management: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the Cluster.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The map of cluster node types in this cluster,
        /// where the key is canonical identifier of the node type (corresponds to the NodeType).
        /// Structure is documented below.
        pub node_type_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::vmwareengine::ClusterNodeTypeConfig>>,
        >,
        /// The resource name of the private cloud to create a new cluster in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// State of the Cluster.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let autoscaling_settings_binding = args
            .autoscaling_settings
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_type_configs_binding = args
            .node_type_configs
            .get_output(context)
            .get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vmwareengine/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscalingSettings".into(),
                    value: &autoscaling_settings_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeTypeConfigs".into(),
                    value: &node_type_configs_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterResult {
            autoscaling_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoscalingSettings"),
            ),
            management: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("management"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            node_type_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeTypeConfigs"),
            ),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
        }
    }
}
