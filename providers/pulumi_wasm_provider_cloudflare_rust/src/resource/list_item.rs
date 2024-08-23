pub struct ListItemArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub asn: pulumi_wasm_rust::Output<Option<i32>>,
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    pub hostname: pulumi_wasm_rust::Output<Option<crate::types::ListItemHostname>>,
    pub ip: pulumi_wasm_rust::Output<Option<String>>,
    pub list_id: pulumi_wasm_rust::Output<String>,
    pub redirect: pulumi_wasm_rust::Output<Option<crate::types::ListItemRedirect>>,
}

pub struct ListItemResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub asn: pulumi_wasm_rust::Output<Option<i32>>,
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    pub hostname: pulumi_wasm_rust::Output<Option<crate::types::ListItemHostname>>,
    pub ip: pulumi_wasm_rust::Output<Option<String>>,
    pub list_id: pulumi_wasm_rust::Output<String>,
    pub redirect: pulumi_wasm_rust::Output<Option<crate::types::ListItemRedirect>>,
}

pub fn create(name: &str, args: ListItemArgs) -> ListItemResult {
    let result = crate::bindings::pulumi::cloudflare::list_item::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::list_item::Args {
            account_id: args.account_id.get_inner(),
            asn: args.asn.get_inner(),
            comment: args.comment.get_inner(),
            hostname: args.hostname.get_inner(),
            ip: args.ip.get_inner(),
            list_id: args.list_id.get_inner(),
            redirect: args.redirect.get_inner(),
        },
    );

    ListItemResult {
        account_id: crate::into_domain(result.account_id),
        asn: crate::into_domain(result.asn),
        comment: crate::into_domain(result.comment),
        hostname: crate::into_domain(result.hostname),
        ip: crate::into_domain(result.ip),
        list_id: crate::into_domain(result.list_id),
        redirect: crate::into_domain(result.redirect),
    }
}
