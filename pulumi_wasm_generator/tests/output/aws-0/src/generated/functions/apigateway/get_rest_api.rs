pub mod get_rest_api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRestApiArgs {
        /// Name of the REST API to look up. If no REST API is found with this name, an error will be returned. If multiple REST APIs are found with this name, an error will be returned.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRestApiResult {
        /// Source of the API key for requests.
        pub api_key_source: pulumi_wasm_rust::Output<String>,
        /// ARN of the REST API.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of binary media types supported by the REST API.
        pub binary_media_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// Description of the REST API.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The endpoint configuration of this RestApi showing the endpoint types of the API.
        pub endpoint_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::apigateway::GetRestApiEndpointConfiguration>,
        >,
        /// Execution ARN part to be used in `lambda_permission`'s `source_arn` when allowing API Gateway to invoke a Lambda function, e.g., `arn:aws:execute-api:eu-west-2:123456789012:z4675bid1j`, which can be concatenated with allowed stage, method and resource path.
        pub execution_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Minimum response size to compress for the REST API.
        pub minimum_compression_size: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// JSON formatted policy document that controls access to the API Gateway.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Set to the ID of the API Gateway Resource on the found REST API where the route matches '/'.
        pub root_resource_id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRestApiArgs) -> GetRestApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getRestApi:getRestApi".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiKeySource".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "binaryMediaTypes".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "endpointConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "executionArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "minimumCompressionSize".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "rootResourceId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRestApiResult {
            api_key_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKeySource").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            binary_media_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("binaryMediaTypes").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            endpoint_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointConfigurations").unwrap(),
            ),
            execution_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            minimum_compression_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumCompressionSize").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            root_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootResourceId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
