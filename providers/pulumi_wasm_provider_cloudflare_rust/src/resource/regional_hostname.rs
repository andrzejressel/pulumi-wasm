//! Provides a Data Localization Suite Regional Hostname.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = record::create(
//!         "example",
//!         RecordArgs::builder()
//!             .content("192.0.2.1")
//!             .name("example.com")
//!             .ttl(3600)
//!             .type_("A")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let exampleRegionalHostname = regional_hostname::create(
//!         "exampleRegionalHostname",
//!         RegionalHostnameArgs::builder()
//!             .hostname("example.com")
//!             .region_key("eu")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RegionalHostnameArgs {
    /// The hostname to regionalize.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The region key. See [the full region list](https://developers.cloudflare.com/data-localization/regional-services/get-started/).
    #[builder(into)]
    pub region_key: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RegionalHostnameResult {
    /// The RFC3339 timestamp of when the hostname was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// The hostname to regionalize.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The region key. See [the full region list](https://developers.cloudflare.com/data-localization/regional-services/get-started/).
    pub region_key: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RegionalHostnameArgs) -> RegionalHostnameResult {

    let result = crate::bindings::pulumi::cloudflare::regional_hostname::invoke(name, &crate::bindings::pulumi::cloudflare::regional_hostname::Args {
        hostname: &args.hostname.get_inner(),
        region_key: &args.region_key.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    RegionalHostnameResult {
        created_on: crate::into_domain(result.created_on),
        hostname: crate::into_domain(result.hostname),
        region_key: crate::into_domain(result.region_key),
        zone_id: crate::into_domain(result.zone_id),
    }
}
