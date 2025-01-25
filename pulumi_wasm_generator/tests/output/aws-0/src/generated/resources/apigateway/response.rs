/// Provides an API Gateway Gateway Response for a REST API Gateway.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:apigateway:RestApi
///     properties:
///       name: MyDemoAPI
///   test:
///     type: aws:apigateway:Response
///     properties:
///       restApiId: ${main.id}
///       statusCode: '401'
///       responseType: UNAUTHORIZED
///       responseTemplates:
///         application/json: '{"message":$context.error.messageString}'
///       responseParameters:
///         gatewayresponse.header.Authorization: '''Basic'''
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_gateway_response` using `REST-API-ID/RESPONSE-TYPE`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/response:Response example 12345abcde/UNAUTHORIZED
/// ```
pub mod response {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResponseArgs {
        /// Map of parameters (paths, query strings and headers) of the Gateway Response.
        #[builder(into, default)]
        pub response_parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of templates used to transform the response body.
        #[builder(into, default)]
        pub response_templates: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Response type of the associated GatewayResponse.
        #[builder(into)]
        pub response_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// String identifier of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// HTTP status code of the Gateway Response.
        #[builder(into, default)]
        pub status_code: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResponseResult {
        /// Map of parameters (paths, query strings and headers) of the Gateway Response.
        pub response_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of templates used to transform the response body.
        pub response_templates: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Response type of the associated GatewayResponse.
        pub response_type: pulumi_wasm_rust::Output<String>,
        /// String identifier of the associated REST API.
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
        /// HTTP status code of the Gateway Response.
        pub status_code: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResponseArgs,
    ) -> ResponseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let response_parameters_binding = args
            .response_parameters
            .get_output(context)
            .get_inner();
        let response_templates_binding = args
            .response_templates
            .get_output(context)
            .get_inner();
        let response_type_binding = args.response_type.get_output(context).get_inner();
        let rest_api_id_binding = args.rest_api_id.get_output(context).get_inner();
        let status_code_binding = args.status_code.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/response:Response".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "responseParameters".into(),
                    value: &response_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "responseTemplates".into(),
                    value: &response_templates_binding,
                },
                register_interface::ObjectField {
                    name: "responseType".into(),
                    value: &response_type_binding,
                },
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "statusCode".into(),
                    value: &status_code_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "responseParameters".into(),
                },
                register_interface::ResultField {
                    name: "responseTemplates".into(),
                },
                register_interface::ResultField {
                    name: "responseType".into(),
                },
                register_interface::ResultField {
                    name: "restApiId".into(),
                },
                register_interface::ResultField {
                    name: "statusCode".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResponseResult {
            response_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseParameters").unwrap(),
            ),
            response_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseTemplates").unwrap(),
            ),
            response_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseType").unwrap(),
            ),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApiId").unwrap(),
            ),
            status_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusCode").unwrap(),
            ),
        }
    }
}
