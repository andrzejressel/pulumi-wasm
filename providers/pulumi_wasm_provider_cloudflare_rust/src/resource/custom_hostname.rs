//! Provides a Cloudflare custom hostname (also known as SSL for SaaS) resource.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = custom_hostname::create(
//!         "example",
//!         CustomHostnameArgs::builder()
//!             .hostname("hostname.example.com")
//!             .ssls(vec![CustomHostnameSsl::builder().method("txt").build_struct(),])
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/customHostname:CustomHostname example 1d5fdc9e88c8a8c4518b068cd94331fe/0d89c70d-ad9f-4843-b99f-6cc0252067e9
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct CustomHostnameArgs {
    /// Custom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_metadata: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The custom origin server used for certificates.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_origin_server: pulumi_wasm_rust::Output<Option<String>>,
    /// The [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_origin_sni: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// SSL properties used when creating the custom hostname.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ssls: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomHostnameSsl>>>,
    /// Whether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub wait_for_ssl_pending_validation: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CustomHostnameResult {
    /// Custom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly.
    pub custom_metadata: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The custom origin server used for certificates.
    pub custom_origin_server: pulumi_wasm_rust::Output<Option<String>>,
    /// The [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates.
    pub custom_origin_sni: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub ownership_verification: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub ownership_verification_http: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// SSL properties used when creating the custom hostname.
    pub ssls: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomHostnameSsl>>>,
    /// Status of the certificate.
    pub status: pulumi_wasm_rust::Output<String>,
    /// Whether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`.
    pub wait_for_ssl_pending_validation: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: CustomHostnameArgs) -> CustomHostnameResult {

    let result = crate::bindings::pulumi::cloudflare::custom_hostname::invoke(name, &crate::bindings::pulumi::cloudflare::custom_hostname::Args {
        custom_metadata: &args.custom_metadata.get_inner(),
        custom_origin_server: &args.custom_origin_server.get_inner(),
        custom_origin_sni: &args.custom_origin_sni.get_inner(),
        hostname: &args.hostname.get_inner(),
        ssls: &args.ssls.get_inner(),
        wait_for_ssl_pending_validation: &args.wait_for_ssl_pending_validation.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    CustomHostnameResult {
        custom_metadata: crate::into_domain(result.custom_metadata),
        custom_origin_server: crate::into_domain(result.custom_origin_server),
        custom_origin_sni: crate::into_domain(result.custom_origin_sni),
        hostname: crate::into_domain(result.hostname),
        ownership_verification: crate::into_domain(result.ownership_verification),
        ownership_verification_http: crate::into_domain(result.ownership_verification_http),
        ssls: crate::into_domain(result.ssls),
        status: crate::into_domain(result.status),
        wait_for_ssl_pending_validation: crate::into_domain(result.wait_for_ssl_pending_validation),
        zone_id: crate::into_domain(result.zone_id),
    }
}
