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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_settings_binding = args.autoscaling_settings.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_type_configs_binding = args.node_type_configs.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vmwareengine/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingSettings".into(),
                    value: autoscaling_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeTypeConfigs".into(),
                    value: node_type_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            autoscaling_settings: o.get_field("autoscalingSettings"),
            management: o.get_field("management"),
            name: o.get_field("name"),
            node_type_configs: o.get_field("nodeTypeConfigs"),
            parent: o.get_field("parent"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
        }
    }
}
