//! Provides a Cloudflare custom SSL resource.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:CustomSsl
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       customSslOptions:
//!         certificate: '-----INSERT CERTIFICATE-----'
//!         privateKey: '-----INSERT PRIVATE KEY-----'
//!         bundleMethod: ubiquitous
//!         geoRestrictions: us
//!         type: legacy_custom
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/customSsl:CustomSsl example <zone_id>/<certificate_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CustomSslArgs {
    /// The certificate associated parameters. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub custom_ssl_options: pulumi_wasm_rust::Output<Option<crate::types::CustomSslCustomSslOptions>>,
    #[builder(into, default)]
    pub custom_ssl_priorities: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomSslCustomSslPriority>>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CustomSslResult {
    /// The certificate associated parameters. **Modifying this attribute will force creation of a new resource.**
    pub custom_ssl_options: pulumi_wasm_rust::Output<Option<crate::types::CustomSslCustomSslOptions>>,
    pub custom_ssl_priorities: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomSslCustomSslPriority>>>,
    pub expires_on: pulumi_wasm_rust::Output<String>,
    pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
    pub issuer: pulumi_wasm_rust::Output<String>,
    pub modified_on: pulumi_wasm_rust::Output<String>,
    pub priority: pulumi_wasm_rust::Output<i32>,
    pub signature: pulumi_wasm_rust::Output<String>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: CustomSslArgs
) -> CustomSslResult {

    let result = crate::bindings::pulumi::cloudflare::custom_ssl::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::custom_ssl::Args {
                custom_ssl_options: &args.custom_ssl_options.get_inner(),
                custom_ssl_priorities: &args.custom_ssl_priorities.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    CustomSslResult {
        custom_ssl_options: crate::into_domain(result.custom_ssl_options),
        custom_ssl_priorities: crate::into_domain(result.custom_ssl_priorities),
        expires_on: crate::into_domain(result.expires_on),
        hosts: crate::into_domain(result.hosts),
        issuer: crate::into_domain(result.issuer),
        modified_on: crate::into_domain(result.modified_on),
        priority: crate::into_domain(result.priority),
        signature: crate::into_domain(result.signature),
        status: crate::into_domain(result.status),
        uploaded_on: crate::into_domain(result.uploaded_on),
        zone_id: crate::into_domain(result.zone_id),
    }
}
