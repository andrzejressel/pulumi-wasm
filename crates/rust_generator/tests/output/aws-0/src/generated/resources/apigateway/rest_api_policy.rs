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
pub mod rest_api_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestApiPolicyArgs {
        /// JSON formatted policy document that controls access to the API Gateway.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RestApiPolicyResult {
        /// JSON formatted policy document that controls access to the API Gateway.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// ID of the REST API.
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RestApiPolicyArgs,
    ) -> RestApiPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_output(context).get_inner();
        let rest_api_id_binding = args.rest_api_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/restApiPolicy:RestApiPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RestApiPolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restApiId"),
            ),
        }
    }
}
