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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StageArgs,
    ) -> StageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_log_settings_binding = args.access_log_settings.get_output(context);
        let api_id_binding = args.api_id.get_output(context);
        let auto_deploy_binding = args.auto_deploy.get_output(context);
        let client_certificate_id_binding = args
            .client_certificate_id
            .get_output(context);
        let default_route_settings_binding = args
            .default_route_settings
            .get_output(context);
        let deployment_id_binding = args.deployment_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let route_settings_binding = args.route_settings.get_output(context);
        let stage_variables_binding = args.stage_variables.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigatewayv2/stage:Stage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessLogSettings".into(),
                    value: &access_log_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoDeploy".into(),
                    value: &auto_deploy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateId".into(),
                    value: &client_certificate_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRouteSettings".into(),
                    value: &default_route_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentId".into(),
                    value: &deployment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeSettings".into(),
                    value: &route_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stageVariables".into(),
                    value: &stage_variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StageResult {
            access_log_settings: o.get_field("accessLogSettings"),
            api_id: o.get_field("apiId"),
            arn: o.get_field("arn"),
            auto_deploy: o.get_field("autoDeploy"),
            client_certificate_id: o.get_field("clientCertificateId"),
            default_route_settings: o.get_field("defaultRouteSettings"),
            deployment_id: o.get_field("deploymentId"),
            description: o.get_field("description"),
            execution_arn: o.get_field("executionArn"),
            invoke_url: o.get_field("invokeUrl"),
            name: o.get_field("name"),
            route_settings: o.get_field("routeSettings"),
            stage_variables: o.get_field("stageVariables"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
