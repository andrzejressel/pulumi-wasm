/// Provides a CodeBuild Fleet Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// Number of machines allocated to the ﬂeet.
        #[builder(into)]
        pub base_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Compute resources the compute fleet uses. See [compute types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types) for more information and valid values.
        #[builder(into)]
        pub compute_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Environment type of the compute fleet. See [environment types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types) for more information and valid values.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub environment_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The service role associated with the compute fleet.
        #[builder(into, default)]
        pub fleet_service_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Machine Image (AMI) of the compute fleet.
        #[builder(into, default)]
        pub image_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Fleet name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Overflow behavior for compute fleet. Valid values: `ON_DEMAND`, `QUEUE`.
        #[builder(into, default)]
        pub overflow_behavior: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block. Detailed below. This option is only valid when your overflow behavior is `QUEUE`.
        #[builder(into, default)]
        pub scaling_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codebuild::FleetScalingConfiguration>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub vpc_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codebuild::FleetVpcConfig>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// ARN of the Fleet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Number of machines allocated to the ﬂeet.
        pub base_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Compute resources the compute fleet uses. See [compute types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types) for more information and valid values.
        pub compute_type: pulumi_gestalt_rust::Output<String>,
        /// Creation time of the fleet.
        pub created: pulumi_gestalt_rust::Output<String>,
        /// Environment type of the compute fleet. See [environment types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types) for more information and valid values.
        ///
        /// The following arguments are optional:
        pub environment_type: pulumi_gestalt_rust::Output<String>,
        /// The service role associated with the compute fleet.
        pub fleet_service_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Machine Image (AMI) of the compute fleet.
        pub image_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Last modification time of the fleet.
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// Fleet name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Overflow behavior for compute fleet. Valid values: `ON_DEMAND`, `QUEUE`.
        pub overflow_behavior: pulumi_gestalt_rust::Output<String>,
        /// Configuration block. Detailed below. This option is only valid when your overflow behavior is `QUEUE`.
        pub scaling_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::codebuild::FleetScalingConfiguration>,
        >,
        /// Nested attribute containing information about the current status of the fleet.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::codebuild::FleetStatus>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block. Detailed below.
        pub vpc_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codebuild::FleetVpcConfig>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetArgs,
    ) -> FleetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let base_capacity_binding = args.base_capacity.get_output(context);
        let compute_type_binding = args.compute_type.get_output(context);
        let environment_type_binding = args.environment_type.get_output(context);
        let fleet_service_role_binding = args.fleet_service_role.get_output(context);
        let image_id_binding = args.image_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let overflow_behavior_binding = args.overflow_behavior.get_output(context);
        let scaling_configuration_binding = args
            .scaling_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_configs_binding = args.vpc_configs.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codebuild/fleet:Fleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseCapacity".into(),
                    value: base_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeType".into(),
                    value: compute_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentType".into(),
                    value: environment_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleetServiceRole".into(),
                    value: fleet_service_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageId".into(),
                    value: image_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overflowBehavior".into(),
                    value: overflow_behavior_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalingConfiguration".into(),
                    value: scaling_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfigs".into(),
                    value: vpc_configs_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FleetResult {
            arn: o.get_field("arn"),
            base_capacity: o.get_field("baseCapacity"),
            compute_type: o.get_field("computeType"),
            created: o.get_field("created"),
            environment_type: o.get_field("environmentType"),
            fleet_service_role: o.get_field("fleetServiceRole"),
            image_id: o.get_field("imageId"),
            last_modified: o.get_field("lastModified"),
            name: o.get_field("name"),
            overflow_behavior: o.get_field("overflowBehavior"),
            scaling_configuration: o.get_field("scalingConfiguration"),
            statuses: o.get_field("statuses"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_configs: o.get_field("vpcConfigs"),
        }
    }
}
