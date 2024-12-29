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
pub mod capacity_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityProviderArgs {
        /// Configuration block for the provider for the ECS auto scaling group. Detailed below.
        #[builder(into)]
        pub auto_scaling_group_provider: pulumi_wasm_rust::Output<
            super::super::types::ecs::CapacityProviderAutoScalingGroupProvider,
        >,
        /// Name of the capacity provider.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CapacityProviderResult {
        /// ARN that identifies the capacity provider.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the provider for the ECS auto scaling group. Detailed below.
        pub auto_scaling_group_provider: pulumi_wasm_rust::Output<
            super::super::types::ecs::CapacityProviderAutoScalingGroupProvider,
        >,
        /// Name of the capacity provider.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: CapacityProviderArgs) -> CapacityProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_scaling_group_provider_binding = args
            .auto_scaling_group_provider
            .get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecs/capacityProvider:CapacityProvider".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoScalingGroupProvider".into(),
                    value: &auto_scaling_group_provider_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoScalingGroupProvider".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        CapacityProviderResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_scaling_group_provider: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoScalingGroupProvider").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
