//! Use this data source to look up [zone](https://api.cloudflare.com/#zone-properties)
//! info. This is the singular alternative to `cloudflare.getZones`.
//! 
//! > **Note** Cloudflare zone names **are not unique**. It is possible for multiple
//! accounts to have the same zone created but in different states. If you are
//! using this setup, it is advised to use the `account_id` attribute on this
//! resource or swap to `cloudflare.getZones` to further filter the results.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_zone::invoke(
//!         GetZoneArgs::builder().name("example.com").build_struct(),
//!     );
//!     let exampleRecord = record::create(
//!         "exampleRecord",
//!         RecordArgs::builder()
//!             .content("203.0.113.1")
//!             .name("www")
//!             .proxied(true)
//!             .type_("A")
//!             .zone_id("${example.id}")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetZoneArgs {
    /// The account identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the zone. Must provide only one of `zone_id`, `name`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetZoneResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The name of the zone. Must provide only one of `zone_id`, `name`.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Cloudflare assigned name servers. This is only populated for zones that use Cloudflare DNS.
    pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// Whether the zone is paused on Cloudflare.
    pub paused: pulumi_wasm_rust::Output<bool>,
    /// The name of the plan associated with the zone.
    pub plan: pulumi_wasm_rust::Output<String>,
    /// Status of the zone.
    pub status: pulumi_wasm_rust::Output<String>,
    /// List of Vanity Nameservers (if set).
    pub vanity_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetZoneArgs
) -> GetZoneResult {

    let result = crate::bindings::pulumi::cloudflare::get_zone::invoke(
        &crate::bindings::pulumi::cloudflare::get_zone::Args {
                account_id: &args.account_id.get_inner(),
                name: &args.name.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetZoneResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        name_servers: crate::into_domain(result.name_servers),
        paused: crate::into_domain(result.paused),
        plan: crate::into_domain(result.plan),
        status: crate::into_domain(result.status),
        vanity_name_servers: crate::into_domain(result.vanity_name_servers),
        zone_id: crate::into_domain(result.zone_id),
    }
}
