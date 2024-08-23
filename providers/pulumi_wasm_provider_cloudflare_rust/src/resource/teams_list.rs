pub struct TeamsListArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct TeamsListResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: TeamsListArgs) -> TeamsListResult {
    let result = crate::bindings::pulumi::cloudflare::teams_list::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::teams_list::Args {
            account_id: args.account_id.get_inner(),
            description: args.description.get_inner(),
            items: args.items.get_inner(),
            name: args.name.get_inner(),
            type_: args.type_.get_inner(),
        },
    );

    TeamsListResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        items: crate::into_domain(result.items),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
