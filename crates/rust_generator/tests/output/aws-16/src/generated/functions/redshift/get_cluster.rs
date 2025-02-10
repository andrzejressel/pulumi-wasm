#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Cluster identifier
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags associated to the cluster
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// Whether major version upgrades can be applied during maintenance period
        pub allow_version_upgrade: pulumi_gestalt_rust::Output<bool>,
        /// The value represents how the cluster is configured to use AQUA.
        pub aqua_configuration_status: pulumi_gestalt_rust::Output<String>,
        /// ARN of cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The backup retention period
        pub automated_snapshot_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// Availability zone of the cluster
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the cluster is able to be relocated to another availability zone.
        pub availability_zone_relocation_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Name of the S3 bucket where the log files are to be stored
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        /// Cluster identifier
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The namespace Amazon Resource Name (ARN) of the cluster
        pub cluster_namespace_arn: pulumi_gestalt_rust::Output<String>,
        /// Nodes in the cluster. Cluster node blocks are documented below
        pub cluster_nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redshift::GetClusterClusterNode>,
        >,
        /// The name of the parameter group to be associated with this cluster
        pub cluster_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// Public key for the cluster
        pub cluster_public_key: pulumi_gestalt_rust::Output<String>,
        /// The cluster revision number
        pub cluster_revision_number: pulumi_gestalt_rust::Output<String>,
        /// The name of a cluster subnet group to be associated with this cluster
        pub cluster_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// Cluster type
        pub cluster_type: pulumi_gestalt_rust::Output<String>,
        pub cluster_version: pulumi_gestalt_rust::Output<String>,
        /// Name of the default database in the cluster
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The ARN for the IAM role that was set as default for the cluster when the cluster was created.
        pub default_iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Elastic IP of the cluster
        pub elastic_ip: pulumi_gestalt_rust::Output<String>,
        /// Whether cluster logging is enabled
        pub enable_logging: pulumi_gestalt_rust::Output<bool>,
        /// Whether the cluster data is encrypted
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Cluster endpoint
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Whether enhanced VPC routing is enabled
        pub enhanced_vpc_routing: pulumi_gestalt_rust::Output<bool>,
        /// IAM roles associated to the cluster
        pub iam_roles: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// KMS encryption key associated to the cluster
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The log destination type.
        pub log_destination_type: pulumi_gestalt_rust::Output<String>,
        /// Collection of exported log types. Log types include the connection log, user log and user activity log.
        pub log_exports: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the maintenance track for the restored cluster.
        pub maintenance_track_name: pulumi_gestalt_rust::Output<String>,
        /// (Optional)  The default number of days to retain a manual snapshot.
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// Username for the master DB user
        pub master_username: pulumi_gestalt_rust::Output<String>,
        /// If the cluster is a Multi-AZ deployment
        pub multi_az: pulumi_gestalt_rust::Output<bool>,
        /// Cluster node type
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// Number of nodes in the cluster
        pub number_of_nodes: pulumi_gestalt_rust::Output<i32>,
        /// Port the cluster responds on
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The maintenance window
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Whether the cluster is publicly accessible
        pub publicly_accessible: pulumi_gestalt_rust::Output<bool>,
        /// Folder inside the S3 bucket where the log files are stored
        pub s3_key_prefix: pulumi_gestalt_rust::Output<String>,
        /// Tags associated to the cluster
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC Id associated with the cluster
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The VPC security group Ids associated with the cluster
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:redshift/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClusterResult {
            allow_version_upgrade: o.get_field("allowVersionUpgrade"),
            aqua_configuration_status: o.get_field("aquaConfigurationStatus"),
            arn: o.get_field("arn"),
            automated_snapshot_retention_period: o
                .get_field("automatedSnapshotRetentionPeriod"),
            availability_zone: o.get_field("availabilityZone"),
            availability_zone_relocation_enabled: o
                .get_field("availabilityZoneRelocationEnabled"),
            bucket_name: o.get_field("bucketName"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            cluster_namespace_arn: o.get_field("clusterNamespaceArn"),
            cluster_nodes: o.get_field("clusterNodes"),
            cluster_parameter_group_name: o.get_field("clusterParameterGroupName"),
            cluster_public_key: o.get_field("clusterPublicKey"),
            cluster_revision_number: o.get_field("clusterRevisionNumber"),
            cluster_subnet_group_name: o.get_field("clusterSubnetGroupName"),
            cluster_type: o.get_field("clusterType"),
            cluster_version: o.get_field("clusterVersion"),
            database_name: o.get_field("databaseName"),
            default_iam_role_arn: o.get_field("defaultIamRoleArn"),
            elastic_ip: o.get_field("elasticIp"),
            enable_logging: o.get_field("enableLogging"),
            encrypted: o.get_field("encrypted"),
            endpoint: o.get_field("endpoint"),
            enhanced_vpc_routing: o.get_field("enhancedVpcRouting"),
            iam_roles: o.get_field("iamRoles"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            log_destination_type: o.get_field("logDestinationType"),
            log_exports: o.get_field("logExports"),
            maintenance_track_name: o.get_field("maintenanceTrackName"),
            manual_snapshot_retention_period: o
                .get_field("manualSnapshotRetentionPeriod"),
            master_username: o.get_field("masterUsername"),
            multi_az: o.get_field("multiAz"),
            node_type: o.get_field("nodeType"),
            number_of_nodes: o.get_field("numberOfNodes"),
            port: o.get_field("port"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            s3_key_prefix: o.get_field("s3KeyPrefix"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
