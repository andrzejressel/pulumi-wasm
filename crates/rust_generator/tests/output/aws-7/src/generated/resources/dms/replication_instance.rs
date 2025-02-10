/// Provides a DMS (Data Migration Service) replication instance resource. DMS replication instances can be created, updated, deleted, and imported.
///
/// ## Example Usage
///
/// Create required roles and then create a DMS instance, setting the depends_on to the required role policy attachments.
///
/// ```yaml
/// resources:
///   dms-access-for-endpoint:
///     type: aws:iam:Role
///     properties:
///       assumeRolePolicy: ${dmsAssumeRole.json}
///       name: dms-access-for-endpoint
///   dms-access-for-endpoint-AmazonDMSRedshiftS3Role:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/service-role/AmazonDMSRedshiftS3Role
///       role: ${["dms-access-for-endpoint"].name}
///   dms-cloudwatch-logs-role:
///     type: aws:iam:Role
///     properties:
///       assumeRolePolicy: ${dmsAssumeRole.json}
///       name: dms-cloudwatch-logs-role
///   dms-cloudwatch-logs-role-AmazonDMSCloudWatchLogsRole:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/service-role/AmazonDMSCloudWatchLogsRole
///       role: ${["dms-cloudwatch-logs-role"].name}
///   dms-vpc-role:
///     type: aws:iam:Role
///     properties:
///       assumeRolePolicy: ${dmsAssumeRole.json}
///       name: dms-vpc-role
///   dms-vpc-role-AmazonDMSVPCManagementRole:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/service-role/AmazonDMSVPCManagementRole
///       role: ${["dms-vpc-role"].name}
///   # Create a new replication instance
///   test:
///     type: aws:dms:ReplicationInstance
///     properties:
///       allocatedStorage: 20
///       applyImmediately: true
///       autoMinorVersionUpgrade: true
///       availabilityZone: us-west-2c
///       engineVersion: 3.1.4
///       kmsKeyArn: arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
///       multiAz: false
///       preferredMaintenanceWindow: sun:10:30-sun:14:30
///       publiclyAccessible: true
///       replicationInstanceClass: dms.t2.micro
///       replicationInstanceId: test-dms-replication-instance-tf
///       replicationSubnetGroupId: ${["test-dms-replication-subnet-group-tf"].id}
///       tags:
///         Name: test
///       vpcSecurityGroupIds:
///         - sg-12345678
///     options:
///       dependsOn:
///         - ${["dms-access-for-endpoint-AmazonDMSRedshiftS3Role"]}
///         - ${["dms-cloudwatch-logs-role-AmazonDMSCloudWatchLogsRole"]}
///         - ${["dms-vpc-role-AmazonDMSVPCManagementRole"]}
/// variables:
///   # Database Migration Service requires the below IAM Roles to be created before
///   # replication instances can be created. See the DMS Documentation for
///   # additional information: https://docs.aws.amazon.com/dms/latest/userguide/security-iam.html#CHAP_Security.APIRole
///   #  * dms-vpc-role
///   #  * dms-cloudwatch-logs-role
///   #  * dms-access-for-endpoint
///   dmsAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - identifiers:
///                   - dms.amazonaws.com
///                 type: Service
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import replication instances using the `replication_instance_id`. For example:
///
/// ```sh
/// $ pulumi import aws:dms/replicationInstance:ReplicationInstance test test-dms-replication-instance-tf
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replication_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationInstanceArgs {
        /// The amount of storage (in gigabytes) to be initially allocated for the replication instance.
        #[builder(into, default)]
        pub allocated_storage: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Indicates that major version upgrades are allowed.
        #[builder(into, default)]
        pub allow_major_version_upgrade: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Indicates whether the changes should be applied immediately or during the next maintenance window. Only used when updating an existing resource.
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates that minor engine upgrades will be applied automatically to the replication instance during the maintenance window.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The EC2 Availability Zone that the replication instance will be created in.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The engine version number of the replication instance.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the KMS key that will be used to encrypt the connection parameters. If you do not specify a value for `kms_key_arn`, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if the replication instance is a multi-az deployment. You cannot set the `availability_zone` parameter if the `multi_az` parameter is set to `true`.
        #[builder(into, default)]
        pub multi_az: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The type of IP address protocol used by a replication instance. Valid values: `IPV4`, `DUAL`.
        #[builder(into, default)]
        pub network_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
        ///
        /// - Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week.
        /// - Format: `ddd:hh24:mi-ddd:hh24:mi`
        /// - Valid Days: `mon, tue, wed, thu, fri, sat, sun`
        /// - Constraints: Minimum 30-minute window.
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the accessibility options for the replication instance. A value of true represents an instance with a public IP address. A value of false represents an instance with a private IP address.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The compute and memory capacity of the replication instance as specified by the replication instance class. See [AWS DMS User Guide](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html) for available instance sizes and advice on which one to choose.
        #[builder(into)]
        pub replication_instance_class: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The replication instance identifier. This parameter is stored as a lowercase string.
        ///
        /// - Must contain from 1 to 63 alphanumeric characters or hyphens.
        /// - First character must be a letter.
        /// - Cannot end with a hyphen
        /// - Cannot contain two consecutive hyphens.
        #[builder(into)]
        pub replication_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A subnet group to associate with the replication instance.
        #[builder(into, default)]
        pub replication_subnet_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of VPC security group IDs to be used with the replication instance. The VPC security groups must work with the VPC containing the replication instance.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicationInstanceResult {
        /// The amount of storage (in gigabytes) to be initially allocated for the replication instance.
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Indicates that major version upgrades are allowed.
        pub allow_major_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Indicates whether the changes should be applied immediately or during the next maintenance window. Only used when updating an existing resource.
        pub apply_immediately: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Indicates that minor engine upgrades will be applied automatically to the replication instance during the maintenance window.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<bool>,
        /// The EC2 Availability Zone that the replication instance will be created in.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The engine version number of the replication instance.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the KMS key that will be used to encrypt the connection parameters. If you do not specify a value for `kms_key_arn`, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the replication instance is a multi-az deployment. You cannot set the `availability_zone` parameter if the `multi_az` parameter is set to `true`.
        pub multi_az: pulumi_gestalt_rust::Output<bool>,
        /// The type of IP address protocol used by a replication instance. Valid values: `IPV4`, `DUAL`.
        pub network_type: pulumi_gestalt_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
        ///
        /// - Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week.
        /// - Format: `ddd:hh24:mi-ddd:hh24:mi`
        /// - Valid Days: `mon, tue, wed, thu, fri, sat, sun`
        /// - Constraints: Minimum 30-minute window.
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Specifies the accessibility options for the replication instance. A value of true represents an instance with a public IP address. A value of false represents an instance with a private IP address.
        pub publicly_accessible: pulumi_gestalt_rust::Output<bool>,
        /// The Amazon Resource Name (ARN) of the replication instance.
        pub replication_instance_arn: pulumi_gestalt_rust::Output<String>,
        /// The compute and memory capacity of the replication instance as specified by the replication instance class. See [AWS DMS User Guide](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html) for available instance sizes and advice on which one to choose.
        pub replication_instance_class: pulumi_gestalt_rust::Output<String>,
        /// The replication instance identifier. This parameter is stored as a lowercase string.
        ///
        /// - Must contain from 1 to 63 alphanumeric characters or hyphens.
        /// - First character must be a letter.
        /// - Cannot end with a hyphen
        /// - Cannot contain two consecutive hyphens.
        pub replication_instance_id: pulumi_gestalt_rust::Output<String>,
        /// A list of the private IP addresses of the replication instance.
        pub replication_instance_private_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of the public IP addresses of the replication instance.
        pub replication_instance_public_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A subnet group to associate with the replication instance.
        pub replication_subnet_group_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of VPC security group IDs to be used with the replication instance. The VPC security groups must work with the VPC containing the replication instance.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicationInstanceArgs,
    ) -> ReplicationInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allocated_storage_binding = args.allocated_storage.get_output(context);
        let allow_major_version_upgrade_binding = args
            .allow_major_version_upgrade
            .get_output(context);
        let apply_immediately_binding = args.apply_immediately.get_output(context);
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_output(context);
        let availability_zone_binding = args.availability_zone.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let multi_az_binding = args.multi_az.get_output(context);
        let network_type_binding = args.network_type.get_output(context);
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context);
        let publicly_accessible_binding = args.publicly_accessible.get_output(context);
        let replication_instance_class_binding = args
            .replication_instance_class
            .get_output(context);
        let replication_instance_id_binding = args
            .replication_instance_id
            .get_output(context);
        let replication_subnet_group_id_binding = args
            .replication_subnet_group_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dms/replicationInstance:ReplicationInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allocatedStorage".into(),
                    value: allocated_storage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowMajorVersionUpgrade".into(),
                    value: allow_major_version_upgrade_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyImmediately".into(),
                    value: apply_immediately_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: auto_minor_version_upgrade_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: engine_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiAz".into(),
                    value: multi_az_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkType".into(),
                    value: network_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: preferred_maintenance_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publiclyAccessible".into(),
                    value: publicly_accessible_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationInstanceClass".into(),
                    value: replication_instance_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationInstanceId".into(),
                    value: replication_instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationSubnetGroupId".into(),
                    value: replication_subnet_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: vpc_security_group_ids_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicationInstanceResult {
            allocated_storage: o.get_field("allocatedStorage"),
            allow_major_version_upgrade: o.get_field("allowMajorVersionUpgrade"),
            apply_immediately: o.get_field("applyImmediately"),
            auto_minor_version_upgrade: o.get_field("autoMinorVersionUpgrade"),
            availability_zone: o.get_field("availabilityZone"),
            engine_version: o.get_field("engineVersion"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            multi_az: o.get_field("multiAz"),
            network_type: o.get_field("networkType"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            replication_instance_arn: o.get_field("replicationInstanceArn"),
            replication_instance_class: o.get_field("replicationInstanceClass"),
            replication_instance_id: o.get_field("replicationInstanceId"),
            replication_instance_private_ips: o
                .get_field("replicationInstancePrivateIps"),
            replication_instance_public_ips: o.get_field("replicationInstancePublicIps"),
            replication_subnet_group_id: o.get_field("replicationSubnetGroupId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
