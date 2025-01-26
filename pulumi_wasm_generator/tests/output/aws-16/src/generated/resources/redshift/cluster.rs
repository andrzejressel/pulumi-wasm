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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// If true , major version upgrades can be applied during the maintenance window to the Amazon Redshift engine that is running on the cluster. Default is `true`.
        #[builder(into, default)]
        pub allow_version_upgrade: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether any cluster modifications are applied immediately, or during the next maintenance window. Default is `false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The value represents how the cluster is configured to use AQUA (Advanced Query Accelerator) after the cluster is restored.
        /// No longer supported by the AWS API.
        /// Always returns `auto`.
        #[builder(into, default)]
        pub aqua_configuration_status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The number of days that automated snapshots are retained. If the value is 0, automated snapshots are disabled. Even if automated snapshots are disabled, you can still create manual snapshots when you want with create-cluster-snapshot. Default is 1.
        #[builder(into, default)]
        pub automated_snapshot_retention_period: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The EC2 Availability Zone (AZ) in which you want Amazon Redshift to provision the cluster. For example, if you have several EC2 instances running in a specific Availability Zone, then you might want the cluster to be provisioned in the same zone in order to decrease network latency. Can only be changed if `availability_zone_relocation_enabled` is `true`.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If true, the cluster can be relocated to another availabity zone, either automatically by AWS or when requested. Default is `false`. Available for use on clusters from the RA3 instance family.
        #[builder(into, default)]
        pub availability_zone_relocation_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Cluster Identifier. Must be a lower case string.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the parameter group to be associated with this cluster.
        #[builder(into, default)]
        pub cluster_parameter_group_name: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The public key for the cluster
        #[builder(into, default)]
        pub cluster_public_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The specific revision number of the database in the cluster
        #[builder(into, default)]
        pub cluster_revision_number: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of a cluster subnet group to be associated with this cluster. If this parameter is not provided the resulting cluster will be deployed outside virtual private cloud (VPC).
        #[builder(into, default)]
        pub cluster_subnet_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The cluster type to use. Either `single-node` or `multi-node`.
        #[builder(into, default)]
        pub cluster_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The version of the Amazon Redshift engine software that you want to deploy on the cluster.
        /// The version selected runs on all the nodes in the cluster.
        #[builder(into, default)]
        pub cluster_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the first database to be created when the cluster is created.
        /// If you do not provide a name, Amazon Redshift will create a default database called `dev`.
        #[builder(into, default)]
        pub database_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM role that was set as default for the cluster when the cluster was created.
        #[builder(into, default)]
        pub default_iam_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Elastic IP (EIP) address for the cluster.
        #[builder(into, default)]
        pub elastic_ip: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If true , the data in the cluster is encrypted at rest.
        #[builder(into, default)]
        pub encrypted: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The connection endpoint
        #[builder(into, default)]
        pub endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If true , enhanced VPC routing is enabled.
        #[builder(into, default)]
        pub enhanced_vpc_routing: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The identifier of the final snapshot that is to be created immediately before deleting the cluster. If this parameter is provided, `skip_final_snapshot` must be false.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of IAM Role ARNs to associate with the cluster. A Maximum of 10 can be associated to the cluster at any time.
        #[builder(into, default)]
        pub iam_roles: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `encrypted` needs to be set to true.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Logging, documented below.
        #[builder(into, default)]
        pub logging: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::redshift::ClusterLogging>,
        >,
        /// The name of the maintenance track for the restored cluster. When you take a snapshot, the snapshot inherits the MaintenanceTrack value from the cluster. The snapshot might be on a different track than the cluster that was the source for the snapshot. For example, suppose that you take a snapshot of  a cluster that is on the current track and then change the cluster to be on the trailing track. In this case, the snapshot and the source cluster are on different tracks. Default value is `current`.
        #[builder(into, default)]
        pub maintenance_track_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to use AWS SecretsManager to manage the cluster admin credentials.
        /// Conflicts with `master_password`.
        /// One of `master_password` or `manage_master_password` is required unless `snapshot_identifier` is provided.
        #[builder(into, default)]
        pub manage_master_password: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The default number of days to retain a manual snapshot. If the value is -1, the snapshot is retained indefinitely. This setting doesn't change the retention period of existing snapshots. Valid values are between `-1` and `3653`. Default value is `-1`.
        #[builder(into, default)]
        pub manual_snapshot_retention_period: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Password for the master DB user.
        /// Conflicts with `manage_master_password`.
        /// One of `master_password` or `manage_master_password` is required unless `snapshot_identifier` is provided.
        /// Note that this may show up in logs, and it will be stored in the state file.
        /// Password must contain at least 8 characters and contain at least one uppercase letter, one lowercase letter, and one number.
        #[builder(into, default)]
        pub master_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the KMS key used to encrypt the cluster admin credentials secret.
        #[builder(into, default)]
        pub master_password_secret_kms_key_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Username for the master DB user.
        #[builder(into, default)]
        pub master_username: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies if the Redshift cluster is multi-AZ.
        #[builder(into, default)]
        pub multi_az: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The node type to be provisioned for the cluster.
        #[builder(into)]
        pub node_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of compute nodes in the cluster. This parameter is required when the ClusterType parameter is specified as multi-node. Default is 1.
        #[builder(into, default)]
        pub number_of_nodes: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The AWS customer account used to create or copy the snapshot. Required if you are restoring a snapshot you do not own, optional if you own the snapshot.
        #[builder(into, default)]
        pub owner_account: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The port number on which the cluster accepts incoming connections. Valid values are between `1115` and `65535`.
        /// The cluster is accessible only via the JDBC and ODBC connection strings.
        /// Part of the connection string requires the port on which the cluster will listen for incoming connections.
        /// Default port is `5439`.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The weekly time range (in UTC) during which automated cluster maintenance can occur.
        /// Format: ddd:hh24:mi-ddd:hh24:mi
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// If true, the cluster can be accessed from a public network. Default is `true`.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Determines whether a final snapshot of the cluster is created before Amazon Redshift deletes the cluster. If true , a final cluster snapshot is not created. If false , a final cluster snapshot is created before the cluster is deleted. Default is false.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ARN of the snapshot from which to create the new cluster. Conflicts with `snapshot_identifier`.
        #[builder(into, default)]
        pub snapshot_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the cluster the source snapshot was created from.
        #[builder(into, default)]
        pub snapshot_cluster_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration of automatic copy of snapshots from one region to another. Documented below.
        #[builder(into, default)]
        pub snapshot_copy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::redshift::ClusterSnapshotCopy>,
        >,
        /// The name of the snapshot from which to create the new cluster.  Conflicts with `snapshot_arn`.
        #[builder(into, default)]
        pub snapshot_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Virtual Private Cloud (VPC) security groups to be associated with the cluster.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// If true , major version upgrades can be applied during the maintenance window to the Amazon Redshift engine that is running on the cluster. Default is `true`.
        pub allow_version_upgrade: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether any cluster modifications are applied immediately, or during the next maintenance window. Default is `false`.
        pub apply_immediately: pulumi_wasm_rust::Output<Option<bool>>,
        /// The value represents how the cluster is configured to use AQUA (Advanced Query Accelerator) after the cluster is restored.
        /// No longer supported by the AWS API.
        /// Always returns `auto`.
        pub aqua_configuration_status: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of cluster
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The number of days that automated snapshots are retained. If the value is 0, automated snapshots are disabled. Even if automated snapshots are disabled, you can still create manual snapshots when you want with create-cluster-snapshot. Default is 1.
        pub automated_snapshot_retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The EC2 Availability Zone (AZ) in which you want Amazon Redshift to provision the cluster. For example, if you have several EC2 instances running in a specific Availability Zone, then you might want the cluster to be provisioned in the same zone in order to decrease network latency. Can only be changed if `availability_zone_relocation_enabled` is `true`.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// If true, the cluster can be relocated to another availabity zone, either automatically by AWS or when requested. Default is `false`. Available for use on clusters from the RA3 instance family.
        pub availability_zone_relocation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Cluster Identifier. Must be a lower case string.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The namespace Amazon Resource Name (ARN) of the cluster
        pub cluster_namespace_arn: pulumi_wasm_rust::Output<String>,
        /// The nodes in the cluster. Cluster node blocks are documented below
        pub cluster_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::types::redshift::ClusterClusterNode>,
        >,
        /// The name of the parameter group to be associated with this cluster.
        pub cluster_parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// The public key for the cluster
        pub cluster_public_key: pulumi_wasm_rust::Output<String>,
        /// The specific revision number of the database in the cluster
        pub cluster_revision_number: pulumi_wasm_rust::Output<String>,
        /// The name of a cluster subnet group to be associated with this cluster. If this parameter is not provided the resulting cluster will be deployed outside virtual private cloud (VPC).
        pub cluster_subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// The cluster type to use. Either `single-node` or `multi-node`.
        pub cluster_type: pulumi_wasm_rust::Output<String>,
        /// The version of the Amazon Redshift engine software that you want to deploy on the cluster.
        /// The version selected runs on all the nodes in the cluster.
        pub cluster_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the first database to be created when the cluster is created.
        /// If you do not provide a name, Amazon Redshift will create a default database called `dev`.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the IAM role that was set as default for the cluster when the cluster was created.
        pub default_iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// The DNS name of the cluster
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// The Elastic IP (EIP) address for the cluster.
        pub elastic_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// If true , the data in the cluster is encrypted at rest.
        pub encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// The connection endpoint
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// If true , enhanced VPC routing is enabled.
        pub enhanced_vpc_routing: pulumi_wasm_rust::Output<bool>,
        /// The identifier of the final snapshot that is to be created immediately before deleting the cluster. If this parameter is provided, `skip_final_snapshot` must be false.
        pub final_snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IAM Role ARNs to associate with the cluster. A Maximum of 10 can be associated to the cluster at any time.
        pub iam_roles: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `encrypted` needs to be set to true.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Logging, documented below.
        pub logging: pulumi_wasm_rust::Output<
            super::super::types::redshift::ClusterLogging,
        >,
        /// The name of the maintenance track for the restored cluster. When you take a snapshot, the snapshot inherits the MaintenanceTrack value from the cluster. The snapshot might be on a different track than the cluster that was the source for the snapshot. For example, suppose that you take a snapshot of  a cluster that is on the current track and then change the cluster to be on the trailing track. In this case, the snapshot and the source cluster are on different tracks. Default value is `current`.
        pub maintenance_track_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to use AWS SecretsManager to manage the cluster admin credentials.
        /// Conflicts with `master_password`.
        /// One of `master_password` or `manage_master_password` is required unless `snapshot_identifier` is provided.
        pub manage_master_password: pulumi_wasm_rust::Output<Option<bool>>,
        /// The default number of days to retain a manual snapshot. If the value is -1, the snapshot is retained indefinitely. This setting doesn't change the retention period of existing snapshots. Valid values are between `-1` and `3653`. Default value is `-1`.
        pub manual_snapshot_retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// Password for the master DB user.
        /// Conflicts with `manage_master_password`.
        /// One of `master_password` or `manage_master_password` is required unless `snapshot_identifier` is provided.
        /// Note that this may show up in logs, and it will be stored in the state file.
        /// Password must contain at least 8 characters and contain at least one uppercase letter, one lowercase letter, and one number.
        pub master_password: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the cluster admin credentials secret
        pub master_password_secret_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the KMS key used to encrypt the cluster admin credentials secret.
        pub master_password_secret_kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Username for the master DB user.
        pub master_username: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if the Redshift cluster is multi-AZ.
        pub multi_az: pulumi_wasm_rust::Output<Option<bool>>,
        /// The node type to be provisioned for the cluster.
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// The number of compute nodes in the cluster. This parameter is required when the ClusterType parameter is specified as multi-node. Default is 1.
        pub number_of_nodes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The AWS customer account used to create or copy the snapshot. Required if you are restoring a snapshot you do not own, optional if you own the snapshot.
        pub owner_account: pulumi_wasm_rust::Output<Option<String>>,
        /// The port number on which the cluster accepts incoming connections. Valid values are between `1115` and `65535`.
        /// The cluster is accessible only via the JDBC and ODBC connection strings.
        /// Part of the connection string requires the port on which the cluster will listen for incoming connections.
        /// Default port is `5439`.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The weekly time range (in UTC) during which automated cluster maintenance can occur.
        /// Format: ddd:hh24:mi-ddd:hh24:mi
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// If true, the cluster can be accessed from a public network. Default is `true`.
        pub publicly_accessible: pulumi_wasm_rust::Output<Option<bool>>,
        /// Determines whether a final snapshot of the cluster is created before Amazon Redshift deletes the cluster. If true , a final cluster snapshot is not created. If false , a final cluster snapshot is created before the cluster is deleted. Default is false.
        pub skip_final_snapshot: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of the snapshot from which to create the new cluster. Conflicts with `snapshot_identifier`.
        pub snapshot_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the cluster the source snapshot was created from.
        pub snapshot_cluster_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration of automatic copy of snapshots from one region to another. Documented below.
        pub snapshot_copy: pulumi_wasm_rust::Output<
            super::super::types::redshift::ClusterSnapshotCopy,
        >,
        /// The name of the snapshot from which to create the new cluster.  Conflicts with `snapshot_arn`.
        pub snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of Virtual Private Cloud (VPC) security groups to be associated with the cluster.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_version_upgrade_binding = args
            .allow_version_upgrade
            .get_output(context)
            .get_inner();
        let apply_immediately_binding = args
            .apply_immediately
            .get_output(context)
            .get_inner();
        let aqua_configuration_status_binding = args
            .aqua_configuration_status
            .get_output(context)
            .get_inner();
        let automated_snapshot_retention_period_binding = args
            .automated_snapshot_retention_period
            .get_output(context)
            .get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let availability_zone_relocation_enabled_binding = args
            .availability_zone_relocation_enabled
            .get_output(context)
            .get_inner();
        let cluster_identifier_binding = args
            .cluster_identifier
            .get_output(context)
            .get_inner();
        let cluster_parameter_group_name_binding = args
            .cluster_parameter_group_name
            .get_output(context)
            .get_inner();
        let cluster_public_key_binding = args
            .cluster_public_key
            .get_output(context)
            .get_inner();
        let cluster_revision_number_binding = args
            .cluster_revision_number
            .get_output(context)
            .get_inner();
        let cluster_subnet_group_name_binding = args
            .cluster_subnet_group_name
            .get_output(context)
            .get_inner();
        let cluster_type_binding = args.cluster_type.get_output(context).get_inner();
        let cluster_version_binding = args
            .cluster_version
            .get_output(context)
            .get_inner();
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let default_iam_role_arn_binding = args
            .default_iam_role_arn
            .get_output(context)
            .get_inner();
        let elastic_ip_binding = args.elastic_ip.get_output(context).get_inner();
        let encrypted_binding = args.encrypted.get_output(context).get_inner();
        let endpoint_binding = args.endpoint.get_output(context).get_inner();
        let enhanced_vpc_routing_binding = args
            .enhanced_vpc_routing
            .get_output(context)
            .get_inner();
        let final_snapshot_identifier_binding = args
            .final_snapshot_identifier
            .get_output(context)
            .get_inner();
        let iam_roles_binding = args.iam_roles.get_output(context).get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let logging_binding = args.logging.get_output(context).get_inner();
        let maintenance_track_name_binding = args
            .maintenance_track_name
            .get_output(context)
            .get_inner();
        let manage_master_password_binding = args
            .manage_master_password
            .get_output(context)
            .get_inner();
        let manual_snapshot_retention_period_binding = args
            .manual_snapshot_retention_period
            .get_output(context)
            .get_inner();
        let master_password_binding = args
            .master_password
            .get_output(context)
            .get_inner();
        let master_password_secret_kms_key_id_binding = args
            .master_password_secret_kms_key_id
            .get_output(context)
            .get_inner();
        let master_username_binding = args
            .master_username
            .get_output(context)
            .get_inner();
        let multi_az_binding = args.multi_az.get_output(context).get_inner();
        let node_type_binding = args.node_type.get_output(context).get_inner();
        let number_of_nodes_binding = args
            .number_of_nodes
            .get_output(context)
            .get_inner();
        let owner_account_binding = args.owner_account.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context)
            .get_inner();
        let publicly_accessible_binding = args
            .publicly_accessible
            .get_output(context)
            .get_inner();
        let skip_final_snapshot_binding = args
            .skip_final_snapshot
            .get_output(context)
            .get_inner();
        let snapshot_arn_binding = args.snapshot_arn.get_output(context).get_inner();
        let snapshot_cluster_identifier_binding = args
            .snapshot_cluster_identifier
            .get_output(context)
            .get_inner();
        let snapshot_copy_binding = args.snapshot_copy.get_output(context).get_inner();
        let snapshot_identifier_binding = args
            .snapshot_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowVersionUpgrade".into(),
                    value: &allow_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding,
                },
                register_interface::ObjectField {
                    name: "aquaConfigurationStatus".into(),
                    value: &aqua_configuration_status_binding,
                },
                register_interface::ObjectField {
                    name: "automatedSnapshotRetentionPeriod".into(),
                    value: &automated_snapshot_retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZoneRelocationEnabled".into(),
                    value: &availability_zone_relocation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "clusterParameterGroupName".into(),
                    value: &cluster_parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "clusterPublicKey".into(),
                    value: &cluster_public_key_binding,
                },
                register_interface::ObjectField {
                    name: "clusterRevisionNumber".into(),
                    value: &cluster_revision_number_binding,
                },
                register_interface::ObjectField {
                    name: "clusterSubnetGroupName".into(),
                    value: &cluster_subnet_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "clusterType".into(),
                    value: &cluster_type_binding,
                },
                register_interface::ObjectField {
                    name: "clusterVersion".into(),
                    value: &cluster_version_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultIamRoleArn".into(),
                    value: &default_iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "elasticIp".into(),
                    value: &elastic_ip_binding,
                },
                register_interface::ObjectField {
                    name: "encrypted".into(),
                    value: &encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "enhancedVpcRouting".into(),
                    value: &enhanced_vpc_routing_binding,
                },
                register_interface::ObjectField {
                    name: "finalSnapshotIdentifier".into(),
                    value: &final_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoles".into(),
                    value: &iam_roles_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "logging".into(),
                    value: &logging_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceTrackName".into(),
                    value: &maintenance_track_name_binding,
                },
                register_interface::ObjectField {
                    name: "manageMasterPassword".into(),
                    value: &manage_master_password_binding,
                },
                register_interface::ObjectField {
                    name: "manualSnapshotRetentionPeriod".into(),
                    value: &manual_snapshot_retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "masterPassword".into(),
                    value: &master_password_binding,
                },
                register_interface::ObjectField {
                    name: "masterPasswordSecretKmsKeyId".into(),
                    value: &master_password_secret_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "masterUsername".into(),
                    value: &master_username_binding,
                },
                register_interface::ObjectField {
                    name: "multiAz".into(),
                    value: &multi_az_binding,
                },
                register_interface::ObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding,
                },
                register_interface::ObjectField {
                    name: "numberOfNodes".into(),
                    value: &number_of_nodes_binding,
                },
                register_interface::ObjectField {
                    name: "ownerAccount".into(),
                    value: &owner_account_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
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
                    name: "skipFinalSnapshot".into(),
                    value: &skip_final_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotArn".into(),
                    value: &snapshot_arn_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotClusterIdentifier".into(),
                    value: &snapshot_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotCopy".into(),
                    value: &snapshot_copy_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotIdentifier".into(),
                    value: &snapshot_identifier_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterResult {
            allow_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowVersionUpgrade"),
            ),
            apply_immediately: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applyImmediately"),
            ),
            aqua_configuration_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("aquaConfigurationStatus"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            automated_snapshot_retention_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("automatedSnapshotRetentionPeriod"),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            availability_zone_relocation_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZoneRelocationEnabled"),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            cluster_namespace_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterNamespaceArn"),
            ),
            cluster_nodes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterNodes"),
            ),
            cluster_parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterParameterGroupName"),
            ),
            cluster_public_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterPublicKey"),
            ),
            cluster_revision_number: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterRevisionNumber"),
            ),
            cluster_subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterSubnetGroupName"),
            ),
            cluster_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterType"),
            ),
            cluster_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterVersion"),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            default_iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultIamRoleArn"),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            elastic_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticIp"),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            enhanced_vpc_routing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enhancedVpcRouting"),
            ),
            final_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("finalSnapshotIdentifier"),
            ),
            iam_roles: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamRoles"),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            logging: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logging"),
            ),
            maintenance_track_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenanceTrackName"),
            ),
            manage_master_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("manageMasterPassword"),
            ),
            manual_snapshot_retention_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("manualSnapshotRetentionPeriod"),
            ),
            master_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterPassword"),
            ),
            master_password_secret_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterPasswordSecretArn"),
            ),
            master_password_secret_kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterPasswordSecretKmsKeyId"),
            ),
            master_username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterUsername"),
            ),
            multi_az: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multiAz"),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeType"),
            ),
            number_of_nodes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("numberOfNodes"),
            ),
            owner_account: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerAccount"),
            ),
            port: pulumi_wasm_rust::__private::into_domain(o.extract_field("port")),
            preferred_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preferredMaintenanceWindow"),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publiclyAccessible"),
            ),
            skip_final_snapshot: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipFinalSnapshot"),
            ),
            snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotArn"),
            ),
            snapshot_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotClusterIdentifier"),
            ),
            snapshot_copy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotCopy"),
            ),
            snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotIdentifier"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcSecurityGroupIds"),
            ),
        }
    }
}
