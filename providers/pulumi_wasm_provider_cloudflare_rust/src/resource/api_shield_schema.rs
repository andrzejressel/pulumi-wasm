pub struct ApiShieldSchemaArgs {
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub source: pulumi_wasm_rust::Output<String>,
    pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldSchemaResult {
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub source: pulumi_wasm_rust::Output<String>,
    pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ApiShieldSchemaArgs) -> ApiShieldSchemaResult {
    let result = crate::bindings::pulumi::cloudflare::api_shield_schema::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::api_shield_schema::Args {
            kind: args.kind.get_inner(),
            name: args.name.get_inner(),
            source: args.source.get_inner(),
            validation_enabled: args.validation_enabled.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ApiShieldSchemaResult {
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        source: crate::into_domain(result.source),
        validation_enabled: crate::into_domain(result.validation_enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
