pub mod get_api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApiArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetApiResult {
        /// URI of the API, of the form `https://{api-id}.execute-api.{region}.amazonaws.com` for HTTP APIs and `wss://{api-id}.execute-api.{region}.amazonaws.com` for WebSocket APIs.
        pub api_endpoint: pulumi_wasm_rust::Output<String>,
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// An [API key selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions).
        /// Applicable for WebSocket APIs.
        pub api_key_selection_expression: pulumi_wasm_rust::Output<String>,
        /// ARN of the API.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Cross-origin resource sharing (CORS) [configuration](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html).
        /// Applicable for HTTP APIs.
        pub cors_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::apigatewayv2::GetApiCorsConfiguration>,
        >,
        /// Description of the API.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether clients can invoke the API by using the default `execute-api` endpoint.
        pub disable_execute_api_endpoint: pulumi_wasm_rust::Output<bool>,
        /// ARN prefix to be used in an `aws.lambda.Permission`'s `source_arn` attribute
        /// or in an `aws.iam.Policy` to authorize access to the [`@connections` API](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-how-to-call-websocket-api-connections.html).
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-control-access-iam.html) for details.
        pub execution_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the API.
        pub name: pulumi_wasm_rust::Output<String>,
        /// API protocol.
        pub protocol_type: pulumi_wasm_rust::Output<String>,
        /// The [route selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-route-selection-expressions) for the API.
        pub route_selection_expression: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Version identifier for the API.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetApiArgs) -> GetApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigatewayv2/getApi:getApi".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "apiKeySelectionExpression".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "corsConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disableExecuteApiEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "executionArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protocolType".into(),
                },
                register_interface::ResultField {
                    name: "routeSelectionExpression".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApiResult {
            api_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiEndpoint").unwrap(),
            ),
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            api_key_selection_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKeySelectionExpression").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cors_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("corsConfigurations").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disable_execute_api_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableExecuteApiEndpoint").unwrap(),
            ),
            execution_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protocol_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocolType").unwrap(),
            ),
            route_selection_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeSelectionExpression").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
