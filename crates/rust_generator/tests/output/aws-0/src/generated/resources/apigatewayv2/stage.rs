/// Manages an Amazon API Gateway Version 2 stage.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StageArgs {
        /// Settings for logging access in this stage.
        /// Use the `aws.apigateway.Account` resource to configure [permissions for CloudWatch Logging](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-logging.html#set-up-access-logging-permissions).
        #[builder(into, default)]
        pub access_log_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigatewayv2::StageAccessLogSettings>,
        >,
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether updates to an API automatically trigger a new deployment. Defaults to `false`. Applicable for HTTP APIs.
        #[builder(into, default)]
        pub auto_deploy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Identifier of a client certificate for the stage. Use the `aws.apigateway.ClientCertificate` resource to configure a client certificate.
        /// Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub client_certificate_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Default route settings for the stage.
        #[builder(into, default)]
        pub default_route_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigatewayv2::StageDefaultRouteSettings>,
        >,
        /// Deployment identifier of the stage. Use the `aws.apigatewayv2.Deployment` resource to configure a deployment.
        #[builder(into, default)]
        pub deployment_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description for the stage. Must be less than or equal to 1024 characters in length.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the stage. Must be between 1 and 128 characters in length.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Route settings for the stage.
        #[builder(into, default)]
        pub route_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apigatewayv2::StageRouteSetting>>,
        >,
        /// Map that defines the stage variables for the stage.
        #[builder(into, default)]
        pub stage_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags to assign to the stage. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StageResult {
        /// Settings for logging access in this stage.
        /// Use the `aws.apigateway.Account` resource to configure [permissions for CloudWatch Logging](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-logging.html#set-up-access-logging-permissions).
        pub access_log_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigatewayv2::StageAccessLogSettings>,
        >,
        /// API identifier.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the stage.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether updates to an API automatically trigger a new deployment. Defaults to `false`. Applicable for HTTP APIs.
        pub auto_deploy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Identifier of a client certificate for the stage. Use the `aws.apigateway.ClientCertificate` resource to configure a client certificate.
        /// Supported only for WebSocket APIs.
        pub client_certificate_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Default route settings for the stage.
        pub default_route_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigatewayv2::StageDefaultRouteSettings>,
        >,
        /// Deployment identifier of the stage. Use the `aws.apigatewayv2.Deployment` resource to configure a deployment.
        pub deployment_id: pulumi_gestalt_rust::Output<String>,
        /// Description for the stage. Must be less than or equal to 1024 characters in length.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN prefix to be used in an `aws.lambda.Permission`'s `source_arn` attribute.
        /// For WebSocket APIs this attribute can additionally be used in an `aws.iam.Policy` to authorize access to the [`@connections` API](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-how-to-call-websocket-api-connections.html).
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-control-access-iam.html) for details.
        pub execution_arn: pulumi_gestalt_rust::Output<String>,
        /// URL to invoke the API pointing to the stage,
        /// e.g., `wss://z4675bid1j.execute-api.eu-west-2.amazonaws.com/example-stage`, or `https://z4675bid1j.execute-api.eu-west-2.amazonaws.com/`
        pub invoke_url: pulumi_gestalt_rust::Output<String>,
        /// Name of the stage. Must be between 1 and 128 characters in length.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Route settings for the stage.
        pub route_settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apigatewayv2::StageRouteSetting>>,
        >,
        /// Map that defines the stage variables for the stage.
        pub stage_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags to assign to the stage. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StageArgs,
    ) -> StageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_log_settings_binding_1 = args.access_log_settings.get_output(context);
        let access_log_settings_binding = access_log_settings_binding_1.get_inner();
        let api_id_binding_1 = args.api_id.get_output(context);
        let api_id_binding = api_id_binding_1.get_inner();
        let auto_deploy_binding_1 = args.auto_deploy.get_output(context);
        let auto_deploy_binding = auto_deploy_binding_1.get_inner();
        let client_certificate_id_binding_1 = args
            .client_certificate_id
            .get_output(context);
        let client_certificate_id_binding = client_certificate_id_binding_1.get_inner();
        let default_route_settings_binding_1 = args
            .default_route_settings
            .get_output(context);
        let default_route_settings_binding = default_route_settings_binding_1
            .get_inner();
        let deployment_id_binding_1 = args.deployment_id.get_output(context);
        let deployment_id_binding = deployment_id_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let route_settings_binding_1 = args.route_settings.get_output(context);
        let route_settings_binding = route_settings_binding_1.get_inner();
        let stage_variables_binding_1 = args.stage_variables.get_output(context);
        let stage_variables_binding = stage_variables_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        StageResult {
            access_log_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessLogSettings"),
            ),
            api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiId"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_deploy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoDeploy"),
            ),
            client_certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientCertificateId"),
            ),
            default_route_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultRouteSettings"),
            ),
            deployment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            execution_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("executionArn"),
            ),
            invoke_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invokeUrl"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            route_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routeSettings"),
            ),
            stage_variables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stageVariables"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
