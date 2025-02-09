/// A Cluster Instance Resource defines attributes that are specific to a single instance in a Neptune Cluster.
///
/// You can simply add neptune instances and Neptune manages the replication. You can use the count
/// meta-parameter to make multiple instances and join them all to the same Neptune Cluster, or you may specify different Cluster Instance resources with various `instance_class` sizes.
///
/// ## Example Usage
///
/// The following example will create a neptune cluster with two neptune instances(one writer and one reader).
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = cluster::create(
///         "default",
///         ClusterArgs::builder()
///             .apply_immediately(true)
///             .backup_retention_period(5)
///             .cluster_identifier("neptune-cluster-demo")
///             .engine("neptune")
///             .iam_database_authentication_enabled(true)
///             .preferred_backup_window("07:00-09:00")
///             .skip_final_snapshot(true)
///             .build_struct(),
///     );
///     let example = cluster_instance::create(
///         "example",
///         ClusterInstanceArgs::builder()
///             .apply_immediately(true)
///             .cluster_identifier("${default.id}")
///             .engine("neptune")
///             .instance_class("db.r4.large")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_neptune_cluster_instance` using the instance identifier. For example:
///
/// ```sh
/// $ pulumi import aws:neptune/clusterInstance:ClusterInstance example my-instance
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterInstanceArgs {
        /// Specifies whether any instance modifications
        /// are applied immediately, or during the next maintenance window. Default is`false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates that minor engine upgrades will be applied automatically to the instance during the maintenance window. Default is `true`.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The EC2 Availability Zone that the neptune instance is created in.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier of the `aws.neptune.Cluster` in which to launch this instance.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the database engine to be used for the neptune instance. Defaults to `neptune`. Valid Values: `neptune`.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The neptune engine version. Currently configuring this argumnet has no effect.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier for the neptune instance, if omitted, this provider will assign a random, unique identifier.
        #[builder(into, default)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique identifier beginning with the specified prefix. Conflicts with `identifier`.
        #[builder(into, default)]
        pub identifier_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The instance class to use.
        #[builder(into)]
        pub instance_class: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the neptune parameter group to associate with this instance.
        #[builder(into, default)]
        pub neptune_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A subnet group to associate with this neptune instance. **NOTE:** This must match the `neptune_subnet_group_name` of the attached `aws.neptune.Cluster`.
        #[builder(into, default)]
        pub neptune_subnet_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The port on which the DB accepts connections. Defaults to `8182`.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The daily time range during which automated backups are created if automated backups are enabled. Eg: "04:00-09:00"
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The window to perform maintenance in.
        /// Syntax: "ddd:hh24:mi-ddd:hh24:mi". Eg: "Mon:00:00-Mon:03:00".
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Default 0. Failover Priority setting on instance level. The reader who has lower tier has higher priority to get promoter to writer.
        #[builder(into, default)]
        pub promotion_tier: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Bool to control if instance is publicly accessible. Default is `false`.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Determines whether a final DB snapshot is created before the DB instance is deleted.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterInstanceResult {
        /// The hostname of the instance. See also `endpoint` and `port`.
        pub address: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether any instance modifications
        /// are applied immediately, or during the next maintenance window. Default is`false`.
        pub apply_immediately: pulumi_gestalt_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of neptune instance
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates that minor engine upgrades will be applied automatically to the instance during the maintenance window. Default is `true`.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The EC2 Availability Zone that the neptune instance is created in.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the `aws.neptune.Cluster` in which to launch this instance.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The region-unique, immutable identifier for the neptune instance.
        pub dbi_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The connection endpoint in `address:port` format.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of the database engine to be used for the neptune instance. Defaults to `neptune`. Valid Values: `neptune`.
        pub engine: pulumi_gestalt_rust::Output<Option<String>>,
        /// The neptune engine version. Currently configuring this argumnet has no effect.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The identifier for the neptune instance, if omitted, this provider will assign a random, unique identifier.
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique identifier beginning with the specified prefix. Conflicts with `identifier`.
        pub identifier_prefix: pulumi_gestalt_rust::Output<String>,
        /// The instance class to use.
        pub instance_class: pulumi_gestalt_rust::Output<String>,
        /// The ARN for the KMS encryption key if one is set to the neptune cluster.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the neptune parameter group to associate with this instance.
        pub neptune_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// A subnet group to associate with this neptune instance. **NOTE:** This must match the `neptune_subnet_group_name` of the attached `aws.neptune.Cluster`.
        pub neptune_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// The port on which the DB accepts connections. Defaults to `8182`.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The daily time range during which automated backups are created if automated backups are enabled. Eg: "04:00-09:00"
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        /// The window to perform maintenance in.
        /// Syntax: "ddd:hh24:mi-ddd:hh24:mi". Eg: "Mon:00:00-Mon:03:00".
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Default 0. Failover Priority setting on instance level. The reader who has lower tier has higher priority to get promoter to writer.
        pub promotion_tier: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Bool to control if instance is publicly accessible. Default is `false`.
        pub publicly_accessible: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Determines whether a final DB snapshot is created before the DB instance is deleted.
        pub skip_final_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether the neptune cluster is encrypted.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Storage type associated with the cluster `standard/iopt1`.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean indicating if this instance is writable. `False` indicates this instance is a read replica.
        pub writer: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterInstanceArgs,
    ) -> ClusterInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let apply_immediately_binding = args.apply_immediately.get_output(context);
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_output(context);
        let availability_zone_binding = args.availability_zone.get_output(context);
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let engine_binding = args.engine.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let identifier_binding = args.identifier.get_output(context);
        let identifier_prefix_binding = args.identifier_prefix.get_output(context);
        let instance_class_binding = args.instance_class.get_output(context);
        let neptune_parameter_group_name_binding = args
            .neptune_parameter_group_name
            .get_output(context);
        let neptune_subnet_group_name_binding = args
            .neptune_subnet_group_name
            .get_output(context);
        let port_binding = args.port.get_output(context);
        let preferred_backup_window_binding = args
            .preferred_backup_window
            .get_output(context);
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context);
        let promotion_tier_binding = args.promotion_tier.get_output(context);
        let publicly_accessible_binding = args.publicly_accessible.get_output(context);
        let skip_final_snapshot_binding = args.skip_final_snapshot.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:neptune/clusterInstance:ClusterInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyImmediately".into(),
                    value: apply_immediately_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: auto_minor_version_upgrade_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: engine_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifierPrefix".into(),
                    value: identifier_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceClass".into(),
                    value: instance_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "neptuneParameterGroupName".into(),
                    value: neptune_parameter_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "neptuneSubnetGroupName".into(),
                    value: neptune_subnet_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
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
                    name: "promotionTier".into(),
                    value: promotion_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publiclyAccessible".into(),
                    value: publicly_accessible_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipFinalSnapshot".into(),
                    value: skip_final_snapshot_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterInstanceResult {
            address: o.get_field("address"),
            apply_immediately: o.get_field("applyImmediately"),
            arn: o.get_field("arn"),
            auto_minor_version_upgrade: o.get_field("autoMinorVersionUpgrade"),
            availability_zone: o.get_field("availabilityZone"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            dbi_resource_id: o.get_field("dbiResourceId"),
            endpoint: o.get_field("endpoint"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            identifier: o.get_field("identifier"),
            identifier_prefix: o.get_field("identifierPrefix"),
            instance_class: o.get_field("instanceClass"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            neptune_parameter_group_name: o.get_field("neptuneParameterGroupName"),
            neptune_subnet_group_name: o.get_field("neptuneSubnetGroupName"),
            port: o.get_field("port"),
            preferred_backup_window: o.get_field("preferredBackupWindow"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            promotion_tier: o.get_field("promotionTier"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            skip_final_snapshot: o.get_field("skipFinalSnapshot"),
            storage_encrypted: o.get_field("storageEncrypted"),
            storage_type: o.get_field("storageType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            writer: o.get_field("writer"),
        }
    }
}
