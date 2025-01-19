pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Cluster identifier
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// Tags associated to the cluster
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// Whether major version upgrades can be applied during maintenance period
        pub allow_version_upgrade: pulumi_wasm_rust::Output<bool>,
        /// The value represents how the cluster is configured to use AQUA.
        pub aqua_configuration_status: pulumi_wasm_rust::Output<String>,
        /// ARN of cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The backup retention period
        pub automated_snapshot_retention_period: pulumi_wasm_rust::Output<i32>,
        /// Availability zone of the cluster
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the cluster is able to be relocated to another availability zone.
        pub availability_zone_relocation_enabled: pulumi_wasm_rust::Output<bool>,
        /// Name of the S3 bucket where the log files are to be stored
        pub bucket_name: pulumi_wasm_rust::Output<String>,
        /// Cluster identifier
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The namespace Amazon Resource Name (ARN) of the cluster
        pub cluster_namespace_arn: pulumi_wasm_rust::Output<String>,
        /// Nodes in the cluster. Cluster node blocks are documented below
        pub cluster_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redshift::GetClusterClusterNode>,
        >,
        /// The name of the parameter group to be associated with this cluster
        pub cluster_parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// Public key for the cluster
        pub cluster_public_key: pulumi_wasm_rust::Output<String>,
        /// The cluster revision number
        pub cluster_revision_number: pulumi_wasm_rust::Output<String>,
        /// The name of a cluster subnet group to be associated with this cluster
        pub cluster_subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// Cluster type
        pub cluster_type: pulumi_wasm_rust::Output<String>,
        pub cluster_version: pulumi_wasm_rust::Output<String>,
        /// Name of the default database in the cluster
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The ARN for the IAM role that was set as default for the cluster when the cluster was created.
        pub default_iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// Elastic IP of the cluster
        pub elastic_ip: pulumi_wasm_rust::Output<String>,
        /// Whether cluster logging is enabled
        pub enable_logging: pulumi_wasm_rust::Output<bool>,
        /// Whether the cluster data is encrypted
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        /// Cluster endpoint
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Whether enhanced VPC routing is enabled
        pub enhanced_vpc_routing: pulumi_wasm_rust::Output<bool>,
        /// IAM roles associated to the cluster
        pub iam_roles: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// KMS encryption key associated to the cluster
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The log destination type.
        pub log_destination_type: pulumi_wasm_rust::Output<String>,
        /// Collection of exported log types. Log types include the connection log, user log and user activity log.
        pub log_exports: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the maintenance track for the restored cluster.
        pub maintenance_track_name: pulumi_wasm_rust::Output<String>,
        /// (Optional)  The default number of days to retain a manual snapshot.
        pub manual_snapshot_retention_period: pulumi_wasm_rust::Output<i32>,
        /// Username for the master DB user
        pub master_username: pulumi_wasm_rust::Output<String>,
        /// If the cluster is a Multi-AZ deployment
        pub multi_az: pulumi_wasm_rust::Output<bool>,
        /// Cluster node type
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// Number of nodes in the cluster
        pub number_of_nodes: pulumi_wasm_rust::Output<i32>,
        /// Port the cluster responds on
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The maintenance window
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Whether the cluster is publicly accessible
        pub publicly_accessible: pulumi_wasm_rust::Output<bool>,
        /// Folder inside the S3 bucket where the log files are stored
        pub s3_key_prefix: pulumi_wasm_rust::Output<String>,
        /// Tags associated to the cluster
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC Id associated with the cluster
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// The VPC security group Ids associated with the cluster
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetClusterArgs) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_identifier_binding = args.cluster_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshift/getCluster:getCluster".into(),
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
                    name: "allowVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "aquaConfigurationStatus".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "automatedSnapshotRetentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneRelocationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "bucketName".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "clusterNamespaceArn".into(),
                },
                register_interface::ResultField {
                    name: "clusterNodes".into(),
                },
                register_interface::ResultField {
                    name: "clusterParameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "clusterPublicKey".into(),
                },
                register_interface::ResultField {
                    name: "clusterRevisionNumber".into(),
                },
                register_interface::ResultField {
                    name: "clusterSubnetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "clusterType".into(),
                },
                register_interface::ResultField {
                    name: "clusterVersion".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "defaultIamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "elasticIp".into(),
                },
                register_interface::ResultField {
                    name: "enableLogging".into(),
                },
                register_interface::ResultField {
                    name: "encrypted".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "enhancedVpcRouting".into(),
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
                    name: "logDestinationType".into(),
                },
                register_interface::ResultField {
                    name: "logExports".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceTrackName".into(),
                },
                register_interface::ResultField {
                    name: "manualSnapshotRetentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "masterUsername".into(),
                },
                register_interface::ResultField {
                    name: "multiAz".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "numberOfNodes".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "preferredMaintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "s3KeyPrefix".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            allow_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowVersionUpgrade").unwrap(),
            ),
            aqua_configuration_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aquaConfigurationStatus").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            automated_snapshot_retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automatedSnapshotRetentionPeriod").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            availability_zone_relocation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneRelocationEnabled").unwrap(),
            ),
            bucket_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketName").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            cluster_namespace_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterNamespaceArn").unwrap(),
            ),
            cluster_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterNodes").unwrap(),
            ),
            cluster_parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterParameterGroupName").unwrap(),
            ),
            cluster_public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterPublicKey").unwrap(),
            ),
            cluster_revision_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterRevisionNumber").unwrap(),
            ),
            cluster_subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterSubnetGroupName").unwrap(),
            ),
            cluster_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterType").unwrap(),
            ),
            cluster_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterVersion").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            default_iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultIamRoleArn").unwrap(),
            ),
            elastic_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticIp").unwrap(),
            ),
            enable_logging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableLogging").unwrap(),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encrypted").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            enhanced_vpc_routing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enhancedVpcRouting").unwrap(),
            ),
            iam_roles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoles").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            log_destination_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logDestinationType").unwrap(),
            ),
            log_exports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logExports").unwrap(),
            ),
            maintenance_track_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceTrackName").unwrap(),
            ),
            manual_snapshot_retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manualSnapshotRetentionPeriod").unwrap(),
            ),
            master_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterUsername").unwrap(),
            ),
            multi_az: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAz").unwrap(),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            number_of_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfNodes").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            preferred_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMaintenanceWindow").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            s3_key_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3KeyPrefix").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
