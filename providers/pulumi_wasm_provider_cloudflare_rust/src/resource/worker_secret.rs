pub struct WorkerSecretArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub script_name: pulumi_wasm_rust::Output<String>,
    pub secret_text: pulumi_wasm_rust::Output<String>,
}

pub struct WorkerSecretResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub script_name: pulumi_wasm_rust::Output<String>,
    pub secret_text: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WorkerSecretArgs) -> WorkerSecretResult {
    let result = crate::bindings::pulumi::cloudflare::worker_secret::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::worker_secret::Args {
            account_id: args.account_id.get_inner(),
            name: args.name.get_inner(),
            script_name: args.script_name.get_inner(),
            secret_text: args.secret_text.get_inner(),
        },
    );

    WorkerSecretResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
        script_name: crate::into_domain(result.script_name),
        secret_text: crate::into_domain(result.secret_text),
    }
}
