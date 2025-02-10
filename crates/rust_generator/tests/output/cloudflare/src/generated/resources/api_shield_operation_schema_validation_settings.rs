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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiShieldOperationSchemaValidationSettingsArgs,
    ) -> ApiShieldOperationSchemaValidationSettingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mitigation_action_binding = args.mitigation_action.get_output(context);
        let operation_id_binding = args.operation_id.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldOperationSchemaValidationSettings:ApiShieldOperationSchemaValidationSettings"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mitigationAction".into(),
                    value: mitigation_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operationId".into(),
                    value: operation_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiShieldOperationSchemaValidationSettingsResult {
            mitigation_action: o.get_field("mitigationAction"),
            operation_id: o.get_field("operationId"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
