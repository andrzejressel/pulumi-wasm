//! Provides a Cloudflare resource to create and modify zone DNSSEC settings.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zone::create(
//!         "example",
//!         ZoneArgs::builder().zone("example.com").build_struct(),
//!     );
//!     let exampleZoneDnssec = zone_dnssec::create(
//!         "exampleZoneDnssec",
//!         ZoneDnssecArgs::builder().zone_id("${example.id}").build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zoneDnssec:ZoneDnssec example <zone_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneDnssecArgs {
    /// Zone DNSSEC updated time.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub modified_on: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneDnssecResult {
    /// Zone DNSSEC algorithm.
    pub algorithm: pulumi_wasm_rust::Output<String>,
    /// Zone DNSSEC digest.
    pub digest: pulumi_wasm_rust::Output<String>,
    /// Digest algorithm use for Zone DNSSEC.
    pub digest_algorithm: pulumi_wasm_rust::Output<String>,
    /// Digest Type for Zone DNSSEC.
    pub digest_type: pulumi_wasm_rust::Output<String>,
    /// DS for the Zone DNSSEC.
    pub ds: pulumi_wasm_rust::Output<String>,
    /// Zone DNSSEC flags.
    pub flags: pulumi_wasm_rust::Output<i32>,
    /// Key Tag for the Zone DNSSEC.
    pub key_tag: pulumi_wasm_rust::Output<i32>,
    /// Key type used for Zone DNSSEC.
    pub key_type: pulumi_wasm_rust::Output<String>,
    /// Zone DNSSEC updated time.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// Public Key for the Zone DNSSEC.
    pub public_key: pulumi_wasm_rust::Output<String>,
    /// The status of the Zone DNSSEC.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneDnssecArgs) -> ZoneDnssecResult {

    let result = crate::bindings::pulumi::cloudflare::zone_dnssec::invoke(name, &crate::bindings::pulumi::cloudflare::zone_dnssec::Args {
        modified_on: &args.modified_on.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZoneDnssecResult {
        algorithm: crate::into_domain(result.algorithm),
        digest: crate::into_domain(result.digest),
        digest_algorithm: crate::into_domain(result.digest_algorithm),
        digest_type: crate::into_domain(result.digest_type),
        ds: crate::into_domain(result.ds),
        flags: crate::into_domain(result.flags),
        key_tag: crate::into_domain(result.key_tag),
        key_type: crate::into_domain(result.key_type),
        modified_on: crate::into_domain(result.modified_on),
        public_key: crate::into_domain(result.public_key),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
