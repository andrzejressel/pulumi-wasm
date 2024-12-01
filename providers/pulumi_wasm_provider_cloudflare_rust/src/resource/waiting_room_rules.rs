//! Provides a Cloudflare Waiting Room Rules resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = waiting_room_rules::create(
//!         "example",
//!         WaitingRoomRulesArgs::builder()
//!             .rules(
//!                 vec![
//!                     WaitingRoomRulesRule::builder().action("bypass_waiting_room")
//!                     .description("bypass ip list")
//!                     .expression("src.ip in {192.0.2.0 192.0.2.1}").status("enabled")
//!                     .build_struct(), WaitingRoomRulesRule::builder()
//!                     .action("bypass_waiting_room").description("bypass query string")
//!                     .expression("http.request.uri.query contains "bypass = true "")
//!                     .status("enabled").build_struct(),
//!                 ],
//!             )
//!             .waiting_room_id("d41d8cd98f00b204e9800998ecf8427e")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/waitingRoomRules:WaitingRoomRules default <zone_id>/<waiting_room_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WaitingRoomRulesArgs {
    /// List of rules to apply to the ruleset.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomRulesRule>>>,
    /// The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub waiting_room_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WaitingRoomRulesResult {
    /// List of rules to apply to the ruleset.
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomRulesRule>>>,
    /// The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**
    pub waiting_room_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WaitingRoomRulesArgs) -> WaitingRoomRulesResult {

    let result = crate::bindings::pulumi::cloudflare::waiting_room_rules::invoke(name, &crate::bindings::pulumi::cloudflare::waiting_room_rules::Args {
        rules: &args.rules.get_inner(),
        waiting_room_id: &args.waiting_room_id.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    WaitingRoomRulesResult {
        rules: crate::into_domain(result.rules),
        waiting_room_id: crate::into_domain(result.waiting_room_id),
        zone_id: crate::into_domain(result.zone_id),
    }
}
