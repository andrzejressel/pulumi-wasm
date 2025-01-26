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
pub mod replication_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationInstanceArgs {
        /// The amount of storage (in gigabytes) to be initially allocated for the replication instance.
        #[builder(into, default)]
        pub allocated_storage: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Indicates that major version upgrades are allowed.
        #[builder(into, default)]
        pub allow_major_version_upgrade: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Indicates whether the changes should be applied immediately or during the next maintenance window. Only used when updating an existing resource.
        #[builder(into, default)]
        pub apply_immediately: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Indicates that minor engine upgrades will be applied automatically to the replication instance during the maintenance window.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The EC2 Availability Zone that the replication instance will be created in.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The engine version number of the replication instance.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the KMS key that will be used to encrypt the connection parameters. If you do not specify a value for `kms_key_arn`, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies if the replication instance is a multi-az deployment. You cannot set the `availability_zone` parameter if the `multi_az` parameter is set to `true`.
        #[builder(into, default)]
        pub multi_az: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The type of IP address protocol used by a replication instance. Valid values: `IPV4`, `DUAL`.
        #[builder(into, default)]
        pub network_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
        ///
        /// - Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week.
        /// - Format: `ddd:hh24:mi-ddd:hh24:mi`
        /// - Valid Days: `mon, tue, wed, thu, fri, sat, sun`
        /// - Constraints: Minimum 30-minute window.
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the accessibility options for the replication instance. A value of true represents an instance with a public IP address. A value of false represents an instance with a private IP address.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The compute and memory capacity of the replication instance as specified by the replication instance class. See [AWS DMS User Guide](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html) for available instance sizes and advice on which one to choose.
        #[builder(into)]
        pub replication_instance_class: pulumi_wasm_rust::InputOrOutput<String>,
        /// The replication instance identifier. This parameter is stored as a lowercase string.
        ///
        /// - Must contain from 1 to 63 alphanumeric characters or hyphens.
        /// - First character must be a letter.
        /// - Cannot end with a hyphen
        /// - Cannot contain two consecutive hyphens.
        #[builder(into)]
        pub replication_instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A subnet group to associate with the replication instance.
        #[builder(into, default)]
        pub replication_subnet_group_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of VPC security group IDs to be used with the replication instance. The VPC security groups must work with the VPC containing the replication instance.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ReplicationInstanceResult {
        /// The amount of storage (in gigabytes) to be initially allocated for the replication instance.
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// Indicates that major version upgrades are allowed.
        pub allow_major_version_upgrade: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates whether the changes should be applied immediately or during the next maintenance window. Only used when updating an existing resource.
        pub apply_immediately: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates that minor engine upgrades will be applied automatically to the replication instance during the maintenance window.
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<bool>,
        /// The EC2 Availability Zone that the replication instance will be created in.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The engine version number of the replication instance.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the KMS key that will be used to encrypt the connection parameters. If you do not specify a value for `kms_key_arn`, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies if the replication instance is a multi-az deployment. You cannot set the `availability_zone` parameter if the `multi_az` parameter is set to `true`.
        pub multi_az: pulumi_wasm_rust::Output<bool>,
        /// The type of IP address protocol used by a replication instance. Valid values: `IPV4`, `DUAL`.
        pub network_type: pulumi_wasm_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
        ///
        /// - Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week.
        /// - Format: `ddd:hh24:mi-ddd:hh24:mi`
        /// - Valid Days: `mon, tue, wed, thu, fri, sat, sun`
        /// - Constraints: Minimum 30-minute window.
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Specifies the accessibility options for the replication instance. A value of true represents an instance with a public IP address. A value of false represents an instance with a private IP address.
        pub publicly_accessible: pulumi_wasm_rust::Output<bool>,
        /// The Amazon Resource Name (ARN) of the replication instance.
        pub replication_instance_arn: pulumi_wasm_rust::Output<String>,
        /// The compute and memory capacity of the replication instance as specified by the replication instance class. See [AWS DMS User Guide](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html) for available instance sizes and advice on which one to choose.
        pub replication_instance_class: pulumi_wasm_rust::Output<String>,
        /// The replication instance identifier. This parameter is stored as a lowercase string.
        ///
        /// - Must contain from 1 to 63 alphanumeric characters or hyphens.
        /// - First character must be a letter.
        /// - Cannot end with a hyphen
        /// - Cannot contain two consecutive hyphens.
        pub replication_instance_id: pulumi_wasm_rust::Output<String>,
        /// A list of the private IP addresses of the replication instance.
        pub replication_instance_private_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of the public IP addresses of the replication instance.
        pub replication_instance_public_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// A subnet group to associate with the replication instance.
        pub replication_subnet_group_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of VPC security group IDs to be used with the replication instance. The VPC security groups must work with the VPC containing the replication instance.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReplicationInstanceArgs,
    ) -> ReplicationInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allocated_storage_binding = args
            .allocated_storage
            .get_output(context)
            .get_inner();
        let allow_major_version_upgrade_binding = args
            .allow_major_version_upgrade
            .get_output(context)
            .get_inner();
        let apply_immediately_binding = args
            .apply_immediately
            .get_output(context)
            .get_inner();
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_output(context)
            .get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_output(context).get_inner();
        let multi_az_binding = args.multi_az.get_output(context).get_inner();
        let network_type_binding = args.network_type.get_output(context).get_inner();
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context)
            .get_inner();
        let publicly_accessible_binding = args
            .publicly_accessible
            .get_output(context)
            .get_inner();
        let replication_instance_class_binding = args
            .replication_instance_class
            .get_output(context)
            .get_inner();
        let replication_instance_id_binding = args
            .replication_instance_id
            .get_output(context)
            .get_inner();
        let replication_subnet_group_id_binding = args
            .replication_subnet_group_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dms/replicationInstance:ReplicationInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocatedStorage".into(),
                    value: &allocated_storage_binding,
                },
                register_interface::ObjectField {
                    name: "allowMajorVersionUpgrade".into(),
                    value: &allow_major_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding,
                },
                register_interface::ObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: &auto_minor_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "multiAz".into(),
                    value: &multi_az_binding,
                },
                register_interface::ObjectField {
                    name: "networkType".into(),
                    value: &network_type_binding,
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
                    name: "replicationInstanceClass".into(),
                    value: &replication_instance_class_binding,
                },
                register_interface::ObjectField {
                    name: "replicationInstanceId".into(),
                    value: &replication_instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "replicationSubnetGroupId".into(),
                    value: &replication_subnet_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocatedStorage".into(),
                },
                register_interface::ResultField {
                    name: "allowMajorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "applyImmediately".into(),
                },
                register_interface::ResultField {
                    name: "autoMinorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "multiAz".into(),
                },
                register_interface::ResultField {
                    name: "networkType".into(),
                },
                register_interface::ResultField {
                    name: "preferredMaintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstanceArn".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstanceClass".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstancePrivateIps".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstancePublicIps".into(),
                },
                register_interface::ResultField {
                    name: "replicationSubnetGroupId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReplicationInstanceResult {
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            allow_major_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowMajorVersionUpgrade").unwrap(),
            ),
            apply_immediately: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applyImmediately").unwrap(),
            ),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoMinorVersionUpgrade").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            multi_az: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAz").unwrap(),
            ),
            network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkType").unwrap(),
            ),
            preferred_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMaintenanceWindow").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            replication_instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstanceArn").unwrap(),
            ),
            replication_instance_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstanceClass").unwrap(),
            ),
            replication_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstanceId").unwrap(),
            ),
            replication_instance_private_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstancePrivateIps").unwrap(),
            ),
            replication_instance_public_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstancePublicIps").unwrap(),
            ),
            replication_subnet_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSubnetGroupId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
