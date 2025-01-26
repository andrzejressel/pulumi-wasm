/// Represents a NodeTemplate resource. Node templates specify properties
/// for creating sole-tenant nodes, such as node type, vCPU and memory
/// requirements, node affinity labels, and region.
///
///
/// To get more information about NodeTemplate, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/nodeTemplates)
/// * How-to Guides
///     * [Sole-Tenant Nodes](https://cloud.google.com/compute/docs/nodes/)
///
/// ## Example Usage
///
/// ### Node Template Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let template = node_template::create(
///         "template",
///         NodeTemplateArgs::builder()
///             .name("soletenant-tmpl")
///             .node_type("n1-node-96-624")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Node Template Server Binding
///
///
/// ```yaml
/// resources:
///   template:
///     type: gcp:compute:NodeTemplate
///     properties:
///       name: soletenant-with-licenses
///       region: us-central1
///       nodeType: n1-node-96-624
///       nodeAffinityLabels:
///         foo: baz
///       serverBinding:
///         type: RESTART_NODE_ON_MINIMAL_SERVERS
/// variables:
///   central1a:
///     fn::invoke:
///       function: gcp:compute:getNodeTypes
///       arguments:
///         zone: us-central1-a
/// ```
/// ### Node Template Accelerators
///
///
/// ```yaml
/// resources:
///   template:
///     type: gcp:compute:NodeTemplate
///     properties:
///       name: soletenant-with-accelerators
///       region: us-central1
///       nodeType: n1-node-96-624
///       accelerators:
///         - acceleratorType: nvidia-tesla-t4
///           acceleratorCount: 4
/// variables:
///   central1a:
///     fn::invoke:
///       function: gcp:compute:getNodeTypes
///       arguments:
///         zone: us-central1-a
/// ```
/// ### Node Template Disks
///
///
/// ```yaml
/// resources:
///   template:
///     type: gcp:compute:NodeTemplate
///     properties:
///       name: soletenant-with-disks
///       region: us-central1
///       nodeType: n2-node-80-640
///       disks:
///         - diskCount: 16
///           diskSizeGb: 375
///           diskType: local-ssd
/// variables:
///   central1a:
///     fn::invoke:
///       function: gcp:compute:getNodeTypes
///       arguments:
///         zone: us-central1-a
/// ```
///
/// ## Import
///
/// NodeTemplate can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/nodeTemplates/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, NodeTemplate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/nodeTemplate:NodeTemplate default projects/{{project}}/regions/{{region}}/nodeTemplates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/nodeTemplate:NodeTemplate default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/nodeTemplate:NodeTemplate default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/nodeTemplate:NodeTemplate default {{name}}
/// ```
///
pub mod node_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodeTemplateArgs {
        /// List of the type and count of accelerator cards attached to the
        /// node template
        /// Structure is documented below.
        #[builder(into, default)]
        pub accelerators: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::NodeTemplateAccelerator>>,
        >,
        /// CPU overcommit.
        /// Default value is `NONE`.
        /// Possible values are: `ENABLED`, `NONE`.
        #[builder(into, default)]
        pub cpu_overcommit_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An optional textual description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of the type, size and count of disks attached to the
        /// node template
        /// Structure is documented below.
        #[builder(into, default)]
        pub disks: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::NodeTemplateDisk>>,
        >,
        /// Name of the resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Labels to use for node affinity, which will be used in
        /// instance scheduling.
        #[builder(into, default)]
        pub node_affinity_labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Node type to use for nodes group that are created from this template.
        /// Only one of nodeTypeFlexibility and nodeType can be specified.
        #[builder(into, default)]
        pub node_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Flexible properties for the desired node type. Node groups that
        /// use this node template will create nodes of a type that matches
        /// these properties. Only one of nodeTypeFlexibility and nodeType can
        /// be specified.
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_type_flexibility: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::NodeTemplateNodeTypeFlexibility>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region where nodes using the node template will be created.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The server binding policy for nodes using this template. Determines
        /// where the nodes should restart following a maintenance event.
        /// Structure is documented below.
        #[builder(into, default)]
        pub server_binding: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::NodeTemplateServerBinding>,
        >,
    }
    #[allow(dead_code)]
    pub struct NodeTemplateResult {
        /// List of the type and count of accelerator cards attached to the
        /// node template
        /// Structure is documented below.
        pub accelerators: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::NodeTemplateAccelerator>>,
        >,
        /// CPU overcommit.
        /// Default value is `NONE`.
        /// Possible values are: `ENABLED`, `NONE`.
        pub cpu_overcommit_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional textual description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// List of the type, size and count of disks attached to the
        /// node template
        /// Structure is documented below.
        pub disks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::NodeTemplateDisk>>,
        >,
        /// Name of the resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Labels to use for node affinity, which will be used in
        /// instance scheduling.
        pub node_affinity_labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Node type to use for nodes group that are created from this template.
        /// Only one of nodeTypeFlexibility and nodeType can be specified.
        pub node_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Flexible properties for the desired node type. Node groups that
        /// use this node template will create nodes of a type that matches
        /// these properties. Only one of nodeTypeFlexibility and nodeType can
        /// be specified.
        /// Structure is documented below.
        pub node_type_flexibility: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::NodeTemplateNodeTypeFlexibility>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Region where nodes using the node template will be created.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The server binding policy for nodes using this template. Determines
        /// where the nodes should restart following a maintenance event.
        /// Structure is documented below.
        pub server_binding: pulumi_wasm_rust::Output<
            super::super::types::compute::NodeTemplateServerBinding,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NodeTemplateArgs,
    ) -> NodeTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accelerators_binding = args.accelerators.get_output(context).get_inner();
        let cpu_overcommit_type_binding = args
            .cpu_overcommit_type
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let disks_binding = args.disks.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_affinity_labels_binding = args
            .node_affinity_labels
            .get_output(context)
            .get_inner();
        let node_type_binding = args.node_type.get_output(context).get_inner();
        let node_type_flexibility_binding = args
            .node_type_flexibility
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let server_binding_binding = args.server_binding.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/nodeTemplate:NodeTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accelerators".into(),
                    value: &accelerators_binding,
                },
                register_interface::ObjectField {
                    name: "cpuOvercommitType".into(),
                    value: &cpu_overcommit_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disks".into(),
                    value: &disks_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeAffinityLabels".into(),
                    value: &node_affinity_labels_binding,
                },
                register_interface::ObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding,
                },
                register_interface::ObjectField {
                    name: "nodeTypeFlexibility".into(),
                    value: &node_type_flexibility_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "serverBinding".into(),
                    value: &server_binding_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accelerators".into(),
                },
                register_interface::ResultField {
                    name: "cpuOvercommitType".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disks".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeAffinityLabels".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "nodeTypeFlexibility".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serverBinding".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NodeTemplateResult {
            accelerators: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accelerators").unwrap(),
            ),
            cpu_overcommit_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuOvercommitType").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disks").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_affinity_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeAffinityLabels").unwrap(),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            node_type_flexibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeTypeFlexibility").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            server_binding: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverBinding").unwrap(),
            ),
        }
    }
}
