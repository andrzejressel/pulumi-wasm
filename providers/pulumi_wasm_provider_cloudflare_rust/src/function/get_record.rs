//! Use this data source to lookup a single [DNS Record](https://api.cloudflare.com/#dns-records-for-a-zone-properties).
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_record::invoke(
//!         GetRecordArgs::builder()
//!             .hostname("example.com")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRecordArgs {
    /// Content to filter record results on.
    #[builder(into, default)]
    pub content: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname to filter DNS record results on.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// DNS priority to filter record results on.
    #[builder(into, default)]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// DNS record type to filter record results on. Defaults to `A`.
    #[builder(into, default)]
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetRecordResult {
    /// Content to filter record results on.
    pub content: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname to filter DNS record results on.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// DNS priority to filter record results on.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Proxiable status of the found DNS record.
    pub proxiable: pulumi_wasm_rust::Output<bool>,
    /// Proxied status of the found DNS record.
    pub proxied: pulumi_wasm_rust::Output<bool>,
    /// TTL of the found DNS record.
    pub ttl: pulumi_wasm_rust::Output<i32>,
    /// DNS record type to filter record results on. Defaults to `A`.
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    /// Value of the found DNS record.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetRecordArgs
) -> GetRecordResult {

    let result = crate::bindings::pulumi::cloudflare::get_record::invoke(
        &crate::bindings::pulumi::cloudflare::get_record::Args {
                content: &args.content.get_inner(),
                hostname: &args.hostname.get_inner(),
                priority: &args.priority.get_inner(),
                type_: &args.type_.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetRecordResult {
        content: crate::into_domain(result.content),
        hostname: crate::into_domain(result.hostname),
        id: crate::into_domain(result.id),
        priority: crate::into_domain(result.priority),
        proxiable: crate::into_domain(result.proxiable),
        proxied: crate::into_domain(result.proxied),
        ttl: crate::into_domain(result.ttl),
        type_: crate::into_domain(result.type_),
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
