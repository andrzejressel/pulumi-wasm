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
pub mod tunnel_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelRouteArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Description of the tunnel route.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 or IPv6 network that should use this tunnel route, in CIDR notation.
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the tunnel that will service the tunnel route.
        #[builder(into)]
        pub tunnel_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual network for which this route is being added; uses the default virtual network of the account if none is provided. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TunnelRouteResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Description of the tunnel route.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 or IPv6 network that should use this tunnel route, in CIDR notation.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the tunnel that will service the tunnel route.
        pub tunnel_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual network for which this route is being added; uses the default virtual network of the account if none is provided. **Modifying this attribute will force creation of a new resource.**
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TunnelRouteArgs) -> TunnelRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let comment_binding = args.comment.get_inner();
        let network_binding = args.network.get_inner();
        let tunnel_id_binding = args.tunnel_id.get_inner();
        let virtual_network_id_binding = args.virtual_network_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/tunnelRoute:TunnelRoute".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "tunnelId".into(),
                    value: &tunnel_id_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "tunnelId".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TunnelRouteResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            tunnel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnelId").unwrap(),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkId").unwrap(),
            ),
        }
    }
}
