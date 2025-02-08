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
#[allow(clippy::doc_lazy_continuation)]
pub mod response {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResponseArgs {
        /// Map of parameters (paths, query strings and headers) of the Gateway Response.
        #[builder(into, default)]
        pub response_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of templates used to transform the response body.
        #[builder(into, default)]
        pub response_templates: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Response type of the associated GatewayResponse.
        #[builder(into)]
        pub response_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// String identifier of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// HTTP status code of the Gateway Response.
        #[builder(into, default)]
        pub status_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResponseResult {
        /// Map of parameters (paths, query strings and headers) of the Gateway Response.
        pub response_parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of templates used to transform the response body.
        pub response_templates: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Response type of the associated GatewayResponse.
        pub response_type: pulumi_gestalt_rust::Output<String>,
        /// String identifier of the associated REST API.
        pub rest_api_id: pulumi_gestalt_rust::Output<String>,
        /// HTTP status code of the Gateway Response.
        pub status_code: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResponseArgs,
    ) -> ResponseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResponseResult {
            response_parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("responseParameters"),
            ),
            response_templates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("responseTemplates"),
            ),
            response_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("responseType"),
            ),
            rest_api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restApiId"),
            ),
            status_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusCode"),
            ),
        }
    }
}
