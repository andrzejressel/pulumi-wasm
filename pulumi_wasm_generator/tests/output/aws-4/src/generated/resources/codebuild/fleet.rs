/// Provides a CodeBuild Fleet Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = fleet::create(
///         "test",
///         FleetArgs::builder()
///             .base_capacity(2)
///             .compute_type("BUILD_GENERAL1_SMALL")
///             .environment_type("LINUX_CONTAINER")
///             .name("full-example-codebuild-fleet")
///             .overflow_behavior("QUEUE")
///             .scaling_configuration(
///                 FleetScalingConfiguration::builder()
///                     .maxCapacity(5)
///                     .scalingType("TARGET_TRACKING_SCALING")
///                     .targetTrackingScalingConfigs(
///                         vec![
///                             FleetScalingConfigurationTargetTrackingScalingConfig::builder()
///                             .metricType("FLEET_UTILIZATION_RATE").targetValue(97.5)
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fleet::create(
///         "example",
///         FleetArgs::builder().name("example-codebuild-fleet").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeBuild Fleet using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:codebuild/fleet:Fleet name fleet-name
/// ```
pub mod fleet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// Number of machines allocated to the ﬂeet.
        #[builder(into)]
        pub base_capacity: pulumi_wasm_rust::Output<i32>,
        /// Compute resources the compute fleet uses. See [compute types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types) for more information and valid values.
        #[builder(into)]
        pub compute_type: pulumi_wasm_rust::Output<String>,
        /// Environment type of the compute fleet. See [environment types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types) for more information and valid values.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub environment_type: pulumi_wasm_rust::Output<String>,
        /// The service role associated with the compute fleet.
        #[builder(into, default)]
        pub fleet_service_role: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Machine Image (AMI) of the compute fleet.
        #[builder(into, default)]
        pub image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Fleet name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Overflow behavior for compute fleet. Valid values: `ON_DEMAND`, `QUEUE`.
        #[builder(into, default)]
        pub overflow_behavior: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block. Detailed below. This option is only valid when your overflow behavior is `QUEUE`.
        #[builder(into, default)]
        pub scaling_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::codebuild::FleetScalingConfiguration>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub vpc_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codebuild::FleetVpcConfig>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// ARN of the Fleet.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Number of machines allocated to the ﬂeet.
        pub base_capacity: pulumi_wasm_rust::Output<i32>,
        /// Compute resources the compute fleet uses. See [compute types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types) for more information and valid values.
        pub compute_type: pulumi_wasm_rust::Output<String>,
        /// Creation time of the fleet.
        pub created: pulumi_wasm_rust::Output<String>,
        /// Environment type of the compute fleet. See [environment types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types) for more information and valid values.
        ///
        /// The following arguments are optional:
        pub environment_type: pulumi_wasm_rust::Output<String>,
        /// The service role associated with the compute fleet.
        pub fleet_service_role: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Machine Image (AMI) of the compute fleet.
        pub image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Last modification time of the fleet.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// Fleet name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Overflow behavior for compute fleet. Valid values: `ON_DEMAND`, `QUEUE`.
        pub overflow_behavior: pulumi_wasm_rust::Output<String>,
        /// Configuration block. Detailed below. This option is only valid when your overflow behavior is `QUEUE`.
        pub scaling_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::codebuild::FleetScalingConfiguration>,
        >,
        /// Nested attribute containing information about the current status of the fleet.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::codebuild::FleetStatus>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block. Detailed below.
        pub vpc_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codebuild::FleetVpcConfig>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FleetArgs) -> FleetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let base_capacity_binding = args.base_capacity.get_inner();
        let compute_type_binding = args.compute_type.get_inner();
        let environment_type_binding = args.environment_type.get_inner();
        let fleet_service_role_binding = args.fleet_service_role.get_inner();
        let image_id_binding = args.image_id.get_inner();
        let name_binding = args.name.get_inner();
        let overflow_behavior_binding = args.overflow_behavior.get_inner();
        let scaling_configuration_binding = args.scaling_configuration.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_configs_binding = args.vpc_configs.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codebuild/fleet:Fleet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baseCapacity".into(),
                    value: &base_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "computeType".into(),
                    value: &compute_type_binding,
                },
                register_interface::ObjectField {
                    name: "environmentType".into(),
                    value: &environment_type_binding,
                },
                register_interface::ObjectField {
                    name: "fleetServiceRole".into(),
                    value: &fleet_service_role_binding,
                },
                register_interface::ObjectField {
                    name: "imageId".into(),
                    value: &image_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "overflowBehavior".into(),
                    value: &overflow_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "scalingConfiguration".into(),
                    value: &scaling_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfigs".into(),
                    value: &vpc_configs_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "baseCapacity".into(),
                },
                register_interface::ResultField {
                    name: "computeType".into(),
                },
                register_interface::ResultField {
                    name: "created".into(),
                },
                register_interface::ResultField {
                    name: "environmentType".into(),
                },
                register_interface::ResultField {
                    name: "fleetServiceRole".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "overflowBehavior".into(),
                },
                register_interface::ResultField {
                    name: "scalingConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "statuses".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfigs".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FleetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            base_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseCapacity").unwrap(),
            ),
            compute_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeType").unwrap(),
            ),
            created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("created").unwrap(),
            ),
            environment_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentType").unwrap(),
            ),
            fleet_service_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetServiceRole").unwrap(),
            ),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            overflow_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overflowBehavior").unwrap(),
            ),
            scaling_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingConfiguration").unwrap(),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statuses").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfigs").unwrap(),
            ),
        }
    }
}
