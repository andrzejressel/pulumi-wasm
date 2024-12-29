/// Manages an API Gateway Stage. A stage is a named reference to a deployment, which can be done via the `aws.apigateway.Deployment` resource. Stages can be optionally managed further with the `aws.apigateway.BasePathMapping` resource, `aws.apigateway.DomainName` resource, and `aws_api_method_settings` resource. For more information, see the [API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-stages.html).
///
/// ### Managing the API Logging CloudWatch Log Group
///
/// API Gateway provides the ability to [enable CloudWatch API logging](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-logging.html). To manage the CloudWatch Log Group when this feature is enabled, the `aws.cloudwatch.LogGroup` resource can be used where the name matches the API Gateway naming convention. If the CloudWatch Log Group previously exists, import the `aws.cloudwatch.LogGroup` resource into Pulumi as a one time operation. You can recreate the environment without import.
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = rest_api::create("example", RestApiArgs::builder().build_struct());
///     let exampleLogGroup = log_group::create(
///         "exampleLogGroup",
///         LogGroupArgs::builder()
///             .name("API-Gateway-Execution-Logs_${example.id}/${stageName}")
///             .retention_in_days(7)
///             .build_struct(),
///     );
///     let exampleStage = stage::create(
///         "exampleStage",
///         StageArgs::builder().stage_name("${stageName}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_stage` using `REST-API-ID/STAGE-NAME`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/stage:Stage example 12345abcde/example
/// ```
pub mod stage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StageArgs {
        /// Enables access logs for the API stage. See Access Log Settings below.
        #[builder(into, default)]
        pub access_log_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::StageAccessLogSettings>,
        >,
        /// Whether a cache cluster is enabled for the stage
        #[builder(into, default)]
        pub cache_cluster_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Size of the cache cluster for the stage, if enabled. Allowed values include `0.5`, `1.6`, `6.1`, `13.5`, `28.4`, `58.2`, `118` and `237`.
        #[builder(into, default)]
        pub cache_cluster_size: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration settings of a canary deployment. See Canary Settings below.
        #[builder(into, default)]
        pub canary_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::StageCanarySettings>,
        >,
        /// Identifier of a client certificate for the stage.
        #[builder(into, default)]
        pub client_certificate_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the deployment that the stage points to
        #[builder(into)]
        pub deployment: pulumi_wasm_rust::Output<String>,
        /// Description of the stage.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the associated API documentation
        #[builder(into, default)]
        pub documentation_version: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Name of the stage
        #[builder(into)]
        pub stage_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map that defines the stage variables
        #[builder(into, default)]
        pub variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether active tracing with X-ray is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub xray_tracing_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct StageResult {
        /// Enables access logs for the API stage. See Access Log Settings below.
        pub access_log_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::StageAccessLogSettings>,
        >,
        /// ARN
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether a cache cluster is enabled for the stage
        pub cache_cluster_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Size of the cache cluster for the stage, if enabled. Allowed values include `0.5`, `1.6`, `6.1`, `13.5`, `28.4`, `58.2`, `118` and `237`.
        pub cache_cluster_size: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration settings of a canary deployment. See Canary Settings below.
        pub canary_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::StageCanarySettings>,
        >,
        /// Identifier of a client certificate for the stage.
        pub client_certificate_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the deployment that the stage points to
        pub deployment: pulumi_wasm_rust::Output<String>,
        /// Description of the stage.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the associated API documentation
        pub documentation_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Execution ARN to be used in `lambda_permission`'s `source_arn`
        /// when allowing API Gateway to invoke a Lambda function,
        /// e.g., `arn:aws:execute-api:eu-west-2:123456789012:z4675bid1j/prod`
        pub execution_arn: pulumi_wasm_rust::Output<String>,
        /// URL to invoke the API pointing to the stage,
        /// e.g., `https://z4675bid1j.execute-api.eu-west-2.amazonaws.com/prod`
        pub invoke_url: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Name of the stage
        pub stage_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map that defines the stage variables
        pub variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN of the WebAcl associated with the Stage.
        pub web_acl_arn: pulumi_wasm_rust::Output<String>,
        /// Whether active tracing with X-ray is enabled. Defaults to `false`.
        pub xray_tracing_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StageArgs) -> StageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_log_settings_binding = args.access_log_settings.get_inner();
        let cache_cluster_enabled_binding = args.cache_cluster_enabled.get_inner();
        let cache_cluster_size_binding = args.cache_cluster_size.get_inner();
        let canary_settings_binding = args.canary_settings.get_inner();
        let client_certificate_id_binding = args.client_certificate_id.get_inner();
        let deployment_binding = args.deployment.get_inner();
        let description_binding = args.description.get_inner();
        let documentation_version_binding = args.documentation_version.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let stage_name_binding = args.stage_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let variables_binding = args.variables.get_inner();
        let xray_tracing_enabled_binding = args.xray_tracing_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/stage:Stage".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessLogSettings".into(),
                    value: &access_log_settings_binding,
                },
                register_interface::ObjectField {
                    name: "cacheClusterEnabled".into(),
                    value: &cache_cluster_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "cacheClusterSize".into(),
                    value: &cache_cluster_size_binding,
                },
                register_interface::ObjectField {
                    name: "canarySettings".into(),
                    value: &canary_settings_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificateId".into(),
                    value: &client_certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "deployment".into(),
                    value: &deployment_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "documentationVersion".into(),
                    value: &documentation_version_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "variables".into(),
                    value: &variables_binding,
                },
                register_interface::ObjectField {
                    name: "xrayTracingEnabled".into(),
                    value: &xray_tracing_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessLogSettings".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cacheClusterEnabled".into(),
                },
                register_interface::ResultField {
                    name: "cacheClusterSize".into(),
                },
                register_interface::ResultField {
                    name: "canarySettings".into(),
                },
                register_interface::ResultField {
                    name: "clientCertificateId".into(),
                },
                register_interface::ResultField {
                    name: "deployment".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "documentationVersion".into(),
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
                    name: "stageName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "variables".into(),
                },
                register_interface::ResultField {
                    name: "webAclArn".into(),
                },
                register_interface::ResultField {
                    name: "xrayTracingEnabled".into(),
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
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cache_cluster_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheClusterEnabled").unwrap(),
            ),
            cache_cluster_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheClusterSize").unwrap(),
            ),
            canary_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("canarySettings").unwrap(),
            ),
            client_certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertificateId").unwrap(),
            ),
            deployment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deployment").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            documentation_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentationVersion").unwrap(),
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
            stage_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stageName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("variables").unwrap(),
            ),
            web_acl_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webAclArn").unwrap(),
            ),
            xray_tracing_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xrayTracingEnabled").unwrap(),
            ),
        }
    }
}
