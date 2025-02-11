/// Provides an ECS cluster capacity provider. More information can be found on the [ECS Developer Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-capacity-providers.html).
///
/// > **NOTE:** Associating an ECS Capacity Provider to an Auto Scaling Group will automatically add the `AmazonECSManaged` tag to the Auto Scaling Group. This tag should be included in the `aws.autoscaling.Group` resource configuration to prevent the provider from removing it in subsequent executions as well as ensuring the `AmazonECSManaged` tag is propagated to all EC2 Instances in the Auto Scaling Group if `min_size` is above 0 on creation. Any EC2 Instances in the Auto Scaling Group without this tag must be manually be updated, otherwise they may cause unexpected scaling behavior and metrics.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:autoscaling:Group
///     properties:
///       tags:
///         - key: AmazonECSManaged
///           value: true
///           propagateAtLaunch: true
///   testCapacityProvider:
///     type: aws:ecs:CapacityProvider
///     name: test
///     properties:
///       name: test
///       autoScalingGroupProvider:
///         autoScalingGroupArn: ${test.arn}
///         managedTerminationProtection: ENABLED
///         managedScaling:
///           maximumScalingStepSize: 1000
///           minimumScalingStepSize: 1
///           status: ENABLED
///           targetCapacity: 10
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECS Capacity Providers using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ecs/capacityProvider:CapacityProvider example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod capacity_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityProviderArgs {
        /// Configuration block for the provider for the ECS auto scaling group. Detailed below.
        #[builder(into)]
        pub auto_scaling_group_provider: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ecs::CapacityProviderAutoScalingGroupProvider,
        >,
        /// Name of the capacity provider.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CapacityProviderResult {
        /// ARN that identifies the capacity provider.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the provider for the ECS auto scaling group. Detailed below.
        pub auto_scaling_group_provider: pulumi_gestalt_rust::Output<
            super::super::types::ecs::CapacityProviderAutoScalingGroupProvider,
        >,
        /// Name of the capacity provider.
        pub name: pulumi_gestalt_rust::Output<String>,
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
        args: CapacityProviderArgs,
    ) -> CapacityProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_scaling_group_provider_binding = args
            .auto_scaling_group_provider
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ecs/capacityProvider:CapacityProvider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoScalingGroupProvider".into(),
                    value: &auto_scaling_group_provider_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CapacityProviderResult {
            arn: o.get_field("arn"),
            auto_scaling_group_provider: o.get_field("autoScalingGroupProvider"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
