/// Provides an AppConfig Deployment Strategy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:DeploymentStrategy
///     properties:
///       name: example-deployment-strategy-tf
///       description: Example Deployment Strategy
///       deploymentDurationInMinutes: 3
///       finalBakeTimeInMinutes: 4
///       growthFactor: 10
///       growthType: LINEAR
///       replicateTo: NONE
///       tags:
///         Type: AppConfig Deployment Strategy
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Deployment Strategies using their deployment strategy ID. For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/deploymentStrategy:DeploymentStrategy example 11xxxxx
/// ```
pub mod deployment_strategy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentStrategyArgs {
        /// Total amount of time for a deployment to last. Minimum value of 0, maximum value of 1440.
        #[builder(into)]
        pub deployment_duration_in_minutes: pulumi_wasm_rust::Output<i32>,
        /// Description of the deployment strategy. Can be at most 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount of time AWS AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back. Minimum value of 0, maximum value of 1440.
        #[builder(into, default)]
        pub final_bake_time_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Percentage of targets to receive a deployed configuration during each interval. Minimum value of 1.0, maximum value of 100.0.
        #[builder(into)]
        pub growth_factor: pulumi_wasm_rust::Output<f64>,
        /// Algorithm used to define how percentage grows over time. Valid value: `LINEAR` and `EXPONENTIAL`. Defaults to `LINEAR`.
        #[builder(into, default)]
        pub growth_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the deployment strategy. Must be between 1 and 64 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Where to save the deployment strategy. Valid values: `NONE` and `SSM_DOCUMENT`.
        #[builder(into)]
        pub replicate_to: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentStrategyResult {
        /// ARN of the AppConfig Deployment Strategy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Total amount of time for a deployment to last. Minimum value of 0, maximum value of 1440.
        pub deployment_duration_in_minutes: pulumi_wasm_rust::Output<i32>,
        /// Description of the deployment strategy. Can be at most 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount of time AWS AppConfig monitors for alarms before considering the deployment to be complete and no longer eligible for automatic roll back. Minimum value of 0, maximum value of 1440.
        pub final_bake_time_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Percentage of targets to receive a deployed configuration during each interval. Minimum value of 1.0, maximum value of 100.0.
        pub growth_factor: pulumi_wasm_rust::Output<f64>,
        /// Algorithm used to define how percentage grows over time. Valid value: `LINEAR` and `EXPONENTIAL`. Defaults to `LINEAR`.
        pub growth_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the deployment strategy. Must be between 1 and 64 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Where to save the deployment strategy. Valid values: `NONE` and `SSM_DOCUMENT`.
        pub replicate_to: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: DeploymentStrategyArgs) -> DeploymentStrategyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deployment_duration_in_minutes_binding = args
            .deployment_duration_in_minutes
            .get_inner();
        let description_binding = args.description.get_inner();
        let final_bake_time_in_minutes_binding = args
            .final_bake_time_in_minutes
            .get_inner();
        let growth_factor_binding = args.growth_factor.get_inner();
        let growth_type_binding = args.growth_type.get_inner();
        let name_binding = args.name.get_inner();
        let replicate_to_binding = args.replicate_to.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appconfig/deploymentStrategy:DeploymentStrategy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deploymentDurationInMinutes".into(),
                    value: &deployment_duration_in_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "finalBakeTimeInMinutes".into(),
                    value: &final_bake_time_in_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "growthFactor".into(),
                    value: &growth_factor_binding,
                },
                register_interface::ObjectField {
                    name: "growthType".into(),
                    value: &growth_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "replicateTo".into(),
                    value: &replicate_to_binding,
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
                    name: "deploymentDurationInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "finalBakeTimeInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "growthFactor".into(),
                },
                register_interface::ResultField {
                    name: "growthType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "replicateTo".into(),
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
        DeploymentStrategyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            deployment_duration_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentDurationInMinutes").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            final_bake_time_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalBakeTimeInMinutes").unwrap(),
            ),
            growth_factor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("growthFactor").unwrap(),
            ),
            growth_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("growthType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            replicate_to: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicateTo").unwrap(),
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
