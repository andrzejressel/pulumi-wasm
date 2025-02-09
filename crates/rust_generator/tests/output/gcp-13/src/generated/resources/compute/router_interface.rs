/// Manages a Cloud Router interface. For more information see
/// [the official documentation](https://cloud.google.com/compute/docs/cloudrouter)
/// and
/// [API](https://cloud.google.com/compute/docs/reference/latest/routers).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = router_interface::create(
///         "foobar",
///         RouterInterfaceArgs::builder()
///             .ip_range("169.254.1.1/30")
///             .name("interface-1")
///             .region("us-central1")
///             .router("router-1")
///             .vpn_tunnel("tunnel-1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Router interfaces can be imported using the `project` (optional), `region`, `router`, and `name`, e.g.
///
/// * `{{project_id}}/{{region}}/{{router}}/{{name}}`
///
/// * `{{region}}/{{router}}/{{name}}`
///
/// When using the `pulumi import` command, router interfaces can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/routerInterface:RouterInterface default {{project_id}}/{{region}}/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerInterface:RouterInterface default {{region}}/{{router}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod router_interface {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouterInterfaceArgs {
        /// The name or resource link to the
        /// VLAN interconnect for this interface. Changing this forces a new interface to
        /// be created. Only one of `vpn_tunnel`, `interconnect_attachment` or `subnetwork` can be specified.
        #[builder(into, default)]
        pub interconnect_attachment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP address and range of the interface. The IP range must be
        /// in the RFC3927 link-local IP space. Changing this forces a new interface to be created.
        #[builder(into, default)]
        pub ip_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP version of this interface. Can be either IPV4 or IPV6.
        #[builder(into, default)]
        pub ip_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique name for the interface, required by GCE. Changing
        /// this forces a new interface to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The regional private internal IP address that is used
        /// to establish BGP sessions to a VM instance acting as a third-party Router Appliance. Changing this forces a new interface to be created.
        #[builder(into, default)]
        pub private_ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which this interface's routerbelongs.
        /// If it is not provided, the provider project is used. Changing this forces a new interface to be created.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the interface that is redundant to
        /// this interface. Changing this forces a new interface to be created.
        #[builder(into, default)]
        pub redundant_interface: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region this interface's router sits in.
        /// If not specified, the project region will be used. Changing this forces a new interface to be created.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the router this interface will be attached to.
        /// Changing this forces a new interface to be created.
        ///
        /// In addition to the above required fields, a router interface must have specified either `ip_range` or exactly one of `vpn_tunnel`, `interconnect_attachment` or `subnetwork`, or both.
        ///
        /// - - -
        #[builder(into)]
        pub router: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URI of the subnetwork resource that this interface
        /// belongs to, which must be in the same region as the Cloud Router. When you establish a BGP session to a VM instance using this interface, the VM instance must belong to the same subnetwork as the subnetwork specified here. Changing this forces a new interface to be created. Only one of `vpn_tunnel`, `interconnect_attachment` or `subnetwork` can be specified.
        #[builder(into, default)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name or resource link to the VPN tunnel this
        /// interface will be linked to. Changing this forces a new interface to be created. Only
        /// one of `vpn_tunnel`, `interconnect_attachment` or `subnetwork` can be specified.
        #[builder(into, default)]
        pub vpn_tunnel: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouterInterfaceResult {
        /// The name or resource link to the
        /// VLAN interconnect for this interface. Changing this forces a new interface to
        /// be created. Only one of `vpn_tunnel`, `interconnect_attachment` or `subnetwork` can be specified.
        pub interconnect_attachment: pulumi_gestalt_rust::Output<Option<String>>,
        /// IP address and range of the interface. The IP range must be
        /// in the RFC3927 link-local IP space. Changing this forces a new interface to be created.
        pub ip_range: pulumi_gestalt_rust::Output<String>,
        /// IP version of this interface. Can be either IPV4 or IPV6.
        pub ip_version: pulumi_gestalt_rust::Output<String>,
        /// A unique name for the interface, required by GCE. Changing
        /// this forces a new interface to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The regional private internal IP address that is used
        /// to establish BGP sessions to a VM instance acting as a third-party Router Appliance. Changing this forces a new interface to be created.
        pub private_ip_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which this interface's routerbelongs.
        /// If it is not provided, the provider project is used. Changing this forces a new interface to be created.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The name of the interface that is redundant to
        /// this interface. Changing this forces a new interface to be created.
        pub redundant_interface: pulumi_gestalt_rust::Output<String>,
        /// The region this interface's router sits in.
        /// If not specified, the project region will be used. Changing this forces a new interface to be created.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The name of the router this interface will be attached to.
        /// Changing this forces a new interface to be created.
        ///
        /// In addition to the above required fields, a router interface must have specified either `ip_range` or exactly one of `vpn_tunnel`, `interconnect_attachment` or `subnetwork`, or both.
        ///
        /// - - -
        pub router: pulumi_gestalt_rust::Output<String>,
        /// The URI of the subnetwork resource that this interface
        /// belongs to, which must be in the same region as the Cloud Router. When you establish a BGP session to a VM instance using this interface, the VM instance must belong to the same subnetwork as the subnetwork specified here. Changing this forces a new interface to be created. Only one of `vpn_tunnel`, `interconnect_attachment` or `subnetwork` can be specified.
        pub subnetwork: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name or resource link to the VPN tunnel this
        /// interface will be linked to. Changing this forces a new interface to be created. Only
        /// one of `vpn_tunnel`, `interconnect_attachment` or `subnetwork` can be specified.
        pub vpn_tunnel: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RouterInterfaceArgs,
    ) -> RouterInterfaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let interconnect_attachment_binding_1 = args
            .interconnect_attachment
            .get_output(context);
        let interconnect_attachment_binding = interconnect_attachment_binding_1
            .get_inner();
        let ip_range_binding_1 = args.ip_range.get_output(context);
        let ip_range_binding = ip_range_binding_1.get_inner();
        let ip_version_binding_1 = args.ip_version.get_output(context);
        let ip_version_binding = ip_version_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let private_ip_address_binding_1 = args.private_ip_address.get_output(context);
        let private_ip_address_binding = private_ip_address_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let redundant_interface_binding_1 = args.redundant_interface.get_output(context);
        let redundant_interface_binding = redundant_interface_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let router_binding_1 = args.router.get_output(context);
        let router_binding = router_binding_1.get_inner();
        let subnetwork_binding_1 = args.subnetwork.get_output(context);
        let subnetwork_binding = subnetwork_binding_1.get_inner();
        let vpn_tunnel_binding_1 = args.vpn_tunnel.get_output(context);
        let vpn_tunnel_binding = vpn_tunnel_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/routerInterface:RouterInterface".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "interconnectAttachment".into(),
                    value: &interconnect_attachment_binding,
                },
                register_interface::ObjectField {
                    name: "ipRange".into(),
                    value: &ip_range_binding,
                },
                register_interface::ObjectField {
                    name: "ipVersion".into(),
                    value: &ip_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpAddress".into(),
                    value: &private_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "redundantInterface".into(),
                    value: &redundant_interface_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "router".into(),
                    value: &router_binding,
                },
                register_interface::ObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding,
                },
                register_interface::ObjectField {
                    name: "vpnTunnel".into(),
                    value: &vpn_tunnel_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouterInterfaceResult {
            interconnect_attachment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interconnectAttachment"),
            ),
            ip_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipRange"),
            ),
            ip_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            redundant_interface: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redundantInterface"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            router: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("router"),
            ),
            subnetwork: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            vpn_tunnel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpnTunnel"),
            ),
        }
    }
}
