//! Provides a Cloudflare record resource.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let _sipTls = record::create(
//!         "_sipTls",
//!         RecordArgs::builder()
//!             .data(
//!                 RecordData::builder()
//!                     .name("terraform-srv")
//!                     .port(443)
//!                     .priority(0)
//!                     .proto("_tls")
//!                     .service("_sip")
//!                     .target("example.com")
//!                     .weight(0)
//!                     .build_struct(),
//!             )
//!             .name("_sip._tls")
//!             .type_("SRV")
//!             .zone_id("${cloudflareZoneId}")
//!             .build_struct(),
//!     );
//!     let example = record::create(
//!         "example",
//!         RecordArgs::builder()
//!             .content("192.0.2.1")
//!             .name("terraform")
//!             .ttl(3600)
//!             .type_("A")
//!             .zone_id("${cloudflareZoneId}")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/record:Record example <zone_id>/<record_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RecordArgs {
    #[builder(into, default)]
    pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
    /// Comments or notes about the DNS record. This field has no effect on DNS responses.
    #[builder(into, default)]
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// The content of the record. Must provide only one of `data`, `content`, `value`.
    #[builder(into, default)]
    pub content: pulumi_wasm_rust::Output<Option<String>>,
    /// Map of attributes that constitute the record value. Must provide only one of `data`, `content`, `value`.
    #[builder(into, default)]
    pub data: pulumi_wasm_rust::Output<Option<crate::types::RecordData>>,
    /// The name of the record. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the record.
    #[builder(into, default)]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the record gets Cloudflare's origin protection.
    #[builder(into, default)]
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom tags for the DNS record.
    #[builder(into, default)]
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The TTL of the record.
    #[builder(into, default)]
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`, `SVCB`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The value of the record. Must provide only one of `data`, `content`, `value`.
    #[builder(into, default)]
    pub value: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RecordResult {
    pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
    /// Comments or notes about the DNS record. This field has no effect on DNS responses.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// The content of the record. Must provide only one of `data`, `content`, `value`.
    pub content: pulumi_wasm_rust::Output<String>,
    /// The RFC3339 timestamp of when the record was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Map of attributes that constitute the record value. Must provide only one of `data`, `content`, `value`.
    pub data: pulumi_wasm_rust::Output<Option<crate::types::RecordData>>,
    /// The FQDN of the record.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// A key-value map of string metadata Cloudflare associates with the record.
    pub metadata: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// The RFC3339 timestamp of when the record was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The name of the record. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the record.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Shows whether this record can be proxied.
    pub proxiable: pulumi_wasm_rust::Output<bool>,
    /// Whether the record gets Cloudflare's origin protection.
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom tags for the DNS record.
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The TTL of the record.
    pub ttl: pulumi_wasm_rust::Output<i32>,
    /// The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`, `SVCB`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The value of the record. Must provide only one of `data`, `content`, `value`.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RecordArgs) -> RecordResult {

    let result = crate::bindings::pulumi::cloudflare::record::invoke(name, &crate::bindings::pulumi::cloudflare::record::Args {
        allow_overwrite: &args.allow_overwrite.get_inner(),
        comment: &args.comment.get_inner(),
        content: &args.content.get_inner(),
        data: &args.data.get_inner(),
        name: &args.name.get_inner(),
        priority: &args.priority.get_inner(),
        proxied: &args.proxied.get_inner(),
        tags: &args.tags.get_inner(),
        ttl: &args.ttl.get_inner(),
        type_: &args.type_.get_inner(),
        value: &args.value.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    RecordResult {
        allow_overwrite: crate::into_domain(result.allow_overwrite),
        comment: crate::into_domain(result.comment),
        content: crate::into_domain(result.content),
        created_on: crate::into_domain(result.created_on),
        data: crate::into_domain(result.data),
        hostname: crate::into_domain(result.hostname),
        metadata: crate::into_domain(result.metadata),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        priority: crate::into_domain(result.priority),
        proxiable: crate::into_domain(result.proxiable),
        proxied: crate::into_domain(result.proxied),
        tags: crate::into_domain(result.tags),
        ttl: crate::into_domain(result.ttl),
        type_: crate::into_domain(result.type_),
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
