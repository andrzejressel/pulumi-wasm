pub struct TeamsLocationArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub networks: pulumi_wasm_rust::Output<Option<Vec<crate::types::TeamsLocationNetwork>>>,
}

pub struct TeamsLocationResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub anonymized_logs_enabled: pulumi_wasm_rust::Output<bool>,
    pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
    pub doh_subdomain: pulumi_wasm_rust::Output<String>,
    pub ip: pulumi_wasm_rust::Output<String>,
    pub ipv4_destination: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub networks: pulumi_wasm_rust::Output<Option<Vec<crate::types::TeamsLocationNetwork>>>,
    pub policy_ids: pulumi_wasm_rust::Output<Vec<String>>,
}

pub fn create(name: &str, args: TeamsLocationArgs) -> TeamsLocationResult {
    let result = crate::bindings::pulumi::cloudflare::teams_location::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::teams_location::Args {
            account_id: args.account_id.get_inner(),
            client_default: args.client_default.get_inner(),
            name: args.name.get_inner(),
            networks: args.networks.get_inner(),
        },
    );

    TeamsLocationResult {
        account_id: crate::into_domain(result.account_id),
        anonymized_logs_enabled: crate::into_domain(result.anonymized_logs_enabled),
        client_default: crate::into_domain(result.client_default),
        doh_subdomain: crate::into_domain(result.doh_subdomain),
        ip: crate::into_domain(result.ip),
        ipv4_destination: crate::into_domain(result.ipv4_destination),
        name: crate::into_domain(result.name),
        networks: crate::into_domain(result.networks),
        policy_ids: crate::into_domain(result.policy_ids),
    }
}
