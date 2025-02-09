/// Provides a resource, that manages Cloudflare tunnel routes for Zero
/// Trust. Tunnel routes are used to direct IP traffic through
/// Cloudflare Tunnels.
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/tunnelRoute:TunnelRoute example <account_id>/<network_cidr>/<virtual_network_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tunnel_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelRouteArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the tunnel route.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 or IPv6 network that should use this tunnel route, in CIDR notation.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the tunnel that will service the tunnel route.
        #[builder(into)]
        pub tunnel_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the virtual network for which this route is being added; uses the default virtual network of the account if none is provided. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub virtual_network_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TunnelRouteResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the tunnel route.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 or IPv6 network that should use this tunnel route, in CIDR notation.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the tunnel that will service the tunnel route.
        pub tunnel_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the virtual network for which this route is being added; uses the default virtual network of the account if none is provided. **Modifying this attribute will force creation of a new resource.**
        pub virtual_network_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelRouteArgs,
    ) -> TunnelRouteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let comment_binding = args.comment.get_output(context);
        let network_binding = args.network.get_output(context);
        let tunnel_id_binding = args.tunnel_id.get_output(context);
        let virtual_network_id_binding = args.virtual_network_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/tunnelRoute:TunnelRoute".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: comment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tunnelId".into(),
                    value: tunnel_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkId".into(),
                    value: virtual_network_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TunnelRouteResult {
            account_id: o.get_field("accountId"),
            comment: o.get_field("comment"),
            network: o.get_field("network"),
            tunnel_id: o.get_field("tunnelId"),
            virtual_network_id: o.get_field("virtualNetworkId"),
        }
    }
}
