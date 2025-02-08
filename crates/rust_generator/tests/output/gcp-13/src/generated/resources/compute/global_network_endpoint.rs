/// A Global Network endpoint represents a IP address and port combination that exists outside of GCP.
/// **NOTE**: Global network endpoints cannot be created outside of a
/// global network endpoint group.
///
///
/// To get more information about GlobalNetworkEndpoint, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/networkEndpointGroups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/negs/)
///
/// ## Example Usage
///
/// ### Global Network Endpoint
///
///
/// ```yaml
/// resources:
///   default-endpoint:
///     type: gcp:compute:GlobalNetworkEndpoint
///     properties:
///       globalNetworkEndpointGroup: ${neg.name}
///       fqdn: www.example.com
///       port: 90
///   neg:
///     type: gcp:compute:GlobalNetworkEndpointGroup
///     properties:
///       name: my-lb-neg
///       defaultPort: '90'
///       networkEndpointType: INTERNET_FQDN_PORT
/// ```
///
/// ## Import
///
/// GlobalNetworkEndpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/networkEndpointGroups/{{global_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}`
///
/// * `{{project}}/{{global_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}`
///
/// * `{{global_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}`
///
/// When using the `pulumi import` command, GlobalNetworkEndpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/globalNetworkEndpoint:GlobalNetworkEndpoint default projects/{{project}}/global/networkEndpointGroups/{{global_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/globalNetworkEndpoint:GlobalNetworkEndpoint default {{project}}/{{global_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/globalNetworkEndpoint:GlobalNetworkEndpoint default {{global_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod global_network_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalNetworkEndpointArgs {
        /// Fully qualified domain name of network endpoint.
        /// This can only be specified when network_endpoint_type of the NEG is INTERNET_FQDN_PORT.
        #[builder(into, default)]
        pub fqdn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The global network endpoint group this endpoint is part of.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub global_network_endpoint_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// IPv4 address external endpoint.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Port number of the external endpoint.
        #[builder(into)]
        pub port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GlobalNetworkEndpointResult {
        /// Fully qualified domain name of network endpoint.
        /// This can only be specified when network_endpoint_type of the NEG is INTERNET_FQDN_PORT.
        pub fqdn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The global network endpoint group this endpoint is part of.
        ///
        ///
        /// - - -
        pub global_network_endpoint_group: pulumi_gestalt_rust::Output<String>,
        /// IPv4 address external endpoint.
        pub ip_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// Port number of the external endpoint.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GlobalNetworkEndpointArgs,
    ) -> GlobalNetworkEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let fqdn_binding = args.fqdn.get_output(context).get_inner();
        let global_network_endpoint_group_binding = args
            .global_network_endpoint_group
            .get_output(context)
            .get_inner();
        let ip_address_binding = args.ip_address.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/globalNetworkEndpoint:GlobalNetworkEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fqdn".into(),
                    value: &fqdn_binding,
                },
                register_interface::ObjectField {
                    name: "globalNetworkEndpointGroup".into(),
                    value: &global_network_endpoint_group_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GlobalNetworkEndpointResult {
            fqdn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fqdn")),
            global_network_endpoint_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalNetworkEndpointGroup"),
            ),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
