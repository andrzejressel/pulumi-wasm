/// Manages an Amazon API Gateway Version 2 stage.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
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
///     let example = stage::create(
///         "example",
///         StageArgs::builder()
///             .api_id("${exampleAwsApigatewayv2Api.id}")
///             .name("example-stage")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_stage` using the API identifier and stage name. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/stage:Stage example aabbccddee/example-stage
/// ```
/// -> __Note:__ The API Gateway managed stage created as part of [_quick_create_](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-basic-concept.html#apigateway-definition-quick-create) cannot be imported.
///
pub mod stage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StageArgs {
        /// Settings for logging access in this stage.
        /// Use the `aws.apigateway.Account` resource to configure [permissions for CloudWatch Logging](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-logging.html#set-up-access-logging-permissions).
        #[builder(into, default)]
        pub access_log_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::StageAccessLogSettings>,
        >,
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Whether updates to an API automatically trigger a new deployment. Defaults to `false`. Applicable for HTTP APIs.
        #[builder(into, default)]
        pub auto_deploy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifier of a client certificate for the stage. Use the `aws.apigateway.ClientCertificate` resource to configure a client certificate.
        /// Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub client_certificate_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Default route settings for the stage.
        #[builder(into, default)]
        pub default_route_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::StageDefaultRouteSettings>,
        >,
        /// Deployment identifier of the stage. Use the `aws.apigatewayv2.Deployment` resource to configure a deployment.
        #[builder(into, default)]
        pub deployment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Description for the stage. Must be less than or equal to 1024 characters in length.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the stage. Must be between 1 and 128 characters in length.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Route settings for the stage.
        #[builder(into, default)]
        pub route_settings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apigatewayv2::StageRouteSetting>>,
        >,
        /// Map that defines the stage variables for the stage.
        #[builder(into, default)]
        pub stage_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags to assign to the stage. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StageResult {
        /// Settings for logging access in this stage.
        /// Use the `aws.apigateway.Account` resource to configure [permissions for CloudWatch Logging](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-logging.html#set-up-access-logging-permissions).
        pub access_log_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::StageAccessLogSettings>,
        >,
        /// API identifier.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the stage.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether updates to an API automatically trigger a new deployment. Defaults to `false`. Applicable for HTTP APIs.
        pub auto_deploy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifier of a client certificate for the stage. Use the `aws.apigateway.ClientCertificate` resource to configure a client certificate.
        /// Supported only for WebSocket APIs.
        pub client_certificate_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Default route settings for the stage.
        pub default_route_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::StageDefaultRouteSettings>,
        >,
        /// Deployment identifier of the stage. Use the `aws.apigatewayv2.Deployment` resource to configure a deployment.
        pub deployment_id: pulumi_wasm_rust::Output<String>,
        /// Description for the stage. Must be less than or equal to 1024 characters in length.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN prefix to be used in an `aws.lambda.Permission`'s `source_arn` attribute.
        /// For WebSocket APIs this attribute can additionally be used in an `aws.iam.Policy` to authorize access to the [`@connections` API](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-how-to-call-websocket-api-connections.html).
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-control-access-iam.html) for details.
        pub execution_arn: pulumi_wasm_rust::Output<String>,
        /// URL to invoke the API pointing to the stage,
        /// e.g., `wss://z4675bid1j.execute-api.eu-west-2.amazonaws.com/example-stage`, or `https://z4675bid1j.execute-api.eu-west-2.amazonaws.com/`
        pub invoke_url: pulumi_wasm_rust::Output<String>,
        /// Name of the stage. Must be between 1 and 128 characters in length.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Route settings for the stage.
        pub route_settings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apigatewayv2::StageRouteSetting>>,
        >,
        /// Map that defines the stage variables for the stage.
        pub stage_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags to assign to the stage. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StageArgs) -> StageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_log_settings_binding = args.access_log_settings.get_inner();
        let api_id_binding = args.api_id.get_inner();
        let auto_deploy_binding = args.auto_deploy.get_inner();
        let client_certificate_id_binding = args.client_certificate_id.get_inner();
        let default_route_settings_binding = args.default_route_settings.get_inner();
        let deployment_id_binding = args.deployment_id.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let route_settings_binding = args.route_settings.get_inner();
        let stage_variables_binding = args.stage_variables.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/stage:Stage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessLogSettings".into(),
                    value: &access_log_settings_binding,
                },
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "autoDeploy".into(),
                    value: &auto_deploy_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificateId".into(),
                    value: &client_certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "defaultRouteSettings".into(),
                    value: &default_route_settings_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentId".into(),
                    value: &deployment_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "routeSettings".into(),
                    value: &route_settings_binding,
                },
                register_interface::ObjectField {
                    name: "stageVariables".into(),
                    value: &stage_variables_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessLogSettings".into(),
                },
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoDeploy".into(),
                },
                register_interface::ResultField {
                    name: "clientCertificateId".into(),
                },
                register_interface::ResultField {
                    name: "defaultRouteSettings".into(),
                },
                register_interface::ResultField {
                    name: "deploymentId".into(),
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "routeSettings".into(),
                },
                register_interface::ResultField {
                    name: "stageVariables".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StageResult {
            access_log_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessLogSettings").unwrap(),
            ),
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_deploy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoDeploy").unwrap(),
            ),
            client_certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertificateId").unwrap(),
            ),
            default_route_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRouteSettings").unwrap(),
            ),
            deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentId").unwrap(),
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
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            route_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeSettings").unwrap(),
            ),
            stage_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stageVariables").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
