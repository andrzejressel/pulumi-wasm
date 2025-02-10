/// Manages API Gateway Stage Method Settings. For example, CloudWatch logging and metrics.
///
/// > **NOTE:** We recommend using this resource in conjunction with the `aws.apigateway.Stage` resource instead of a stage managed by the `aws.apigateway.Deployment` resource optional `stage_name` argument. Stages managed by the `aws.apigateway.Deployment` resource are recreated on redeployment and this resource will require a second apply to recreate the method settings.
///
/// ## Example Usage
///
/// ### End-to-end
///
///
/// ### Off
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pathSpecific = method_settings::create(
///         "pathSpecific",
///         MethodSettingsArgs::builder()
///             .method_path("path1/GET")
///             .rest_api("${example.id}")
///             .settings(
///                 MethodSettingsSettings::builder().loggingLevel("OFF").build_struct(),
///             )
///             .stage_name("${exampleAwsApiGatewayStage.stageName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Errors Only
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pathSpecific = method_settings::create(
///         "pathSpecific",
///         MethodSettingsArgs::builder()
///             .method_path("path1/GET")
///             .rest_api("${example.id}")
///             .settings(
///                 MethodSettingsSettings::builder()
///                     .dataTraceEnabled(false)
///                     .loggingLevel("ERROR")
///                     .metricsEnabled(true)
///                     .build_struct(),
///             )
///             .stage_name("${exampleAwsApiGatewayStage.stageName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Errors and Info Logs
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pathSpecific = method_settings::create(
///         "pathSpecific",
///         MethodSettingsArgs::builder()
///             .method_path("path1/GET")
///             .rest_api("${example.id}")
///             .settings(
///                 MethodSettingsSettings::builder()
///                     .dataTraceEnabled(false)
///                     .loggingLevel("INFO")
///                     .metricsEnabled(true)
///                     .build_struct(),
///             )
///             .stage_name("${exampleAwsApiGatewayStage.stageName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Full Request and Response Logs
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pathSpecific = method_settings::create(
///         "pathSpecific",
///         MethodSettingsArgs::builder()
///             .method_path("path1/GET")
///             .rest_api("${example.id}")
///             .settings(
///                 MethodSettingsSettings::builder()
///                     .dataTraceEnabled(true)
///                     .loggingLevel("INFO")
///                     .metricsEnabled(true)
///                     .build_struct(),
///             )
///             .stage_name("${exampleAwsApiGatewayStage.stageName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_method_settings` using `REST-API-ID/STAGE-NAME/METHOD-PATH`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/methodSettings:MethodSettings example 12345abcde/example/test/GET
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod method_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MethodSettingsArgs {
        /// Method path defined as `{resource_path}/{http_method}` for an individual method override, or `*/*` for overriding all methods in the stage. Ensure to trim any leading forward slashes in the path (e.g., `trimprefix(aws_api_gateway_resource.example.path, "/")`).
        #[builder(into)]
        pub method_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the REST API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Settings block, see below.
        #[builder(into)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::apigateway::MethodSettingsSettings,
        >,
        /// Name of the stage
        #[builder(into)]
        pub stage_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MethodSettingsResult {
        /// Method path defined as `{resource_path}/{http_method}` for an individual method override, or `*/*` for overriding all methods in the stage. Ensure to trim any leading forward slashes in the path (e.g., `trimprefix(aws_api_gateway_resource.example.path, "/")`).
        pub method_path: pulumi_gestalt_rust::Output<String>,
        /// ID of the REST API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Settings block, see below.
        pub settings: pulumi_gestalt_rust::Output<
            super::super::types::apigateway::MethodSettingsSettings,
        >,
        /// Name of the stage
        pub stage_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MethodSettingsArgs,
    ) -> MethodSettingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let method_path_binding = args.method_path.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let settings_binding = args.settings.get_output(context);
        let stage_name_binding = args.stage_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/methodSettings:MethodSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "methodPath".into(),
                    value: method_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: rest_api_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stageName".into(),
                    value: stage_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MethodSettingsResult {
            method_path: o.get_field("methodPath"),
            rest_api: o.get_field("restApi"),
            settings: o.get_field("settings"),
            stage_name: o.get_field("stageName"),
        }
    }
}
