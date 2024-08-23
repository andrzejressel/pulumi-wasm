pub struct EmailRoutingAddressArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub email: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingAddressResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub created: pulumi_wasm_rust::Output<String>,
    pub email: pulumi_wasm_rust::Output<String>,
    pub modified: pulumi_wasm_rust::Output<String>,
    pub tag: pulumi_wasm_rust::Output<String>,
    pub verified: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: EmailRoutingAddressArgs) -> EmailRoutingAddressResult {
    let result = crate::bindings::pulumi::cloudflare::email_routing_address::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::email_routing_address::Args {
            account_id: args.account_id.get_inner(),
            email: args.email.get_inner(),
        },
    );

    EmailRoutingAddressResult {
        account_id: crate::into_domain(result.account_id),
        created: crate::into_domain(result.created),
        email: crate::into_domain(result.email),
        modified: crate::into_domain(result.modified),
        tag: crate::into_domain(result.tag),
        verified: crate::into_domain(result.verified),
    }
}
