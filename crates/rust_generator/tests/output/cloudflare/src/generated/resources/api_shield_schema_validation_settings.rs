/// Provides a resource to manage settings in API Shield Schema Validation 2.0.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_shield_schema_validation_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldSchemaValidationSettingsArgs {
        /// The default mitigation action used when there is no mitigation action defined on the operation.
        #[builder(into)]
        pub validation_default_mitigation_action: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// When set, this overrides both zone level and operation level mitigation actions.
        #[builder(into, default)]
        pub validation_override_mitigation_action: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldSchemaValidationSettingsResult {
        /// The default mitigation action used when there is no mitigation action defined on the operation.
        pub validation_default_mitigation_action: pulumi_gestalt_rust::Output<String>,
        /// When set, this overrides both zone level and operation level mitigation actions.
        pub validation_override_mitigation_action: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
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
        args: ApiShieldSchemaValidationSettingsArgs,
    ) -> ApiShieldSchemaValidationSettingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let validation_default_mitigation_action_binding = args
            .validation_default_mitigation_action
            .get_output(context);
        let validation_override_mitigation_action_binding = args
            .validation_override_mitigation_action
            .get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchemaValidationSettings:ApiShieldSchemaValidationSettings"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationDefaultMitigationAction".into(),
                    value: validation_default_mitigation_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationOverrideMitigationAction".into(),
                    value: validation_override_mitigation_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiShieldSchemaValidationSettingsResult {
            validation_default_mitigation_action: o
                .get_field("validationDefaultMitigationAction"),
            validation_override_mitigation_action: o
                .get_field("validationOverrideMitigationAction"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
