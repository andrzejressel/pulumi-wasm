//! Provides a resource for managing Email Routing Addresses catch all behaviour.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = email_routing_catch_all::create(
//!         "example",
//!         EmailRoutingCatchAllArgs::builder()
//!             .actions(
//!                 vec![
//!                     EmailRoutingCatchAllAction::builder(). type ("forward")
//!                     .values(vec!["destinationaddress@example.net",]).build_struct(),
//!                 ],
//!             )
//!             .enabled(true)
//!             .matchers(
//!                 vec![
//!                     EmailRoutingCatchAllMatcher::builder(). type ("all").build_struct(),
//!                 ],
//!             )
//!             .name("example catch all")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingCatchAllArgs {
    /// List actions patterns.
    #[builder(into)]
    pub actions: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllAction>>,
    /// Routing rule status.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    #[builder(into)]
    pub matchers: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllMatcher>>,
    /// Routing rule name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingCatchAllResult {
    /// List actions patterns.
    pub actions: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllAction>>,
    /// Routing rule status.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    pub matchers: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllMatcher>>,
    /// Routing rule name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Routing rule identifier.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: EmailRoutingCatchAllArgs) -> EmailRoutingCatchAllResult {

    let result = crate::bindings::pulumi::cloudflare::email_routing_catch_all::invoke(name, &crate::bindings::pulumi::cloudflare::email_routing_catch_all::Args {
        actions: &args.actions.get_inner(),
        enabled: &args.enabled.get_inner(),
        matchers: &args.matchers.get_inner(),
        name: &args.name.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    EmailRoutingCatchAllResult {
        actions: crate::into_domain(result.actions),
        enabled: crate::into_domain(result.enabled),
        matchers: crate::into_domain(result.matchers),
        name: crate::into_domain(result.name),
        tag: crate::into_domain(result.tag),
        zone_id: crate::into_domain(result.zone_id),
    }
}
