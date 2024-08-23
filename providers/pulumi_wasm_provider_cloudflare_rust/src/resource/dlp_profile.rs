pub struct DlpProfileArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    pub context_awareness:
        pulumi_wasm_rust::Output<Option<crate::types::DlpProfileContextAwareness>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DlpProfileResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
    pub context_awareness: pulumi_wasm_rust::Output<crate::types::DlpProfileContextAwareness>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: DlpProfileArgs) -> DlpProfileResult {
    let result = crate::bindings::pulumi::cloudflare::dlp_profile::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::dlp_profile::Args {
            account_id: args.account_id.get_inner(),
            allowed_match_count: args.allowed_match_count.get_inner(),
            context_awareness: args.context_awareness.get_inner(),
            description: args.description.get_inner(),
            entries: args.entries.get_inner(),
            name: args.name.get_inner(),
            type_: args.type_.get_inner(),
        },
    );

    DlpProfileResult {
        account_id: crate::into_domain(result.account_id),
        allowed_match_count: crate::into_domain(result.allowed_match_count),
        context_awareness: crate::into_domain(result.context_awareness),
        description: crate::into_domain(result.description),
        entries: crate::into_domain(result.entries),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
