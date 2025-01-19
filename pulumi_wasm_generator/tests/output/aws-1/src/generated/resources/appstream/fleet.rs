/// Provides an AppStream fleet.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testFleet:
///     type: aws:appstream:Fleet
///     name: test_fleet
///     properties:
///       name: test-fleet
///       computeCapacity:
///         desiredInstances: 1
///       description: test fleet
///       idleDisconnectTimeoutInSeconds: 60
///       displayName: test-fleet
///       enableDefaultInternetAccess: false
///       fleetType: ON_DEMAND
///       imageName: Amazon-AppStream2-Sample-Image-03-11-2023
///       instanceType: stream.standard.large
///       maxUserDurationInSeconds: 600
///       vpcConfig:
///         subnetIds:
///           - subnet-06e9b13400c225127
///       tags:
///         TagName: tag-value
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appstream_fleet` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:appstream/fleet:Fleet example fleetNameExample
/// ```
pub mod fleet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// Configuration block for the desired capacity of the fleet. See below.
        #[builder(into)]
        pub compute_capacity: pulumi_wasm_rust::Output<
            super::super::types::appstream::FleetComputeCapacity,
        >,
        /// Description to display.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount of time that a streaming session remains active after users disconnect.
        #[builder(into, default)]
        pub disconnect_timeout_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Human-readable friendly name for the AppStream fleet.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. See below.
        #[builder(into, default)]
        pub domain_join_info: pulumi_wasm_rust::Output<
            Option<super::super::types::appstream::FleetDomainJoinInfo>,
        >,
        /// Enables or disables default internet access for the fleet.
        #[builder(into, default)]
        pub enable_default_internet_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// Fleet type. Valid values are: `ON_DEMAND`, `ALWAYS_ON`
        #[builder(into, default)]
        pub fleet_type: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the IAM role to apply to the fleet.
        #[builder(into, default)]
        pub iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the `disconnect_timeout_in_seconds` time interval begins. Defaults to `0`. Valid value is between `60` and `3600 `seconds.
        #[builder(into, default)]
        pub idle_disconnect_timeout_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// ARN of the public, private, or shared image to use.
        #[builder(into, default)]
        pub image_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the image used to create the fleet.
        #[builder(into, default)]
        pub image_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Instance type to use when launching fleet instances.
        #[builder(into)]
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// The maximum number of user sessions on an instance. This only applies to multi-session fleets.
        #[builder(into, default)]
        pub max_sessions_per_instance: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum amount of time that a streaming session can remain active, in seconds.
        #[builder(into, default)]
        pub max_user_duration_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Unique name for the fleet.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// AppStream 2.0 view that is displayed to your users when they stream from the fleet. When `APP` is specified, only the windows of applications opened by users display. When `DESKTOP` is specified, the standard desktop that is provided by the operating system displays. If not specified, defaults to `APP`.
        #[builder(into, default)]
        pub stream_view: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to attach to AppStream instances.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for the VPC configuration for the image builder. See below.
        #[builder(into, default)]
        pub vpc_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appstream::FleetVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// ARN of the appstream fleet.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the desired capacity of the fleet. See below.
        pub compute_capacity: pulumi_wasm_rust::Output<
            super::super::types::appstream::FleetComputeCapacity,
        >,
        /// Date and time, in UTC and extended RFC 3339 format, when the fleet was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Description to display.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Amount of time that a streaming session remains active after users disconnect.
        pub disconnect_timeout_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// Human-readable friendly name for the AppStream fleet.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. See below.
        pub domain_join_info: pulumi_wasm_rust::Output<
            super::super::types::appstream::FleetDomainJoinInfo,
        >,
        /// Enables or disables default internet access for the fleet.
        pub enable_default_internet_access: pulumi_wasm_rust::Output<bool>,
        /// Fleet type. Valid values are: `ON_DEMAND`, `ALWAYS_ON`
        pub fleet_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role to apply to the fleet.
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// Amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the `disconnect_timeout_in_seconds` time interval begins. Defaults to `0`. Valid value is between `60` and `3600 `seconds.
        pub idle_disconnect_timeout_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// ARN of the public, private, or shared image to use.
        pub image_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the image used to create the fleet.
        pub image_name: pulumi_wasm_rust::Output<String>,
        /// Instance type to use when launching fleet instances.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// The maximum number of user sessions on an instance. This only applies to multi-session fleets.
        pub max_sessions_per_instance: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum amount of time that a streaming session can remain active, in seconds.
        pub max_user_duration_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// Unique name for the fleet.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// State of the fleet. Can be `STARTING`, `RUNNING`, `STOPPING` or `STOPPED`
        pub state: pulumi_wasm_rust::Output<String>,
        /// AppStream 2.0 view that is displayed to your users when they stream from the fleet. When `APP` is specified, only the windows of applications opened by users display. When `DESKTOP` is specified, the standard desktop that is provided by the operating system displays. If not specified, defaults to `APP`.
        pub stream_view: pulumi_wasm_rust::Output<String>,
        /// Map of tags to attach to AppStream instances.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for the VPC configuration for the image builder. See below.
        pub vpc_config: pulumi_wasm_rust::Output<
            super::super::types::appstream::FleetVpcConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FleetArgs) -> FleetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let compute_capacity_binding = args.compute_capacity.get_inner();
        let description_binding = args.description.get_inner();
        let disconnect_timeout_in_seconds_binding = args
            .disconnect_timeout_in_seconds
            .get_inner();
        let display_name_binding = args.display_name.get_inner();
        let domain_join_info_binding = args.domain_join_info.get_inner();
        let enable_default_internet_access_binding = args
            .enable_default_internet_access
            .get_inner();
        let fleet_type_binding = args.fleet_type.get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_inner();
        let idle_disconnect_timeout_in_seconds_binding = args
            .idle_disconnect_timeout_in_seconds
            .get_inner();
        let image_arn_binding = args.image_arn.get_inner();
        let image_name_binding = args.image_name.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let max_sessions_per_instance_binding = args
            .max_sessions_per_instance
            .get_inner();
        let max_user_duration_in_seconds_binding = args
            .max_user_duration_in_seconds
            .get_inner();
        let name_binding = args.name.get_inner();
        let stream_view_binding = args.stream_view.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_config_binding = args.vpc_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appstream/fleet:Fleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "computeCapacity".into(),
                    value: &compute_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disconnectTimeoutInSeconds".into(),
                    value: &disconnect_timeout_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainJoinInfo".into(),
                    value: &domain_join_info_binding,
                },
                register_interface::ObjectField {
                    name: "enableDefaultInternetAccess".into(),
                    value: &enable_default_internet_access_binding,
                },
                register_interface::ObjectField {
                    name: "fleetType".into(),
                    value: &fleet_type_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "idleDisconnectTimeoutInSeconds".into(),
                    value: &idle_disconnect_timeout_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "imageArn".into(),
                    value: &image_arn_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "maxSessionsPerInstance".into(),
                    value: &max_sessions_per_instance_binding,
                },
                register_interface::ObjectField {
                    name: "maxUserDurationInSeconds".into(),
                    value: &max_user_duration_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "streamView".into(),
                    value: &stream_view_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "computeCapacity".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disconnectTimeoutInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "domainJoinInfo".into(),
                },
                register_interface::ResultField {
                    name: "enableDefaultInternetAccess".into(),
                },
                register_interface::ResultField {
                    name: "fleetType".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "idleDisconnectTimeoutInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "imageArn".into(),
                },
                register_interface::ResultField {
                    name: "imageName".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "maxSessionsPerInstance".into(),
                },
                register_interface::ResultField {
                    name: "maxUserDurationInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "streamView".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfig".into(),
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
            compute_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeCapacity").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disconnect_timeout_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disconnectTimeoutInSeconds").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            domain_join_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainJoinInfo").unwrap(),
            ),
            enable_default_internet_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDefaultInternetAccess").unwrap(),
            ),
            fleet_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetType").unwrap(),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
            idle_disconnect_timeout_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleDisconnectTimeoutInSeconds").unwrap(),
            ),
            image_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageArn").unwrap(),
            ),
            image_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageName").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            max_sessions_per_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSessionsPerInstance").unwrap(),
            ),
            max_user_duration_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxUserDurationInSeconds").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            stream_view: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamView").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfig").unwrap(),
            ),
        }
    }
}
