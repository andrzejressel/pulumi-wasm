pub struct ApiShieldArgs {
    pub auth_id_characteristics:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldResult {
    pub auth_id_characteristics:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ApiShieldArgs) -> ApiShieldResult {
    let result = crate::bindings::pulumi::cloudflare::api_shield::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::api_shield::Args {
            auth_id_characteristics: args.auth_id_characteristics.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ApiShieldResult {
        auth_id_characteristics: crate::into_domain(result.auth_id_characteristics),
        zone_id: crate::into_domain(result.zone_id),
    }
}
