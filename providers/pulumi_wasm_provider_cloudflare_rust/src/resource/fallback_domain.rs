pub struct FallbackDomainArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub domains: pulumi_wasm_rust::Output<Vec<crate::types::FallbackDomainDomain>>,
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct FallbackDomainResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub domains: pulumi_wasm_rust::Output<Vec<crate::types::FallbackDomainDomain>>,
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: FallbackDomainArgs) -> FallbackDomainResult {
    let result = crate::bindings::pulumi::cloudflare::fallback_domain::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::fallback_domain::Args {
            account_id: args.account_id.get_inner(),
            domains: args.domains.get_inner(),
            policy_id: args.policy_id.get_inner(),
        },
    );

    FallbackDomainResult {
        account_id: crate::into_domain(result.account_id),
        domains: crate::into_domain(result.domains),
        policy_id: crate::into_domain(result.policy_id),
    }
}
