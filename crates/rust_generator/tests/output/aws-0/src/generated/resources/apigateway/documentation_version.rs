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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DocumentationVersionArgs,
    ) -> DocumentationVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let rest_api_id_binding_1 = args.rest_api_id.get_output(context);
        let rest_api_id_binding = rest_api_id_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/documentationVersion:DocumentationVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DocumentationVersionResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            rest_api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restApiId"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
