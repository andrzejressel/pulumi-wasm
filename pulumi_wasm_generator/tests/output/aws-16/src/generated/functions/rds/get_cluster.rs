pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Cluster identifier of the RDS cluster.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        pub backtrack_window: pulumi_wasm_rust::Output<i32>,
        pub backup_retention_period: pulumi_wasm_rust::Output<i32>,
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        pub cluster_members: pulumi_wasm_rust::Output<Vec<String>>,
        pub cluster_resource_id: pulumi_wasm_rust::Output<String>,
        pub database_name: pulumi_wasm_rust::Output<String>,
        pub db_cluster_parameter_group_name: pulumi_wasm_rust::Output<String>,
        pub db_subnet_group_name: pulumi_wasm_rust::Output<String>,
        pub db_system_id: pulumi_wasm_rust::Output<String>,
        pub enabled_cloudwatch_logs_exports: pulumi_wasm_rust::Output<Vec<String>>,
        pub endpoint: pulumi_wasm_rust::Output<String>,
        pub engine: pulumi_wasm_rust::Output<String>,
        pub engine_mode: pulumi_wasm_rust::Output<String>,
        pub engine_version: pulumi_wasm_rust::Output<String>,
        pub final_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        pub iam_database_authentication_enabled: pulumi_wasm_rust::Output<bool>,
        pub iam_roles: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        pub master_user_secrets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::rds::GetClusterMasterUserSecret>,
        >,
        pub master_username: pulumi_wasm_rust::Output<String>,
        pub network_type: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<i32>,
        pub preferred_backup_window: pulumi_wasm_rust::Output<String>,
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        pub reader_endpoint: pulumi_wasm_rust::Output<String>,
        pub replication_source_identifier: pulumi_wasm_rust::Output<String>,
        pub storage_encrypted: pulumi_wasm_rust::Output<bool>,
        /// A map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_identifier_binding = args
            .cluster_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
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
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "backtrackWindow".into(),
                },
                register_interface::ResultField {
                    name: "backupRetentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "clusterMembers".into(),
                },
                register_interface::ResultField {
                    name: "clusterResourceId".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterParameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "dbSubnetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "dbSystemId".into(),
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
                    name: "engineMode".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "finalSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "iamDatabaseAuthenticationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "iamRoles".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "masterUserSecrets".into(),
                },
                register_interface::ResultField {
                    name: "masterUsername".into(),
                },
                register_interface::ResultField {
                    name: "networkType".into(),
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
                    name: "readerEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "replicationSourceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "storageEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            backtrack_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backtrackWindow").unwrap(),
            ),
            backup_retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRetentionPeriod").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            cluster_members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterMembers").unwrap(),
            ),
            cluster_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterResourceId").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            db_cluster_parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterParameterGroupName").unwrap(),
            ),
            db_subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSubnetGroupName").unwrap(),
            ),
            db_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSystemId").unwrap(),
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
            engine_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineMode").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            final_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalSnapshotIdentifier").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            iam_database_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamDatabaseAuthenticationEnabled").unwrap(),
            ),
            iam_roles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoles").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            master_user_secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterUserSecrets").unwrap(),
            ),
            master_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterUsername").unwrap(),
            ),
            network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkType").unwrap(),
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
            reader_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readerEndpoint").unwrap(),
            ),
            replication_source_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSourceIdentifier").unwrap(),
            ),
            storage_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEncrypted").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
