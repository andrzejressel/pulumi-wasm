/// Manages an Amazon FSx for OpenZFS file system.
/// See the [FSx OpenZFS User Guide](https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/what-is-fsx.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = open_zfs_file_system::create(
///         "test",
///         OpenZfsFileSystemArgs::builder()
///             .deployment_type("SINGLE_AZ_1")
///             .storage_capacity(64)
///             .subnet_ids(vec!["${test1.id}",])
///             .throughput_capacity(64)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FSx File Systems using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/openZfsFileSystem:OpenZfsFileSystem example fs-543ab12b1ca672f33
/// ```
/// Certain resource arguments, like `security_group_ids`, do not have a FSx API method for reading the information after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod open_zfs_file_system {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenZfsFileSystemArgs {
        /// The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days.
        #[builder(into, default)]
        pub automatic_backup_retention_days: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The ID of the source backup to create the filesystem from.
        #[builder(into, default)]
        pub backup_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean flag indicating whether tags for the file system should be copied to backups. The default value is false.
        #[builder(into, default)]
        pub copy_tags_to_backups: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
        #[builder(into, default)]
        pub copy_tags_to_volumes: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A recurring daily time, in the format HH:MM. HH is the zero-padded hour of the day (0-23), and MM is the zero-padded minute of the hour. For example, 05:00 specifies 5 AM daily. Requires `automatic_backup_retention_days` to be set.
        #[builder(into, default)]
        pub daily_automatic_backup_start_time: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// List of delete options, which at present supports only one value that specifies whether to delete all child volumes and snapshots when the file system is deleted. Valid values: `DELETE_CHILD_VOLUMES_AND_SNAPSHOTS`.
        #[builder(into, default)]
        pub delete_options: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Filesystem deployment type. See the [AWS API documentation](https://docs.aws.amazon.com/fsx/latest/APIReference/API_CreateFileSystemOpenZFSConfiguration.html#FSx-Type-CreateFileSystemOpenZFSConfiguration-DeploymentType) for a list of valid values.
        #[builder(into)]
        pub deployment_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SSD IOPS configuration for the Amazon FSx for OpenZFS file system. See `disk_iops_configuration` Block for details.
        #[builder(into, default)]
        pub disk_iops_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::OpenZfsFileSystemDiskIopsConfiguration>,
        >,
        /// (Multi-AZ only) Specifies the IP address range in which the endpoints to access your file system will be created.
        #[builder(into, default)]
        pub endpoint_ip_address_range: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A map of tags to apply to the file system's final backup.
        #[builder(into, default)]
        pub final_backup_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN for the KMS Key to encrypt the file system at rest, Defaults to an AWS managed KMS Key.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// (Multi-AZ only) Required when `deployment_type` is set to `MULTI_AZ_1`. This specifies the subnet in which you want the preferred file server to be located.
        #[builder(into, default)]
        pub preferred_subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The configuration for the root volume of the file system. All other volumes are children or the root volume. See `root_volume_configuration` Block for details.
        #[builder(into, default)]
        pub root_volume_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::OpenZfsFileSystemRootVolumeConfiguration>,
        >,
        /// (Multi-AZ only) Specifies the route tables in which Amazon FSx creates the rules for routing traffic to the correct file server. You should specify all virtual private cloud (VPC) route tables associated with the subnets in which your clients are located. By default, Amazon FSx selects your VPC's default route table.
        #[builder(into, default)]
        pub route_table_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// When enabled, will skip the default final backup taken when the file system is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        #[builder(into, default)]
        pub skip_final_backup: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The storage capacity (GiB) of the file system. Valid values between `64` and `524288`.
        #[builder(into, default)]
        pub storage_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The filesystem storage type. Only `SSD` is supported.
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Throughput (MB/s) of the file system. Valid values depend on `deployment_type`. Must be one of `64`, `128`, `256`, `512`, `1024`, `2048`, `3072`, `4096` for `SINGLE_AZ_1`. Must be one of `160`, `320`, `640`, `1280`, `2560`, `3840`, `5120`, `7680`, `10240` for `SINGLE_AZ_2`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub throughput_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        #[builder(into, default)]
        pub weekly_maintenance_start_time: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct OpenZfsFileSystemResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days.
        pub automatic_backup_retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the source backup to create the filesystem from.
        pub backup_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A boolean flag indicating whether tags for the file system should be copied to backups. The default value is false.
        pub copy_tags_to_backups: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
        pub copy_tags_to_volumes: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A recurring daily time, in the format HH:MM. HH is the zero-padded hour of the day (0-23), and MM is the zero-padded minute of the hour. For example, 05:00 specifies 5 AM daily. Requires `automatic_backup_retention_days` to be set.
        pub daily_automatic_backup_start_time: pulumi_gestalt_rust::Output<String>,
        /// List of delete options, which at present supports only one value that specifies whether to delete all child volumes and snapshots when the file system is deleted. Valid values: `DELETE_CHILD_VOLUMES_AND_SNAPSHOTS`.
        pub delete_options: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Filesystem deployment type. See the [AWS API documentation](https://docs.aws.amazon.com/fsx/latest/APIReference/API_CreateFileSystemOpenZFSConfiguration.html#FSx-Type-CreateFileSystemOpenZFSConfiguration-DeploymentType) for a list of valid values.
        pub deployment_type: pulumi_gestalt_rust::Output<String>,
        /// The SSD IOPS configuration for the Amazon FSx for OpenZFS file system. See `disk_iops_configuration` Block for details.
        pub disk_iops_configuration: pulumi_gestalt_rust::Output<
            super::super::types::fsx::OpenZfsFileSystemDiskIopsConfiguration,
        >,
        /// DNS name for the file system, e.g., `fs-12345678.fsx.us-west-2.amazonaws.com`
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// IP address of the endpoint that is used to access data or to manage the file system.
        pub endpoint_ip_address: pulumi_gestalt_rust::Output<String>,
        /// (Multi-AZ only) Specifies the IP address range in which the endpoints to access your file system will be created.
        pub endpoint_ip_address_range: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to apply to the file system's final backup.
        pub final_backup_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN for the KMS Key to encrypt the file system at rest, Defaults to an AWS managed KMS Key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Set of Elastic Network Interface identifiers from which the file system is accessible The first network interface returned is the primary network interface.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// (Multi-AZ only) Required when `deployment_type` is set to `MULTI_AZ_1`. This specifies the subnet in which you want the preferred file server to be located.
        pub preferred_subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The configuration for the root volume of the file system. All other volumes are children or the root volume. See `root_volume_configuration` Block for details.
        pub root_volume_configuration: pulumi_gestalt_rust::Output<
            super::super::types::fsx::OpenZfsFileSystemRootVolumeConfiguration,
        >,
        /// Identifier of the root volume, e.g., `fsvol-12345678`
        pub root_volume_id: pulumi_gestalt_rust::Output<String>,
        /// (Multi-AZ only) Specifies the route tables in which Amazon FSx creates the rules for routing traffic to the correct file server. You should specify all virtual private cloud (VPC) route tables associated with the subnets in which your clients are located. By default, Amazon FSx selects your VPC's default route table.
        pub route_table_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// When enabled, will skip the default final backup taken when the file system is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        pub skip_final_backup: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The storage capacity (GiB) of the file system. Valid values between `64` and `524288`.
        pub storage_capacity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The filesystem storage type. Only `SSD` is supported.
        pub storage_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Throughput (MB/s) of the file system. Valid values depend on `deployment_type`. Must be one of `64`, `128`, `256`, `512`, `1024`, `2048`, `3072`, `4096` for `SINGLE_AZ_1`. Must be one of `160`, `320`, `640`, `1280`, `2560`, `3840`, `5120`, `7680`, `10240` for `SINGLE_AZ_2`.
        ///
        /// The following arguments are optional:
        pub throughput_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Identifier of the Virtual Private Cloud for the file system.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        pub weekly_maintenance_start_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OpenZfsFileSystemArgs,
    ) -> OpenZfsFileSystemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automatic_backup_retention_days_binding = args
            .automatic_backup_retention_days
            .get_output(context);
        let backup_id_binding = args.backup_id.get_output(context);
        let copy_tags_to_backups_binding = args.copy_tags_to_backups.get_output(context);
        let copy_tags_to_volumes_binding = args.copy_tags_to_volumes.get_output(context);
        let daily_automatic_backup_start_time_binding = args
            .daily_automatic_backup_start_time
            .get_output(context);
        let delete_options_binding = args.delete_options.get_output(context);
        let deployment_type_binding = args.deployment_type.get_output(context);
        let disk_iops_configuration_binding = args
            .disk_iops_configuration
            .get_output(context);
        let endpoint_ip_address_range_binding = args
            .endpoint_ip_address_range
            .get_output(context);
        let final_backup_tags_binding = args.final_backup_tags.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let preferred_subnet_id_binding = args.preferred_subnet_id.get_output(context);
        let root_volume_configuration_binding = args
            .root_volume_configuration
            .get_output(context);
        let route_table_ids_binding = args.route_table_ids.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let skip_final_backup_binding = args.skip_final_backup.get_output(context);
        let storage_capacity_binding = args.storage_capacity.get_output(context);
        let storage_type_binding = args.storage_type.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let throughput_capacity_binding = args.throughput_capacity.get_output(context);
        let weekly_maintenance_start_time_binding = args
            .weekly_maintenance_start_time
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:fsx/openZfsFileSystem:OpenZfsFileSystem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticBackupRetentionDays".into(),
                    value: &automatic_backup_retention_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupId".into(),
                    value: &backup_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyTagsToBackups".into(),
                    value: &copy_tags_to_backups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyTagsToVolumes".into(),
                    value: &copy_tags_to_volumes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dailyAutomaticBackupStartTime".into(),
                    value: &daily_automatic_backup_start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteOptions".into(),
                    value: &delete_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentType".into(),
                    value: &deployment_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskIopsConfiguration".into(),
                    value: &disk_iops_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointIpAddressRange".into(),
                    value: &endpoint_ip_address_range_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "finalBackupTags".into(),
                    value: &final_backup_tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredSubnetId".into(),
                    value: &preferred_subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rootVolumeConfiguration".into(),
                    value: &root_volume_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableIds".into(),
                    value: &route_table_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipFinalBackup".into(),
                    value: &skip_final_backup_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageCapacity".into(),
                    value: &storage_capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughputCapacity".into(),
                    value: &throughput_capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "weeklyMaintenanceStartTime".into(),
                    value: &weekly_maintenance_start_time_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OpenZfsFileSystemResult {
            arn: o.get_field("arn"),
            automatic_backup_retention_days: o.get_field("automaticBackupRetentionDays"),
            backup_id: o.get_field("backupId"),
            copy_tags_to_backups: o.get_field("copyTagsToBackups"),
            copy_tags_to_volumes: o.get_field("copyTagsToVolumes"),
            daily_automatic_backup_start_time: o
                .get_field("dailyAutomaticBackupStartTime"),
            delete_options: o.get_field("deleteOptions"),
            deployment_type: o.get_field("deploymentType"),
            disk_iops_configuration: o.get_field("diskIopsConfiguration"),
            dns_name: o.get_field("dnsName"),
            endpoint_ip_address: o.get_field("endpointIpAddress"),
            endpoint_ip_address_range: o.get_field("endpointIpAddressRange"),
            final_backup_tags: o.get_field("finalBackupTags"),
            kms_key_id: o.get_field("kmsKeyId"),
            network_interface_ids: o.get_field("networkInterfaceIds"),
            owner_id: o.get_field("ownerId"),
            preferred_subnet_id: o.get_field("preferredSubnetId"),
            root_volume_configuration: o.get_field("rootVolumeConfiguration"),
            root_volume_id: o.get_field("rootVolumeId"),
            route_table_ids: o.get_field("routeTableIds"),
            security_group_ids: o.get_field("securityGroupIds"),
            skip_final_backup: o.get_field("skipFinalBackup"),
            storage_capacity: o.get_field("storageCapacity"),
            storage_type: o.get_field("storageType"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            throughput_capacity: o.get_field("throughputCapacity"),
            vpc_id: o.get_field("vpcId"),
            weekly_maintenance_start_time: o.get_field("weeklyMaintenanceStartTime"),
        }
    }
}
