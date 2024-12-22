//! Provides a Cloudflare Device Managed Network resource. Device managed networks allow for building location-aware device settings policies.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let managedNetworks = device_managed_networks::create(
//!         "managedNetworks",
//!         DeviceManagedNetworksArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .config(
//!                 DeviceManagedNetworksConfig::builder()
//!                     .sha256(
//!                         "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c",
//!                     )
//!                     .tlsSockaddr("foobar:1234")
//!                     .build_struct(),
//!             )
//!             .name("managed-network-1")
//!             .type_("tls")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/deviceManagedNetworks:DeviceManagedNetworks example <account_id>/<device_managed_networks_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct DeviceManagedNetworksArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration containing information for the WARP client to detect the managed network.
    #[builder(into)]
    pub config: pulumi_wasm_rust::Output<crate::types::DeviceManagedNetworksConfig>,
    /// The name of the Device Managed Network. Must be unique.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of Device Managed Network. Available values: `tls`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DeviceManagedNetworksResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration containing information for the WARP client to detect the managed network.
    pub config: pulumi_wasm_rust::Output<crate::types::DeviceManagedNetworksConfig>,
    /// The name of the Device Managed Network. Must be unique.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of Device Managed Network. Available values: `tls`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: DeviceManagedNetworksArgs
) -> DeviceManagedNetworksResult {

    let result = crate::bindings::pulumi::cloudflare::device_managed_networks::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::device_managed_networks::Args {
                account_id: &args.account_id.get_inner(),
                config: &args.config.get_inner(),
                name: &args.name.get_inner(),
                type_: &args.type_.get_inner(),
        }
    );

    DeviceManagedNetworksResult {
        account_id: crate::into_domain(result.account_id),
        config: crate::into_domain(result.config),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
