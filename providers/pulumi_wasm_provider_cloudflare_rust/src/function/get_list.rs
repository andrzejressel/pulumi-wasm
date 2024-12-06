//! Use this data source to lookup a [List](https://developers.cloudflare.com/api/operations/lists-get-lists).
//! 
//! ## Example Usage
//! 
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getList
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//!         name: list_name
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The list name to target for the resource.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// List description.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// List kind.
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The list name to target for the resource.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Number of items in list.
    pub numitems: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetListArgs
) -> GetListResult {

    let result = crate::bindings::pulumi::cloudflare::get_list::invoke(
        &crate::bindings::pulumi::cloudflare::get_list::Args {
                account_id: &args.account_id.get_inner(),
                name: &args.name.get_inner(),
        }
    );

    GetListResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        id: crate::into_domain(result.id),
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        numitems: crate::into_domain(result.numitems),
    }
}
