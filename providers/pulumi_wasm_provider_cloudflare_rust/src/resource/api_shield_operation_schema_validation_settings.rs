pub struct ApiShieldOperationSchemaValidationSettingsArgs {
    pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    pub operation_id: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldOperationSchemaValidationSettingsResult {
    pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    pub operation_id: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(
    name: &str,
    args: ApiShieldOperationSchemaValidationSettingsArgs,
) -> ApiShieldOperationSchemaValidationSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::api_shield_operation_schema_validation_settings::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_operation_schema_validation_settings::Args {
        mitigation_action: args.mitigation_action.get_inner(),
        operation_id: args.operation_id.get_inner(),
        zone_id: args.zone_id.get_inner(),
    });

    ApiShieldOperationSchemaValidationSettingsResult {
        mitigation_action: crate::into_domain(result.mitigation_action),
        operation_id: crate::into_domain(result.operation_id),
        zone_id: crate::into_domain(result.zone_id),
    }
}
