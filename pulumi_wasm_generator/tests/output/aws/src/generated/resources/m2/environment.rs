/// Resource for managing an [AWS Mainframe Modernization Environment](https://docs.aws.amazon.com/m2/latest/userguide/environments-m2.html).
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:m2:Environment
///     properties:
///       name: test-env
///       engineType: bluage
///       instanceType: M2.m5.large
///       securityGroups:
///         - sg-01234567890abcdef
///       subnetIds:
///         - subnet-01234567890abcdef
///         - subnet-01234567890abcdea
/// ```
///
/// ### High Availability
///
/// ```yaml
/// resources:
///   test:
///     type: aws:m2:Environment
///     properties:
///       name: test-env
///       engineType: bluage
///       instanceType: M2.m5.large
///       securityGroups:
///         - sg-01234567890abcdef
///       subnetIds:
///         - subnet-01234567890abcdef
///         - subnet-01234567890abcdea
///       highAvailabilityConfig:
///         desiredCapacity: 2
/// ```
///
/// ### EFS Filesystem
///
/// ```yaml
/// resources:
///   test:
///     type: aws:m2:Environment
///     properties:
///       name: test-env
///       engineType: bluage
///       instanceType: M2.m5.large
///       securityGroups:
///         - sg-01234567890abcdef
///       subnetIds:
///         - subnet-01234567890abcdef
///         - subnet-01234567890abcdea
///       storageConfiguration:
///         efs:
///           fileSystemId: fs-01234567890abcdef
///           mountPoint: /m2/mount/example
/// ```
///
/// ### FSX Filesystem
///
/// ```yaml
/// resources:
///   test:
///     type: aws:m2:Environment
///     properties:
///       name: test-env
///       engineType: bluage
///       instanceType: M2.m5.large
///       securityGroups:
///         - sg-01234567890abcdef
///       subnetIds:
///         - subnet-01234567890abcdef
///         - subnet-01234567890abcdea
///       storageConfiguration:
///         fsx:
///           fileSystemId: fs-01234567890abcdef
///           mountPoint: /m2/mount/example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Mainframe Modernization Environment using the `01234567890abcdef012345678`. For example:
///
/// ```sh
/// $ pulumi import aws:m2/environment:Environment example 01234567890abcdef012345678
/// ```
pub mod environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        #[builder(into, default)]
        pub apply_changes_during_maintenance_window: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Engine type must be `microfocus` or `bluage`.
        #[builder(into)]
        pub engine_type: pulumi_wasm_rust::Output<String>,
        /// The specific version of the engine for the Environment.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Force update the environment even if applications are running.
        #[builder(into, default)]
        pub force_update: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub high_availability_config: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::EnvironmentHighAvailabilityConfig>,
        >,
        /// M2 Instance Type.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the KMS key to use for the Environment.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the runtime environment. Must be unique within the account.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configures the maintenance window that you want for the runtime environment. The maintenance window must have the format `ddd:hh24:mi-ddd:hh24:mi` and must be less than 24 hours. If not provided a random value will be used.
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<Option<String>>,
        /// Allow applications deployed to this environment to be publicly accessible.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of security group ids.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub storage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::EnvironmentStorageConfiguration>,
        >,
        /// List of subnet ids to deploy environment to.
        #[builder(into, default)]
        pub subnet_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key-value tags for the place index. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::EnvironmentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        pub apply_changes_during_maintenance_window: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// ARN of the Environment.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Engine type must be `microfocus` or `bluage`.
        pub engine_type: pulumi_wasm_rust::Output<String>,
        /// The specific version of the engine for the Environment.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The id of the Environment.
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// Force update the environment even if applications are running.
        pub force_update: pulumi_wasm_rust::Output<Option<bool>>,
        pub high_availability_config: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::EnvironmentHighAvailabilityConfig>,
        >,
        /// M2 Instance Type.
        ///
        /// The following arguments are optional:
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the KMS key to use for the Environment.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the load balancer created by the Environment.
        pub load_balancer_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the runtime environment. Must be unique within the account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configures the maintenance window that you want for the runtime environment. The maintenance window must have the format `ddd:hh24:mi-ddd:hh24:mi` and must be less than 24 hours. If not provided a random value will be used.
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Allow applications deployed to this environment to be publicly accessible.
        pub publicly_accessible: pulumi_wasm_rust::Output<bool>,
        /// List of security group ids.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub storage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::EnvironmentStorageConfiguration>,
        >,
        /// List of subnet ids to deploy environment to.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value tags for the place index. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::EnvironmentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvironmentArgs) -> EnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let apply_changes_during_maintenance_window_binding = args
            .apply_changes_during_maintenance_window
            .get_inner();
        let description_binding = args.description.get_inner();
        let engine_type_binding = args.engine_type.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let force_update_binding = args.force_update.get_inner();
        let high_availability_config_binding = args.high_availability_config.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let name_binding = args.name.get_inner();
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_inner();
        let publicly_accessible_binding = args.publicly_accessible.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let storage_configuration_binding = args.storage_configuration.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:m2/environment:Environment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applyChangesDuringMaintenanceWindow".into(),
                    value: &apply_changes_during_maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "forceUpdate".into(),
                    value: &force_update_binding,
                },
                register_interface::ObjectField {
                    name: "highAvailabilityConfig".into(),
                    value: &high_availability_config_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: &preferred_maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "publiclyAccessible".into(),
                    value: &publicly_accessible_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "storageConfiguration".into(),
                    value: &storage_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applyChangesDuringMaintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "engineType".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "forceUpdate".into(),
                },
                register_interface::ResultField {
                    name: "highAvailabilityConfig".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "preferredMaintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "storageConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvironmentResult {
            apply_changes_during_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applyChangesDuringMaintenanceWindow").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            engine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineType").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            force_update: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceUpdate").unwrap(),
            ),
            high_availability_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("highAvailabilityConfig").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            load_balancer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            preferred_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMaintenanceWindow").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            storage_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageConfiguration").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
