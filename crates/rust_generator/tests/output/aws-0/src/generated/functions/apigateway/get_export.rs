#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExportArgs {
        /// Content-type of the export. Valid values are `application/json` and `application/yaml` are supported for `export_type` `ofoas30` and `swagger`.
        #[builder(into, default)]
        pub accepts: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of export. Acceptable values are `oas30` for OpenAPI 3.0.x and `swagger` for Swagger/OpenAPI 2.0.
        #[builder(into)]
        pub export_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of query string parameters that specify properties of the export. the following parameters are supported: `extensions='integrations'` or `extensions='apigateway'` will export the API with x-amazon-apigateway-integration extensions. `extensions='authorizers'` will export the API with x-amazon-apigateway-authorizer extensions.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Stage that will be exported.
        #[builder(into)]
        pub stage_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExportResult {
        pub accepts: pulumi_gestalt_rust::Output<Option<String>>,
        /// API Spec.
        pub body: pulumi_gestalt_rust::Output<String>,
        /// Content-disposition header value in the HTTP response.
        pub content_disposition: pulumi_gestalt_rust::Output<String>,
        /// Content-type header value in the HTTP response.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        pub export_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub rest_api_id: pulumi_gestalt_rust::Output<String>,
        pub stage_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetExportArgs,
    ) -> GetExportResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accepts_binding_1 = args.accepts.get_output(context);
        let accepts_binding = accepts_binding_1.get_inner();
        let export_type_binding_1 = args.export_type.get_output(context);
        let export_type_binding = export_type_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let rest_api_id_binding_1 = args.rest_api_id.get_output(context);
        let rest_api_id_binding = rest_api_id_binding_1.get_inner();
        let stage_name_binding_1 = args.stage_name.get_output(context);
        let stage_name_binding = stage_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getExport:getExport".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accepts".into(),
                    value: &accepts_binding,
                },
                register_interface::ObjectField {
                    name: "exportType".into(),
                    value: &export_type_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetExportResult {
            accepts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accepts"),
            ),
            body: pulumi_gestalt_rust::__private::into_domain(o.extract_field("body")),
            content_disposition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentDisposition"),
            ),
            content_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            export_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exportType"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            rest_api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restApiId"),
            ),
            stage_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stageName"),
            ),
        }
    }
}
