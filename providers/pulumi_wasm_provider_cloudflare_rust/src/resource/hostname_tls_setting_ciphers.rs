//! Provides a Cloudflare per-hostname TLS setting resource, specifically for ciphers suites. Used to set ciphers suites for hostnames under the specified zone.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = hostname_tls_setting_ciphers::create(
//!         "example",
//!         HostnameTlsSettingCiphersArgs::builder()
//!             .hostname("sub.example.com")
//!             .values(vec!["ECDHE-RSA-AES128-GCM-SHA256",])
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/hostnameTlsSettingCiphers:HostnameTlsSettingCiphers example <zone_id>/<hostname>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct HostnameTlsSettingCiphersArgs {
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Ports to use within the IP rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
    /// Ciphers suites value.
    #[builder(into)]
    pub values: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct HostnameTlsSettingCiphersResult {
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Ports to use within the IP rule.
    pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
    pub updated_at: pulumi_wasm_rust::Output<String>,
    /// Ciphers suites value.
    pub values: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: HostnameTlsSettingCiphersArgs) -> HostnameTlsSettingCiphersResult {

    let result = crate::bindings::pulumi::cloudflare::hostname_tls_setting_ciphers::invoke(name, &crate::bindings::pulumi::cloudflare::hostname_tls_setting_ciphers::Args {
        hostname: &args.hostname.get_inner(),
        ports: &args.ports.get_inner(),
        values: &args.values.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    HostnameTlsSettingCiphersResult {
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        ports: crate::into_domain(result.ports),
        updated_at: crate::into_domain(result.updated_at),
        values: crate::into_domain(result.values),
        zone_id: crate::into_domain(result.zone_id),
    }
}

