//! Provides a Cloudflare per-hostname TLS setting resource. Used to set TLS settings for hostnames under the specified zone.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = hostname_tls_setting::create(
//!         "example",
//!         HostnameTlsSettingArgs::builder()
//!             .hostname("sub.example.com")
//!             .setting("min_tls_version")
//!             .value("1.2")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/hostnameTlsSetting:HostnameTlsSetting example <zone_id>/<hostname>/<setting_name>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct HostnameTlsSettingArgs {
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// TLS setting name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub setting: pulumi_wasm_rust::Output<String>,
    /// TLS setting value.
    #[builder(into)]
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct HostnameTlsSettingResult {
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// TLS setting name. **Modifying this attribute will force creation of a new resource.**
    pub setting: pulumi_wasm_rust::Output<String>,
    pub updated_at: pulumi_wasm_rust::Output<String>,
    /// TLS setting value.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: HostnameTlsSettingArgs) -> HostnameTlsSettingResult {

    let result = crate::bindings::pulumi::cloudflare::hostname_tls_setting::invoke(name, &crate::bindings::pulumi::cloudflare::hostname_tls_setting::Args {
        hostname: &args.hostname.get_inner(),
        setting: &args.setting.get_inner(),
        value: &args.value.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    HostnameTlsSettingResult {
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        setting: crate::into_domain(result.setting),
        updated_at: crate::into_domain(result.updated_at),
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
