/// Manages an API Gateway REST Deployment. A deployment is a snapshot of the REST API configuration. The deployment can then be published to callable endpoints via the `aws.apigateway.Stage` resource and optionally managed further with the `aws.apigateway.BasePathMapping` resource, `aws.apigateway.DomainName` resource, and `aws_api_method_settings` resource. For more information, see the [API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-deploy-api.html).
///
/// To properly capture all REST API configuration in a deployment, this resource must have dependencies on all prior resources that manage resources/paths, methods, integrations, etc.
///
/// * For REST APIs that are configured via OpenAPI specification (`aws.apigateway.RestApi` resource `body` argument), no special dependency setup is needed beyond referencing the  `id` attribute of that resource unless additional resources have further customized the REST API.
/// * When the REST API configuration involves other resources (`aws.apigateway.Integration` resource), the dependency setup can be done with implicit resource references in the `triggers` argument or explicit resource references using the [resource `dependsOn` custom option](https://www.pulumi.com/docs/intro/concepts/resources/#dependson). The `triggers` argument should be preferred over `depends_on`, since `depends_on` can only capture dependency ordering and will not cause the resource to recreate (redeploy the REST API) with upstream configuration changes.
///
/// !> **WARNING:** It is recommended to use the `aws.apigateway.Stage` resource instead of managing an API Gateway Stage via the `stage_name` argument of this resource. When this resource is recreated (REST API redeployment) with the `stage_name` configured, the stage is deleted and recreated. This will cause a temporary service interruption, increase provide plan differences, and can require a second apply to recreate any downstream stage configuration such as associated `aws_api_method_settings` resources.
///
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_deployment` using `REST-API-ID/DEPLOYMENT-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/deployment:Deployment example aabbccddee/1122334
/// ```
/// The `stage_name`, `stage_description`, and `variables` arguments cannot be imported. Use the `aws_api_gateway_stage` resource to import and manage stages.
///
/// The `triggers` argument cannot be imported.
///
pub mod deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// Input configuration for the canary deployment when the deployment is a canary release deployment.
        /// See `canary_settings below.
        /// Has no effect when `stage_name` is not set.
        #[builder(into, default)]
        pub canary_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::DeploymentCanarySettings>,
        >,
        /// Description of the deployment
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// REST API identifier.
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Description to set on the stage managed by the `stage_name` argument.
        /// Has no effect when `stage_name` is not set.
        #[builder(into, default)]
        pub stage_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the stage to create with this deployment.
        /// If the specified stage already exists, it will be updated to point to the new deployment.
        /// We recommend using the `aws.apigateway.Stage` resource instead to manage stages.
        #[builder(into, default)]
        pub stage_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a redeployment.
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map to set on the stage managed by the `stage_name` argument.
        #[builder(into, default)]
        pub variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// Input configuration for the canary deployment when the deployment is a canary release deployment.
        /// See `canary_settings below.
        /// Has no effect when `stage_name` is not set.
        pub canary_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::DeploymentCanarySettings>,
        >,
        /// Creation date of the deployment
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Description of the deployment
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Execution ARN to be used in `lambda_permission`'s `source_arn`
        /// when allowing API Gateway to invoke a Lambda function,
        /// e.g., `arn:aws:execute-api:eu-west-2:123456789012:z4675bid1j/prod`
        pub execution_arn: pulumi_wasm_rust::Output<String>,
        /// URL to invoke the API pointing to the stage,
        /// e.g., `https://z4675bid1j.execute-api.eu-west-2.amazonaws.com/prod`
        pub invoke_url: pulumi_wasm_rust::Output<String>,
        /// REST API identifier.
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Description to set on the stage managed by the `stage_name` argument.
        /// Has no effect when `stage_name` is not set.
        pub stage_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the stage to create with this deployment.
        /// If the specified stage already exists, it will be updated to point to the new deployment.
        /// We recommend using the `aws.apigateway.Stage` resource instead to manage stages.
        pub stage_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a redeployment.
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map to set on the stage managed by the `stage_name` argument.
        pub variables: pulumi_wasm_rust::Output<
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
        let canary_settings_binding = args.canary_settings.get_inner();
        let description_binding = args.description.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let stage_description_binding = args.stage_description.get_inner();
        let stage_name_binding = args.stage_name.get_inner();
        let triggers_binding = args.triggers.get_inner();
        let variables_binding = args.variables.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/deployment:Deployment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "canarySettings".into(),
                    value: &canary_settings_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "stageDescription".into(),
                    value: &stage_description_binding,
                },
                register_interface::ObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
                register_interface::ObjectField {
                    name: "variables".into(),
                    value: &variables_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "canarySettings".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "executionArn".into(),
                },
                register_interface::ResultField {
                    name: "invokeUrl".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
                },
                register_interface::ResultField {
                    name: "stageDescription".into(),
                },
                register_interface::ResultField {
                    name: "stageName".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
                register_interface::ResultField {
                    name: "variables".into(),
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
            canary_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("canarySettings").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            execution_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionArn").unwrap(),
            ),
            invoke_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invokeUrl").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
            stage_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stageDescription").unwrap(),
            ),
            stage_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stageName").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
            variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("variables").unwrap(),
            ),
        }
    }
}
