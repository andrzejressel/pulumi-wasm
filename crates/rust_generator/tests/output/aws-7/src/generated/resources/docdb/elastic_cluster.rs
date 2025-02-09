/// Manages an AWS DocDB (DocumentDB) Elastic Cluster.
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
///     let example = elastic_cluster::create(
///         "example",
///         ElasticClusterArgs::builder()
///             .admin_user_name("foo")
///             .admin_user_password("mustbeeightchars")
///             .auth_type("PLAIN_TEXT")
///             .name("my-docdb-cluster")
///             .shard_capacity(2)
///             .shard_count(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DocDB (DocumentDB) Elastic Cluster using the `arn` argument. For example,
///
/// ```sh
/// $ pulumi import aws:docdb/elasticCluster:ElasticCluster example arn:aws:docdb-elastic:us-east-1:000011112222:cluster/12345678-7abc-def0-1234-56789abcdef
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod elastic_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ElasticClusterArgs {
        /// Name of the Elastic DocumentDB cluster administrator
        #[builder(into)]
        pub admin_user_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Password for the Elastic DocumentDB cluster administrator. Can contain any printable ASCII characters. Must be at least 8 characters
        #[builder(into)]
        pub admin_user_password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Authentication type for the Elastic DocumentDB cluster. Valid values are `PLAIN_TEXT` and `SECRET_ARN`
        #[builder(into)]
        pub auth_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of days for which automatic snapshots are retained. It should be in between 1 and 35. If not specified, the default value of 1 is set.
        #[builder(into, default)]
        pub backup_retention_period: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// ARN of a KMS key that is used to encrypt the Elastic DocumentDB cluster. If not specified, the default encryption key that KMS creates for your account is used.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Elastic DocumentDB cluster
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The daily time range during which automated backups are created if automated backups are enabled, as determined by the `backup_retention_period`.
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Weekly time range during which system maintenance can occur in UTC. Format: `ddd:hh24:mi-ddd:hh24:mi`. If not specified, AWS will choose a random 30-minute window on a random day of the week.
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Number of vCPUs assigned to each elastic cluster shard. Maximum is 64. Allowed values are 2, 4, 8, 16, 32, 64
        #[builder(into)]
        pub shard_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Number of shards assigned to the elastic cluster. Maximum is 32
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub shard_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// IDs of subnets in which the Elastic DocumentDB Cluster operates.
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::docdb::ElasticClusterTimeouts>,
        >,
        /// List of VPC security groups to associate with the Elastic DocumentDB Cluster
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ElasticClusterResult {
        /// Name of the Elastic DocumentDB cluster administrator
        pub admin_user_name: pulumi_gestalt_rust::Output<String>,
        /// Password for the Elastic DocumentDB cluster administrator. Can contain any printable ASCII characters. Must be at least 8 characters
        pub admin_user_password: pulumi_gestalt_rust::Output<String>,
        /// ARN of the DocumentDB Elastic Cluster
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Authentication type for the Elastic DocumentDB cluster. Valid values are `PLAIN_TEXT` and `SECRET_ARN`
        pub auth_type: pulumi_gestalt_rust::Output<String>,
        /// The number of days for which automatic snapshots are retained. It should be in between 1 and 35. If not specified, the default value of 1 is set.
        pub backup_retention_period: pulumi_gestalt_rust::Output<f64>,
        /// The DNS address of the DocDB instance
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// ARN of a KMS key that is used to encrypt the Elastic DocumentDB cluster. If not specified, the default encryption key that KMS creates for your account is used.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Elastic DocumentDB cluster
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The daily time range during which automated backups are created if automated backups are enabled, as determined by the `backup_retention_period`.
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        /// Weekly time range during which system maintenance can occur in UTC. Format: `ddd:hh24:mi-ddd:hh24:mi`. If not specified, AWS will choose a random 30-minute window on a random day of the week.
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Number of vCPUs assigned to each elastic cluster shard. Maximum is 64. Allowed values are 2, 4, 8, 16, 32, 64
        pub shard_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Number of shards assigned to the elastic cluster. Maximum is 32
        ///
        /// The following arguments are optional:
        pub shard_count: pulumi_gestalt_rust::Output<i32>,
        /// IDs of subnets in which the Elastic DocumentDB Cluster operates.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::docdb::ElasticClusterTimeouts>,
        >,
        /// List of VPC security groups to associate with the Elastic DocumentDB Cluster
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ElasticClusterArgs,
    ) -> ElasticClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_user_name_binding = args.admin_user_name.get_output(context);
        let admin_user_password_binding = args.admin_user_password.get_output(context);
        let auth_type_binding = args.auth_type.get_output(context);
        let backup_retention_period_binding = args
            .backup_retention_period
            .get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let preferred_backup_window_binding = args
            .preferred_backup_window
            .get_output(context);
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context);
        let shard_capacity_binding = args.shard_capacity.get_output(context);
        let shard_count_binding = args.shard_count.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:docdb/elasticCluster:ElasticCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminUserName".into(),
                    value: admin_user_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminUserPassword".into(),
                    value: admin_user_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authType".into(),
                    value: auth_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRetentionPeriod".into(),
                    value: backup_retention_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredBackupWindow".into(),
                    value: preferred_backup_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: preferred_maintenance_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardCapacity".into(),
                    value: shard_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardCount".into(),
                    value: shard_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: vpc_security_group_ids_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ElasticClusterResult {
            admin_user_name: o.get_field("adminUserName"),
            admin_user_password: o.get_field("adminUserPassword"),
            arn: o.get_field("arn"),
            auth_type: o.get_field("authType"),
            backup_retention_period: o.get_field("backupRetentionPeriod"),
            endpoint: o.get_field("endpoint"),
            kms_key_id: o.get_field("kmsKeyId"),
            name: o.get_field("name"),
            preferred_backup_window: o.get_field("preferredBackupWindow"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            shard_capacity: o.get_field("shardCapacity"),
            shard_count: o.get_field("shardCount"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
