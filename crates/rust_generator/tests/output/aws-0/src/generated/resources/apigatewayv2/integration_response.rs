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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationResponseArgs,
    ) -> IntegrationResponseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let content_handling_strategy_binding = args
            .content_handling_strategy
            .get_output(context);
        let integration_id_binding = args.integration_id.get_output(context);
        let integration_response_key_binding = args
            .integration_response_key
            .get_output(context);
        let response_templates_binding = args.response_templates.get_output(context);
        let template_selection_expression_binding = args
            .template_selection_expression
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigatewayv2/integrationResponse:IntegrationResponse".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentHandlingStrategy".into(),
                    value: content_handling_strategy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationId".into(),
                    value: integration_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationResponseKey".into(),
                    value: integration_response_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "responseTemplates".into(),
                    value: response_templates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateSelectionExpression".into(),
                    value: template_selection_expression_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationResponseResult {
            api_id: o.get_field("apiId"),
            content_handling_strategy: o.get_field("contentHandlingStrategy"),
            integration_id: o.get_field("integrationId"),
            integration_response_key: o.get_field("integrationResponseKey"),
            response_templates: o.get_field("responseTemplates"),
            template_selection_expression: o.get_field("templateSelectionExpression"),
        }
    }
}
