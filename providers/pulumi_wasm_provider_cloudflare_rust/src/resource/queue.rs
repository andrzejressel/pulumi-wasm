pub struct QueueArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct QueueResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: QueueArgs) -> QueueResult {
    let result = crate::bindings::pulumi::cloudflare::queue::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::queue::Args {
            account_id: args.account_id.get_inner(),
            name: args.name.get_inner(),
        },
    );

    QueueResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
    }
}
