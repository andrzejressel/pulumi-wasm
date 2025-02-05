pub mod get_authorizer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizerArgs {
        /// Authorizer identifier.
        #[builder(into)]
        pub authorizer_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAuthorizerArgs,
    ) -> GetAuthorizerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorizer_id_binding = args.authorizer_id.get_output(context).get_inner();
        let rest_api_id_binding = args.rest_api_id.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAuthorizerResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            authorizer_credentials: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizerCredentials"),
            ),
            authorizer_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizerId"),
            ),
            authorizer_result_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizerResultTtlInSeconds"),
            ),
            authorizer_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizerUri"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identity_source: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identitySource"),
            ),
            identity_validation_expression: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identityValidationExpression"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            provider_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("providerArns"),
            ),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restApiId"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
