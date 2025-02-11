/// Provides a Redshift Cluster Resource.
///
///
/// > **NOTE:** A Redshift cluster's default IAM role can be managed both by this resource's `default_iam_role_arn` argument and the `aws.redshift.ClusterIamRoles` resource's `default_iam_role_arn` argument. Do not configure different values for both arguments. Doing so will cause a conflict of default IAM roles.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .cluster_identifier("tf-redshift-cluster")
///             .cluster_type("single-node")
///             .database_name("mydb")
///             .master_password("Mustbe8characters")
///             .master_username("exampleuser")
///             .node_type("dc1.large")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Managed Credentials
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .cluster_identifier("tf-redshift-cluster")
///             .cluster_type("single-node")
///             .database_name("mydb")
///             .manage_master_password(true)
///             .master_username("exampleuser")
///             .node_type("dc1.large")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Clusters using the `cluster_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/cluster:Cluster myprodcluster tf-redshift-cluster-12345
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// If true , major version upgrades can be applied during the maintenance window to the Amazon Redshift engine that is running on the cluster. Default is `true`.
        #[builder(into, default)]
        pub allow_version_upgrade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether any cluster modifications are applied immediately, or during the next maintenance window. Default is `false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The value represents how the cluster is configured to use AQUA (Advanced Query Accelerator) after the cluster is restored.
        /// No longer supported by the AWS API.
        /// Always returns `auto`.
        #[builder(into, default)]
        pub aqua_configuration_status: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The number of days that automated snapshots are retained. If the value is 0, automated snapshots are disabled. Even if automated snapshots are disabled, you can still create manual snapshots when you want with create-cluster-snapshot. Default is 1.
        #[builder(into, default)]
        pub automated_snapshot_retention_period: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The EC2 Availability Zone (AZ) in which you want Amazon Redshift to provision the cluster. For example, if you have several EC2 instances running in a specific Availability Zone, then you might want the cluster to be provisioned in the same zone in order to decrease network latency. Can only be changed if `availability_zone_relocation_enabled` is `true`.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, the cluster can be relocated to another availabity zone, either automatically by AWS or when requested. Default is `false`. Available for use on clusters from the RA3 instance family.
        #[builder(into, default)]
        pub availability_zone_relocation_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Cluster Identifier. Must be a lower case string.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the parameter group to be associated with this cluster.
        #[builder(into, default)]
        pub cluster_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The public key for the cluster
        #[builder(into, default)]
        pub cluster_public_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The specific revision number of the database in the cluster
        #[builder(into, default)]
        pub cluster_revision_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a cluster subnet group to be associated with this cluster. If this parameter is not provided the resulting cluster will be deployed outside virtual private cloud (VPC).
        #[builder(into, default)]
        pub cluster_subnet_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The cluster type to use. Either `single-node` or `multi-node`.
        #[builder(into, default)]
        pub cluster_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version of the Amazon Redshift engine software that you want to deploy on the cluster.
        /// The version selected runs on all the nodes in the cluster.
        #[builder(into, default)]
        pub cluster_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the first database to be created when the cluster is created.
        /// If you do not provide a name, Amazon Redshift will create a default database called `dev`.
        #[builder(into, default)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM role that was set as default for the cluster when the cluster was created.
        #[builder(into, default)]
        pub default_iam_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Elastic IP (EIP) address for the cluster.
        #[builder(into, default)]
        pub elastic_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true , the data in the cluster is encrypted at rest.
        #[builder(into, default)]
        pub encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The connection endpoint
        #[builder(into, default)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true , enhanced VPC routing is enabled.
        #[builder(into, default)]
        pub enhanced_vpc_routing: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The identifier of the final snapshot that is to be created immediately before deleting the cluster. If this parameter is provided, `skip_final_snapshot` must be false.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of IAM Role ARNs to associate with the cluster. A Maximum of 10 can be associated to the cluster at any time.
        #[builder(into, default)]
        pub iam_roles: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `encrypted` needs to be set to true.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Logging, documented below.
        #[builder(into, default)]
        pub logging: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::redshift::ClusterLogging>,
        >,
        /// The name of the maintenance track for the restored cluster. When you take a snapshot, the snapshot inherits the MaintenanceTrack value from the cluster. The snapshot might be on a different track than the cluster that was the source for the snapshot. For example, suppose that you take a snapshot of  a cluster that is on the current track and then change the cluster to be on the trailing track. In this case, the snapshot and the source cluster are on different tracks. Default value is `current`.
        #[builder(into, default)]
        pub maintenance_track_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to use AWS SecretsManager to manage the cluster admin credentials.
        /// Conflicts with `master_password`.
        /// One of `master_password` or `manage_master_password` is required unless `snapshot_identifier` is provided.
        #[builder(into, default)]
        pub manage_master_password: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The default number of days to retain a manual snapshot. If the value is -1, the snapshot is retained indefinitely. This setting doesn't change the retention period of existing snapshots. Valid values are between `-1` and `3653`. Default value is `-1`.
        #[builder(into, default)]
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Password for the master DB user.
        /// Conflicts with `manage_master_password`.
        /// One of `master_password` or `manage_master_password` is required unless `snapshot_identifier` is provided.
        /// Note that this may show up in logs, and it will be stored in the state file.
        /// Password must contain at least 8 characters and contain at least one uppercase letter, one lowercase letter, and one number.
        #[builder(into, default)]
        pub master_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the KMS key used to encrypt the cluster admin credentials secret.
        #[builder(into, default)]
        pub master_password_secret_kms_key_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Username for the master DB user.
        #[builder(into, default)]
        pub master_username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if the Redshift cluster is multi-AZ.
        #[builder(into, default)]
        pub multi_az: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The node type to be provisioned for the cluster.
        #[builder(into)]
        pub node_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of compute nodes in the cluster. This parameter is required when the ClusterType parameter is specified as multi-node. Default is 1.
        #[builder(into, default)]
        pub number_of_nodes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The AWS customer account used to create or copy the snapshot. Required if you are restoring a snapshot you do not own, optional if you own the snapshot.
        #[builder(into, default)]
        pub owner_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port number on which the cluster accepts incoming connections. Valid values are between `1115` and `65535`.
        /// The cluster is accessible only via the JDBC and ODBC connection strings.
        /// Part of the connection string requires the port on which the cluster will listen for incoming connections.
        /// Default port is `5439`.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The weekly time range (in UTC) during which automated cluster maintenance can occur.
        /// Format: ddd:hh24:mi-ddd:hh24:mi
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// If true, the cluster can be accessed from a public network. Default is `true`.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Determines whether a final snapshot of the cluster is created before Amazon Redshift deletes the cluster. If true , a final cluster snapshot is not created. If false , a final cluster snapshot is created before the cluster is deleted. Default is false.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ARN of the snapshot from which to create the new cluster. Conflicts with `snapshot_identifier`.
        #[builder(into, default)]
        pub snapshot_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the cluster the source snapshot was created from.
        #[builder(into, default)]
        pub snapshot_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Configuration of automatic copy of snapshots from one region to another. Documented below.
        #[builder(into, default)]
        pub snapshot_copy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::redshift::ClusterSnapshotCopy>,
        >,
        /// The name of the snapshot from which to create the new cluster.  Conflicts with `snapshot_arn`.
        #[builder(into, default)]
        pub snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Virtual Private Cloud (VPC) security groups to be associated with the cluster.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// If true , major version upgrades can be applied during the maintenance window to the Amazon Redshift engine that is running on the cluster. Default is `true`.
        pub allow_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether any cluster modifications are applied immediately, or during the next maintenance window. Default is `false`.
        pub apply_immediately: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The value represents how the cluster is configured to use AQUA (Advanced Query Accelerator) after the cluster is restored.
        /// No longer supported by the AWS API.
        /// Always returns `auto`.
        pub aqua_configuration_status: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of cluster
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The number of days that automated snapshots are retained. If the value is 0, automated snapshots are disabled. Even if automated snapshots are disabled, you can still create manual snapshots when you want with create-cluster-snapshot. Default is 1.
        pub automated_snapshot_retention_period: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// The EC2 Availability Zone (AZ) in which you want Amazon Redshift to provision the cluster. For example, if you have several EC2 instances running in a specific Availability Zone, then you might want the cluster to be provisioned in the same zone in order to decrease network latency. Can only be changed if `availability_zone_relocation_enabled` is `true`.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// If true, the cluster can be relocated to another availabity zone, either automatically by AWS or when requested. Default is `false`. Available for use on clusters from the RA3 instance family.
        pub availability_zone_relocation_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The Cluster Identifier. Must be a lower case string.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The namespace Amazon Resource Name (ARN) of the cluster
        pub cluster_namespace_arn: pulumi_gestalt_rust::Output<String>,
        /// The nodes in the cluster. Cluster node blocks are documented below
        pub cluster_nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redshift::ClusterClusterNode>,
        >,
        /// The name of the parameter group to be associated with this cluster.
        pub cluster_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// The public key for the cluster
        pub cluster_public_key: pulumi_gestalt_rust::Output<String>,
        /// The specific revision number of the database in the cluster
        pub cluster_revision_number: pulumi_gestalt_rust::Output<String>,
        /// The name of a cluster subnet group to be associated with this cluster. If this parameter is not provided the resulting cluster will be deployed outside virtual private cloud (VPC).
        pub cluster_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// The cluster type to use. Either `single-node` or `multi-node`.
        pub cluster_type: pulumi_gestalt_rust::Output<String>,
        /// The version of the Amazon Redshift engine software that you want to deploy on the cluster.
        /// The version selected runs on all the nodes in the cluster.
        pub cluster_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the first database to be created when the cluster is created.
        /// If you do not provide a name, Amazon Redshift will create a default database called `dev`.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the IAM role that was set as default for the cluster when the cluster was created.
        pub default_iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The DNS name of the cluster
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// The Elastic IP (EIP) address for the cluster.
        pub elastic_ip: pulumi_gestalt_rust::Output<Option<String>>,
        /// If true , the data in the cluster is encrypted at rest.
        pub encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The connection endpoint
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// If true , enhanced VPC routing is enabled.
        pub enhanced_vpc_routing: pulumi_gestalt_rust::Output<bool>,
        /// The identifier of the final snapshot that is to be created immediately before deleting the cluster. If this parameter is provided, `skip_final_snapshot` must be false.
        pub final_snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of IAM Role ARNs to associate with the cluster. A Maximum of 10 can be associated to the cluster at any time.
        pub iam_roles: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `encrypted` needs to be set to true.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Logging, documented below.
        pub logging: pulumi_gestalt_rust::Output<
            super::super::types::redshift::ClusterLogging,
        >,
        /// The name of the maintenance track for the restored cluster. When you take a snapshot, the snapshot inherits the MaintenanceTrack value from the cluster. The snapshot might be on a different track than the cluster that was the source for the snapshot. For example, suppose that you take a snapshot of  a cluster that is on the current track and then change the cluster to be on the trailing track. In this case, the snapshot and the source cluster are on different tracks. Default value is `current`.
        pub maintenance_track_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to use AWS SecretsManager to manage the cluster admin credentials.
        /// Conflicts with `master_password`.
        /// One of `master_password` or `manage_master_password` is required unless `snapshot_identifier` is provided.
        pub manage_master_password: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The default number of days to retain a manual snapshot. If the value is -1, the snapshot is retained indefinitely. This setting doesn't change the retention period of existing snapshots. Valid values are between `-1` and `3653`. Default value is `-1`.
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Password for the master DB user.
        /// Conflicts with `manage_master_password`.
        /// One of `master_password` or `manage_master_password` is required unless `snapshot_identifier` is provided.
        /// Note that this may show up in logs, and it will be stored in the state file.
        /// Password must contain at least 8 characters and contain at least one uppercase letter, one lowercase letter, and one number.
        pub master_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the cluster admin credentials secret
        pub master_password_secret_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the KMS key used to encrypt the cluster admin credentials secret.
        pub master_password_secret_kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Username for the master DB user.
        pub master_username: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if the Redshift cluster is multi-AZ.
        pub multi_az: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The node type to be provisioned for the cluster.
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// The number of compute nodes in the cluster. This parameter is required when the ClusterType parameter is specified as multi-node. Default is 1.
        pub number_of_nodes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The AWS customer account used to create or copy the snapshot. Required if you are restoring a snapshot you do not own, optional if you own the snapshot.
        pub owner_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// The port number on which the cluster accepts incoming connections. Valid values are between `1115` and `65535`.
        /// The cluster is accessible only via the JDBC and ODBC connection strings.
        /// Part of the connection string requires the port on which the cluster will listen for incoming connections.
        /// Default port is `5439`.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The weekly time range (in UTC) during which automated cluster maintenance can occur.
        /// Format: ddd:hh24:mi-ddd:hh24:mi
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// If true, the cluster can be accessed from a public network. Default is `true`.
        pub publicly_accessible: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Determines whether a final snapshot of the cluster is created before Amazon Redshift deletes the cluster. If true , a final cluster snapshot is not created. If false , a final cluster snapshot is created before the cluster is deleted. Default is false.
        pub skip_final_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN of the snapshot from which to create the new cluster. Conflicts with `snapshot_identifier`.
        pub snapshot_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the cluster the source snapshot was created from.
        pub snapshot_cluster_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration of automatic copy of snapshots from one region to another. Documented below.
        pub snapshot_copy: pulumi_gestalt_rust::Output<
            super::super::types::redshift::ClusterSnapshotCopy,
        >,
        /// The name of the snapshot from which to create the new cluster.  Conflicts with `snapshot_arn`.
        pub snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of Virtual Private Cloud (VPC) security groups to be associated with the cluster.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_version_upgrade_binding = args
            .allow_version_upgrade
            .get_output(context);
        let apply_immediately_binding = args.apply_immediately.get_output(context);
        let aqua_configuration_status_binding = args
            .aqua_configuration_status
            .get_output(context);
        let automated_snapshot_retention_period_binding = args
            .automated_snapshot_retention_period
            .get_output(context);
        let availability_zone_binding = args.availability_zone.get_output(context);
        let availability_zone_relocation_enabled_binding = args
            .availability_zone_relocation_enabled
            .get_output(context);
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let cluster_parameter_group_name_binding = args
            .cluster_parameter_group_name
            .get_output(context);
        let cluster_public_key_binding = args.cluster_public_key.get_output(context);
        let cluster_revision_number_binding = args
            .cluster_revision_number
            .get_output(context);
        let cluster_subnet_group_name_binding = args
            .cluster_subnet_group_name
            .get_output(context);
        let cluster_type_binding = args.cluster_type.get_output(context);
        let cluster_version_binding = args.cluster_version.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let default_iam_role_arn_binding = args.default_iam_role_arn.get_output(context);
        let elastic_ip_binding = args.elastic_ip.get_output(context);
        let encrypted_binding = args.encrypted.get_output(context);
        let endpoint_binding = args.endpoint.get_output(context);
        let enhanced_vpc_routing_binding = args.enhanced_vpc_routing.get_output(context);
        let final_snapshot_identifier_binding = args
            .final_snapshot_identifier
            .get_output(context);
        let iam_roles_binding = args.iam_roles.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let logging_binding = args.logging.get_output(context);
        let maintenance_track_name_binding = args
            .maintenance_track_name
            .get_output(context);
        let manage_master_password_binding = args
            .manage_master_password
            .get_output(context);
        let manual_snapshot_retention_period_binding = args
            .manual_snapshot_retention_period
            .get_output(context);
        let master_password_binding = args.master_password.get_output(context);
        let master_password_secret_kms_key_id_binding = args
            .master_password_secret_kms_key_id
            .get_output(context);
        let master_username_binding = args.master_username.get_output(context);
        let multi_az_binding = args.multi_az.get_output(context);
        let node_type_binding = args.node_type.get_output(context);
        let number_of_nodes_binding = args.number_of_nodes.get_output(context);
        let owner_account_binding = args.owner_account.get_output(context);
        let port_binding = args.port.get_output(context);
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context);
        let publicly_accessible_binding = args.publicly_accessible.get_output(context);
        let skip_final_snapshot_binding = args.skip_final_snapshot.get_output(context);
        let snapshot_arn_binding = args.snapshot_arn.get_output(context);
        let snapshot_cluster_identifier_binding = args
            .snapshot_cluster_identifier
            .get_output(context);
        let snapshot_copy_binding = args.snapshot_copy.get_output(context);
        let snapshot_identifier_binding = args.snapshot_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowVersionUpgrade".into(),
                    value: &allow_version_upgrade_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aquaConfigurationStatus".into(),
                    value: &aqua_configuration_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automatedSnapshotRetentionPeriod".into(),
                    value: &automated_snapshot_retention_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZoneRelocationEnabled".into(),
                    value: &availability_zone_relocation_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterParameterGroupName".into(),
                    value: &cluster_parameter_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterPublicKey".into(),
                    value: &cluster_public_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterRevisionNumber".into(),
                    value: &cluster_revision_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterSubnetGroupName".into(),
                    value: &cluster_subnet_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterType".into(),
                    value: &cluster_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterVersion".into(),
                    value: &cluster_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultIamRoleArn".into(),
                    value: &default_iam_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elasticIp".into(),
                    value: &elastic_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encrypted".into(),
                    value: &encrypted_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enhancedVpcRouting".into(),
                    value: &enhanced_vpc_routing_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "finalSnapshotIdentifier".into(),
                    value: &final_snapshot_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRoles".into(),
                    value: &iam_roles_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logging".into(),
                    value: &logging_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceTrackName".into(),
                    value: &maintenance_track_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manageMasterPassword".into(),
                    value: &manage_master_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manualSnapshotRetentionPeriod".into(),
                    value: &manual_snapshot_retention_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterPassword".into(),
                    value: &master_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterPasswordSecretKmsKeyId".into(),
                    value: &master_password_secret_kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterUsername".into(),
                    value: &master_username_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiAz".into(),
                    value: &multi_az_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numberOfNodes".into(),
                    value: &number_of_nodes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerAccount".into(),
                    value: &owner_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: &port_binding.drop_type(),
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
                    name: "skipFinalSnapshot".into(),
                    value: &skip_final_snapshot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotArn".into(),
                    value: &snapshot_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotClusterIdentifier".into(),
                    value: &snapshot_cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotCopy".into(),
                    value: &snapshot_copy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotIdentifier".into(),
                    value: &snapshot_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            allow_version_upgrade: o.get_field("allowVersionUpgrade"),
            apply_immediately: o.get_field("applyImmediately"),
            aqua_configuration_status: o.get_field("aquaConfigurationStatus"),
            arn: o.get_field("arn"),
            automated_snapshot_retention_period: o
                .get_field("automatedSnapshotRetentionPeriod"),
            availability_zone: o.get_field("availabilityZone"),
            availability_zone_relocation_enabled: o
                .get_field("availabilityZoneRelocationEnabled"),
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
            dns_name: o.get_field("dnsName"),
            elastic_ip: o.get_field("elasticIp"),
            encrypted: o.get_field("encrypted"),
            endpoint: o.get_field("endpoint"),
            enhanced_vpc_routing: o.get_field("enhancedVpcRouting"),
            final_snapshot_identifier: o.get_field("finalSnapshotIdentifier"),
            iam_roles: o.get_field("iamRoles"),
            kms_key_id: o.get_field("kmsKeyId"),
            logging: o.get_field("logging"),
            maintenance_track_name: o.get_field("maintenanceTrackName"),
            manage_master_password: o.get_field("manageMasterPassword"),
            manual_snapshot_retention_period: o
                .get_field("manualSnapshotRetentionPeriod"),
            master_password: o.get_field("masterPassword"),
            master_password_secret_arn: o.get_field("masterPasswordSecretArn"),
            master_password_secret_kms_key_id: o
                .get_field("masterPasswordSecretKmsKeyId"),
            master_username: o.get_field("masterUsername"),
            multi_az: o.get_field("multiAz"),
            node_type: o.get_field("nodeType"),
            number_of_nodes: o.get_field("numberOfNodes"),
            owner_account: o.get_field("ownerAccount"),
            port: o.get_field("port"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            skip_final_snapshot: o.get_field("skipFinalSnapshot"),
            snapshot_arn: o.get_field("snapshotArn"),
            snapshot_cluster_identifier: o.get_field("snapshotClusterIdentifier"),
            snapshot_copy: o.get_field("snapshotCopy"),
            snapshot_identifier: o.get_field("snapshotIdentifier"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
