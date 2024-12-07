//! Provides a Cloudflare page rule resource.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let foobar = page_rule::create(
//!         "foobar",
//!         PageRuleArgs::builder()
//!             .actions(
//!                 PageRuleActions::builder()
//!                     .emailObfuscation("on")
//!                     .minifies(
//!                         vec![
//!                             PageRuleActionsMinify::builder().css("on").html("off")
//!                             .js("on").build_struct(),
//!                         ],
//!                     )
//!                     .ssl("flexible")
//!                     .build_struct(),
//!             )
//!             .priority(1)
//!             .target("sub.${cloudflareZone}/page")
//!             .zone_id("${cloudflareZoneId}")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! Page rules can be imported using a composite ID formed of zone ID and page rule ID, e.g.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/pageRule:PageRule default d41d8cd98f00b204e9800998ecf8427e/ch8374ftwdghsif43
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleArgs {
    /// The actions taken by the page rule, options given below.
    #[builder(into)]
    pub actions: pulumi_wasm_rust::Output<crate::types::PageRuleActions>,
    /// The priority of the page rule among others for this target, the higher the number the higher the priority as per [API documentation](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the page rule is active or disabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub status: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL pattern to target with the page rule.
    #[builder(into)]
    pub target: pulumi_wasm_rust::Output<String>,
    /// The DNS zone ID to which the page rule should be added.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct PageRuleResult {
    /// The actions taken by the page rule, options given below.
    pub actions: pulumi_wasm_rust::Output<crate::types::PageRuleActions>,
    /// The priority of the page rule among others for this target, the higher the number the higher the priority as per [API documentation](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule).
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the page rule is active or disabled.
    pub status: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL pattern to target with the page rule.
    pub target: pulumi_wasm_rust::Output<String>,
    /// The DNS zone ID to which the page rule should be added.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PageRuleArgs) -> PageRuleResult {

    let result = crate::bindings::pulumi::cloudflare::page_rule::invoke(name, &crate::bindings::pulumi::cloudflare::page_rule::Args {
        actions: &args.actions.get_inner(),
        priority: &args.priority.get_inner(),
        status: &args.status.get_inner(),
        target: &args.target.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    PageRuleResult {
        actions: crate::into_domain(result.actions),
        priority: crate::into_domain(result.priority),
        status: crate::into_domain(result.status),
        target: crate::into_domain(result.target),
        zone_id: crate::into_domain(result.zone_id),
    }
}
