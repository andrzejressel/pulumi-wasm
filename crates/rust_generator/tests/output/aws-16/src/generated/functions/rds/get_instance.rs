pub mod get_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// Name of the RDS instance.
        #[builder(into, default)]
        pub db_instance_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        /// Hostname of the RDS instance. See also `endpoint` and `port`.
        pub address: pulumi_gestalt_rust::Output<String>,
        /// Allocated storage size specified in gigabytes.
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Indicates that minor version patches are applied automatically.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<bool>,
        /// Name of the Availability Zone the DB instance is located in.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Specifies the number of days for which automatic DB snapshots are retained.
        pub backup_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// Identifier of the CA certificate for the DB instance.
        pub ca_cert_identifier: pulumi_gestalt_rust::Output<String>,
        /// If the DB instance is a member of a DB cluster, contains the name of the DB cluster that the DB instance is a member of.
        pub db_cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// ARN for the DB instance.
        pub db_instance_arn: pulumi_gestalt_rust::Output<String>,
        /// Contains the name of the compute and memory capacity class of the DB instance.
        pub db_instance_class: pulumi_gestalt_rust::Output<String>,
        pub db_instance_identifier: pulumi_gestalt_rust::Output<String>,
        /// Port that the DB instance listens on.
        pub db_instance_port: pulumi_gestalt_rust::Output<i32>,
        /// Contains the name of the initial database of this instance that was provided at create time, if one was specified when the DB instance was created. This same name is returned for the life of the DB instance.
        pub db_name: pulumi_gestalt_rust::Output<String>,
        /// Provides the list of DB parameter groups applied to this DB instance.
        pub db_parameter_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the subnet group associated with the DB instance.
        pub db_subnet_group: pulumi_gestalt_rust::Output<String>,
        /// List of log types to export to cloudwatch.
        pub enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Connection endpoint in `address:port` format.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Provides the name of the database engine to be used for this DB instance.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Database engine version.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Canonical hosted zone ID of the DB instance (to be used in a Route 53 Alias record).
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Provisioned IOPS (I/O operations per second) value.
        pub iops: pulumi_gestalt_rust::Output<i32>,
        /// The Amazon Web Services KMS key identifier that is used to encrypt the secret.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// License model information for this DB instance.
        pub license_model: pulumi_gestalt_rust::Output<String>,
        /// Provides the master user secret. Only available when `manage_master_user_password` is set to true. Documented below.
        pub master_user_secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::rds::GetInstanceMasterUserSecret>,
        >,
        /// Contains the master username for the DB instance.
        pub master_username: pulumi_gestalt_rust::Output<String>,
        /// The upper limit to which Amazon RDS can automatically scale the storage of the DB instance.
        pub max_allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance.
        pub monitoring_interval: pulumi_gestalt_rust::Output<i32>,
        /// ARN for the IAM role that permits RDS to send Enhanced Monitoring metrics to CloudWatch Logs.
        pub monitoring_role_arn: pulumi_gestalt_rust::Output<String>,
        /// If the DB instance is a Multi-AZ deployment.
        pub multi_az: pulumi_gestalt_rust::Output<bool>,
        /// Network type of the DB instance.
        pub network_type: pulumi_gestalt_rust::Output<String>,
        /// Provides the list of option group memberships for this DB instance.
        pub option_group_memberships: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Database endpoint port, primarily used by an Aurora DB cluster. For a conventional RDS DB instance, the `db_instance_port` is typically the preferred choice.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the daily time range during which automated backups are created.
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        /// Specifies the weekly time range during which system maintenance can occur in UTC.
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Accessibility options for the DB instance.
        pub publicly_accessible: pulumi_gestalt_rust::Output<bool>,
        /// Identifier of the source DB that this is a replica of.
        pub replicate_source_db: pulumi_gestalt_rust::Output<String>,
        /// RDS Resource ID of this instance.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the DB instance is encrypted.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Storage throughput value for the DB instance.
        pub storage_throughput: pulumi_gestalt_rust::Output<i32>,
        /// Storage type associated with DB instance.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Time zone of the DB instance.
        pub timezone: pulumi_gestalt_rust::Output<String>,
        /// Provides a list of VPC security group elements that the DB instance belongs to.
        pub vpc_security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let db_instance_identifier_binding = args
            .db_instance_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbInstanceIdentifier".into(),
                    value: &db_instance_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceResult {
            address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("address"),
            ),
            allocated_storage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocatedStorage"),
            ),
            auto_minor_version_upgrade: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoMinorVersionUpgrade"),
            ),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            backup_retention_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupRetentionPeriod"),
            ),
            ca_cert_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("caCertIdentifier"),
            ),
            db_cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbClusterIdentifier"),
            ),
            db_instance_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbInstanceArn"),
            ),
            db_instance_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbInstanceClass"),
            ),
            db_instance_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbInstanceIdentifier"),
            ),
            db_instance_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbInstancePort"),
            ),
            db_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbName"),
            ),
            db_parameter_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbParameterGroups"),
            ),
            db_subnet_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbSubnetGroup"),
            ),
            enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabledCloudwatchLogsExports"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            hosted_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostedZoneId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            iops: pulumi_gestalt_rust::__private::into_domain(o.extract_field("iops")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            license_model: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseModel"),
            ),
            master_user_secrets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterUserSecrets"),
            ),
            master_username: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterUsername"),
            ),
            max_allocated_storage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxAllocatedStorage"),
            ),
            monitoring_interval: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringInterval"),
            ),
            monitoring_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringRoleArn"),
            ),
            multi_az: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiAz"),
            ),
            network_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkType"),
            ),
            option_group_memberships: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("optionGroupMemberships"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            preferred_backup_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredBackupWindow"),
            ),
            preferred_maintenance_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredMaintenanceWindow"),
            ),
            publicly_accessible: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publiclyAccessible"),
            ),
            replicate_source_db: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicateSourceDb"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            storage_encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageEncrypted"),
            ),
            storage_throughput: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageThroughput"),
            ),
            storage_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            timezone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timezone"),
            ),
            vpc_security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcSecurityGroups"),
            ),
        }
    }
}
