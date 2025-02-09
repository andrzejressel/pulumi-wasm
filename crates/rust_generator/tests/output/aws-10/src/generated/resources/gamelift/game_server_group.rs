/// Provides an GameLift Game Server Group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = game_server_group::create(
///         "example",
///         GameServerGroupArgs::builder()
///             .game_server_group_name("example")
///             .instance_definitions(
///                 vec![
///                     GameServerGroupInstanceDefinition::builder().instanceType("c5.large")
///                     .build_struct(), GameServerGroupInstanceDefinition::builder()
///                     .instanceType("c5a.large").build_struct(),
///                 ],
///             )
///             .launch_template(
///                 GameServerGroupLaunchTemplate::builder()
///                     .id("${exampleAwsLaunchTemplate.id}")
///                     .build_struct(),
///             )
///             .max_size(1)
///             .min_size(1)
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Full usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:gamelift:GameServerGroup
///     properties:
///       autoScalingPolicy:
///         estimatedInstanceWarmup: 60
///         targetTrackingConfiguration:
///           targetValue: 75
///       balancingStrategy: SPOT_ONLY
///       gameServerGroupName: example
///       gameServerProtectionPolicy: FULL_PROTECTION
///       instanceDefinitions:
///         - instanceType: c5.large
///           weightedCapacity: '1'
///         - instanceType: c5.2xlarge
///           weightedCapacity: '2'
///       launchTemplate:
///         id: ${exampleAwsLaunchTemplate.id}
///         version: '1'
///       maxSize: 1
///       minSize: 1
///       roleArn: ${exampleAwsIamRole.arn}
///       tags:
///         Name: example
///       vpcSubnets:
///         - subnet-12345678
///         - subnet-23456789
///     options:
///       dependsOn:
///         - ${exampleAwsIamRolePolicyAttachment}
/// ```
///
/// ### Example IAM Role for GameLift Game Server Group
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       assumeRolePolicy: ${assumeRole.json}
///       name: gamelift-game-server-group-example
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       policyArn: arn:${current.partition}:iam::aws:policy/GameLiftGameServerGroupPolicy
///       role: ${example.name}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - autoscaling.amazonaws.com
///                   - gamelift.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GameLift Game Server Group using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:gamelift/gameServerGroup:GameServerGroup example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod game_server_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GameServerGroupArgs {
        #[builder(into, default)]
        pub auto_scaling_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gamelift::GameServerGroupAutoScalingPolicy>,
        >,
        /// Indicates how GameLift FleetIQ balances the use of Spot Instances and On-Demand Instances.
        /// Valid values: `SPOT_ONLY`, `SPOT_PREFERRED`, `ON_DEMAND_ONLY`. Defaults to `SPOT_PREFERRED`.
        #[builder(into, default)]
        pub balancing_strategy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the game server group.
        /// This value is used to generate unique ARN identifiers for the EC2 Auto Scaling group and the GameLift FleetIQ game server group.
        #[builder(into)]
        pub game_server_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether instances in the game server group are protected from early termination.
        /// Unprotected instances that have active game servers running might be terminated during a scale-down event,
        /// causing players to be dropped from the game.
        /// Protected instances cannot be terminated while there are active game servers running except in the event
        /// of a forced game server group deletion.
        /// Valid values: `NO_PROTECTION`, `FULL_PROTECTION`. Defaults to `NO_PROTECTION`.
        #[builder(into, default)]
        pub game_server_protection_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into)]
        pub instance_definitions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::gamelift::GameServerGroupInstanceDefinition>,
        >,
        #[builder(into)]
        pub launch_template: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gamelift::GameServerGroupLaunchTemplate,
        >,
        /// The maximum number of instances allowed in the EC2 Auto Scaling group.
        /// During automatic scaling events, GameLift FleetIQ and EC2 do not scale up the group above this maximum.
        #[builder(into)]
        pub max_size: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The minimum number of instances allowed in the EC2 Auto Scaling group.
        /// During automatic scaling events, GameLift FleetIQ and EC2 do not scale down the group below this minimum.
        #[builder(into)]
        pub min_size: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// ARN for an IAM role that allows Amazon GameLift to access your EC2 Auto Scaling groups.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of VPC subnets to use with instances in the game server group.
        /// By default, all GameLift FleetIQ-supported Availability Zones are used.
        #[builder(into, default)]
        pub vpc_subnets: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GameServerGroupResult {
        /// The ARN of the GameLift Game Server Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the created EC2 Auto Scaling group.
        pub auto_scaling_group_arn: pulumi_gestalt_rust::Output<String>,
        pub auto_scaling_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::gamelift::GameServerGroupAutoScalingPolicy>,
        >,
        /// Indicates how GameLift FleetIQ balances the use of Spot Instances and On-Demand Instances.
        /// Valid values: `SPOT_ONLY`, `SPOT_PREFERRED`, `ON_DEMAND_ONLY`. Defaults to `SPOT_PREFERRED`.
        pub balancing_strategy: pulumi_gestalt_rust::Output<String>,
        /// Name of the game server group.
        /// This value is used to generate unique ARN identifiers for the EC2 Auto Scaling group and the GameLift FleetIQ game server group.
        pub game_server_group_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether instances in the game server group are protected from early termination.
        /// Unprotected instances that have active game servers running might be terminated during a scale-down event,
        /// causing players to be dropped from the game.
        /// Protected instances cannot be terminated while there are active game servers running except in the event
        /// of a forced game server group deletion.
        /// Valid values: `NO_PROTECTION`, `FULL_PROTECTION`. Defaults to `NO_PROTECTION`.
        pub game_server_protection_policy: pulumi_gestalt_rust::Output<String>,
        pub instance_definitions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gamelift::GameServerGroupInstanceDefinition>,
        >,
        pub launch_template: pulumi_gestalt_rust::Output<
            super::super::types::gamelift::GameServerGroupLaunchTemplate,
        >,
        /// The maximum number of instances allowed in the EC2 Auto Scaling group.
        /// During automatic scaling events, GameLift FleetIQ and EC2 do not scale up the group above this maximum.
        pub max_size: pulumi_gestalt_rust::Output<i32>,
        /// The minimum number of instances allowed in the EC2 Auto Scaling group.
        /// During automatic scaling events, GameLift FleetIQ and EC2 do not scale down the group below this minimum.
        pub min_size: pulumi_gestalt_rust::Output<i32>,
        /// ARN for an IAM role that allows Amazon GameLift to access your EC2 Auto Scaling groups.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of VPC subnets to use with instances in the game server group.
        /// By default, all GameLift FleetIQ-supported Availability Zones are used.
        pub vpc_subnets: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GameServerGroupArgs,
    ) -> GameServerGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_scaling_policy_binding_1 = args.auto_scaling_policy.get_output(context);
        let auto_scaling_policy_binding = auto_scaling_policy_binding_1.get_inner();
        let balancing_strategy_binding_1 = args.balancing_strategy.get_output(context);
        let balancing_strategy_binding = balancing_strategy_binding_1.get_inner();
        let game_server_group_name_binding_1 = args
            .game_server_group_name
            .get_output(context);
        let game_server_group_name_binding = game_server_group_name_binding_1
            .get_inner();
        let game_server_protection_policy_binding_1 = args
            .game_server_protection_policy
            .get_output(context);
        let game_server_protection_policy_binding = game_server_protection_policy_binding_1
            .get_inner();
        let instance_definitions_binding_1 = args
            .instance_definitions
            .get_output(context);
        let instance_definitions_binding = instance_definitions_binding_1.get_inner();
        let launch_template_binding_1 = args.launch_template.get_output(context);
        let launch_template_binding = launch_template_binding_1.get_inner();
        let max_size_binding_1 = args.max_size.get_output(context);
        let max_size_binding = max_size_binding_1.get_inner();
        let min_size_binding_1 = args.min_size.get_output(context);
        let min_size_binding = min_size_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let vpc_subnets_binding_1 = args.vpc_subnets.get_output(context);
        let vpc_subnets_binding = vpc_subnets_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:gamelift/gameServerGroup:GameServerGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoScalingPolicy".into(),
                    value: &auto_scaling_policy_binding,
                },
                register_interface::ObjectField {
                    name: "balancingStrategy".into(),
                    value: &balancing_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "gameServerGroupName".into(),
                    value: &game_server_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "gameServerProtectionPolicy".into(),
                    value: &game_server_protection_policy_binding,
                },
                register_interface::ObjectField {
                    name: "instanceDefinitions".into(),
                    value: &instance_definitions_binding,
                },
                register_interface::ObjectField {
                    name: "launchTemplate".into(),
                    value: &launch_template_binding,
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
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSubnets".into(),
                    value: &vpc_subnets_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GameServerGroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_scaling_group_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoScalingGroupArn"),
            ),
            auto_scaling_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoScalingPolicy"),
            ),
            balancing_strategy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("balancingStrategy"),
            ),
            game_server_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gameServerGroupName"),
            ),
            game_server_protection_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gameServerProtectionPolicy"),
            ),
            instance_definitions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceDefinitions"),
            ),
            launch_template: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("launchTemplate"),
            ),
            max_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxSize"),
            ),
            min_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minSize"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_subnets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcSubnets"),
            ),
        }
    }
}
