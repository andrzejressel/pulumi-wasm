//! Provides a resource, that manages Cloudflare tunnel virtual networks
//! for Zero Trust. Tunnel virtual networks are used for segregation of
//! Tunnel IP Routes via Virtualized Networks to handle overlapping
//! private IPs in your origins.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = tunnel_virtual_network::create(
//!         "example",
//!         TunnelVirtualNetworkArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .comment("New tunnel virtual network for documentation")
//!             .name("vnet-for-documentation")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/tunnelVirtualNetwork:TunnelVirtualNetwork example <account_id>/<vnet_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TunnelVirtualNetworkArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Description of the tunnel virtual network.
    #[builder(into, default)]
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
    #[builder(into, default)]
    pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
    /// A user-friendly name chosen when the virtual network is created.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct TunnelVirtualNetworkResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Description of the tunnel virtual network.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
    pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
    /// A user-friendly name chosen when the virtual network is created.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: TunnelVirtualNetworkArgs
) -> TunnelVirtualNetworkResult {

    let result = crate::bindings::pulumi::cloudflare::tunnel_virtual_network::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::tunnel_virtual_network::Args {
                account_id: &args.account_id.get_inner(),
                comment: &args.comment.get_inner(),
                is_default_network: &args.is_default_network.get_inner(),
                name: &args.name.get_inner(),
        }
    );

    TunnelVirtualNetworkResult {
        account_id: crate::into_domain(result.account_id),
        comment: crate::into_domain(result.comment),
        is_default_network: crate::into_domain(result.is_default_network),
        name: crate::into_domain(result.name),
    }
}
