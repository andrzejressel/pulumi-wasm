pub struct PagesDomainArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub domain: pulumi_wasm_rust::Output<String>,
    pub project_name: pulumi_wasm_rust::Output<String>,
}

pub struct PagesDomainResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub domain: pulumi_wasm_rust::Output<String>,
    pub project_name: pulumi_wasm_rust::Output<String>,
    pub status: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: PagesDomainArgs) -> PagesDomainResult {
    let result = crate::bindings::pulumi::cloudflare::pages_domain::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::pages_domain::Args {
            account_id: args.account_id.get_inner(),
            domain: args.domain.get_inner(),
            project_name: args.project_name.get_inner(),
        },
    );

    PagesDomainResult {
        account_id: crate::into_domain(result.account_id),
        domain: crate::into_domain(result.domain),
        project_name: crate::into_domain(result.project_name),
        status: crate::into_domain(result.status),
    }
}
