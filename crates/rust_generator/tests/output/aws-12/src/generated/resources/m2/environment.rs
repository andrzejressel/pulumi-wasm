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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        #[builder(into, default)]
        pub apply_changes_during_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Engine type must be `microfocus` or `bluage`.
        #[builder(into)]
        pub engine_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The specific version of the engine for the Environment.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Force update the environment even if applications are running.
        #[builder(into, default)]
        pub force_update: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub high_availability_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::m2::EnvironmentHighAvailabilityConfig>,
        >,
        /// M2 Instance Type.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the KMS key to use for the Environment.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the runtime environment. Must be unique within the account.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configures the maintenance window that you want for the runtime environment. The maintenance window must have the format `ddd:hh24:mi-ddd:hh24:mi` and must be less than 24 hours. If not provided a random value will be used.
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Allow applications deployed to this environment to be publicly accessible.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of security group ids.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub storage_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::m2::EnvironmentStorageConfiguration>,
        >,
        /// List of subnet ids to deploy environment to.
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-value tags for the place index. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::m2::EnvironmentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        pub apply_changes_during_maintenance_window: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// ARN of the Environment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Engine type must be `microfocus` or `bluage`.
        pub engine_type: pulumi_gestalt_rust::Output<String>,
        /// The specific version of the engine for the Environment.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The id of the Environment.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// Force update the environment even if applications are running.
        pub force_update: pulumi_gestalt_rust::Output<Option<bool>>,
        pub high_availability_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::m2::EnvironmentHighAvailabilityConfig>,
        >,
        /// M2 Instance Type.
        ///
        /// The following arguments are optional:
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the KMS key to use for the Environment.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the load balancer created by the Environment.
        pub load_balancer_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the runtime environment. Must be unique within the account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configures the maintenance window that you want for the runtime environment. The maintenance window must have the format `ddd:hh24:mi-ddd:hh24:mi` and must be less than 24 hours. If not provided a random value will be used.
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Allow applications deployed to this environment to be publicly accessible.
        pub publicly_accessible: pulumi_gestalt_rust::Output<bool>,
        /// List of security group ids.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub storage_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::m2::EnvironmentStorageConfiguration>,
        >,
        /// List of subnet ids to deploy environment to.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value tags for the place index. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::m2::EnvironmentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let apply_changes_during_maintenance_window_binding = args
            .apply_changes_during_maintenance_window
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let engine_type_binding = args.engine_type.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let force_update_binding = args.force_update.get_output(context);
        let high_availability_config_binding = args
            .high_availability_config
            .get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context);
        let publicly_accessible_binding = args.publicly_accessible.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let storage_configuration_binding = args
            .storage_configuration
            .get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:m2/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyChangesDuringMaintenanceWindow".into(),
                    value: &apply_changes_during_maintenance_window_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceUpdate".into(),
                    value: &force_update_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "highAvailabilityConfig".into(),
                    value: &high_availability_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: &preferred_maintenance_window_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publiclyAccessible".into(),
                    value: &publicly_accessible_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageConfiguration".into(),
                    value: &storage_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentResult {
            apply_changes_during_maintenance_window: o
                .get_field("applyChangesDuringMaintenanceWindow"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            engine_type: o.get_field("engineType"),
            engine_version: o.get_field("engineVersion"),
            environment_id: o.get_field("environmentId"),
            force_update: o.get_field("forceUpdate"),
            high_availability_config: o.get_field("highAvailabilityConfig"),
            instance_type: o.get_field("instanceType"),
            kms_key_id: o.get_field("kmsKeyId"),
            load_balancer_arn: o.get_field("loadBalancerArn"),
            name: o.get_field("name"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            security_group_ids: o.get_field("securityGroupIds"),
            storage_configuration: o.get_field("storageConfiguration"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
