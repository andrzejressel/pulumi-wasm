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
pub mod node_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodeGroupArgs {
        /// If you use sole-tenant nodes for your workloads, you can use the node
        /// group autoscaler to automatically manage the sizes of your node groups.
        /// One of `initial_size` or `autoscaling_policy` must be configured on resource creation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub autoscaling_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::NodeGroupAutoscalingPolicy>,
        >,
        /// An optional textual description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The initial number of nodes in the node group. One of `initial_size` or `autoscaling_policy` must be configured on resource creation.
        #[builder(into, default)]
        pub initial_size: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies the frequency of planned maintenance events. Set to one of the following:
        /// - AS_NEEDED: Hosts are eligible to receive infrastructure and hypervisor updates as they become available.
        /// - RECURRENT: Hosts receive planned infrastructure and hypervisor updates on a periodic basis, but not more frequently than every 28 days. This minimizes the number of planned maintenance operations on individual hosts and reduces the frequency of disruptions, both live migrations and terminations, on individual VMs.
        /// Possible values are: `AS_NEEDED`, `RECURRENT`.
        #[builder(into, default)]
        pub maintenance_interval: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT.
        #[builder(into, default)]
        pub maintenance_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// contains properties for the timeframe of maintenance
        /// Structure is documented below.
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::NodeGroupMaintenanceWindow>,
        >,
        /// Name of the resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The URL of the node template to which this node group belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub node_template: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Share settings for the node group.
        /// Structure is documented below.
        #[builder(into, default)]
        pub share_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::NodeGroupShareSettings>,
        >,
        /// Zone where this node group is located
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NodeGroupResult {
        /// If you use sole-tenant nodes for your workloads, you can use the node
        /// group autoscaler to automatically manage the sizes of your node groups.
        /// One of `initial_size` or `autoscaling_policy` must be configured on resource creation.
        /// Structure is documented below.
        pub autoscaling_policy: pulumi_wasm_rust::Output<
            super::super::types::compute::NodeGroupAutoscalingPolicy,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional textual description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The initial number of nodes in the node group. One of `initial_size` or `autoscaling_policy` must be configured on resource creation.
        pub initial_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the frequency of planned maintenance events. Set to one of the following:
        /// - AS_NEEDED: Hosts are eligible to receive infrastructure and hypervisor updates as they become available.
        /// - RECURRENT: Hosts receive planned infrastructure and hypervisor updates on a periodic basis, but not more frequently than every 28 days. This minimizes the number of planned maintenance operations on individual hosts and reduces the frequency of disruptions, both live migrations and terminations, on individual VMs.
        /// Possible values are: `AS_NEEDED`, `RECURRENT`.
        pub maintenance_interval: pulumi_wasm_rust::Output<String>,
        /// Specifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT.
        pub maintenance_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// contains properties for the timeframe of maintenance
        /// Structure is documented below.
        pub maintenance_window: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::NodeGroupMaintenanceWindow>,
        >,
        /// Name of the resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The URL of the node template to which this node group belongs.
        ///
        ///
        /// - - -
        pub node_template: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Share settings for the node group.
        /// Structure is documented below.
        pub share_settings: pulumi_wasm_rust::Output<
            super::super::types::compute::NodeGroupShareSettings,
        >,
        /// The total number of nodes in the node group.
        pub size: pulumi_wasm_rust::Output<i32>,
        /// Zone where this node group is located
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NodeGroupArgs,
    ) -> NodeGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autoscaling_policy_binding = args
            .autoscaling_policy
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let initial_size_binding = args.initial_size.get_output(context).get_inner();
        let maintenance_interval_binding = args
            .maintenance_interval
            .get_output(context)
            .get_inner();
        let maintenance_policy_binding = args
            .maintenance_policy
            .get_output(context)
            .get_inner();
        let maintenance_window_binding = args
            .maintenance_window
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_template_binding = args.node_template.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let share_settings_binding = args.share_settings.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/nodeGroup:NodeGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscalingPolicy".into(),
                    value: &autoscaling_policy_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "initialSize".into(),
                    value: &initial_size_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceInterval".into(),
                    value: &maintenance_interval_binding,
                },
                register_interface::ObjectField {
                    name: "maintenancePolicy".into(),
                    value: &maintenance_policy_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeTemplate".into(),
                    value: &node_template_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "shareSettings".into(),
                    value: &share_settings_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoscalingPolicy".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "initialSize".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceInterval".into(),
                },
                register_interface::ResultField {
                    name: "maintenancePolicy".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeTemplate".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "shareSettings".into(),
                },
                register_interface::ResultField {
                    name: "size".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NodeGroupResult {
            autoscaling_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingPolicy").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            initial_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initialSize").unwrap(),
            ),
            maintenance_interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceInterval").unwrap(),
            ),
            maintenance_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenancePolicy").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeTemplate").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            share_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareSettings").unwrap(),
            ),
            size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("size").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
