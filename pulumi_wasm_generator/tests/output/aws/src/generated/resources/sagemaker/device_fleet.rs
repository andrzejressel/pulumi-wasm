/// Provides a SageMaker Device Fleet resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod device_fleet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceFleetArgs {
        /// A description of the fleet.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Device Fleet (must be unique).
        #[builder(into)]
        pub device_fleet_name: pulumi_wasm_rust::Output<String>,
        /// Whether to create an AWS IoT Role Alias during device fleet creation. The name of the role alias generated will match this pattern: "SageMakerEdge-{DeviceFleetName}".
        #[builder(into, default)]
        pub enable_iot_role_alias: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies details about the repository. see Output Config details below.
        #[builder(into)]
        pub output_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DeviceFleetOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) that has access to AWS Internet of Things (IoT).
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeviceFleetResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Device Fleet.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description of the fleet.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Device Fleet (must be unique).
        pub device_fleet_name: pulumi_wasm_rust::Output<String>,
        /// Whether to create an AWS IoT Role Alias during device fleet creation. The name of the role alias generated will match this pattern: "SageMakerEdge-{DeviceFleetName}".
        pub enable_iot_role_alias: pulumi_wasm_rust::Output<Option<bool>>,
        pub iot_role_alias: pulumi_wasm_rust::Output<String>,
        /// Specifies details about the repository. see Output Config details below.
        pub output_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DeviceFleetOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) that has access to AWS Internet of Things (IoT).
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeviceFleetArgs) -> DeviceFleetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let device_fleet_name_binding = args.device_fleet_name.get_inner();
        let enable_iot_role_alias_binding = args.enable_iot_role_alias.get_inner();
        let output_config_binding = args.output_config.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/deviceFleet:DeviceFleet".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "deviceFleetName".into(),
                },
                register_interface::ResultField {
                    name: "enableIotRoleAlias".into(),
                },
                register_interface::ResultField {
                    name: "iotRoleAlias".into(),
                },
                register_interface::ResultField {
                    name: "outputConfig".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
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
        DeviceFleetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            device_fleet_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceFleetName").unwrap(),
            ),
            enable_iot_role_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableIotRoleAlias").unwrap(),
            ),
            iot_role_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iotRoleAlias").unwrap(),
            ),
            output_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputConfig").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
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