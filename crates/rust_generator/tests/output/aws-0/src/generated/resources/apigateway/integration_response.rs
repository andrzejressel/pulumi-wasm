/// Provides an HTTP Method Integration Response for an API Gateway Resource.
///
/// > **Note:** Depends on having `aws.apigateway.Integration` inside your rest api. To ensure this
/// you might need to add an explicit `depends_on` for clean runs.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myDemoAPI:
///     type: aws:apigateway:RestApi
///     name: MyDemoAPI
///     properties:
///       name: MyDemoAPI
///       description: This is my API for demonstration purposes
///   myDemoResource:
///     type: aws:apigateway:Resource
///     name: MyDemoResource
///     properties:
///       restApi: ${myDemoAPI.id}
///       parentId: ${myDemoAPI.rootResourceId}
///       pathPart: mydemoresource
///   myDemoMethod:
///     type: aws:apigateway:Method
///     name: MyDemoMethod
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: GET
///       authorization: NONE
///   myDemoIntegration:
///     type: aws:apigateway:Integration
///     name: MyDemoIntegration
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: ${myDemoMethod.httpMethod}
///       type: MOCK
///   response200:
///     type: aws:apigateway:MethodResponse
///     name: response_200
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: ${myDemoMethod.httpMethod}
///       statusCode: '200'
///   myDemoIntegrationResponse:
///     type: aws:apigateway:IntegrationResponse
///     name: MyDemoIntegrationResponse
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: ${myDemoMethod.httpMethod}
///       statusCode: ${response200.statusCode}
///       responseTemplates:
///         application/xml: |
///           #set($inputRoot = $input.path('$'))
///           <?xml version="1.0" encoding="UTF-8"?>
///           <message>
///               $inputRoot.body
///           </message>
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_integration_response` using `REST-API-ID/RESOURCE-ID/HTTP-METHOD/STATUS-CODE`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/integrationResponse:IntegrationResponse example 12345abcde/67890fghij/GET/200
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_response {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationResponseArgs {
        /// How to handle request payload content type conversions. Supported values are `CONVERT_TO_BINARY` and `CONVERT_TO_TEXT`. If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.
        #[builder(into, default)]
        pub content_handling: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`).
        #[builder(into)]
        pub http_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// API resource ID.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of response parameters that can be read from the backend response. For example: `response_parameters = { "method.response.header.X-Some-Header" = "integration.response.header.X-Some-Other-Header" }`.
        #[builder(into, default)]
        pub response_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of templates used to transform the integration response body.
        #[builder(into, default)]
        pub response_templates: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Regular expression pattern used to choose an integration response based on the response from the backend. Omit configuring this to make the integration the default one. If the backend is an `AWS` Lambda function, the AWS Lambda function error header is matched. For all other `HTTP` and `AWS` backends, the HTTP status code is matched.
        #[builder(into, default)]
        pub selection_pattern: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// HTTP status code.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub status_code: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationResponseResult {
        /// How to handle request payload content type conversions. Supported values are `CONVERT_TO_BINARY` and `CONVERT_TO_TEXT`. If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.
        pub content_handling: pulumi_gestalt_rust::Output<Option<String>>,
        /// HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`).
        pub http_method: pulumi_gestalt_rust::Output<String>,
        /// API resource ID.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// Map of response parameters that can be read from the backend response. For example: `response_parameters = { "method.response.header.X-Some-Header" = "integration.response.header.X-Some-Other-Header" }`.
        pub response_parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of templates used to transform the integration response body.
        pub response_templates: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the associated REST API.
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Regular expression pattern used to choose an integration response based on the response from the backend. Omit configuring this to make the integration the default one. If the backend is an `AWS` Lambda function, the AWS Lambda function error header is matched. For all other `HTTP` and `AWS` backends, the HTTP status code is matched.
        pub selection_pattern: pulumi_gestalt_rust::Output<Option<String>>,
        /// HTTP status code.
        ///
        /// The following arguments are optional:
        pub status_code: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationResponseArgs,
    ) -> IntegrationResponseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let content_handling_binding = args.content_handling.get_output(context);
        let http_method_binding = args.http_method.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let response_parameters_binding = args.response_parameters.get_output(context);
        let response_templates_binding = args.response_templates.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let selection_pattern_binding = args.selection_pattern.get_output(context);
        let status_code_binding = args.status_code.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/integrationResponse:IntegrationResponse".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentHandling".into(),
                    value: content_handling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpMethod".into(),
                    value: http_method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "responseParameters".into(),
                    value: response_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "responseTemplates".into(),
                    value: response_templates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: rest_api_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selectionPattern".into(),
                    value: selection_pattern_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statusCode".into(),
                    value: status_code_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationResponseResult {
            content_handling: o.get_field("contentHandling"),
            http_method: o.get_field("httpMethod"),
            resource_id: o.get_field("resourceId"),
            response_parameters: o.get_field("responseParameters"),
            response_templates: o.get_field("responseTemplates"),
            rest_api: o.get_field("restApi"),
            selection_pattern: o.get_field("selectionPattern"),
            status_code: o.get_field("statusCode"),
        }
    }
}
