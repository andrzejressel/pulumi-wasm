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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// Configuration block for the desired capacity of the fleet. See below.
        #[builder(into)]
        pub compute_capacity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appstream::FleetComputeCapacity,
        >,
        /// Description to display.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amount of time that a streaming session remains active after users disconnect.
        #[builder(into, default)]
        pub disconnect_timeout_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Human-readable friendly name for the AppStream fleet.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. See below.
        #[builder(into, default)]
        pub domain_join_info: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appstream::FleetDomainJoinInfo>,
        >,
        /// Enables or disables default internet access for the fleet.
        #[builder(into, default)]
        pub enable_default_internet_access: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Fleet type. Valid values are: `ON_DEMAND`, `ALWAYS_ON`
        #[builder(into, default)]
        pub fleet_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the IAM role to apply to the fleet.
        #[builder(into, default)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the `disconnect_timeout_in_seconds` time interval begins. Defaults to `0`. Valid value is between `60` and `3600 `seconds.
        #[builder(into, default)]
        pub idle_disconnect_timeout_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// ARN of the public, private, or shared image to use.
        #[builder(into, default)]
        pub image_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the image used to create the fleet.
        #[builder(into, default)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Instance type to use when launching fleet instances.
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The maximum number of user sessions on an instance. This only applies to multi-session fleets.
        #[builder(into, default)]
        pub max_sessions_per_instance: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Maximum amount of time that a streaming session can remain active, in seconds.
        #[builder(into, default)]
        pub max_user_duration_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Unique name for the fleet.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AppStream 2.0 view that is displayed to your users when they stream from the fleet. When `APP` is specified, only the windows of applications opened by users display. When `DESKTOP` is specified, the standard desktop that is provided by the operating system displays. If not specified, defaults to `APP`.
        #[builder(into, default)]
        pub stream_view: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to attach to AppStream instances.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for the VPC configuration for the image builder. See below.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appstream::FleetVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// ARN of the appstream fleet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the desired capacity of the fleet. See below.
        pub compute_capacity: pulumi_gestalt_rust::Output<
            super::super::types::appstream::FleetComputeCapacity,
        >,
        /// Date and time, in UTC and extended RFC 3339 format, when the fleet was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Description to display.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Amount of time that a streaming session remains active after users disconnect.
        pub disconnect_timeout_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// Human-readable friendly name for the AppStream fleet.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. See below.
        pub domain_join_info: pulumi_gestalt_rust::Output<
            super::super::types::appstream::FleetDomainJoinInfo,
        >,
        /// Enables or disables default internet access for the fleet.
        pub enable_default_internet_access: pulumi_gestalt_rust::Output<bool>,
        /// Fleet type. Valid values are: `ON_DEMAND`, `ALWAYS_ON`
        pub fleet_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role to apply to the fleet.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Amount of time that users can be idle (inactive) before they are disconnected from their streaming session and the `disconnect_timeout_in_seconds` time interval begins. Defaults to `0`. Valid value is between `60` and `3600 `seconds.
        pub idle_disconnect_timeout_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// ARN of the public, private, or shared image to use.
        pub image_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the image used to create the fleet.
        pub image_name: pulumi_gestalt_rust::Output<String>,
        /// Instance type to use when launching fleet instances.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of user sessions on an instance. This only applies to multi-session fleets.
        pub max_sessions_per_instance: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Maximum amount of time that a streaming session can remain active, in seconds.
        pub max_user_duration_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// Unique name for the fleet.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// State of the fleet. Can be `STARTING`, `RUNNING`, `STOPPING` or `STOPPED`
        pub state: pulumi_gestalt_rust::Output<String>,
        /// AppStream 2.0 view that is displayed to your users when they stream from the fleet. When `APP` is specified, only the windows of applications opened by users display. When `DESKTOP` is specified, the standard desktop that is provided by the operating system displays. If not specified, defaults to `APP`.
        pub stream_view: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to attach to AppStream instances.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for the VPC configuration for the image builder. See below.
        pub vpc_config: pulumi_gestalt_rust::Output<
            super::super::types::appstream::FleetVpcConfig,
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
        let compute_capacity_binding = args.compute_capacity.get_output(context);
        let description_binding = args.description.get_output(context);
        let disconnect_timeout_in_seconds_binding = args
            .disconnect_timeout_in_seconds
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let domain_join_info_binding = args.domain_join_info.get_output(context);
        let enable_default_internet_access_binding = args
            .enable_default_internet_access
            .get_output(context);
        let fleet_type_binding = args.fleet_type.get_output(context);
        let iam_role_arn_binding = args.iam_role_arn.get_output(context);
        let idle_disconnect_timeout_in_seconds_binding = args
            .idle_disconnect_timeout_in_seconds
            .get_output(context);
        let image_arn_binding = args.image_arn.get_output(context);
        let image_name_binding = args.image_name.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let max_sessions_per_instance_binding = args
            .max_sessions_per_instance
            .get_output(context);
        let max_user_duration_in_seconds_binding = args
            .max_user_duration_in_seconds
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let stream_view_binding = args.stream_view.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appstream/fleet:Fleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeCapacity".into(),
                    value: &compute_capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disconnectTimeoutInSeconds".into(),
                    value: &disconnect_timeout_in_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainJoinInfo".into(),
                    value: &domain_join_info_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableDefaultInternetAccess".into(),
                    value: &enable_default_internet_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleetType".into(),
                    value: &fleet_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idleDisconnectTimeoutInSeconds".into(),
                    value: &idle_disconnect_timeout_in_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageArn".into(),
                    value: &image_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxSessionsPerInstance".into(),
                    value: &max_sessions_per_instance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxUserDurationInSeconds".into(),
                    value: &max_user_duration_in_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamView".into(),
                    value: &stream_view_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FleetResult {
            arn: o.get_field("arn"),
            compute_capacity: o.get_field("computeCapacity"),
            created_time: o.get_field("createdTime"),
            description: o.get_field("description"),
            disconnect_timeout_in_seconds: o.get_field("disconnectTimeoutInSeconds"),
            display_name: o.get_field("displayName"),
            domain_join_info: o.get_field("domainJoinInfo"),
            enable_default_internet_access: o.get_field("enableDefaultInternetAccess"),
            fleet_type: o.get_field("fleetType"),
            iam_role_arn: o.get_field("iamRoleArn"),
            idle_disconnect_timeout_in_seconds: o
                .get_field("idleDisconnectTimeoutInSeconds"),
            image_arn: o.get_field("imageArn"),
            image_name: o.get_field("imageName"),
            instance_type: o.get_field("instanceType"),
            max_sessions_per_instance: o.get_field("maxSessionsPerInstance"),
            max_user_duration_in_seconds: o.get_field("maxUserDurationInSeconds"),
            name: o.get_field("name"),
            state: o.get_field("state"),
            stream_view: o.get_field("streamView"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_config: o.get_field("vpcConfig"),
        }
    }
}
