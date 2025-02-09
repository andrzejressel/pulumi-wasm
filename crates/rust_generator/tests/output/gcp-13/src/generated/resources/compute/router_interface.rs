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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouterInterfaceArgs,
    ) -> RouterInterfaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let interconnect_attachment_binding = args
            .interconnect_attachment
            .get_output(context);
        let ip_range_binding = args.ip_range.get_output(context);
        let ip_version_binding = args.ip_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let private_ip_address_binding = args.private_ip_address.get_output(context);
        let project_binding = args.project.get_output(context);
        let redundant_interface_binding = args.redundant_interface.get_output(context);
        let region_binding = args.region.get_output(context);
        let router_binding = args.router.get_output(context);
        let subnetwork_binding = args.subnetwork.get_output(context);
        let vpn_tunnel_binding = args.vpn_tunnel.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/routerInterface:RouterInterface".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interconnectAttachment".into(),
                    value: interconnect_attachment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipRange".into(),
                    value: ip_range_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipVersion".into(),
                    value: ip_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateIpAddress".into(),
                    value: private_ip_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redundantInterface".into(),
                    value: redundant_interface_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "router".into(),
                    value: router_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetwork".into(),
                    value: subnetwork_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnTunnel".into(),
                    value: vpn_tunnel_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouterInterfaceResult {
            interconnect_attachment: o.get_field("interconnectAttachment"),
            ip_range: o.get_field("ipRange"),
            ip_version: o.get_field("ipVersion"),
            name: o.get_field("name"),
            private_ip_address: o.get_field("privateIpAddress"),
            project: o.get_field("project"),
            redundant_interface: o.get_field("redundantInterface"),
            region: o.get_field("region"),
            router: o.get_field("router"),
            subnetwork: o.get_field("subnetwork"),
            vpn_tunnel: o.get_field("vpnTunnel"),
        }
    }
}
