//! Provides a Cloudflare Split Tunnel resource. Split tunnels are used to either
//! include or exclude lists of routes from the WARP client's tunnel.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustSplitTunnelArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// The settings policy for which to configure this split tunnel policy.
    #[builder(into, default)]
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The value of the tunnel attributes.
    #[builder(into)]
    pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustSplitTunnelTunnel>>,
}

pub struct ZeroTrustSplitTunnelResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// The settings policy for which to configure this split tunnel policy.
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The value of the tunnel attributes.
    pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustSplitTunnelTunnel>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ZeroTrustSplitTunnelArgs
) -> ZeroTrustSplitTunnelResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_split_tunnel::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zero_trust_split_tunnel::Args {
                account_id: &args.account_id.get_inner(),
                mode: &args.mode.get_inner(),
                policy_id: &args.policy_id.get_inner(),
                tunnels: &args.tunnels.get_inner(),
        }
    );

    ZeroTrustSplitTunnelResult {
        account_id: crate::into_domain(result.account_id),
        mode: crate::into_domain(result.mode),
        policy_id: crate::into_domain(result.policy_id),
        tunnels: crate::into_domain(result.tunnels),
    }
}
