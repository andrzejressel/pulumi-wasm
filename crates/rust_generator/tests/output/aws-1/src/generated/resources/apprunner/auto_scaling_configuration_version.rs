/// Manages an App Runner AutoScaling Configuration Version.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apprunner:AutoScalingConfigurationVersion
///     properties:
///       autoScalingConfigurationName: example
///       maxConcurrency: 50
///       maxSize: 10
///       minSize: 2
///       tags:
///         Name: example-apprunner-autoscaling
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner AutoScaling Configuration Versions using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/autoScalingConfigurationVersion:AutoScalingConfigurationVersion example "arn:aws:apprunner:us-east-1:1234567890:autoscalingconfiguration/example/1/69bdfe0115224b0db49398b7beb68e0f
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod auto_scaling_configuration_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutoScalingConfigurationVersionArgs {
        /// Name of the auto scaling configuration.
        #[builder(into)]
        pub auto_scaling_configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Maximal number of concurrent requests that you want an instance to process. When the number of concurrent requests goes over this limit, App Runner scales up your service.
        #[builder(into, default)]
        pub max_concurrency: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Maximal number of instances that App Runner provisions for your service.
        #[builder(into, default)]
        pub max_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimal number of instances that App Runner provisions for your service.
        #[builder(into, default)]
        pub min_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AutoScalingConfigurationVersionResult {
        /// ARN of this auto scaling configuration version.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the auto scaling configuration.
        pub auto_scaling_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The revision of this auto scaling configuration.
        pub auto_scaling_configuration_revision: pulumi_gestalt_rust::Output<i32>,
        pub has_associated_service: pulumi_gestalt_rust::Output<bool>,
        pub is_default: pulumi_gestalt_rust::Output<bool>,
        /// Whether the auto scaling configuration has the highest `auto_scaling_configuration_revision` among all configurations that share the same `auto_scaling_configuration_name`.
        pub latest: pulumi_gestalt_rust::Output<bool>,
        /// Maximal number of concurrent requests that you want an instance to process. When the number of concurrent requests goes over this limit, App Runner scales up your service.
        pub max_concurrency: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Maximal number of instances that App Runner provisions for your service.
        pub max_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Minimal number of instances that App Runner provisions for your service.
        pub min_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Current state of the auto scaling configuration. An INACTIVE configuration revision has been deleted and can't be used. It is permanently removed some time after deletion.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: AutoScalingConfigurationVersionArgs,
    ) -> AutoScalingConfigurationVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_scaling_configuration_name_binding = args
            .auto_scaling_configuration_name
            .get_output(context);
        let max_concurrency_binding = args.max_concurrency.get_output(context);
        let max_size_binding = args.max_size.get_output(context);
        let min_size_binding = args.min_size.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apprunner/autoScalingConfigurationVersion:AutoScalingConfigurationVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoScalingConfigurationName".into(),
                    value: auto_scaling_configuration_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxConcurrency".into(),
                    value: max_concurrency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxSize".into(),
                    value: max_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minSize".into(),
                    value: min_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutoScalingConfigurationVersionResult {
            arn: o.get_field("arn"),
            auto_scaling_configuration_name: o.get_field("autoScalingConfigurationName"),
            auto_scaling_configuration_revision: o
                .get_field("autoScalingConfigurationRevision"),
            has_associated_service: o.get_field("hasAssociatedService"),
            is_default: o.get_field("isDefault"),
            latest: o.get_field("latest"),
            max_concurrency: o.get_field("maxConcurrency"),
            max_size: o.get_field("maxSize"),
            min_size: o.get_field("minSize"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
