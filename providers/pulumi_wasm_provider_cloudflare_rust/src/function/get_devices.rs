//! Use this data source to lookup [Devices](https://api.cloudflare.com/#devices-list-devices).
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_devices::invoke(
//!         GetDevicesArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetDevicesArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetDevicesResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub devices: pulumi_wasm_rust::Output<Vec<crate::types::GetDevicesDevice>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetDevicesArgs
) -> GetDevicesResult {

    let result = crate::bindings::pulumi::cloudflare::get_devices::invoke(
        &crate::bindings::pulumi::cloudflare::get_devices::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetDevicesResult {
        account_id: crate::into_domain(result.account_id),
        devices: crate::into_domain(result.devices),
        id: crate::into_domain(result.id),
    }
}
