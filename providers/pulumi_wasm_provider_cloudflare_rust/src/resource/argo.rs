//! Cloudflare Argo controls the routing to your origin and tiered
//! caching options to speed up your website browsing experience.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = argo::create(
//!         "example",
//!         ArgoArgs::builder()
//!             .smart_routing("on")
//!             .tiered_caching("on")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/argo:Argo example <zone_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ArgoArgs {
    /// Whether smart routing is enabled. Available values: `on`, `off`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether tiered caching is enabled. Available values: `on`, `off`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ArgoResult {
    /// Whether smart routing is enabled. Available values: `on`, `off`.
    pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether tiered caching is enabled. Available values: `on`, `off`.
    pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ArgoArgs) -> ArgoResult {

    let result = crate::bindings::pulumi::cloudflare::argo::invoke(name, &crate::bindings::pulumi::cloudflare::argo::Args {
        smart_routing: &args.smart_routing.get_inner(),
        tiered_caching: &args.tiered_caching.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ArgoResult {
        smart_routing: crate::into_domain(result.smart_routing),
        tiered_caching: crate::into_domain(result.tiered_caching),
        zone_id: crate::into_domain(result.zone_id),
    }
}
