//! Use this data source to lookup [Lists](https://developers.cloudflare.com/api/operations/lists-get-lists).
//! 
//! ## Example Usage
//! 
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getLists
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetListsArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetListsResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    pub lists: pulumi_wasm_rust::Output<Vec<crate::types::GetListsList>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetListsArgs
) -> GetListsResult {

    let result = crate::bindings::pulumi::cloudflare::get_lists::invoke(
        &crate::bindings::pulumi::cloudflare::get_lists::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetListsResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        lists: crate::into_domain(result.lists),
    }
}
