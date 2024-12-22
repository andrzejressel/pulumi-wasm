//! Provides a Cloudflare Device Posture Integration resource. Device
//! posture integrations configure third-party data providers for device
//! posture rules.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zero_trust_device_posture_integration::create(
//!         "example",
//!         ZeroTrustDevicePostureIntegrationArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .configs(
//!                 vec![
//!                     ZeroTrustDevicePostureIntegrationConfig::builder()
//!                     .apiUrl("https://example.com/api")
//!                     .authUrl("https://example.com/connect/token").clientId("client-id")
//!                     .clientSecret("client-secret").build_struct(),
//!                 ],
//!             )
//!             .interval("24h")
//!             .name("Device posture integration")
//!             .type_("workspace_one")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustDevicePostureIntegration:ZeroTrustDevicePostureIntegration example <account_id>/<device_posture_integration_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDevicePostureIntegrationArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The device posture integration's connection authorization parameters.
    #[builder(into, default)]
    pub configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDevicePostureIntegrationConfig>>>,
    #[builder(into, default)]
    pub identifier: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
    #[builder(into, default)]
    pub interval: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the device posture integration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`, `custom_s2s`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustDevicePostureIntegrationResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The device posture integration's connection authorization parameters.
    pub configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDevicePostureIntegrationConfig>>>,
    pub identifier: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
    pub interval: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the device posture integration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`, `custom_s2s`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ZeroTrustDevicePostureIntegrationArgs
) -> ZeroTrustDevicePostureIntegrationResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_device_posture_integration::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zero_trust_device_posture_integration::Args {
                account_id: &args.account_id.get_inner(),
                configs: &args.configs.get_inner(),
                identifier: &args.identifier.get_inner(),
                interval: &args.interval.get_inner(),
                name: &args.name.get_inner(),
                type_: &args.type_.get_inner(),
        }
    );

    ZeroTrustDevicePostureIntegrationResult {
        account_id: crate::into_domain(result.account_id),
        configs: crate::into_domain(result.configs),
        identifier: crate::into_domain(result.identifier),
        interval: crate::into_domain(result.interval),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
