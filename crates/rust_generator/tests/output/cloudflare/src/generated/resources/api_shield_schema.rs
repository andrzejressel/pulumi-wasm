/// Provides a resource to manage a schema in API Shield Schema Validation 2.0.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   petstoreSchema:
///     type: cloudflare:ApiShieldSchema
///     name: petstore_schema
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       name: myschema
///       kind: openapi_v3
///       validationEnabled: true # optional, default false
///       source:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: ./schemas/petstore.json
///           Return: result
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_shield_schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldSchemaArgs {
        /// Kind of schema. Defaults to `openapi_v3`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the schema. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Schema file bytes. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Flag whether schema is enabled for validation.
        #[builder(into, default)]
        pub validation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldSchemaResult {
        /// Kind of schema. Defaults to `openapi_v3`. **Modifying this attribute will force creation of a new resource.**
        pub kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the schema. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Schema file bytes. **Modifying this attribute will force creation of a new resource.**
        pub source: pulumi_gestalt_rust::Output<String>,
        /// Flag whether schema is enabled for validation.
        pub validation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
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
        args: ApiShieldSchemaArgs,
    ) -> ApiShieldSchemaResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let kind_binding = args.kind.get_output(context);
        let name_binding = args.name.get_output(context);
        let source_binding = args.source.get_output(context);
        let validation_enabled_binding = args.validation_enabled.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchema:ApiShieldSchema".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationEnabled".into(),
                    value: validation_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiShieldSchemaResult {
            kind: o.get_field("kind"),
            name: o.get_field("name"),
            source: o.get_field("source"),
            validation_enabled: o.get_field("validationEnabled"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
