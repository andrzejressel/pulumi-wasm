//! Use this datasource to lookup a tunnel in an account.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_tunnel::invoke(
//!         GetTunnelArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("my-tunnel")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetTunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If true, only include deleted tunnels. If false, exclude deleted tunnels. If empty, all tunnels will be included. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub is_deleted: pulumi_wasm_rust::Output<Option<bool>>,
    /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetTunnelResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// ID of the tunnel.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If true, only include deleted tunnels. If false, exclude deleted tunnels. If empty, all tunnels will be included. **Modifying this attribute will force creation of a new resource.**
    pub is_deleted: pulumi_wasm_rust::Output<Option<bool>>,
    /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// Whether the tunnel can be configured remotely from the Zero Trust dashboard.
    pub remote_config: pulumi_wasm_rust::Output<bool>,
    /// The status of the tunnel. Available values: `inactive`, `degraded`, `healthy`, `down`.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The type of the tunnel. Available values: `cfd_tunnel`, `warp_connector`.
    pub tunnel_type: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetTunnelArgs
) -> GetTunnelResult {

    let result = crate::bindings::pulumi::cloudflare::get_tunnel::invoke(
        &crate::bindings::pulumi::cloudflare::get_tunnel::Args {
                account_id: &args.account_id.get_inner(),
                is_deleted: &args.is_deleted.get_inner(),
                name: &args.name.get_inner(),
        }
    );

    GetTunnelResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        is_deleted: crate::into_domain(result.is_deleted),
        name: crate::into_domain(result.name),
        remote_config: crate::into_domain(result.remote_config),
        status: crate::into_domain(result.status),
        tunnel_type: crate::into_domain(result.tunnel_type),
    }
}
