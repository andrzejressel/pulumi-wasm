/// Creates a group of dissimilar Compute Engine virtual machine instances.
/// For more information, see [the official documentation](https://cloud.google.com/compute/docs/instance-groups/#unmanaged_instance_groups)
/// and [API](https://cloud.google.com/compute/docs/reference/latest/instanceGroups)
///
///
/// ## Example Usage
///
/// ### Empty Instance Group
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = instance_group::create(
///         "test",
///         InstanceGroupArgs::builder()
///             .description("Test instance group")
///             .name("test")
///             .network("${default.id}")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example Usage - With instances and named ports
///
/// ```yaml
/// resources:
///   webservers:
///     type: gcp:compute:InstanceGroup
///     properties:
///       name: webservers
///       description: Test instance group
///       instances:
///         - ${test.id}
///         - ${test2.id}
///       namedPorts:
///         - name: http
///           port: '8080'
///         - name: https
///           port: '8443'
///       zone: us-central1-a
/// ```
///
/// ## Import
///
/// Instance groups can be imported using the `zone` and `name` with an optional `project`, e.g.
///
/// * `projects/{{project_id}}/zones/{{zone}}/instanceGroups/{{instance_group_id}}`
///
/// * `{{project_id}}/{{zone}}/{{instance_group_id}}`
///
/// * `{{zone}}/{{instance_group_id}}`
///
/// When using the `pulumi import` command, instance groups can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroup:InstanceGroup default {{zone}}/{{instance_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroup:InstanceGroup default {{project_id}}/{{zone}}/{{instance_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroup:InstanceGroup default projects/{{project_id}}/zones/{{zone}}/instanceGroups/{{instance_group_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceGroupArgs {
        /// An optional textual description of the instance
        /// group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of instances in the group, in `self_link` format.
        /// When adding instances they must all be in the same network and zone as the instance group.
        #[builder(into, default)]
        pub instances: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the instance group. Must be 1-63
        /// characters long and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
        /// include lowercase letters, numbers, and hyphens.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The named port configuration. See the section below
        /// for details on configuration. Structure is documented below.
        #[builder(into, default)]
        pub named_ports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::InstanceGroupNamedPort>>,
        >,
        /// The URL of the network the instance group is in. If
        /// this is different from the network where the instances are in, the creation
        /// fails. Defaults to the network where the instances are in (if neither
        /// `network` nor `instances` is specified, this field will be blank).
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone that this instance group should be created in.
        ///
        /// - - -
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceGroupResult {
        /// An optional textual description of the instance
        /// group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The list of instances in the group, in `self_link` format.
        /// When adding instances they must all be in the same network and zone as the instance group.
        pub instances: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the instance group. Must be 1-63
        /// characters long and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
        /// include lowercase letters, numbers, and hyphens.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The named port configuration. See the section below
        /// for details on configuration. Structure is documented below.
        pub named_ports: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::InstanceGroupNamedPort>>,
        >,
        /// The URL of the network the instance group is in. If
        /// this is different from the network where the instances are in, the creation
        /// fails. Defaults to the network where the instances are in (if neither
        /// `network` nor `instances` is specified, this field will be blank).
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The number of instances in the group.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// The zone that this instance group should be created in.
        ///
        /// - - -
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceGroupArgs,
    ) -> InstanceGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let instances_binding_1 = args.instances.get_output(context);
        let instances_binding = instances_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let named_ports_binding_1 = args.named_ports.get_output(context);
        let named_ports_binding = named_ports_binding_1.get_inner();
        let network_binding_1 = args.network.get_output(context);
        let network_binding = network_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let zone_binding_1 = args.zone.get_output(context);
        let zone_binding = zone_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/instanceGroup:InstanceGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instances".into(),
                    value: &instances_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namedPorts".into(),
                    value: &named_ports_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceGroupResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instances"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            named_ports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namedPorts"),
            ),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            size: pulumi_gestalt_rust::__private::into_domain(o.extract_field("size")),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
