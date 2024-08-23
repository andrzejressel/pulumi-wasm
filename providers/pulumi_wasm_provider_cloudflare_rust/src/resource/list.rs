pub struct ListArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub items: pulumi_wasm_rust::Output<Option<Vec<crate::types::ListItem>>>,
    pub kind: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct ListResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub items: pulumi_wasm_rust::Output<Option<Vec<crate::types::ListItem>>>,
    pub kind: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ListArgs) -> ListResult {
    let result = crate::bindings::pulumi::cloudflare::list::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::list::Args {
            account_id: args.account_id.get_inner(),
            description: args.description.get_inner(),
            items: args.items.get_inner(),
            kind: args.kind.get_inner(),
            name: args.name.get_inner(),
        },
    );

    ListResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        items: crate::into_domain(result.items),
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
    }
}
