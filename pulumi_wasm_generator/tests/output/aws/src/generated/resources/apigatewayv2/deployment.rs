/// Manages an Amazon API Gateway Version 2 deployment.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
///
/// > **Note:** Creating a deployment for an API requires at least one `aws.apigatewayv2.Route` resource associated with that API. To avoid race conditions when all resources are being created together, you need to add implicit resource references via the `triggers` argument or explicit resource references using the [resource `dependsOn` meta-argument](https://www.pulumi.com/docs/intro/concepts/programming-model/#dependson).
///
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_deployment` using the API identifier and deployment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/deployment:Deployment example aabbccddee/1122334
/// ```
/// The `triggers` argument cannot be imported.
///
pub mod deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Description for the deployment resource. Must be less than or equal to 1024 characters in length.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a redeployment.
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// API identifier.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Whether the deployment was automatically released.
        pub auto_deployed: pulumi_wasm_rust::Output<bool>,
        /// Description for the deployment resource. Must be less than or equal to 1024 characters in length.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a redeployment.
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeploymentArgs) -> DeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let description_binding = args.description.get_inner();
        let triggers_binding = args.triggers.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/deployment:Deployment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "autoDeployed".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeploymentResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            auto_deployed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoDeployed").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
        }
    }
}
