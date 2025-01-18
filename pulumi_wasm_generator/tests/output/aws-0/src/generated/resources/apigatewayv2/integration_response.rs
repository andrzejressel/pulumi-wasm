/// Manages an Amazon API Gateway Version 2 integration response.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = integration_response::create(
///         "example",
///         IntegrationResponseArgs::builder()
///             .api_id("${exampleAwsApigatewayv2Api.id}")
///             .integration_id("${exampleAwsApigatewayv2Integration.id}")
///             .integration_response_key("/200/")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_integration_response` using the API identifier, integration identifier and integration response identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/integrationResponse:IntegrationResponse example aabbccddee/1122334/998877
/// ```
pub mod integration_response {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationResponseArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// How to handle response payload content type conversions. Valid values: `CONVERT_TO_BINARY`, `CONVERT_TO_TEXT`.
        #[builder(into, default)]
        pub content_handling_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the `aws.apigatewayv2.Integration`.
        #[builder(into)]
        pub integration_id: pulumi_wasm_rust::Output<String>,
        /// Integration response key.
        #[builder(into)]
        pub integration_response_key: pulumi_wasm_rust::Output<String>,
        /// Map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client.
        #[builder(into, default)]
        pub response_templates: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The [template selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-template-selection-expressions) for the integration response.
        #[builder(into, default)]
        pub template_selection_expression: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IntegrationResponseResult {
        /// API identifier.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// How to handle response payload content type conversions. Valid values: `CONVERT_TO_BINARY`, `CONVERT_TO_TEXT`.
        pub content_handling_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the `aws.apigatewayv2.Integration`.
        pub integration_id: pulumi_wasm_rust::Output<String>,
        /// Integration response key.
        pub integration_response_key: pulumi_wasm_rust::Output<String>,
        /// Map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client.
        pub response_templates: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The [template selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-template-selection-expressions) for the integration response.
        pub template_selection_expression: pulumi_wasm_rust::Output<Option<String>>,
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
        let api_id_binding = args.api_id.get_inner();
        let content_handling_strategy_binding = args
            .content_handling_strategy
            .get_inner();
        let integration_id_binding = args.integration_id.get_inner();
        let integration_response_key_binding = args.integration_response_key.get_inner();
        let response_templates_binding = args.response_templates.get_inner();
        let template_selection_expression_binding = args
            .template_selection_expression
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/integrationResponse:IntegrationResponse".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "contentHandlingStrategy".into(),
                    value: &content_handling_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "integrationId".into(),
                    value: &integration_id_binding,
                },
                register_interface::ObjectField {
                    name: "integrationResponseKey".into(),
                    value: &integration_response_key_binding,
                },
                register_interface::ObjectField {
                    name: "responseTemplates".into(),
                    value: &response_templates_binding,
                },
                register_interface::ObjectField {
                    name: "templateSelectionExpression".into(),
                    value: &template_selection_expression_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "contentHandlingStrategy".into(),
                },
                register_interface::ResultField {
                    name: "integrationId".into(),
                },
                register_interface::ResultField {
                    name: "integrationResponseKey".into(),
                },
                register_interface::ResultField {
                    name: "responseTemplates".into(),
                },
                register_interface::ResultField {
                    name: "templateSelectionExpression".into(),
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
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            content_handling_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentHandlingStrategy").unwrap(),
            ),
            integration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationId").unwrap(),
            ),
            integration_response_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationResponseKey").unwrap(),
            ),
            response_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseTemplates").unwrap(),
            ),
            template_selection_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateSelectionExpression").unwrap(),
            ),
        }
    }
}
