//! Provides a resource which customizes Cloudflare zone settings.
//! 
//! > You **should not** use this resource to manage every zone setting. This
//!   resource is only intended to override those which you do not want the default.
//!   Attempting to manage all settings will result in problems with the resource
//!   applying in a consistent manner.
//! 
//! ## Plan-Dependent Settings
//! 
//! Note that some settings are only available on certain plans. Setting an argument
//! for a feature that is not available on the plan configured for the zone will
//! result in an error:
//! 
//! ```
//! Error: invalid zone setting "\<argument\>" (value: \<value\>) found - cannot be set as it is read only
//! ```
//! 
//! This is true even when setting the argument to its default value. These values
//! should either be omitted or set to `null` for zones with plans that don't
//! support the feature. See the [plan feature matrices](https://www.cloudflare.com/plans/) for details on
//! feature support by plan.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let test = zone_settings_override::create(
//!         "test",
//!         ZoneSettingsOverrideArgs::builder()
//!             .settings(
//!                 ZoneSettingsOverrideSettings::builder()
//!                     .automaticHttpsRewrites("on")
//!                     .brotli("on")
//!                     .challengeTtl(2700)
//!                     .minify(
//!                         ZoneSettingsOverrideSettingsMinify::builder()
//!                             .css("on")
//!                             .html("off")
//!                             .js("off")
//!                             .build_struct(),
//!                     )
//!                     .mirage("on")
//!                     .opportunisticEncryption("on")
//!                     .securityHeader(
//!                         ZoneSettingsOverrideSettingsSecurityHeader::builder()
//!                             .enabled(true)
//!                             .build_struct(),
//!                     )
//!                     .securityLevel("high")
//!                     .waf("on")
//!                     .build_struct(),
//!             )
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZoneSettingsOverrideArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub settings: pulumi_wasm_rust::Output<Option<crate::types::ZoneSettingsOverrideSettings>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneSettingsOverrideResult {
    pub initial_settings: pulumi_wasm_rust::Output<Vec<crate::types::ZoneSettingsOverrideInitialSetting>>,
    pub initial_settings_read_at: pulumi_wasm_rust::Output<String>,
    pub readonly_settings: pulumi_wasm_rust::Output<Vec<String>>,
    pub settings: pulumi_wasm_rust::Output<crate::types::ZoneSettingsOverrideSettings>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
    pub zone_status: pulumi_wasm_rust::Output<String>,
    pub zone_type: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneSettingsOverrideArgs) -> ZoneSettingsOverrideResult {

    let result = crate::bindings::pulumi::cloudflare::zone_settings_override::invoke(name, &crate::bindings::pulumi::cloudflare::zone_settings_override::Args {
        settings: &args.settings.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZoneSettingsOverrideResult {
        initial_settings: crate::into_domain(result.initial_settings),
        initial_settings_read_at: crate::into_domain(result.initial_settings_read_at),
        readonly_settings: crate::into_domain(result.readonly_settings),
        settings: crate::into_domain(result.settings),
        zone_id: crate::into_domain(result.zone_id),
        zone_status: crate::into_domain(result.zone_status),
        zone_type: crate::into_domain(result.zone_type),
    }
}
