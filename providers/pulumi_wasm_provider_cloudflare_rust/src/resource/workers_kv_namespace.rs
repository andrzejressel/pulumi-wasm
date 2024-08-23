pub struct WorkersKvNamespaceArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub title: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersKvNamespaceResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub title: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WorkersKvNamespaceArgs) -> WorkersKvNamespaceResult {
    let result = crate::bindings::pulumi::cloudflare::workers_kv_namespace::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::workers_kv_namespace::Args {
            account_id: args.account_id.get_inner(),
            title: args.title.get_inner(),
        },
    );

    WorkersKvNamespaceResult {
        account_id: crate::into_domain(result.account_id),
        title: crate::into_domain(result.title),
    }
}
