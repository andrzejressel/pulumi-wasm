//! Provides a Cloudflare Split Tunnel resource. Split tunnels are used to either
//! include or exclude lists of routes from the WARP client's tunnel.
//! 
//! ## Import
//! 
//! Split Tunnels for default device policies must use "default" as the policy ID.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/splitTunnel:SplitTunnel example <account_id>/<policy_id>/<mode>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct SplitTunnelArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// The settings policy for which to configure this split tunnel policy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The value of the tunnel attributes.
    #[builder(into)]
    pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::SplitTunnelTunnel>>,
}

pub struct SplitTunnelResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// The settings policy for which to configure this split tunnel policy.
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The value of the tunnel attributes.
    pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::SplitTunnelTunnel>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: SplitTunnelArgs) -> SplitTunnelResult {

    let result = crate::bindings::pulumi::cloudflare::split_tunnel::invoke(name, &crate::bindings::pulumi::cloudflare::split_tunnel::Args {
        account_id: &args.account_id.get_inner(),
        mode: &args.mode.get_inner(),
        policy_id: &args.policy_id.get_inner(),
        tunnels: &args.tunnels.get_inner(),
    });

    SplitTunnelResult {
        account_id: crate::into_domain(result.account_id),
        mode: crate::into_domain(result.mode),
        policy_id: crate::into_domain(result.policy_id),
        tunnels: crate::into_domain(result.tunnels),
    }
}

