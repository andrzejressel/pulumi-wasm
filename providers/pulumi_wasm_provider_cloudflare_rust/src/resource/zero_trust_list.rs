//! Provides a Cloudflare Teams List resource. Teams lists are
//! referenced when creating secure web gateway policies or device
//! posture rules.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zero_trust_list::create(
//!         "example",
//!         ZeroTrustListArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .description("Serial numbers for all corporate devices.")
//!             .items(vec!["8GE8721REF", "5RE8543EGG", "1YE2880LNP",])
//!             .name("Corporate devices")
//!             .type_("SERIAL")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustList:ZeroTrustList example <account_id>/<teams_list_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The description of the teams list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items of the teams list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The items of the teams list that has explicit description.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub items_with_descriptions: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustListItemsWithDescription>>>,
    /// Name of the teams list.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The teams list type. Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The description of the teams list.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items of the teams list.
    pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The items of the teams list that has explicit description.
    pub items_with_descriptions: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustListItemsWithDescription>>>,
    /// Name of the teams list.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The teams list type. Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustListArgs) -> ZeroTrustListResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_list::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_list::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        items: &args.items.get_inner(),
        items_with_descriptions: &args.items_with_descriptions.get_inner(),
        name: &args.name.get_inner(),
        type_: &args.type_.get_inner(),
    });

    ZeroTrustListResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        items: crate::into_domain(result.items),
        items_with_descriptions: crate::into_domain(result.items_with_descriptions),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
