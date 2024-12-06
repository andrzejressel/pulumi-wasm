//! Use this data source to lookup a list of [Device Posture Rule](https://developers.cloudflare.com/cloudflare-one/identity/devices)
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_device_posture_rules::invoke(
//!         GetDevicePostureRulesArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("check for /dev/random")
//!             .type_("file")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetDevicePostureRulesArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the Device Posture Rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetDevicePostureRulesResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Name of the Device Posture Rule.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// A list of matching Device Posture Rules.
    pub rules: pulumi_wasm_rust::Output<Vec<crate::types::GetDevicePostureRulesRule>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetDevicePostureRulesArgs
) -> GetDevicePostureRulesResult {

    let result = crate::bindings::pulumi::cloudflare::get_device_posture_rules::invoke(
        &crate::bindings::pulumi::cloudflare::get_device_posture_rules::Args {
                account_id: &args.account_id.get_inner(),
                name: &args.name.get_inner(),
                type_: &args.type_.get_inner(),
        }
    );

    GetDevicePostureRulesResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        rules: crate::into_domain(result.rules),
        type_: crate::into_domain(result.type_),
    }
}
