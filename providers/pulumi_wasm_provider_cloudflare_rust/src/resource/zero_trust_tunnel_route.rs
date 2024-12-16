//! Provides a resource, that manages Cloudflare tunnel routes for Zero
//! Trust. Tunnel routes are used to direct IP traffic through
//! Cloudflare Tunnels.
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustTunnelRoute:ZeroTrustTunnelRoute example <account_id>/<network_cidr>/<virtual_network_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustTunnelRouteArgs {
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

pub struct ZeroTrustTunnelRouteResult {
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
pub fn create(name: &str, args: ZeroTrustTunnelRouteArgs) -> ZeroTrustTunnelRouteResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_tunnel_route::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_tunnel_route::Args {
        account_id: &args.account_id.get_inner(),
        comment: &args.comment.get_inner(),
        network: &args.network.get_inner(),
        tunnel_id: &args.tunnel_id.get_inner(),
        virtual_network_id: &args.virtual_network_id.get_inner(),
    });

    ZeroTrustTunnelRouteResult {
        account_id: crate::into_domain(result.account_id),
        comment: crate::into_domain(result.comment),
        network: crate::into_domain(result.network),
        tunnel_id: crate::into_domain(result.tunnel_id),
        virtual_network_id: crate::into_domain(result.virtual_network_id),
    }
}
