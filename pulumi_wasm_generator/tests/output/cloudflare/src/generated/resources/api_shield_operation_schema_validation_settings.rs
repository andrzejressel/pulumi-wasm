/// Provides a resource to manage operation-level settings in API Shield Schema Validation 2.0.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api_shield_operation::create(
///         "example",
///         ApiShieldOperationArgs::builder()
///             .endpoint("/path")
///             .host("api.example.com")
///             .method("GET")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let exampleApiShieldOperationSchemaValidationSettings = api_shield_operation_schema_validation_settings::create(
///         "exampleApiShieldOperationSchemaValidationSettings",
///         ApiShieldOperationSchemaValidationSettingsArgs::builder()
///             .mitigation_action("block")
///             .operation_id("${example.id}")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
pub mod api_shield_operation_schema_validation_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldOperationSchemaValidationSettingsArgs {
        /// The mitigation action to apply to this operation.
        #[builder(into, default)]
        pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
        /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub operation_id: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldOperationSchemaValidationSettingsResult {
        /// The mitigation action to apply to this operation.
        pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
        /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
        pub operation_id: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ApiShieldOperationSchemaValidationSettingsArgs,
    ) -> ApiShieldOperationSchemaValidationSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mitigation_action_binding = args.mitigation_action.get_inner();
        let operation_id_binding = args.operation_id.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldOperationSchemaValidationSettings:ApiShieldOperationSchemaValidationSettings"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mitigationAction".into(),
                    value: &mitigation_action_binding,
                },
                register_interface::ObjectField {
                    name: "operationId".into(),
                    value: &operation_id_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "mitigationAction".into(),
                },
                register_interface::ResultField {
                    name: "operationId".into(),
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
        ApiShieldOperationSchemaValidationSettingsResult {
            mitigation_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mitigationAction").unwrap(),
            ),
            operation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operationId").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
