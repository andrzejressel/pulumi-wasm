#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_instance_identifier_binding = args
            .db_instance_identifier
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbInstanceIdentifier".into(),
                    value: db_instance_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceResult {
            address: o.get_field("address"),
            allocated_storage: o.get_field("allocatedStorage"),
            auto_minor_version_upgrade: o.get_field("autoMinorVersionUpgrade"),
            availability_zone: o.get_field("availabilityZone"),
            backup_retention_period: o.get_field("backupRetentionPeriod"),
            ca_cert_identifier: o.get_field("caCertIdentifier"),
            db_cluster_identifier: o.get_field("dbClusterIdentifier"),
            db_instance_arn: o.get_field("dbInstanceArn"),
            db_instance_class: o.get_field("dbInstanceClass"),
            db_instance_identifier: o.get_field("dbInstanceIdentifier"),
            db_instance_port: o.get_field("dbInstancePort"),
            db_name: o.get_field("dbName"),
            db_parameter_groups: o.get_field("dbParameterGroups"),
            db_subnet_group: o.get_field("dbSubnetGroup"),
            enabled_cloudwatch_logs_exports: o.get_field("enabledCloudwatchLogsExports"),
            endpoint: o.get_field("endpoint"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            hosted_zone_id: o.get_field("hostedZoneId"),
            id: o.get_field("id"),
            iops: o.get_field("iops"),
            kms_key_id: o.get_field("kmsKeyId"),
            license_model: o.get_field("licenseModel"),
            master_user_secrets: o.get_field("masterUserSecrets"),
            master_username: o.get_field("masterUsername"),
            max_allocated_storage: o.get_field("maxAllocatedStorage"),
            monitoring_interval: o.get_field("monitoringInterval"),
            monitoring_role_arn: o.get_field("monitoringRoleArn"),
            multi_az: o.get_field("multiAz"),
            network_type: o.get_field("networkType"),
            option_group_memberships: o.get_field("optionGroupMemberships"),
            port: o.get_field("port"),
            preferred_backup_window: o.get_field("preferredBackupWindow"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            replicate_source_db: o.get_field("replicateSourceDb"),
            resource_id: o.get_field("resourceId"),
            storage_encrypted: o.get_field("storageEncrypted"),
            storage_throughput: o.get_field("storageThroughput"),
            storage_type: o.get_field("storageType"),
            tags: o.get_field("tags"),
            timezone: o.get_field("timezone"),
            vpc_security_groups: o.get_field("vpcSecurityGroups"),
        }
    }
}
