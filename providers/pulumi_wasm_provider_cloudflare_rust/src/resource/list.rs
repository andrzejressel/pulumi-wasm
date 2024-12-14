//! ## Example Usage
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/list:List example <account_id>/<list_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// An optional description of the list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items in the list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub items: pulumi_wasm_rust::Output<Option<Vec<crate::types::ListItem>>>,
    /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
    #[builder(into)]
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The name of the list.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct ListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// An optional description of the list.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items in the list.
    pub items: pulumi_wasm_rust::Output<Option<Vec<crate::types::ListItem>>>,
    /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The name of the list.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ListArgs) -> ListResult {

    let result = crate::bindings::pulumi::cloudflare::list::invoke(name, &crate::bindings::pulumi::cloudflare::list::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        items: &args.items.get_inner(),
        kind: &args.kind.get_inner(),
        name: &args.name.get_inner(),
    });

    ListResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        items: crate::into_domain(result.items),
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
    }
}
