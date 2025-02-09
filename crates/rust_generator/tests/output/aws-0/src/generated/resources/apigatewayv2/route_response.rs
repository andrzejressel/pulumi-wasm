/// Manages an Amazon API Gateway Version 2 route response.
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
///     let example = route_response::create(
///         "example",
///         RouteResponseArgs::builder()
///             .api_id("${exampleAwsApigatewayv2Api.id}")
///             .route_id("${exampleAwsApigatewayv2Route.id}")
///             .route_response_key("$default")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Enabling Two-Way Communication
///
/// For websocket routes that require two-way communication enabled, an `aws.apigatewayv2.RouteResponse` needs to be added to the route with `route_response_key = "$default"`. More information available  is available in [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
///
/// You can only define the $default route response for WebSocket APIs. You can use an integration response to manipulate the response from a backend service. For more information, see [Overview of integration responses](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-integration-responses.html#apigateway-websocket-api-integration-response-overview).
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_route_response` using the API identifier, route identifier and route response identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/routeResponse:RouteResponse example aabbccddee/1122334/998877
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_response {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteResponseArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The [model selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-model-selection-expressions) for the route response.
        #[builder(into, default)]
        pub model_selection_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Response models for the route response.
        #[builder(into, default)]
        pub response_models: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the `aws.apigatewayv2.Route`.
        #[builder(into)]
        pub route_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Route response key.
        #[builder(into)]
        pub route_response_key: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResponseResult {
        /// API identifier.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// The [model selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-model-selection-expressions) for the route response.
        pub model_selection_expression: pulumi_gestalt_rust::Output<Option<String>>,
        /// Response models for the route response.
        pub response_models: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the `aws.apigatewayv2.Route`.
        pub route_id: pulumi_gestalt_rust::Output<String>,
        /// Route response key.
        pub route_response_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteResponseArgs,
    ) -> RouteResponseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let model_selection_expression_binding = args
            .model_selection_expression
            .get_output(context);
        let response_models_binding = args.response_models.get_output(context);
        let route_id_binding = args.route_id.get_output(context);
        let route_response_key_binding = args.route_response_key.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigatewayv2/routeResponse:RouteResponse".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modelSelectionExpression".into(),
                    value: model_selection_expression_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "responseModels".into(),
                    value: response_models_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeId".into(),
                    value: route_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeResponseKey".into(),
                    value: route_response_key_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteResponseResult {
            api_id: o.get_field("apiId"),
            model_selection_expression: o.get_field("modelSelectionExpression"),
            response_models: o.get_field("responseModels"),
            route_id: o.get_field("routeId"),
            route_response_key: o.get_field("routeResponseKey"),
        }
    }
}
