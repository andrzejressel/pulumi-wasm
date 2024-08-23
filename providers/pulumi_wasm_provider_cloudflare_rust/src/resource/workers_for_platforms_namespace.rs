pub struct WorkersForPlatformsNamespaceArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersForPlatformsNamespaceResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub fn create(
    name: &str,
    args: WorkersForPlatformsNamespaceArgs,
) -> WorkersForPlatformsNamespaceResult {
    let result = crate::bindings::pulumi::cloudflare::workers_for_platforms_namespace::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::workers_for_platforms_namespace::Args {
            account_id: args.account_id.get_inner(),
            name: args.name.get_inner(),
        },
    );

    WorkersForPlatformsNamespaceResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
    }
}
