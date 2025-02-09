/// Provides an API Gateway REST API Policy.
///
/// > **Note:** Amazon API Gateway Version 1 resources are used for creating and deploying REST APIs. To create and deploy WebSocket and HTTP APIs, use Amazon API Gateway Version 2 resources.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   testRestApi:
///     type: aws:apigateway:RestApi
///     name: test
///     properties:
///       name: example-rest-api
///   testRestApiPolicy:
///     type: aws:apigateway:RestApiPolicy
///     name: test
///     properties:
///       restApiId: ${testRestApi.id}
///       policy: ${test.json}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             actions:
///               - execute-api:Invoke
///             resources:
///               - ${testRestApi.executionArn}
///             conditions:
///               - test: IpAddress
///                 variable: aws:SourceIp
///                 values:
///                   - 123.123.123.123/32
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_rest_api_policy` using the REST API ID. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/restApiPolicy:RestApiPolicy example 12345abcde
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rest_api_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestApiPolicyArgs {
        /// JSON formatted policy document that controls access to the API Gateway.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RestApiPolicyResult {
        /// JSON formatted policy document that controls access to the API Gateway.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// ID of the REST API.
        pub rest_api_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RestApiPolicyArgs,
    ) -> RestApiPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let rest_api_id_binding = args.rest_api_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/restApiPolicy:RestApiPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApiId".into(),
                    value: rest_api_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RestApiPolicyResult {
            policy: o.get_field("policy"),
            rest_api_id: o.get_field("restApiId"),
        }
    }
}
