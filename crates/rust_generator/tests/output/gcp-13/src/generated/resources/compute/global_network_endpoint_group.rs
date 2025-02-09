/// A global network endpoint group contains endpoints that reside outside of Google Cloud.
/// Currently a global network endpoint group can only support a single endpoint.
///
/// Recreating a global network endpoint group that's in use by another resource will give a
/// `resourceInUseByAnotherResource` error. Use `lifecycle.create_before_destroy`
/// to avoid this type of error.
///
///
/// To get more information about GlobalNetworkEndpointGroup, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/networkEndpointGroups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/negs/internet-neg-concepts)
///
/// ## Example Usage
///
/// ### Global Network Endpoint Group
///
///
/// ```yaml
/// resources:
///   neg:
///     type: gcp:compute:GlobalNetworkEndpointGroup
///     properties:
///       name: my-lb-neg
///       defaultPort: '90'
///       networkEndpointType: INTERNET_FQDN_PORT
/// ```
/// ### Global Network Endpoint Group Ip Address
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let neg = global_network_endpoint_group::create(
///         "neg",
///         GlobalNetworkEndpointGroupArgs::builder()
///             .default_port(90)
///             .name("my-lb-neg")
///             .network_endpoint_type("INTERNET_IP_PORT")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// GlobalNetworkEndpointGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/networkEndpointGroups/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, GlobalNetworkEndpointGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/globalNetworkEndpointGroup:GlobalNetworkEndpointGroup default projects/{{project}}/global/networkEndpointGroups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/globalNetworkEndpointGroup:GlobalNetworkEndpointGroup default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/globalNetworkEndpointGroup:GlobalNetworkEndpointGroup default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod global_network_endpoint_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalNetworkEndpointGroupArgs {
        /// The default port used if the port number is not specified in the
        /// network endpoint.
        #[builder(into, default)]
        pub default_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of network endpoints in this network endpoint group.
        /// Possible values are: `INTERNET_IP_PORT`, `INTERNET_FQDN_PORT`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network_endpoint_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GlobalNetworkEndpointGroupResult {
        /// The default port used if the port number is not specified in the
        /// network endpoint.
        pub default_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Type of network endpoints in this network endpoint group.
        /// Possible values are: `INTERNET_IP_PORT`, `INTERNET_FQDN_PORT`.
        ///
        ///
        /// - - -
        pub network_endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GlobalNetworkEndpointGroupArgs,
    ) -> GlobalNetworkEndpointGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let default_port_binding_1 = args.default_port.get_output(context);
        let default_port_binding = default_port_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let network_endpoint_type_binding_1 = args
            .network_endpoint_type
            .get_output(context);
        let network_endpoint_type_binding = network_endpoint_type_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/globalNetworkEndpointGroup:GlobalNetworkEndpointGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultPort".into(),
                    value: &default_port_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkEndpointType".into(),
                    value: &network_endpoint_type_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GlobalNetworkEndpointGroupResult {
            default_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultPort"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_endpoint_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkEndpointType"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
