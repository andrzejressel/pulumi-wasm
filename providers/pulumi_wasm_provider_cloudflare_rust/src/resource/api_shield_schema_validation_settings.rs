pub struct ApiShieldSchemaValidationSettingsArgs {
    pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
    pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldSchemaValidationSettingsResult {
    pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
    pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(
    name: &str,
    args: ApiShieldSchemaValidationSettingsArgs,
) -> ApiShieldSchemaValidationSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::api_shield_schema_validation_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::api_shield_schema_validation_settings::Args {
            validation_default_mitigation_action: args
                .validation_default_mitigation_action
                .get_inner(),
            validation_override_mitigation_action: args
                .validation_override_mitigation_action
                .get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ApiShieldSchemaValidationSettingsResult {
        validation_default_mitigation_action: crate::into_domain(
            result.validation_default_mitigation_action,
        ),
        validation_override_mitigation_action: crate::into_domain(
            result.validation_override_mitigation_action,
        ),
        zone_id: crate::into_domain(result.zone_id),
    }
}
