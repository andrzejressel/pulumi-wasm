/// Manages an Amazon FSx for OpenZFS file system.
/// See the [FSx OpenZFS User Guide](https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/what-is-fsx.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod open_zfs_file_system {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenZfsFileSystemArgs {
        /// The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days.
        #[builder(into, default)]
        pub automatic_backup_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the source backup to create the filesystem from.
        #[builder(into, default)]
        pub backup_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A boolean flag indicating whether tags for the file system should be copied to backups. The default value is false.
        #[builder(into, default)]
        pub copy_tags_to_backups: pulumi_wasm_rust::Output<Option<bool>>,
        /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
        #[builder(into, default)]
        pub copy_tags_to_volumes: pulumi_wasm_rust::Output<Option<bool>>,
        /// A recurring daily time, in the format HH:MM. HH is the zero-padded hour of the day (0-23), and MM is the zero-padded minute of the hour. For example, 05:00 specifies 5 AM daily. Requires `automatic_backup_retention_days` to be set.
        #[builder(into, default)]
        pub daily_automatic_backup_start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// List of delete options, which at present supports only one value that specifies whether to delete all child volumes and snapshots when the file system is deleted. Valid values: `DELETE_CHILD_VOLUMES_AND_SNAPSHOTS`.
        #[builder(into, default)]
        pub delete_options: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Filesystem deployment type. See the [AWS API documentation](https://docs.aws.amazon.com/fsx/latest/APIReference/API_CreateFileSystemOpenZFSConfiguration.html#FSx-Type-CreateFileSystemOpenZFSConfiguration-DeploymentType) for a list of valid values.
        #[builder(into)]
        pub deployment_type: pulumi_wasm_rust::Output<String>,
        /// The SSD IOPS configuration for the Amazon FSx for OpenZFS file system. See `disk_iops_configuration` Block for details.
        #[builder(into, default)]
        pub disk_iops_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OpenZfsFileSystemDiskIopsConfiguration>,
        >,
        /// (Multi-AZ only) Specifies the IP address range in which the endpoints to access your file system will be created.
        #[builder(into, default)]
        pub endpoint_ip_address_range: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to apply to the file system's final backup.
        #[builder(into, default)]
        pub final_backup_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN for the KMS Key to encrypt the file system at rest, Defaults to an AWS managed KMS Key.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// (Multi-AZ only) Required when `deployment_type` is set to `MULTI_AZ_1`. This specifies the subnet in which you want the preferred file server to be located.
        #[builder(into, default)]
        pub preferred_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The configuration for the root volume of the file system. All other volumes are children or the root volume. See `root_volume_configuration` Block for details.
        #[builder(into, default)]
        pub root_volume_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OpenZfsFileSystemRootVolumeConfiguration>,
        >,
        /// (Multi-AZ only) Specifies the route tables in which Amazon FSx creates the rules for routing traffic to the correct file server. You should specify all virtual private cloud (VPC) route tables associated with the subnets in which your clients are located. By default, Amazon FSx selects your VPC's default route table.
        #[builder(into, default)]
        pub route_table_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// When enabled, will skip the default final backup taken when the file system is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        #[builder(into, default)]
        pub skip_final_backup: pulumi_wasm_rust::Output<Option<bool>>,
        /// The storage capacity (GiB) of the file system. Valid values between `64` and `524288`.
        #[builder(into, default)]
        pub storage_capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// The filesystem storage type. Only `SSD` is supported.
        #[builder(into, default)]
        pub storage_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from.
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Throughput (MB/s) of the file system. Valid values depend on `deployment_type`. Must be one of `64`, `128`, `256`, `512`, `1024`, `2048`, `3072`, `4096` for `SINGLE_AZ_1`. Must be one of `160`, `320`, `640`, `1280`, `2560`, `3840`, `5120`, `7680`, `10240` for `SINGLE_AZ_2`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub throughput_capacity: pulumi_wasm_rust::Output<i32>,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        #[builder(into, default)]
        pub weekly_maintenance_start_time: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OpenZfsFileSystemResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days.
        pub automatic_backup_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the source backup to create the filesystem from.
        pub backup_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A boolean flag indicating whether tags for the file system should be copied to backups. The default value is false.
        pub copy_tags_to_backups: pulumi_wasm_rust::Output<Option<bool>>,
        /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
        pub copy_tags_to_volumes: pulumi_wasm_rust::Output<Option<bool>>,
        /// A recurring daily time, in the format HH:MM. HH is the zero-padded hour of the day (0-23), and MM is the zero-padded minute of the hour. For example, 05:00 specifies 5 AM daily. Requires `automatic_backup_retention_days` to be set.
        pub daily_automatic_backup_start_time: pulumi_wasm_rust::Output<String>,
        /// List of delete options, which at present supports only one value that specifies whether to delete all child volumes and snapshots when the file system is deleted. Valid values: `DELETE_CHILD_VOLUMES_AND_SNAPSHOTS`.
        pub delete_options: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Filesystem deployment type. See the [AWS API documentation](https://docs.aws.amazon.com/fsx/latest/APIReference/API_CreateFileSystemOpenZFSConfiguration.html#FSx-Type-CreateFileSystemOpenZFSConfiguration-DeploymentType) for a list of valid values.
        pub deployment_type: pulumi_wasm_rust::Output<String>,
        /// The SSD IOPS configuration for the Amazon FSx for OpenZFS file system. See `disk_iops_configuration` Block for details.
        pub disk_iops_configuration: pulumi_wasm_rust::Output<
            super::super::types::fsx::OpenZfsFileSystemDiskIopsConfiguration,
        >,
        /// DNS name for the file system, e.g., `fs-12345678.fsx.us-west-2.amazonaws.com`
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// IP address of the endpoint that is used to access data or to manage the file system.
        pub endpoint_ip_address: pulumi_wasm_rust::Output<String>,
        /// (Multi-AZ only) Specifies the IP address range in which the endpoints to access your file system will be created.
        pub endpoint_ip_address_range: pulumi_wasm_rust::Output<String>,
        /// A map of tags to apply to the file system's final backup.
        pub final_backup_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN for the KMS Key to encrypt the file system at rest, Defaults to an AWS managed KMS Key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Set of Elastic Network Interface identifiers from which the file system is accessible The first network interface returned is the primary network interface.
        pub network_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// (Multi-AZ only) Required when `deployment_type` is set to `MULTI_AZ_1`. This specifies the subnet in which you want the preferred file server to be located.
        pub preferred_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The configuration for the root volume of the file system. All other volumes are children or the root volume. See `root_volume_configuration` Block for details.
        pub root_volume_configuration: pulumi_wasm_rust::Output<
            super::super::types::fsx::OpenZfsFileSystemRootVolumeConfiguration,
        >,
        /// Identifier of the root volume, e.g., `fsvol-12345678`
        pub root_volume_id: pulumi_wasm_rust::Output<String>,
        /// (Multi-AZ only) Specifies the route tables in which Amazon FSx creates the rules for routing traffic to the correct file server. You should specify all virtual private cloud (VPC) route tables associated with the subnets in which your clients are located. By default, Amazon FSx selects your VPC's default route table.
        pub route_table_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// When enabled, will skip the default final backup taken when the file system is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        pub skip_final_backup: pulumi_wasm_rust::Output<Option<bool>>,
        /// The storage capacity (GiB) of the file system. Valid values between `64` and `524288`.
        pub storage_capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// The filesystem storage type. Only `SSD` is supported.
        pub storage_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Throughput (MB/s) of the file system. Valid values depend on `deployment_type`. Must be one of `64`, `128`, `256`, `512`, `1024`, `2048`, `3072`, `4096` for `SINGLE_AZ_1`. Must be one of `160`, `320`, `640`, `1280`, `2560`, `3840`, `5120`, `7680`, `10240` for `SINGLE_AZ_2`.
        ///
        /// The following arguments are optional:
        pub throughput_capacity: pulumi_wasm_rust::Output<i32>,
        /// Identifier of the Virtual Private Cloud for the file system.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        pub weekly_maintenance_start_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OpenZfsFileSystemArgs) -> OpenZfsFileSystemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automatic_backup_retention_days_binding = args
            .automatic_backup_retention_days
            .get_inner();
        let backup_id_binding = args.backup_id.get_inner();
        let copy_tags_to_backups_binding = args.copy_tags_to_backups.get_inner();
        let copy_tags_to_volumes_binding = args.copy_tags_to_volumes.get_inner();
        let daily_automatic_backup_start_time_binding = args
            .daily_automatic_backup_start_time
            .get_inner();
        let delete_options_binding = args.delete_options.get_inner();
        let deployment_type_binding = args.deployment_type.get_inner();
        let disk_iops_configuration_binding = args.disk_iops_configuration.get_inner();
        let endpoint_ip_address_range_binding = args
            .endpoint_ip_address_range
            .get_inner();
        let final_backup_tags_binding = args.final_backup_tags.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let preferred_subnet_id_binding = args.preferred_subnet_id.get_inner();
        let root_volume_configuration_binding = args
            .root_volume_configuration
            .get_inner();
        let route_table_ids_binding = args.route_table_ids.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let skip_final_backup_binding = args.skip_final_backup.get_inner();
        let storage_capacity_binding = args.storage_capacity.get_inner();
        let storage_type_binding = args.storage_type.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let throughput_capacity_binding = args.throughput_capacity.get_inner();
        let weekly_maintenance_start_time_binding = args
            .weekly_maintenance_start_time
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/openZfsFileSystem:OpenZfsFileSystem".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automaticBackupRetentionDays".into(),
                    value: &automatic_backup_retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "backupId".into(),
                    value: &backup_id_binding,
                },
                register_interface::ObjectField {
                    name: "copyTagsToBackups".into(),
                    value: &copy_tags_to_backups_binding,
                },
                register_interface::ObjectField {
                    name: "copyTagsToVolumes".into(),
                    value: &copy_tags_to_volumes_binding,
                },
                register_interface::ObjectField {
                    name: "dailyAutomaticBackupStartTime".into(),
                    value: &daily_automatic_backup_start_time_binding,
                },
                register_interface::ObjectField {
                    name: "deleteOptions".into(),
                    value: &delete_options_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentType".into(),
                    value: &deployment_type_binding,
                },
                register_interface::ObjectField {
                    name: "diskIopsConfiguration".into(),
                    value: &disk_iops_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "endpointIpAddressRange".into(),
                    value: &endpoint_ip_address_range_binding,
                },
                register_interface::ObjectField {
                    name: "finalBackupTags".into(),
                    value: &final_backup_tags_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "preferredSubnetId".into(),
                    value: &preferred_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "rootVolumeConfiguration".into(),
                    value: &root_volume_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "routeTableIds".into(),
                    value: &route_table_ids_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "skipFinalBackup".into(),
                    value: &skip_final_backup_binding,
                },
                register_interface::ObjectField {
                    name: "storageCapacity".into(),
                    value: &storage_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "throughputCapacity".into(),
                    value: &throughput_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "weeklyMaintenanceStartTime".into(),
                    value: &weekly_maintenance_start_time_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "automaticBackupRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "backupId".into(),
                },
                register_interface::ResultField {
                    name: "copyTagsToBackups".into(),
                },
                register_interface::ResultField {
                    name: "copyTagsToVolumes".into(),
                },
                register_interface::ResultField {
                    name: "dailyAutomaticBackupStartTime".into(),
                },
                register_interface::ResultField {
                    name: "deleteOptions".into(),
                },
                register_interface::ResultField {
                    name: "deploymentType".into(),
                },
                register_interface::ResultField {
                    name: "diskIopsConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "endpointIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "endpointIpAddressRange".into(),
                },
                register_interface::ResultField {
                    name: "finalBackupTags".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceIds".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "preferredSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "rootVolumeConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "rootVolumeId".into(),
                },
                register_interface::ResultField {
                    name: "routeTableIds".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "skipFinalBackup".into(),
                },
                register_interface::ResultField {
                    name: "storageCapacity".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "throughputCapacity".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "weeklyMaintenanceStartTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OpenZfsFileSystemResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            automatic_backup_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticBackupRetentionDays").unwrap(),
            ),
            backup_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupId").unwrap(),
            ),
            copy_tags_to_backups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyTagsToBackups").unwrap(),
            ),
            copy_tags_to_volumes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyTagsToVolumes").unwrap(),
            ),
            daily_automatic_backup_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyAutomaticBackupStartTime").unwrap(),
            ),
            delete_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteOptions").unwrap(),
            ),
            deployment_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentType").unwrap(),
            ),
            disk_iops_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskIopsConfiguration").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            endpoint_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointIpAddress").unwrap(),
            ),
            endpoint_ip_address_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointIpAddressRange").unwrap(),
            ),
            final_backup_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalBackupTags").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            network_interface_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceIds").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            preferred_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredSubnetId").unwrap(),
            ),
            root_volume_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootVolumeConfiguration").unwrap(),
            ),
            root_volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootVolumeId").unwrap(),
            ),
            route_table_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableIds").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            skip_final_backup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipFinalBackup").unwrap(),
            ),
            storage_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageCapacity").unwrap(),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            throughput_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throughputCapacity").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            weekly_maintenance_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeklyMaintenanceStartTime").unwrap(),
            ),
        }
    }
}
