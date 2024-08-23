pub struct WorkersKvArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub key: pulumi_wasm_rust::Output<String>,
    pub namespace_id: pulumi_wasm_rust::Output<String>,
    pub value: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersKvResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub key: pulumi_wasm_rust::Output<String>,
    pub namespace_id: pulumi_wasm_rust::Output<String>,
    pub value: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WorkersKvArgs) -> WorkersKvResult {
    let result = crate::bindings::pulumi::cloudflare::workers_kv::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::workers_kv::Args {
            account_id: args.account_id.get_inner(),
            key: args.key.get_inner(),
            namespace_id: args.namespace_id.get_inner(),
            value: args.value.get_inner(),
        },
    );

    WorkersKvResult {
        account_id: crate::into_domain(result.account_id),
        key: crate::into_domain(result.key),
        namespace_id: crate::into_domain(result.namespace_id),
        value: crate::into_domain(result.value),
    }
}
