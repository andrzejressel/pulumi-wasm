#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_authorizer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizerArgs {
        /// Authorizer identifier.
        #[builder(into)]
        pub authorizer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorizerResult {
        /// ARN of the API Gateway Authorizer.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Credentials required for the authorizer.
        pub authorizer_credentials: pulumi_gestalt_rust::Output<String>,
        pub authorizer_id: pulumi_gestalt_rust::Output<String>,
        /// TTL of cached authorizer results in seconds.
        pub authorizer_result_ttl_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// Authorizer's Uniform Resource Identifier (URI).
        pub authorizer_uri: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Source of the identity in an incoming request.
        pub identity_source: pulumi_gestalt_rust::Output<String>,
        /// Validation expression for the incoming identity.
        pub identity_validation_expression: pulumi_gestalt_rust::Output<String>,
        /// Name of the authorizer.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of the Amazon Cognito user pool ARNs.
        pub provider_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub rest_api_id: pulumi_gestalt_rust::Output<String>,
        /// Type of the authorizer.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAuthorizerArgs,
    ) -> GetAuthorizerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorizer_id_binding = args.authorizer_id.get_output(context);
        let rest_api_id_binding = args.rest_api_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigateway/getAuthorizer:getAuthorizer".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizerId".into(),
                    value: authorizer_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApiId".into(),
                    value: rest_api_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAuthorizerResult {
            arn: o.get_field("arn"),
            authorizer_credentials: o.get_field("authorizerCredentials"),
            authorizer_id: o.get_field("authorizerId"),
            authorizer_result_ttl_in_seconds: o
                .get_field("authorizerResultTtlInSeconds"),
            authorizer_uri: o.get_field("authorizerUri"),
            id: o.get_field("id"),
            identity_source: o.get_field("identitySource"),
            identity_validation_expression: o.get_field("identityValidationExpression"),
            name: o.get_field("name"),
            provider_arns: o.get_field("providerArns"),
            rest_api_id: o.get_field("restApiId"),
            type_: o.get_field("type"),
        }
    }
}
