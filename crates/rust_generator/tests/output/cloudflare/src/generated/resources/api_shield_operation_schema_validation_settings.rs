/// Provides a resource to manage operation-level settings in API Shield Schema Validation 2.0.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod api_shield_operation_schema_validation_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldOperationSchemaValidationSettingsArgs {
        /// The mitigation action to apply to this operation.
        #[builder(into, default)]
        pub mitigation_action: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub operation_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldOperationSchemaValidationSettingsResult {
        /// The mitigation action to apply to this operation.
        pub mitigation_action: pulumi_gestalt_rust::Output<Option<String>>,
        /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
        pub operation_id: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApiShieldOperationSchemaValidationSettingsArgs,
    ) -> ApiShieldOperationSchemaValidationSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mitigation_action_binding = args
            .mitigation_action
            .get_output(context)
            .get_inner();
        let operation_id_binding = args.operation_id.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApiShieldOperationSchemaValidationSettingsResult {
            mitigation_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mitigationAction"),
            ),
            operation_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("operationId"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
