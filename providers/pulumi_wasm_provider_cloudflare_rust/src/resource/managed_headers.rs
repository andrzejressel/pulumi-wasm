//! The [Cloudflare Managed Headers](https://developers.cloudflare.com/rules/transform/managed-transforms/)
//! allows you to add or remove some predefined headers to one's
//! requests or origin responses.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = managed_headers::create(
//!         "example",
//!         ManagedHeadersArgs::builder()
//!             .managed_request_headers(
//!                 vec![
//!                     ManagedHeadersManagedRequestHeader::builder().enabled(true)
//!                     .id("add_true_client_ip_headers").build_struct(),
//!                 ],
//!             )
//!             .managed_response_headers(
//!                 vec![
//!                     ManagedHeadersManagedResponseHeader::builder().enabled(true)
//!                     .id("remove_x-powered-by_header").build_struct(),
//!                 ],
//!             )
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ManagedHeadersArgs {
    /// The list of managed request headers.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub managed_request_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
    /// The list of managed response headers.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub managed_response_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ManagedHeadersResult {
    /// The list of managed request headers.
    pub managed_request_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
    /// The list of managed response headers.
    pub managed_response_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ManagedHeadersArgs) -> ManagedHeadersResult {

    let result = crate::bindings::pulumi::cloudflare::managed_headers::invoke(name, &crate::bindings::pulumi::cloudflare::managed_headers::Args {
        managed_request_headers: &args.managed_request_headers.get_inner(),
        managed_response_headers: &args.managed_response_headers.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ManagedHeadersResult {
        managed_request_headers: crate::into_domain(result.managed_request_headers),
        managed_response_headers: crate::into_domain(result.managed_response_headers),
        zone_id: crate::into_domain(result.zone_id),
    }
}
