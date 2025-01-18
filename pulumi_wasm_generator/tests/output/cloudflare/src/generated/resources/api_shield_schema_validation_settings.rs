/// Provides a resource to manage settings in API Shield Schema Validation 2.0.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api_shield_schema_validation_settings::create(
///         "example",
///         ApiShieldSchemaValidationSettingsArgs::builder()
///             .validation_default_mitigation_action("log")
///             .validation_override_mitigation_action("none")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
pub mod api_shield_schema_validation_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldSchemaValidationSettingsArgs {
        /// The default mitigation action used when there is no mitigation action defined on the operation.
        #[builder(into)]
        pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
        /// When set, this overrides both zone level and operation level mitigation actions.
        #[builder(into, default)]
        pub validation_override_mitigation_action: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldSchemaValidationSettingsResult {
        /// The default mitigation action used when there is no mitigation action defined on the operation.
        pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
        /// When set, this overrides both zone level and operation level mitigation actions.
        pub validation_override_mitigation_action: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ApiShieldSchemaValidationSettingsArgs,
    ) -> ApiShieldSchemaValidationSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let validation_default_mitigation_action_binding = args
            .validation_default_mitigation_action
            .get_inner();
        let validation_override_mitigation_action_binding = args
            .validation_override_mitigation_action
            .get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchemaValidationSettings:ApiShieldSchemaValidationSettings"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "validationDefaultMitigationAction".into(),
                    value: &validation_default_mitigation_action_binding,
                },
                register_interface::ObjectField {
                    name: "validationOverrideMitigationAction".into(),
                    value: &validation_override_mitigation_action_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "validationDefaultMitigationAction".into(),
                },
                register_interface::ResultField {
                    name: "validationOverrideMitigationAction".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiShieldSchemaValidationSettingsResult {
            validation_default_mitigation_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationDefaultMitigationAction").unwrap(),
            ),
            validation_override_mitigation_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationOverrideMitigationAction").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
