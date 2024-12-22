//! Provides a Cloudflare custom hostname fallback origin resource.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = custom_hostname_fallback_origin::create(
//!         "example",
//!         CustomHostnameFallbackOriginArgs::builder()
//!             .origin("fallback.example.com")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/customHostnameFallbackOrigin:CustomHostnameFallbackOrigin example <zone_id>/<fallback_hostname>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CustomHostnameFallbackOriginArgs {
    /// Hostname you intend to fallback requests to. Origin must be a proxied A/AAAA/CNAME DNS record within Clouldflare.
    #[builder(into)]
    pub origin: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CustomHostnameFallbackOriginResult {
    /// Hostname you intend to fallback requests to. Origin must be a proxied A/AAAA/CNAME DNS record within Clouldflare.
    pub origin: pulumi_wasm_rust::Output<String>,
    /// Status of the fallback origin's activation.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: CustomHostnameFallbackOriginArgs
) -> CustomHostnameFallbackOriginResult {

    let result = crate::bindings::pulumi::cloudflare::custom_hostname_fallback_origin::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::custom_hostname_fallback_origin::Args {
                origin: &args.origin.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    CustomHostnameFallbackOriginResult {
        origin: crate::into_domain(result.origin),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
