//! Provides a Cloudflare Device Posture Rule resource. Device posture rules configure security policies for device posture checks.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   eaxmple:
//!     type: cloudflare:DevicePostureRule
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Corporate devices posture rule
//!       type: os_version
//!       description: Device posture rule for corporate devices.
//!       schedule: 24h
//!       expiration: 24h
//!       matches:
//!         - platform: linux
//!       inputs:
//!         - id: ${corporateDevices.id}
//!           version: 1.0.0
//!           operator: <
//!           osDistroName: ubuntu
//!           osDistroRevision: 1.0.0
//!           osVersionExtra: (a)
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/devicePostureRule:DevicePostureRule example <account_id>/<device_posture_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct DevicePostureRuleArgs {
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
    pub inputs: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleInput>>>,
    /// The conditions that the client must match to run the rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleMatch>>>,
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

pub struct DevicePostureRuleResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    /// Required for all rule types except `warp`, `gateway`, and `tanium`.
    pub inputs: pulumi_wasm_rust::Output<Vec<crate::types::DevicePostureRuleInput>>,
    /// The conditions that the client must match to run the rule.
    pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleMatch>>>,
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
pub fn create(name: &str, args: DevicePostureRuleArgs) -> DevicePostureRuleResult {

    let result = crate::bindings::pulumi::cloudflare::device_posture_rule::invoke(name, &crate::bindings::pulumi::cloudflare::device_posture_rule::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        expiration: &args.expiration.get_inner(),
        inputs: &args.inputs.get_inner(),
        matches: &args.matches.get_inner(),
        name: &args.name.get_inner(),
        schedule: &args.schedule.get_inner(),
        type_: &args.type_.get_inner(),
    });

    DevicePostureRuleResult {
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
