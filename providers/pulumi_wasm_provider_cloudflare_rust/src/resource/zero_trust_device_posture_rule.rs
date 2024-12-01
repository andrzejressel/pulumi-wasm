//! Provides a Cloudflare Device Posture Rule resource. Device posture rules configure security policies for device posture checks.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let eaxmple = zero_trust_device_posture_rule::create(
//!         "eaxmple",
//!         ZeroTrustDevicePostureRuleArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .description("Device posture rule for corporate devices.")
//!             .expiration("24h")
//!             .inputs(
//!                 vec![
//!                     ZeroTrustDevicePostureRuleInput::builder()
//!                     .id("${corporateDevices.id}").operator("<").osDistroName("ubuntu")
//!                     .osDistroRevision("1.0.0").osVersionExtra("(a)").version("1.0.0")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .matches(
//!                 vec![
//!                     ZeroTrustDevicePostureRuleMatch::builder().platform("linux")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .name("Corporate devices posture rule")
//!             .schedule("24h")
//!             .type_("os_version")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustDevicePostureRule:ZeroTrustDevicePostureRule example <account_id>/<device_posture_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDevicePostureRuleArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    /// Required for all rule types except `warp`, `gateway`, and `tanium`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub inputs: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDevicePostureRuleInput>>>,
    /// The conditions that the client must match to run the rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDevicePostureRuleMatch>>>,
    /// Name of the device posture rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub schedule: pulumi_wasm_rust::Output<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustDevicePostureRuleResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    /// Required for all rule types except `warp`, `gateway`, and `tanium`.
    pub inputs: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustDevicePostureRuleInput>>,
    /// The conditions that the client must match to run the rule.
    pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDevicePostureRuleMatch>>>,
    /// Name of the device posture rule.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    pub schedule: pulumi_wasm_rust::Output<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustDevicePostureRuleArgs) -> ZeroTrustDevicePostureRuleResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_device_posture_rule::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_device_posture_rule::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        expiration: &args.expiration.get_inner(),
        inputs: &args.inputs.get_inner(),
        matches: &args.matches.get_inner(),
        name: &args.name.get_inner(),
        schedule: &args.schedule.get_inner(),
        type_: &args.type_.get_inner(),
    });

    ZeroTrustDevicePostureRuleResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        expiration: crate::into_domain(result.expiration),
        inputs: crate::into_domain(result.inputs),
        matches: crate::into_domain(result.matches),
        name: crate::into_domain(result.name),
        schedule: crate::into_domain(result.schedule),
        type_: crate::into_domain(result.type_),
    }
}
