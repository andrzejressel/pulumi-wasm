/// Provides an API Gateway REST API Policy.
///
/// > **Note:** Amazon API Gateway Version 1 resources are used for creating and deploying REST APIs. To create and deploy WebSocket and HTTP APIs, use Amazon API Gateway Version 2 resources.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["execute-api:Invoke",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("IpAddress").values(vec!["123.123.123.123/32",])
///                     .variable("aws:SourceIp").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("AWS").build_struct(),])
///                     .resources(vec!["${testRestApi.executionArn}",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let testRestApi = rest_api::create(
///         "testRestApi",
///         RestApiArgs::builder().name("example-rest-api").build_struct(),
///     );
///     let testRestApiPolicy = rest_api_policy::create(
///         "testRestApiPolicy",
///         RestApiPolicyArgs::builder()
///             .policy("${test.json}")
///             .rest_api_id("${testRestApi.id}")
///             .build_struct(),
///     );
/// }
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestApiPolicyArgs {
        /// JSON formatted policy document that controls access to the API Gateway.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// ID of the REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: RestApiPolicyArgs) -> RestApiPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let rest_api_id_binding = args.rest_api_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/restApiPolicy:RestApiPolicy".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "restApiId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RestApiPolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApiId").unwrap(),
            ),
        }
    }
}