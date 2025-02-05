pub mod get_export {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExportArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Version of the API Gateway export algorithm. API Gateway uses the latest version by default. Currently, the only supported version is `1.0`.
        #[builder(into, default)]
        pub export_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to include API Gateway extensions in the exported API definition. API Gateway extensions are included by default.
        #[builder(into, default)]
        pub include_extensions: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Output type of the exported definition file. Valid values are `JSON` and `YAML`.
        #[builder(into)]
        pub output_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Version of the API specification to use. `OAS30`, for OpenAPI 3.0, is the only supported value.
        #[builder(into)]
        pub specification: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the API stage to export. If you don't specify this property, a representation of the latest API configuration is exported.
        #[builder(into, default)]
        pub stage_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetExportResult {
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// ID of the API.
        pub body: pulumi_wasm_rust::Output<String>,
        pub export_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_extensions: pulumi_wasm_rust::Output<Option<bool>>,
        pub output_type: pulumi_wasm_rust::Output<String>,
        pub specification: pulumi_wasm_rust::Output<String>,
        pub stage_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetExportArgs,
    ) -> GetExportResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_output(context).get_inner();
        let export_version_binding = args.export_version.get_output(context).get_inner();
        let include_extensions_binding = args
            .include_extensions
            .get_output(context)
            .get_inner();
        let output_type_binding = args.output_type.get_output(context).get_inner();
        let specification_binding = args.specification.get_output(context).get_inner();
        let stage_name_binding = args.stage_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigatewayv2/getExport:getExport".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "exportVersion".into(),
                    value: &export_version_binding,
                },
                register_interface::ObjectField {
                    name: "includeExtensions".into(),
                    value: &include_extensions_binding,
                },
                register_interface::ObjectField {
                    name: "outputType".into(),
                    value: &output_type_binding,
                },
                register_interface::ObjectField {
                    name: "specification".into(),
                    value: &specification_binding,
                },
                register_interface::ObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetExportResult {
            api_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("apiId")),
            body: pulumi_wasm_rust::__private::into_domain(o.extract_field("body")),
            export_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exportVersion"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            include_extensions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includeExtensions"),
            ),
            output_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outputType"),
            ),
            specification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("specification"),
            ),
            stage_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stageName"),
            ),
        }
    }
}
