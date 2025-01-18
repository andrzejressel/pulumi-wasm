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
pub mod integration_response {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationResponseArgs {
        /// How to handle request payload content type conversions. Supported values are `CONVERT_TO_BINARY` and `CONVERT_TO_TEXT`. If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.
        #[builder(into, default)]
        pub content_handling: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`).
        #[builder(into)]
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// API resource ID.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// Map of response parameters that can be read from the backend response. For example: `response_parameters = { "method.response.header.X-Some-Header" = "integration.response.header.X-Some-Other-Header" }`.
        #[builder(into, default)]
        pub response_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of templates used to transform the integration response body.
        #[builder(into, default)]
        pub response_templates: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Regular expression pattern used to choose an integration response based on the response from the backend. Omit configuring this to make the integration the default one. If the backend is an `AWS` Lambda function, the AWS Lambda function error header is matched. For all other `HTTP` and `AWS` backends, the HTTP status code is matched.
        #[builder(into, default)]
        pub selection_pattern: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP status code.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub status_code: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationResponseResult {
        /// How to handle request payload content type conversions. Supported values are `CONVERT_TO_BINARY` and `CONVERT_TO_TEXT`. If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.
        pub content_handling: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`).
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// API resource ID.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// Map of response parameters that can be read from the backend response. For example: `response_parameters = { "method.response.header.X-Some-Header" = "integration.response.header.X-Some-Other-Header" }`.
        pub response_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of templates used to transform the integration response body.
        pub response_templates: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the associated REST API.
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Regular expression pattern used to choose an integration response based on the response from the backend. Omit configuring this to make the integration the default one. If the backend is an `AWS` Lambda function, the AWS Lambda function error header is matched. For all other `HTTP` and `AWS` backends, the HTTP status code is matched.
        pub selection_pattern: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP status code.
        ///
        /// The following arguments are optional:
        pub status_code: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IntegrationResponseArgs,
    ) -> IntegrationResponseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_handling_binding = args.content_handling.get_inner();
        let http_method_binding = args.http_method.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let response_parameters_binding = args.response_parameters.get_inner();
        let response_templates_binding = args.response_templates.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let selection_pattern_binding = args.selection_pattern.get_inner();
        let status_code_binding = args.status_code.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/integrationResponse:IntegrationResponse".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contentHandling".into(),
                    value: &content_handling_binding,
                },
                register_interface::ObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "responseParameters".into(),
                    value: &response_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "responseTemplates".into(),
                    value: &response_templates_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "selectionPattern".into(),
                    value: &selection_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "statusCode".into(),
                    value: &status_code_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contentHandling".into(),
                },
                register_interface::ResultField {
                    name: "httpMethod".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "responseParameters".into(),
                },
                register_interface::ResultField {
                    name: "responseTemplates".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
                },
                register_interface::ResultField {
                    name: "selectionPattern".into(),
                },
                register_interface::ResultField {
                    name: "statusCode".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationResponseResult {
            content_handling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentHandling").unwrap(),
            ),
            http_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpMethod").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            response_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseParameters").unwrap(),
            ),
            response_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseTemplates").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
            selection_pattern: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selectionPattern").unwrap(),
            ),
            status_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusCode").unwrap(),
            ),
        }
    }
}
