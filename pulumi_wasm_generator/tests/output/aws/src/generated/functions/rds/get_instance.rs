pub mod get_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// Name of the RDS instance.
        #[builder(into, default)]
        pub db_instance_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired instance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        /// Hostname of the RDS instance. See also `endpoint` and `port`.
        pub address: pulumi_wasm_rust::Output<String>,
        /// Allocated storage size specified in gigabytes.
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// Indicates that minor version patches are applied automatically.
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<bool>,
        /// Name of the Availability Zone the DB instance is located in.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Specifies the number of days for which automatic DB snapshots are retained.
        pub backup_retention_period: pulumi_wasm_rust::Output<i32>,
        /// Identifier of the CA certificate for the DB instance.
        pub ca_cert_identifier: pulumi_wasm_rust::Output<String>,
        /// If the DB instance is a member of a DB cluster, contains the name of the DB cluster that the DB instance is a member of.
        pub db_cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// ARN for the DB instance.
        pub db_instance_arn: pulumi_wasm_rust::Output<String>,
        /// Contains the name of the compute and memory capacity class of the DB instance.
        pub db_instance_class: pulumi_wasm_rust::Output<String>,
        pub db_instance_identifier: pulumi_wasm_rust::Output<String>,
        /// Port that the DB instance listens on.
        pub db_instance_port: pulumi_wasm_rust::Output<i32>,
        /// Contains the name of the initial database of this instance that was provided at create time, if one was specified when the DB instance was created. This same name is returned for the life of the DB instance.
        pub db_name: pulumi_wasm_rust::Output<String>,
        /// Provides the list of DB parameter groups applied to this DB instance.
        pub db_parameter_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the subnet group associated with the DB instance.
        pub db_subnet_group: pulumi_wasm_rust::Output<String>,
        /// List of log types to export to cloudwatch.
        pub enabled_cloudwatch_logs_exports: pulumi_wasm_rust::Output<Vec<String>>,
        /// Connection endpoint in `address:port` format.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Provides the name of the database engine to be used for this DB instance.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Database engine version.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// Canonical hosted zone ID of the DB instance (to be used in a Route 53 Alias record).
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Provisioned IOPS (I/O operations per second) value.
        pub iops: pulumi_wasm_rust::Output<i32>,
        /// The Amazon Web Services KMS key identifier that is used to encrypt the secret.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// License model information for this DB instance.
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// Provides the master user secret. Only available when `manage_master_user_password` is set to true. Documented below.
        pub master_user_secrets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::rds::GetInstanceMasterUserSecret>,
        >,
        /// Contains the master username for the DB instance.
        pub master_username: pulumi_wasm_rust::Output<String>,
        /// The upper limit to which Amazon RDS can automatically scale the storage of the DB instance.
        pub max_allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// Interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance.
        pub monitoring_interval: pulumi_wasm_rust::Output<i32>,
        /// ARN for the IAM role that permits RDS to send Enhanced Monitoring metrics to CloudWatch Logs.
        pub monitoring_role_arn: pulumi_wasm_rust::Output<String>,
        /// If the DB instance is a Multi-AZ deployment.
        pub multi_az: pulumi_wasm_rust::Output<bool>,
        /// Network type of the DB instance.
        pub network_type: pulumi_wasm_rust::Output<String>,
        /// Provides the list of option group memberships for this DB instance.
        pub option_group_memberships: pulumi_wasm_rust::Output<Vec<String>>,
        /// Database endpoint port, primarily used by an Aurora DB cluster. For a conventional RDS DB instance, the `db_instance_port` is typically the preferred choice.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Specifies the daily time range during which automated backups are created.
        pub preferred_backup_window: pulumi_wasm_rust::Output<String>,
        /// Specifies the weekly time range during which system maintenance can occur in UTC.
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Accessibility options for the DB instance.
        pub publicly_accessible: pulumi_wasm_rust::Output<bool>,
        /// Identifier of the source DB that this is a replica of.
        pub replicate_source_db: pulumi_wasm_rust::Output<String>,
        /// RDS Resource ID of this instance.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// Whether the DB instance is encrypted.
        pub storage_encrypted: pulumi_wasm_rust::Output<bool>,
        /// Storage throughput value for the DB instance.
        pub storage_throughput: pulumi_wasm_rust::Output<i32>,
        /// Storage type associated with DB instance.
        pub storage_type: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Time zone of the DB instance.
        pub timezone: pulumi_wasm_rust::Output<String>,
        /// Provides a list of VPC security group elements that the DB instance belongs to.
        pub vpc_security_groups: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetInstanceArgs) -> GetInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_instance_identifier_binding = args.db_instance_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getInstance:getInstance".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "address".into(),
                },
                register_interface::ResultField {
                    name: "allocatedStorage".into(),
                },
                register_interface::ResultField {
                    name: "autoMinorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "backupRetentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "caCertIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbInstanceArn".into(),
                },
                register_interface::ResultField {
                    name: "dbInstanceClass".into(),
                },
                register_interface::ResultField {
                    name: "dbInstanceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbInstancePort".into(),
                },
                register_interface::ResultField {
                    name: "dbName".into(),
                },
                register_interface::ResultField {
                    name: "dbParameterGroups".into(),
                },
                register_interface::ResultField {
                    name: "dbSubnetGroup".into(),
                },
                register_interface::ResultField {
                    name: "enabledCloudwatchLogsExports".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "iops".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "licenseModel".into(),
                },
                register_interface::ResultField {
                    name: "masterUserSecrets".into(),
                },
                register_interface::ResultField {
                    name: "masterUsername".into(),
                },
                register_interface::ResultField {
                    name: "maxAllocatedStorage".into(),
                },
                register_interface::ResultField {
                    name: "monitoringInterval".into(),
                },
                register_interface::ResultField {
                    name: "monitoringRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "multiAz".into(),
                },
                register_interface::ResultField {
                    name: "networkType".into(),
                },
                register_interface::ResultField {
                    name: "optionGroupMemberships".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "preferredBackupWindow".into(),
                },
                register_interface::ResultField {
                    name: "preferredMaintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "replicateSourceDb".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "storageEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "storageThroughput".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timezone".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroups".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceResult {
            address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("address").unwrap(),
            ),
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoMinorVersionUpgrade").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            backup_retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRetentionPeriod").unwrap(),
            ),
            ca_cert_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caCertIdentifier").unwrap(),
            ),
            db_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterIdentifier").unwrap(),
            ),
            db_instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbInstanceArn").unwrap(),
            ),
            db_instance_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbInstanceClass").unwrap(),
            ),
            db_instance_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbInstanceIdentifier").unwrap(),
            ),
            db_instance_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbInstancePort").unwrap(),
            ),
            db_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbName").unwrap(),
            ),
            db_parameter_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbParameterGroups").unwrap(),
            ),
            db_subnet_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSubnetGroup").unwrap(),
            ),
            enabled_cloudwatch_logs_exports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledCloudwatchLogsExports").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            iops: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iops").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseModel").unwrap(),
            ),
            master_user_secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterUserSecrets").unwrap(),
            ),
            master_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterUsername").unwrap(),
            ),
            max_allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxAllocatedStorage").unwrap(),
            ),
            monitoring_interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringInterval").unwrap(),
            ),
            monitoring_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringRoleArn").unwrap(),
            ),
            multi_az: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAz").unwrap(),
            ),
            network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkType").unwrap(),
            ),
            option_group_memberships: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optionGroupMemberships").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            preferred_backup_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredBackupWindow").unwrap(),
            ),
            preferred_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMaintenanceWindow").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            replicate_source_db: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicateSourceDb").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            storage_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEncrypted").unwrap(),
            ),
            storage_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageThroughput").unwrap(),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            timezone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timezone").unwrap(),
            ),
            vpc_security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroups").unwrap(),
            ),
        }
    }
}