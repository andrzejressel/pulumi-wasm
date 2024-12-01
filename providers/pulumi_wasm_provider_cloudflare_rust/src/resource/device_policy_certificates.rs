//! Provides a Cloudflare device policy certificates resource. Device
//! policy certificate resources enable client device certificate
//! generation.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = device_policy_certificates::create(
//!         "example",
//!         DevicePolicyCertificatesArgs::builder()
//!             .enabled(true)
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/devicePolicyCertificates:DevicePolicyCertificates example <zone_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct DevicePolicyCertificatesArgs {
    /// `true` if certificate generation is enabled.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct DevicePolicyCertificatesResult {
    /// `true` if certificate generation is enabled.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DevicePolicyCertificatesArgs) -> DevicePolicyCertificatesResult {

    let result = crate::bindings::pulumi::cloudflare::device_policy_certificates::invoke(name, &crate::bindings::pulumi::cloudflare::device_policy_certificates::Args {
        enabled: &args.enabled.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    DevicePolicyCertificatesResult {
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
