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
pub mod auto_scaling_configuration_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutoScalingConfigurationVersionArgs {
        /// Name of the auto scaling configuration.
        #[builder(into)]
        pub auto_scaling_configuration_name: pulumi_wasm_rust::Output<String>,
        /// Maximal number of concurrent requests that you want an instance to process. When the number of concurrent requests goes over this limit, App Runner scales up your service.
        #[builder(into, default)]
        pub max_concurrency: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximal number of instances that App Runner provisions for your service.
        #[builder(into, default)]
        pub max_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimal number of instances that App Runner provisions for your service.
        #[builder(into, default)]
        pub min_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AutoScalingConfigurationVersionResult {
        /// ARN of this auto scaling configuration version.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the auto scaling configuration.
        pub auto_scaling_configuration_name: pulumi_wasm_rust::Output<String>,
        /// The revision of this auto scaling configuration.
        pub auto_scaling_configuration_revision: pulumi_wasm_rust::Output<i32>,
        pub has_associated_service: pulumi_wasm_rust::Output<bool>,
        pub is_default: pulumi_wasm_rust::Output<bool>,
        /// Whether the auto scaling configuration has the highest `auto_scaling_configuration_revision` among all configurations that share the same `auto_scaling_configuration_name`.
        pub latest: pulumi_wasm_rust::Output<bool>,
        /// Maximal number of concurrent requests that you want an instance to process. When the number of concurrent requests goes over this limit, App Runner scales up your service.
        pub max_concurrency: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximal number of instances that App Runner provisions for your service.
        pub max_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimal number of instances that App Runner provisions for your service.
        pub min_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Current state of the auto scaling configuration. An INACTIVE configuration revision has been deleted and can't be used. It is permanently removed some time after deletion.
        pub status: pulumi_wasm_rust::Output<String>,
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
    pub fn create(
        name: &str,
        args: AutoScalingConfigurationVersionArgs,
    ) -> AutoScalingConfigurationVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_scaling_configuration_name_binding = args
            .auto_scaling_configuration_name
            .get_inner();
        let max_concurrency_binding = args.max_concurrency.get_inner();
        let max_size_binding = args.max_size.get_inner();
        let min_size_binding = args.min_size.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apprunner/autoScalingConfigurationVersion:AutoScalingConfigurationVersion"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoScalingConfigurationName".into(),
                    value: &auto_scaling_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "maxConcurrency".into(),
                    value: &max_concurrency_binding,
                },
                register_interface::ObjectField {
                    name: "maxSize".into(),
                    value: &max_size_binding,
                },
                register_interface::ObjectField {
                    name: "minSize".into(),
                    value: &min_size_binding,
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
                    name: "autoScalingConfigurationName".into(),
                },
                register_interface::ResultField {
                    name: "autoScalingConfigurationRevision".into(),
                },
                register_interface::ResultField {
                    name: "hasAssociatedService".into(),
                },
                register_interface::ResultField {
                    name: "isDefault".into(),
                },
                register_interface::ResultField {
                    name: "latest".into(),
                },
                register_interface::ResultField {
                    name: "maxConcurrency".into(),
                },
                register_interface::ResultField {
                    name: "maxSize".into(),
                },
                register_interface::ResultField {
                    name: "minSize".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
        AutoScalingConfigurationVersionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_scaling_configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoScalingConfigurationName").unwrap(),
            ),
            auto_scaling_configuration_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoScalingConfigurationRevision").unwrap(),
            ),
            has_associated_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasAssociatedService").unwrap(),
            ),
            is_default: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefault").unwrap(),
            ),
            latest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latest").unwrap(),
            ),
            max_concurrency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxConcurrency").unwrap(),
            ),
            max_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSize").unwrap(),
            ),
            min_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minSize").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
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
