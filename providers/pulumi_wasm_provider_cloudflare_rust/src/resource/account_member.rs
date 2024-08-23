pub struct AccountMemberArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub email_address: pulumi_wasm_rust::Output<String>,
    pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
    pub status: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccountMemberResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub email_address: pulumi_wasm_rust::Output<String>,
    pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
    pub status: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: AccountMemberArgs) -> AccountMemberResult {
    let result = crate::bindings::pulumi::cloudflare::account_member::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::account_member::Args {
            account_id: args.account_id.get_inner(),
            email_address: args.email_address.get_inner(),
            role_ids: args.role_ids.get_inner(),
            status: args.status.get_inner(),
        },
    );

    AccountMemberResult {
        account_id: crate::into_domain(result.account_id),
        email_address: crate::into_domain(result.email_address),
        role_ids: crate::into_domain(result.role_ids),
        status: crate::into_domain(result.status),
    }
}
