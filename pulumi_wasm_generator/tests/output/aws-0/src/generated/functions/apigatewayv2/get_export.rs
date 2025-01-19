pub mod get_export {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExportArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Version of the API Gateway export algorithm. API Gateway uses the latest version by default. Currently, the only supported version is `1.0`.
        #[builder(into, default)]
        pub export_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to include API Gateway extensions in the exported API definition. API Gateway extensions are included by default.
        #[builder(into, default)]
        pub include_extensions: pulumi_wasm_rust::Output<Option<bool>>,
        /// Output type of the exported definition file. Valid values are `JSON` and `YAML`.
        #[builder(into)]
        pub output_type: pulumi_wasm_rust::Output<String>,
        /// Version of the API specification to use. `OAS30`, for OpenAPI 3.0, is the only supported value.
        #[builder(into)]
        pub specification: pulumi_wasm_rust::Output<String>,
        /// Name of the API stage to export. If you don't specify this property, a representation of the latest API configuration is exported.
        #[builder(into, default)]
        pub stage_name: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetExportArgs) -> GetExportResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let export_version_binding = args.export_version.get_inner();
        let include_extensions_binding = args.include_extensions.get_inner();
        let output_type_binding = args.output_type.get_inner();
        let specification_binding = args.specification.get_inner();
        let stage_name_binding = args.stage_name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "body".into(),
                },
                register_interface::ResultField {
                    name: "exportVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "includeExtensions".into(),
                },
                register_interface::ResultField {
                    name: "outputType".into(),
                },
                register_interface::ResultField {
                    name: "specification".into(),
                },
                register_interface::ResultField {
                    name: "stageName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetExportResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("body").unwrap(),
            ),
            export_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            include_extensions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeExtensions").unwrap(),
            ),
            output_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputType").unwrap(),
            ),
            specification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("specification").unwrap(),
            ),
            stage_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stageName").unwrap(),
            ),
        }
    }
}
