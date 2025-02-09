/// Provides a SageMaker Device Fleet resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device_fleet::create(
///         "example",
///         DeviceFleetArgs::builder()
///             .device_fleet_name("example")
///             .output_config(
///                 DeviceFleetOutputConfig::builder()
///                     .s3OutputLocation("s3://${exampleAwsS3Bucket.bucket}/prefix/")
///                     .build_struct(),
///             )
///             .role_arn("${test.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Device Fleets using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/deviceFleet:DeviceFleet example my-fleet
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod device_fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceFleetArgs {
        /// A description of the fleet.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Device Fleet (must be unique).
        #[builder(into)]
        pub device_fleet_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to create an AWS IoT Role Alias during device fleet creation. The name of the role alias generated will match this pattern: "SageMakerEdge-{DeviceFleetName}".
        #[builder(into, default)]
        pub enable_iot_role_alias: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies details about the repository. see Output Config details below.
        #[builder(into)]
        pub output_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sagemaker::DeviceFleetOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) that has access to AWS Internet of Things (IoT).
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeviceFleetResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Device Fleet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the fleet.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Device Fleet (must be unique).
        pub device_fleet_name: pulumi_gestalt_rust::Output<String>,
        /// Whether to create an AWS IoT Role Alias during device fleet creation. The name of the role alias generated will match this pattern: "SageMakerEdge-{DeviceFleetName}".
        pub enable_iot_role_alias: pulumi_gestalt_rust::Output<Option<bool>>,
        pub iot_role_alias: pulumi_gestalt_rust::Output<String>,
        /// Specifies details about the repository. see Output Config details below.
        pub output_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::DeviceFleetOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) that has access to AWS Internet of Things (IoT).
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DeviceFleetArgs,
    ) -> DeviceFleetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let device_fleet_name_binding_1 = args.device_fleet_name.get_output(context);
        let device_fleet_name_binding = device_fleet_name_binding_1.get_inner();
        let enable_iot_role_alias_binding_1 = args
            .enable_iot_role_alias
            .get_output(context);
        let enable_iot_role_alias_binding = enable_iot_role_alias_binding_1.get_inner();
        let output_config_binding_1 = args.output_config.get_output(context);
        let output_config_binding = output_config_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/deviceFleet:DeviceFleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "deviceFleetName".into(),
                    value: &device_fleet_name_binding,
                },
                register_interface::ObjectField {
                    name: "enableIotRoleAlias".into(),
                    value: &enable_iot_role_alias_binding,
                },
                register_interface::ObjectField {
                    name: "outputConfig".into(),
                    value: &output_config_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeviceFleetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            device_fleet_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceFleetName"),
            ),
            enable_iot_role_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableIotRoleAlias"),
            ),
            iot_role_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iotRoleAlias"),
            ),
            output_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputConfig"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
