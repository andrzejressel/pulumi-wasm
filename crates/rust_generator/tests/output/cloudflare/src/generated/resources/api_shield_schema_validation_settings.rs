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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApiShieldSchemaValidationSettingsArgs,
    ) -> ApiShieldSchemaValidationSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let validation_default_mitigation_action_binding = args
            .validation_default_mitigation_action
            .get_output(context)
            .get_inner();
        let validation_override_mitigation_action_binding = args
            .validation_override_mitigation_action
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApiShieldSchemaValidationSettingsResult {
            validation_default_mitigation_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validationDefaultMitigationAction"),
            ),
            validation_override_mitigation_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validationOverrideMitigationAction"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
