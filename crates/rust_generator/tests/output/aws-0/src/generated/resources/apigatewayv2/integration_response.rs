/// Manages an Amazon API Gateway Version 2 integration response.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_response {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationResponseArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// How to handle response payload content type conversions. Valid values: `CONVERT_TO_BINARY`, `CONVERT_TO_TEXT`.
        #[builder(into, default)]
        pub content_handling_strategy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Identifier of the `aws.apigatewayv2.Integration`.
        #[builder(into)]
        pub integration_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Integration response key.
        #[builder(into)]
        pub integration_response_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client.
        #[builder(into, default)]
        pub response_templates: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The [template selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-template-selection-expressions) for the integration response.
        #[builder(into, default)]
        pub template_selection_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationResponseResult {
        /// API identifier.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// How to handle response payload content type conversions. Valid values: `CONVERT_TO_BINARY`, `CONVERT_TO_TEXT`.
        pub content_handling_strategy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the `aws.apigatewayv2.Integration`.
        pub integration_id: pulumi_gestalt_rust::Output<String>,
        /// Integration response key.
        pub integration_response_key: pulumi_gestalt_rust::Output<String>,
        /// Map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client.
        pub response_templates: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The [template selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-template-selection-expressions) for the integration response.
        pub template_selection_expression: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IntegrationResponseArgs,
    ) -> IntegrationResponseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_id_binding_1 = args.api_id.get_output(context);
        let api_id_binding = api_id_binding_1.get_inner();
        let content_handling_strategy_binding_1 = args
            .content_handling_strategy
            .get_output(context);
        let content_handling_strategy_binding = content_handling_strategy_binding_1
            .get_inner();
        let integration_id_binding_1 = args.integration_id.get_output(context);
        let integration_id_binding = integration_id_binding_1.get_inner();
        let integration_response_key_binding_1 = args
            .integration_response_key
            .get_output(context);
        let integration_response_key_binding = integration_response_key_binding_1
            .get_inner();
        let response_templates_binding_1 = args.response_templates.get_output(context);
        let response_templates_binding = response_templates_binding_1.get_inner();
        let template_selection_expression_binding_1 = args
            .template_selection_expression
            .get_output(context);
        let template_selection_expression_binding = template_selection_expression_binding_1
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        IntegrationResponseResult {
            api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiId"),
            ),
            content_handling_strategy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentHandlingStrategy"),
            ),
            integration_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationId"),
            ),
            integration_response_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationResponseKey"),
            ),
            response_templates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("responseTemplates"),
            ),
            template_selection_expression: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateSelectionExpression"),
            ),
        }
    }
}
