//! Use this data source to retrieve all Gateway categories for an account.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_gateway_categories::invoke(
//!         GetGatewayCategoriesArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetGatewayCategoriesArgs {
    /// The account ID to fetch Gateway Categories from.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetGatewayCategoriesResult {
    /// The account ID to fetch Gateway Categories from.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of Gateway Categories.
    pub categories: pulumi_wasm_rust::Output<Vec<crate::types::GetGatewayCategoriesCategory>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetGatewayCategoriesArgs
) -> GetGatewayCategoriesResult {

    let result = crate::bindings::pulumi::cloudflare::get_gateway_categories::invoke(
        &crate::bindings::pulumi::cloudflare::get_gateway_categories::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetGatewayCategoriesResult {
        account_id: crate::into_domain(result.account_id),
        categories: crate::into_domain(result.categories),
        id: crate::into_domain(result.id),
    }
}
