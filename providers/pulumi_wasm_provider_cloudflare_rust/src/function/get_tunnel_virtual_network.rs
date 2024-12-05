//! Use this datasource to lookup a tunnel virtual network in an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_tunnel_virtual_network::invoke(
//!         GetTunnelVirtualNetworkArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("example")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetTunnelVirtualNetworkArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The Virtual Network Name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetTunnelVirtualNetworkResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The Virtual Network Comment.
    pub comment: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If true, only include deleted virtual networks. If false, exclude deleted virtual networks. If empty, all virtual networks will be included.
    pub is_default: pulumi_wasm_rust::Output<bool>,
    /// The Virtual Network Name.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetTunnelVirtualNetworkArgs
) -> GetTunnelVirtualNetworkResult {

    let result = crate::bindings::pulumi::cloudflare::get_tunnel_virtual_network::invoke(
        &crate::bindings::pulumi::cloudflare::get_tunnel_virtual_network::Args {
                account_id: &args.account_id.get_inner(),
                name: &args.name.get_inner(),
        }
    );

    GetTunnelVirtualNetworkResult {
        account_id: crate::into_domain(result.account_id),
        comment: crate::into_domain(result.comment),
        id: crate::into_domain(result.id),
        is_default: crate::into_domain(result.is_default),
        name: crate::into_domain(result.name),
    }
}
