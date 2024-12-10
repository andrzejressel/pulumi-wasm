//! Provides a resource to manage URL Normalization Settings.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = url_normalization_settings::create(
//!         "example",
//!         UrlNormalizationSettingsArgs::builder()
//!             .scope("incoming")
//!             .type_("cloudflare")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct UrlNormalizationSettingsArgs {
    /// The scope of the URL normalization.
    #[builder(into)]
    pub scope: pulumi_wasm_rust::Output<String>,
    /// The type of URL normalization performed by Cloudflare.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct UrlNormalizationSettingsResult {
    /// The scope of the URL normalization.
    pub scope: pulumi_wasm_rust::Output<String>,
    /// The type of URL normalization performed by Cloudflare.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: UrlNormalizationSettingsArgs) -> UrlNormalizationSettingsResult {

    let result = crate::bindings::pulumi::cloudflare::url_normalization_settings::invoke(name, &crate::bindings::pulumi::cloudflare::url_normalization_settings::Args {
        scope: &args.scope.get_inner(),
        type_: &args.type_.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    UrlNormalizationSettingsResult {
        scope: crate::into_domain(result.scope),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}

