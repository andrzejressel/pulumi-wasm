/// Represents a NodeGroup resource to manage a group of sole-tenant nodes.
///
///
/// To get more information about NodeGroup, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/nodeGroups)
/// * How-to Guides
///     * [Sole-Tenant Nodes](https://cloud.google.com/compute/docs/nodes/)
///
/// > **Warning:** Due to limitations of the API, this provider cannot update the
/// number of nodes in a node group and changes to node group size either
/// through provider config or through external changes will cause
/// the provider to delete and recreate the node group.
///
/// ## Example Usage
///
/// ### Node Group Basic
///
///
/// ```yaml
/// resources:
///   soletenant-tmpl:
///     type: gcp:compute:NodeTemplate
///     properties:
///       name: soletenant-tmpl
///       region: us-central1
///       nodeType: n1-node-96-624
///   nodes:
///     type: gcp:compute:NodeGroup
///     properties:
///       name: soletenant-group
///       zone: us-central1-a
///       description: example google_compute_node_group for the Google Provider
///       initialSize: 1
///       nodeTemplate: ${["soletenant-tmpl"].id}
/// ```
/// ### Node Group Maintenance Interval
///
///
/// ```yaml
/// resources:
///   soletenant-tmpl:
///     type: gcp:compute:NodeTemplate
///     properties:
///       name: soletenant-tmpl
///       region: us-central1
///       nodeType: c2-node-60-240
///   nodes:
///     type: gcp:compute:NodeGroup
///     properties:
///       name: soletenant-group
///       zone: us-central1-a
///       description: example google_compute_node_group for Terraform Google Provider
///       initialSize: 1
///       nodeTemplate: ${["soletenant-tmpl"].id}
///       maintenanceInterval: RECURRENT
/// ```
/// ### Node Group Autoscaling Policy
///
///
/// ```yaml
/// resources:
///   soletenant-tmpl:
///     type: gcp:compute:NodeTemplate
///     properties:
///       name: soletenant-tmpl
///       region: us-central1
///       nodeType: n1-node-96-624
///   nodes:
///     type: gcp:compute:NodeGroup
///     properties:
///       name: soletenant-group
///       zone: us-central1-a
///       description: example google_compute_node_group for Google Provider
///       maintenancePolicy: RESTART_IN_PLACE
///       maintenanceWindow:
///         startTime: 08:00
///       initialSize: 1
///       nodeTemplate: ${["soletenant-tmpl"].id}
///       autoscalingPolicy:
///         mode: ONLY_SCALE_OUT
///         minNodes: 1
///         maxNodes: 10
/// ```
/// ### Node Group Share Settings
///
///
/// ```yaml
/// resources:
///   guestProject:
///     type: gcp:organizations:Project
///     name: guest_project
///     properties:
///       projectId: project-id
///       name: project-name
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   soletenant-tmpl:
///     type: gcp:compute:NodeTemplate
///     properties:
///       name: soletenant-tmpl
///       region: us-central1
///       nodeType: n1-node-96-624
///   nodes:
///     type: gcp:compute:NodeGroup
///     properties:
///       name: soletenant-group
///       zone: us-central1-f
///       description: example google_compute_node_group for Terraform Google Provider
///       initialSize: 1
///       nodeTemplate: ${["soletenant-tmpl"].id}
///       shareSettings:
///         shareType: SPECIFIC_PROJECTS
///         projectMaps:
///           - id: ${guestProject.projectId}
///             projectId: ${guestProject.projectId}
/// ```
///
/// ## Import
///
/// NodeGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/nodeGroups/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, NodeGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/nodeGroup:NodeGroup default projects/{{project}}/zones/{{zone}}/nodeGroups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/nodeGroup:NodeGroup default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/nodeGroup:NodeGroup default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/nodeGroup:NodeGroup default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod node_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodeGroupArgs {
        /// If you use sole-tenant nodes for your workloads, you can use the node
        /// group autoscaler to automatically manage the sizes of your node groups.
        /// One of `initial_size` or `autoscaling_policy` must be configured on resource creation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub autoscaling_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::NodeGroupAutoscalingPolicy>,
        >,
        /// An optional textual description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The initial number of nodes in the node group. One of `initial_size` or `autoscaling_policy` must be configured on resource creation.
        #[builder(into, default)]
        pub initial_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the frequency of planned maintenance events. Set to one of the following:
        /// - AS_NEEDED: Hosts are eligible to receive infrastructure and hypervisor updates as they become available.
        /// - RECURRENT: Hosts receive planned infrastructure and hypervisor updates on a periodic basis, but not more frequently than every 28 days. This minimizes the number of planned maintenance operations on individual hosts and reduces the frequency of disruptions, both live migrations and terminations, on individual VMs.
        /// Possible values are: `AS_NEEDED`, `RECURRENT`.
        #[builder(into, default)]
        pub maintenance_interval: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT.
        #[builder(into, default)]
        pub maintenance_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// contains properties for the timeframe of maintenance
        /// Structure is documented below.
        #[builder(into, default)]
        pub maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::NodeGroupMaintenanceWindow>,
        >,
        /// Name of the resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the node template to which this node group belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub node_template: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Share settings for the node group.
        /// Structure is documented below.
        #[builder(into, default)]
        pub share_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::NodeGroupShareSettings>,
        >,
        /// Zone where this node group is located
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NodeGroupResult {
        /// If you use sole-tenant nodes for your workloads, you can use the node
        /// group autoscaler to automatically manage the sizes of your node groups.
        /// One of `initial_size` or `autoscaling_policy` must be configured on resource creation.
        /// Structure is documented below.
        pub autoscaling_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::NodeGroupAutoscalingPolicy,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional textual description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The initial number of nodes in the node group. One of `initial_size` or `autoscaling_policy` must be configured on resource creation.
        pub initial_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the frequency of planned maintenance events. Set to one of the following:
        /// - AS_NEEDED: Hosts are eligible to receive infrastructure and hypervisor updates as they become available.
        /// - RECURRENT: Hosts receive planned infrastructure and hypervisor updates on a periodic basis, but not more frequently than every 28 days. This minimizes the number of planned maintenance operations on individual hosts and reduces the frequency of disruptions, both live migrations and terminations, on individual VMs.
        /// Possible values are: `AS_NEEDED`, `RECURRENT`.
        pub maintenance_interval: pulumi_gestalt_rust::Output<String>,
        /// Specifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT.
        pub maintenance_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// contains properties for the timeframe of maintenance
        /// Structure is documented below.
        pub maintenance_window: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::NodeGroupMaintenanceWindow>,
        >,
        /// Name of the resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The URL of the node template to which this node group belongs.
        ///
        ///
        /// - - -
        pub node_template: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Share settings for the node group.
        /// Structure is documented below.
        pub share_settings: pulumi_gestalt_rust::Output<
            super::super::types::compute::NodeGroupShareSettings,
        >,
        /// The total number of nodes in the node group.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// Zone where this node group is located
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NodeGroupArgs,
    ) -> NodeGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_policy_binding = args.autoscaling_policy.get_output(context);
        let description_binding = args.description.get_output(context);
        let initial_size_binding = args.initial_size.get_output(context);
        let maintenance_interval_binding = args.maintenance_interval.get_output(context);
        let maintenance_policy_binding = args.maintenance_policy.get_output(context);
        let maintenance_window_binding = args.maintenance_window.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_template_binding = args.node_template.get_output(context);
        let project_binding = args.project.get_output(context);
        let share_settings_binding = args.share_settings.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/nodeGroup:NodeGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingPolicy".into(),
                    value: autoscaling_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialSize".into(),
                    value: initial_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceInterval".into(),
                    value: maintenance_interval_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenancePolicy".into(),
                    value: maintenance_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceWindow".into(),
                    value: maintenance_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeTemplate".into(),
                    value: node_template_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shareSettings".into(),
                    value: share_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NodeGroupResult {
            autoscaling_policy: o.get_field("autoscalingPolicy"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            initial_size: o.get_field("initialSize"),
            maintenance_interval: o.get_field("maintenanceInterval"),
            maintenance_policy: o.get_field("maintenancePolicy"),
            maintenance_window: o.get_field("maintenanceWindow"),
            name: o.get_field("name"),
            node_template: o.get_field("nodeTemplate"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            share_settings: o.get_field("shareSettings"),
            size: o.get_field("size"),
            zone: o.get_field("zone"),
        }
    }
}
