pub mod get_authorizer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizerArgs {
        /// Authorizer identifier.
        #[builder(into)]
        pub authorizer_id: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorizerResult {
        /// ARN of the API Gateway Authorizer.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Credentials required for the authorizer.
        pub authorizer_credentials: pulumi_wasm_rust::Output<String>,
        pub authorizer_id: pulumi_wasm_rust::Output<String>,
        /// TTL of cached authorizer results in seconds.
        pub authorizer_result_ttl_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// Authorizer's Uniform Resource Identifier (URI).
        pub authorizer_uri: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Source of the identity in an incoming request.
        pub identity_source: pulumi_wasm_rust::Output<String>,
        /// Validation expression for the incoming identity.
        pub identity_validation_expression: pulumi_wasm_rust::Output<String>,
        /// Name of the authorizer.
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of the Amazon Cognito user pool ARNs.
        pub provider_arns: pulumi_wasm_rust::Output<Vec<String>>,
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
        /// Type of the authorizer.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAuthorizerArgs) -> GetAuthorizerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorizer_id_binding = args.authorizer_id.get_inner();
        let rest_api_id_binding = args.rest_api_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getAuthorizer:getAuthorizer".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizerId".into(),
                    value: &authorizer_id_binding,
                },
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authorizerCredentials".into(),
                },
                register_interface::ResultField {
                    name: "authorizerId".into(),
                },
                register_interface::ResultField {
                    name: "authorizerResultTtlInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "authorizerUri".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identitySource".into(),
                },
                register_interface::ResultField {
                    name: "identityValidationExpression".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "providerArns".into(),
                },
                register_interface::ResultField {
                    name: "restApiId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAuthorizerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authorizer_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerCredentials").unwrap(),
            ),
            authorizer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerId").unwrap(),
            ),
            authorizer_result_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerResultTtlInSeconds").unwrap(),
            ),
            authorizer_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerUri").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identitySource").unwrap(),
            ),
            identity_validation_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityValidationExpression").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            provider_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerArns").unwrap(),
            ),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApiId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
