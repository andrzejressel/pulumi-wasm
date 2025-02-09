/// Provides a resource to manage an API Gateway Documentation Version.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apigateway:DocumentationVersion
///     properties:
///       version: example_version
///       restApiId: ${exampleRestApi.id}
///       description: Example description
///     options:
///       dependsOn:
///         - ${exampleDocumentationPart}
///   exampleRestApi:
///     type: aws:apigateway:RestApi
///     name: example
///     properties:
///       name: example_api
///   exampleDocumentationPart:
///     type: aws:apigateway:DocumentationPart
///     name: example
///     properties:
///       location:
///         type: API
///       properties: '{"description":"Example"}'
///       restApiId: ${exampleRestApi.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import API Gateway documentation versions using `REST-API-ID/VERSION`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/documentationVersion:DocumentationVersion example 5i4e1ko720/example-version
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod documentation_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DocumentationVersionArgs {
        /// Description of the API documentation version.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the associated Rest API
        #[builder(into)]
        pub rest_api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version identifier of the API documentation snapshot.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DocumentationVersionResult {
        /// Description of the API documentation version.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the associated Rest API
        pub rest_api_id: pulumi_gestalt_rust::Output<String>,
        /// Version identifier of the API documentation snapshot.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DocumentationVersionArgs,
    ) -> DocumentationVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let rest_api_id_binding = args.rest_api_id.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/documentationVersion:DocumentationVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApiId".into(),
                    value: rest_api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DocumentationVersionResult {
            description: o.get_field("description"),
            rest_api_id: o.get_field("restApiId"),
            version: o.get_field("version"),
        }
    }
}
