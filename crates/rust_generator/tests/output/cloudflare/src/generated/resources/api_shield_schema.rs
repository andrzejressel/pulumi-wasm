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
pub mod api_shield_schema {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldSchemaArgs {
        /// Kind of schema. Defaults to `openapi_v3`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub kind: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the schema. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Schema file bytes. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub source: pulumi_wasm_rust::InputOrOutput<String>,
        /// Flag whether schema is enabled for validation.
        #[builder(into, default)]
        pub validation_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldSchemaResult {
        /// Kind of schema. Defaults to `openapi_v3`. **Modifying this attribute will force creation of a new resource.**
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the schema. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_wasm_rust::Output<String>,
        /// Schema file bytes. **Modifying this attribute will force creation of a new resource.**
        pub source: pulumi_wasm_rust::Output<String>,
        /// Flag whether schema is enabled for validation.
        pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ApiShieldSchemaArgs,
    ) -> ApiShieldSchemaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kind_binding = args.kind.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let validation_enabled_binding = args
            .validation_enabled
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchema:ApiShieldSchema".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "validationEnabled".into(),
                    value: &validation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApiShieldSchemaResult {
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            source: pulumi_wasm_rust::__private::into_domain(o.extract_field("source")),
            validation_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validationEnabled"),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
