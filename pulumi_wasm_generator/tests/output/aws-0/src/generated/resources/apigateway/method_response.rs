/// Provides an HTTP Method Response for an API Gateway Resource. More information about API Gateway method responses can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-settings-method-response.html).
///
/// ## Example Usage
///
/// ### Basic Response
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myDemoAPI = rest_api::create(
///         "myDemoAPI",
///         RestApiArgs::builder()
///             .description("This is my API for demonstration purposes")
///             .name("MyDemoAPI")
///             .build_struct(),
///     );
///     let myDemoIntegration = integration::create(
///         "myDemoIntegration",
///         IntegrationArgs::builder()
///             .http_method("${myDemoMethod.httpMethod}")
///             .resource_id("${myDemoResource.id}")
///             .rest_api("${myDemoAPI.id}")
///             .type_("MOCK")
///             .build_struct(),
///     );
///     let myDemoMethod = method::create(
///         "myDemoMethod",
///         MethodArgs::builder()
///             .authorization("NONE")
///             .http_method("GET")
///             .resource_id("${myDemoResource.id}")
///             .rest_api("${myDemoAPI.id}")
///             .build_struct(),
///     );
///     let myDemoResource = resource::create(
///         "myDemoResource",
///         ResourceArgs::builder()
///             .parent_id("${myDemoAPI.rootResourceId}")
///             .path_part("mydemoresource")
///             .rest_api("${myDemoAPI.id}")
///             .build_struct(),
///     );
///     let response200 = method_response::create(
///         "response200",
///         MethodResponseArgs::builder()
///             .http_method("${myDemoMethod.httpMethod}")
///             .resource_id("${myDemoResource.id}")
///             .rest_api("${myDemoAPI.id}")
///             .status_code("200")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Response with Custom Header and Model
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
///   myDemoResponseModel:
///     type: aws:apigateway:Model
///     name: MyDemoResponseModel
///     properties:
///       restApi: ${myDemoAPI.id}
///       name: MyDemoResponseModel
///       description: API response for MyDemoMethod
///       contentType: application/json
///       schema:
///         fn::toJSON:
///           $schema: http://json-schema.org/draft-04/schema#
///           title: MyDemoResponse
///           type: object
///           properties:
///             Message:
///               type: string
///   response200:
///     type: aws:apigateway:MethodResponse
///     name: response_200
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: ${myDemoMethod.httpMethod}
///       statusCode: '200'
///       responseModels:
///         application/json: MyDemoResponseModel
///       responseParameters:
///         method.response.header.Content-Type: false
///         method-response-header.X-My-Demo-Header: false
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_method_response` using `REST-API-ID/RESOURCE-ID/HTTP-METHOD/STATUS-CODE`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/methodResponse:MethodResponse example 12345abcde/67890fghij/GET/200
/// ```
pub mod method_response {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MethodResponseArgs {
        /// The HTTP verb of the method resource (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`).
        #[builder(into)]
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// The Resource identifier for the method resource.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// A map specifying the model resources used for the response's content type. Response models are represented as a key/value map, with a content type as the key and a Model name as the value.
        #[builder(into, default)]
        pub response_models: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header name and the associated value is a boolean flag indicating whether the method response parameter is required. The method response header names must match the pattern of `method.response.header.{name}`, where `name` is a valid and unique header name.
        ///
        /// The response parameter names defined here are available in the integration response to be mapped from an integration response header expressed in `integration.response.header.{name}`, a static value enclosed within a pair of single quotes (e.g., '`application/json'`), or a JSON expression from the back-end response payload in the form of `integration.response.body.{JSON-expression}`, where `JSON-expression` is a valid JSON expression without the `$` prefix.)
        #[builder(into, default)]
        pub response_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, bool>>,
        >,
        /// The string identifier of the associated REST API.
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// The method response's status code.
        #[builder(into)]
        pub status_code: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MethodResponseResult {
        /// The HTTP verb of the method resource (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`).
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// The Resource identifier for the method resource.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// A map specifying the model resources used for the response's content type. Response models are represented as a key/value map, with a content type as the key and a Model name as the value.
        pub response_models: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header name and the associated value is a boolean flag indicating whether the method response parameter is required. The method response header names must match the pattern of `method.response.header.{name}`, where `name` is a valid and unique header name.
        ///
        /// The response parameter names defined here are available in the integration response to be mapped from an integration response header expressed in `integration.response.header.{name}`, a static value enclosed within a pair of single quotes (e.g., '`application/json'`), or a JSON expression from the back-end response payload in the form of `integration.response.body.{JSON-expression}`, where `JSON-expression` is a valid JSON expression without the `$` prefix.)
        pub response_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, bool>>,
        >,
        /// The string identifier of the associated REST API.
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// The method response's status code.
        pub status_code: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MethodResponseArgs) -> MethodResponseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let http_method_binding = args.http_method.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let response_models_binding = args.response_models.get_inner();
        let response_parameters_binding = args.response_parameters.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let status_code_binding = args.status_code.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/methodResponse:MethodResponse".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "responseModels".into(),
                    value: &response_models_binding,
                },
                register_interface::ObjectField {
                    name: "responseParameters".into(),
                    value: &response_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "statusCode".into(),
                    value: &status_code_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "httpMethod".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "responseModels".into(),
                },
                register_interface::ResultField {
                    name: "responseParameters".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
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
        MethodResponseResult {
            http_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpMethod").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            response_models: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseModels").unwrap(),
            ),
            response_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseParameters").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
            status_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusCode").unwrap(),
            ),
        }
    }
}
