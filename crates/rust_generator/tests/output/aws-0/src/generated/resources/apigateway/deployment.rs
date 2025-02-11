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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// Input configuration for the canary deployment when the deployment is a canary release deployment.
        /// See `canary_settings below.
        /// Has no effect when `stage_name` is not set.
        #[builder(into, default)]
        pub canary_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigateway::DeploymentCanarySettings>,
        >,
        /// Description of the deployment
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// REST API identifier.
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description to set on the stage managed by the `stage_name` argument.
        /// Has no effect when `stage_name` is not set.
        #[builder(into, default)]
        pub stage_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the stage to create with this deployment.
        /// If the specified stage already exists, it will be updated to point to the new deployment.
        /// We recommend using the `aws.apigateway.Stage` resource instead to manage stages.
        #[builder(into, default)]
        pub stage_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a redeployment.
        #[builder(into, default)]
        pub triggers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map to set on the stage managed by the `stage_name` argument.
        #[builder(into, default)]
        pub variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// Input configuration for the canary deployment when the deployment is a canary release deployment.
        /// See `canary_settings below.
        /// Has no effect when `stage_name` is not set.
        pub canary_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigateway::DeploymentCanarySettings>,
        >,
        /// Creation date of the deployment
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the deployment
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Execution ARN to be used in `lambda_permission`'s `source_arn`
        /// when allowing API Gateway to invoke a Lambda function,
        /// e.g., `arn:aws:execute-api:eu-west-2:123456789012:z4675bid1j/prod`
        pub execution_arn: pulumi_gestalt_rust::Output<String>,
        /// URL to invoke the API pointing to the stage,
        /// e.g., `https://z4675bid1j.execute-api.eu-west-2.amazonaws.com/prod`
        pub invoke_url: pulumi_gestalt_rust::Output<String>,
        /// REST API identifier.
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Description to set on the stage managed by the `stage_name` argument.
        /// Has no effect when `stage_name` is not set.
        pub stage_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the stage to create with this deployment.
        /// If the specified stage already exists, it will be updated to point to the new deployment.
        /// We recommend using the `aws.apigateway.Stage` resource instead to manage stages.
        pub stage_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a redeployment.
        pub triggers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map to set on the stage managed by the `stage_name` argument.
        pub variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentArgs,
    ) -> DeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let canary_settings_binding = args.canary_settings.get_output(context);
        let description_binding = args.description.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let stage_description_binding = args.stage_description.get_output(context);
        let stage_name_binding = args.stage_name.get_output(context);
        let triggers_binding = args.triggers.get_output(context);
        let variables_binding = args.variables.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "canarySettings".into(),
                    value: &canary_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stageDescription".into(),
                    value: &stage_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "variables".into(),
                    value: &variables_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeploymentResult {
            canary_settings: o.get_field("canarySettings"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            execution_arn: o.get_field("executionArn"),
            invoke_url: o.get_field("invokeUrl"),
            rest_api: o.get_field("restApi"),
            stage_description: o.get_field("stageDescription"),
            stage_name: o.get_field("stageName"),
            triggers: o.get_field("triggers"),
            variables: o.get_field("variables"),
        }
    }
}
