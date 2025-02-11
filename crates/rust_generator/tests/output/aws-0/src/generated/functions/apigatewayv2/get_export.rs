#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExportArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the API Gateway export algorithm. API Gateway uses the latest version by default. Currently, the only supported version is `1.0`.
        #[builder(into, default)]
        pub export_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to include API Gateway extensions in the exported API definition. API Gateway extensions are included by default.
        #[builder(into, default)]
        pub include_extensions: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Output type of the exported definition file. Valid values are `JSON` and `YAML`.
        #[builder(into)]
        pub output_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the API specification to use. `OAS30`, for OpenAPI 3.0, is the only supported value.
        #[builder(into)]
        pub specification: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the API stage to export. If you don't specify this property, a representation of the latest API configuration is exported.
        #[builder(into, default)]
        pub stage_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetExportResult {
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the API.
        pub body: pulumi_gestalt_rust::Output<String>,
        pub export_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_extensions: pulumi_gestalt_rust::Output<Option<bool>>,
        pub output_type: pulumi_gestalt_rust::Output<String>,
        pub specification: pulumi_gestalt_rust::Output<String>,
        pub stage_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetExportArgs,
    ) -> GetExportResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let export_version_binding = args.export_version.get_output(context);
        let include_extensions_binding = args.include_extensions.get_output(context);
        let output_type_binding = args.output_type.get_output(context);
        let specification_binding = args.specification.get_output(context);
        let stage_name_binding = args.stage_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigatewayv2/getExport:getExport".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportVersion".into(),
                    value: &export_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeExtensions".into(),
                    value: &include_extensions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputType".into(),
                    value: &output_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "specification".into(),
                    value: &specification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetExportResult {
            api_id: o.get_field("apiId"),
            body: o.get_field("body"),
            export_version: o.get_field("exportVersion"),
            id: o.get_field("id"),
            include_extensions: o.get_field("includeExtensions"),
            output_type: o.get_field("outputType"),
            specification: o.get_field("specification"),
            stage_name: o.get_field("stageName"),
        }
    }
}
