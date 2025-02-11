/// Manages the default App Runner auto scaling configuration.
/// When creating or updating this resource the existing default auto scaling configuration will be set to non-default automatically.
/// When creating or updating this resource the configuration is automatically assigned as the default to the new services you create in the future. The new default designation doesn't affect the associations that were previously set for existing services.
/// Each account can have only one default auto scaling configuration per Region.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = auto_scaling_configuration_version::create(
///         "example",
///         AutoScalingConfigurationVersionArgs::builder()
///             .auto_scaling_configuration_name("example")
///             .max_concurrency(50)
///             .max_size(10)
///             .min_size(2)
///             .build_struct(),
///     );
///     let exampleDefaultAutoScalingConfigurationVersion = default_auto_scaling_configuration_version::create(
///         "exampleDefaultAutoScalingConfigurationVersion",
///         DefaultAutoScalingConfigurationVersionArgs::builder()
///             .auto_scaling_configuration_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner default auto scaling configurations using the current Region. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/defaultAutoScalingConfigurationVersion:DefaultAutoScalingConfigurationVersion example us-west-2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_auto_scaling_configuration_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultAutoScalingConfigurationVersionArgs {
        /// The ARN of the App Runner auto scaling configuration that you want to set as the default.
        #[builder(into)]
        pub auto_scaling_configuration_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultAutoScalingConfigurationVersionResult {
        /// The ARN of the App Runner auto scaling configuration that you want to set as the default.
        pub auto_scaling_configuration_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultAutoScalingConfigurationVersionArgs,
    ) -> DefaultAutoScalingConfigurationVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_scaling_configuration_arn_binding = args
            .auto_scaling_configuration_arn
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apprunner/defaultAutoScalingConfigurationVersion:DefaultAutoScalingConfigurationVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoScalingConfigurationArn".into(),
                    value: &auto_scaling_configuration_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultAutoScalingConfigurationVersionResult {
            auto_scaling_configuration_arn: o.get_field("autoScalingConfigurationArn"),
        }
    }
}
