/// Provides a GameLift Fleet resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fleet::create(
///         "example",
///         FleetArgs::builder()
///             .build_id("${exampleAwsGameliftBuild.id}")
///             .ec_2_instance_type("t2.micro")
///             .fleet_type("ON_DEMAND")
///             .name("example-fleet-name")
///             .runtime_configuration(
///                 FleetRuntimeConfiguration::builder()
///                     .serverProcesses(
///                         vec![
///                             FleetRuntimeConfigurationServerProcess::builder()
///                             .concurrentExecutions(1)
///                             .launchPath("C:\\game\\GomokuServer.exe").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GameLift Fleets using the ID. For example:
///
/// ```sh
/// $ pulumi import aws:gamelift/fleet:Fleet example <fleet-id>
/// ```
pub mod fleet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// ID of the GameLift Build to be deployed on the fleet.
        #[builder(into, default)]
        pub build_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Prompts GameLift to generate a TLS/SSL certificate for the fleet. See certificate_configuration.
        #[builder(into, default)]
        pub certificate_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::gamelift::FleetCertificateConfiguration>,
        >,
        /// Human-readable description of the fleet.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Range of IP addresses and port settings that permit inbound traffic to access server processes running on the fleet. See below.
        #[builder(into, default)]
        pub ec2_inbound_permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::gamelift::FleetEc2InboundPermission>>,
        >,
        /// Name of an EC2 instance typeE.g., `t2.micro`
        #[builder(into)]
        pub ec2_instance_type: pulumi_wasm_rust::Output<String>,
        /// Type of fleet. This value must be `ON_DEMAND` or `SPOT`. Defaults to `ON_DEMAND`.
        #[builder(into, default)]
        pub fleet_type: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of an IAM role that instances in the fleet can assume.
        #[builder(into, default)]
        pub instance_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// List of names of metric groups to add this fleet to. A metric group tracks metrics across all fleets in the group. Defaults to `default`.
        #[builder(into, default)]
        pub metric_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the fleet.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Game session protection policy to apply to all instances in this fleetE.g., `FullProtection`. Defaults to `NoProtection`.
        #[builder(into, default)]
        pub new_game_session_protection_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Policy that limits the number of game sessions an individual player can create over a span of time for this fleet. See below.
        #[builder(into, default)]
        pub resource_creation_limit_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::gamelift::FleetResourceCreationLimitPolicy>,
        >,
        /// Instructions for launching server processes on each instance in the fleet. See below.
        #[builder(into, default)]
        pub runtime_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::gamelift::FleetRuntimeConfiguration>,
        >,
        /// ID of the GameLift Script to be deployed on the fleet.
        #[builder(into, default)]
        pub script_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// Fleet ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Build ARN.
        pub build_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the GameLift Build to be deployed on the fleet.
        pub build_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Prompts GameLift to generate a TLS/SSL certificate for the fleet. See certificate_configuration.
        pub certificate_configuration: pulumi_wasm_rust::Output<
            super::super::types::gamelift::FleetCertificateConfiguration,
        >,
        /// Human-readable description of the fleet.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Range of IP addresses and port settings that permit inbound traffic to access server processes running on the fleet. See below.
        pub ec2_inbound_permissions: pulumi_wasm_rust::Output<
            Vec<super::super::types::gamelift::FleetEc2InboundPermission>,
        >,
        /// Name of an EC2 instance typeE.g., `t2.micro`
        pub ec2_instance_type: pulumi_wasm_rust::Output<String>,
        /// Type of fleet. This value must be `ON_DEMAND` or `SPOT`. Defaults to `ON_DEMAND`.
        pub fleet_type: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of an IAM role that instances in the fleet can assume.
        pub instance_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        pub log_paths: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of names of metric groups to add this fleet to. A metric group tracks metrics across all fleets in the group. Defaults to `default`.
        pub metric_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the fleet.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Game session protection policy to apply to all instances in this fleetE.g., `FullProtection`. Defaults to `NoProtection`.
        pub new_game_session_protection_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Operating system of the fleet's computing resources.
        pub operating_system: pulumi_wasm_rust::Output<String>,
        /// Policy that limits the number of game sessions an individual player can create over a span of time for this fleet. See below.
        pub resource_creation_limit_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::gamelift::FleetResourceCreationLimitPolicy>,
        >,
        /// Instructions for launching server processes on each instance in the fleet. See below.
        pub runtime_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::gamelift::FleetRuntimeConfiguration>,
        >,
        /// Script ARN.
        pub script_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the GameLift Script to be deployed on the fleet.
        pub script_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: FleetArgs) -> FleetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let build_id_binding = args.build_id.get_inner();
        let certificate_configuration_binding = args
            .certificate_configuration
            .get_inner();
        let description_binding = args.description.get_inner();
        let ec2_inbound_permissions_binding = args.ec2_inbound_permissions.get_inner();
        let ec2_instance_type_binding = args.ec2_instance_type.get_inner();
        let fleet_type_binding = args.fleet_type.get_inner();
        let instance_role_arn_binding = args.instance_role_arn.get_inner();
        let metric_groups_binding = args.metric_groups.get_inner();
        let name_binding = args.name.get_inner();
        let new_game_session_protection_policy_binding = args
            .new_game_session_protection_policy
            .get_inner();
        let resource_creation_limit_policy_binding = args
            .resource_creation_limit_policy
            .get_inner();
        let runtime_configuration_binding = args.runtime_configuration.get_inner();
        let script_id_binding = args.script_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:gamelift/fleet:Fleet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "buildId".into(),
                    value: &build_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificateConfiguration".into(),
                    value: &certificate_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ec2InboundPermissions".into(),
                    value: &ec2_inbound_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "ec2InstanceType".into(),
                    value: &ec2_instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "fleetType".into(),
                    value: &fleet_type_binding,
                },
                register_interface::ObjectField {
                    name: "instanceRoleArn".into(),
                    value: &instance_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "metricGroups".into(),
                    value: &metric_groups_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "newGameSessionProtectionPolicy".into(),
                    value: &new_game_session_protection_policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceCreationLimitPolicy".into(),
                    value: &resource_creation_limit_policy_binding,
                },
                register_interface::ObjectField {
                    name: "runtimeConfiguration".into(),
                    value: &runtime_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "scriptId".into(),
                    value: &script_id_binding,
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
                    name: "buildArn".into(),
                },
                register_interface::ResultField {
                    name: "buildId".into(),
                },
                register_interface::ResultField {
                    name: "certificateConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "ec2InboundPermissions".into(),
                },
                register_interface::ResultField {
                    name: "ec2InstanceType".into(),
                },
                register_interface::ResultField {
                    name: "fleetType".into(),
                },
                register_interface::ResultField {
                    name: "instanceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "logPaths".into(),
                },
                register_interface::ResultField {
                    name: "metricGroups".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "newGameSessionProtectionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "operatingSystem".into(),
                },
                register_interface::ResultField {
                    name: "resourceCreationLimitPolicy".into(),
                },
                register_interface::ResultField {
                    name: "runtimeConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "scriptArn".into(),
                },
                register_interface::ResultField {
                    name: "scriptId".into(),
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
        FleetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            build_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildArn").unwrap(),
            ),
            build_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildId").unwrap(),
            ),
            certificate_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateConfiguration").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            ec2_inbound_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ec2InboundPermissions").unwrap(),
            ),
            ec2_instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ec2InstanceType").unwrap(),
            ),
            fleet_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetType").unwrap(),
            ),
            instance_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceRoleArn").unwrap(),
            ),
            log_paths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logPaths").unwrap(),
            ),
            metric_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricGroups").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            new_game_session_protection_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("newGameSessionProtectionPolicy").unwrap(),
            ),
            operating_system: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operatingSystem").unwrap(),
            ),
            resource_creation_limit_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceCreationLimitPolicy").unwrap(),
            ),
            runtime_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeConfiguration").unwrap(),
            ),
            script_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptArn").unwrap(),
            ),
            script_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptId").unwrap(),
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