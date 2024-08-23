pub struct AccessServiceTokenArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub duration: pulumi_wasm_rust::Output<Option<String>>,
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessServiceTokenResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub client_id: pulumi_wasm_rust::Output<String>,
    pub client_secret: pulumi_wasm_rust::Output<String>,
    pub duration: pulumi_wasm_rust::Output<String>,
    pub expires_at: pulumi_wasm_rust::Output<String>,
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: AccessServiceTokenArgs) -> AccessServiceTokenResult {
    let result = crate::bindings::pulumi::cloudflare::access_service_token::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_service_token::Args {
            account_id: args.account_id.get_inner(),
            duration: args.duration.get_inner(),
            min_days_for_renewal: args.min_days_for_renewal.get_inner(),
            name: args.name.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessServiceTokenResult {
        account_id: crate::into_domain(result.account_id),
        client_id: crate::into_domain(result.client_id),
        client_secret: crate::into_domain(result.client_secret),
        duration: crate::into_domain(result.duration),
        expires_at: crate::into_domain(result.expires_at),
        min_days_for_renewal: crate::into_domain(result.min_days_for_renewal),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
