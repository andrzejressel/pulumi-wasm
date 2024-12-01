//! Provides a Cloudflare Access Mutual TLS Certificate Settings resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = access_mutual_tls_hostname_settings::create(
//!         "example",
//!         AccessMutualTlsHostnameSettingsArgs::builder()
//!             .settings(
//!                 vec![
//!                     AccessMutualTlsHostnameSettingsSetting::builder().chinaNetwork(false)
//!                     .clientCertificateForwarding(true).hostname("example.com")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! Account level mTLS hostname settings import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings example account/<account_id>
//! ```
//! 
//! Zone level mTLS hostname settings import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings example zone/<zone_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccessMutualTlsHostnameSettingsArgs {
    /// The account identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub settings: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>>,
    /// The zone identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessMutualTlsHostnameSettingsResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub settings: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessMutualTlsHostnameSettingsArgs) -> AccessMutualTlsHostnameSettingsResult {

    let result = crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::invoke(name, &crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::Args {
        account_id: &args.account_id.get_inner(),
        settings: &args.settings.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AccessMutualTlsHostnameSettingsResult {
        account_id: crate::into_domain(result.account_id),
        settings: crate::into_domain(result.settings),
        zone_id: crate::into_domain(result.zone_id),
    }
}
