/// Manages a FSx Lustre File System. See the [FSx Lustre Guide](https://docs.aws.amazon.com/fsx/latest/LustreGuide/what-is.html) for more information.
///
/// > **NOTE:** `auto_import_policy`, `export_path`, `import_path` and `imported_file_chunk_size` are not supported with the `PERSISTENT_2` deployment type. Use `aws.fsx.DataRepositoryAssociation` instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = lustre_file_system::create(
///         "example",
///         LustreFileSystemArgs::builder()
///             .import_path("s3://${exampleAwsS3Bucket.bucket}")
///             .storage_capacity(1200)
///             .subnet_ids("${exampleAwsSubnet.id}")
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
/// $ pulumi import aws:fsx/lustreFileSystem:LustreFileSystem example fs-543ab12b1ca672f33
/// ```
/// Certain resource arguments, like `security_group_ids`, do not have a FSx API method for reading the information after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
pub mod lustre_file_system {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LustreFileSystemArgs {
        /// How Amazon FSx keeps your file and directory listings up to date as you add or modify objects in your linked S3 bucket. see [Auto Import Data Repo](https://docs.aws.amazon.com/fsx/latest/LustreGuide/autoimport-data-repo.html) for more details. Only supported on `PERSISTENT_1` deployment types.
        #[builder(into, default)]
        pub auto_import_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days. only valid for `PERSISTENT_1` and `PERSISTENT_2` deployment_type.
        #[builder(into, default)]
        pub automatic_backup_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the source backup to create the filesystem from.
        #[builder(into, default)]
        pub backup_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A boolean flag indicating whether tags for the file system should be copied to backups. Applicable for `PERSISTENT_1` and `PERSISTENT_2` deployment_type. The default value is false.
        #[builder(into, default)]
        pub copy_tags_to_backups: pulumi_wasm_rust::Output<Option<bool>>,
        /// A recurring daily time, in the format HH:MM. HH is the zero-padded hour of the day (0-23), and MM is the zero-padded minute of the hour. For example, 05:00 specifies 5 AM daily. only valid for `PERSISTENT_1` and `PERSISTENT_2` deployment_type. Requires `automatic_backup_retention_days` to be set.
        #[builder(into, default)]
        pub daily_automatic_backup_start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the data compression configuration for the file system. Valid values are `LZ4` and `NONE`. Default value is `NONE`. Unsetting this value reverts the compression type back to `NONE`.
        #[builder(into, default)]
        pub data_compression_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The filesystem deployment type. One of: `SCRATCH_1`, `SCRATCH_2`, `PERSISTENT_1`, `PERSISTENT_2`.
        #[builder(into, default)]
        pub deployment_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of drive cache used by `PERSISTENT_1` filesystems that are provisioned with `HDD` storage_type. Required for `HDD` storage_type, set to either `READ` or `NONE`.
        #[builder(into, default)]
        pub drive_cache_type: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 URI (with optional prefix) where the root of your Amazon FSx file system is exported. Can only be specified with `import_path` argument and the path must use the same Amazon S3 bucket as specified in `import_path`. Set equal to `import_path` to overwrite files on export. Defaults to `s3://{IMPORT BUCKET}/FSxLustre{CREATION TIMESTAMP}`. Only supported on `PERSISTENT_1` deployment types.
        #[builder(into, default)]
        pub export_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the Lustre version for the file system that you're creating. Valid values are 2.10 for `SCRATCH_1`, `SCRATCH_2` and `PERSISTENT_1` deployment types. Valid values for 2.12 include all deployment types.
        #[builder(into, default)]
        pub file_system_type_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to apply to the file system's final backup.
        ///
        /// **Note:** If the filesystem uses a Scratch deployment type, final backup during delete will always be skipped and this argument will not be used even when set.
        #[builder(into, default)]
        pub final_backup_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// S3 URI (with optional prefix) that you're using as the data repository for your FSx for Lustre file system. For example, `s3://example-bucket/optional-prefix/`. Only supported on `PERSISTENT_1` deployment types.
        #[builder(into, default)]
        pub import_path: pulumi_wasm_rust::Output<Option<String>>,
        /// For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. Can only be specified with `import_path` argument. Defaults to `1024`. Minimum of `1` and maximum of `512000`. Only supported on `PERSISTENT_1` deployment types.
        #[builder(into, default)]
        pub imported_file_chunk_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// ARN for the KMS Key to encrypt the file system at rest, applicable for `PERSISTENT_1` and `PERSISTENT_2` deployment_type. Defaults to an AWS managed KMS Key.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Lustre logging configuration used when creating an Amazon FSx for Lustre file system. When logging is enabled, Lustre logs error and warning events for data repositories associated with your file system to Amazon CloudWatch Logs. See `log_configuration` Block for details.
        #[builder(into, default)]
        pub log_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::LustreFileSystemLogConfiguration>,
        >,
        /// The Lustre metadata configuration used when creating an Amazon FSx for Lustre file system. This can be used to specify a user provisioned metadata scale. This is only supported when `deployment_type` is set to `PERSISTENT_2`. See `metadata_configuration` Block for details.
        #[builder(into, default)]
        pub metadata_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::LustreFileSystemMetadataConfiguration>,
        >,
        /// Describes the amount of read and write throughput for each 1 tebibyte of storage, in MB/s/TiB, required for the `PERSISTENT_1` and `PERSISTENT_2` deployment_type. Valid values for `PERSISTENT_1` deployment_type and `SSD` storage_type are 50, 100, 200. Valid values for `PERSISTENT_1` deployment_type and `HDD` storage_type are 12, 40. Valid values for `PERSISTENT_2` deployment_type and ` SSD` storage_type are 125, 250, 500, 1000.
        #[builder(into, default)]
        pub per_unit_storage_throughput: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Lustre root squash configuration used when creating an Amazon FSx for Lustre file system. When enabled, root squash restricts root-level access from clients that try to access your file system as a root user. See `root_squash_configuration` Block for details.
        #[builder(into, default)]
        pub root_squash_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::LustreFileSystemRootSquashConfiguration>,
        >,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// When enabled, will skip the default final backup taken when the file system is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `true`.
        ///
        /// **Note:** If the filesystem uses a Scratch deployment type, final backup during delete will always be skipped and this argument will not be used even when set.
        #[builder(into, default)]
        pub skip_final_backup: pulumi_wasm_rust::Output<Option<bool>>,
        /// The storage capacity (GiB) of the file system. Minimum of `1200`. See more details at [Allowed values for Fsx storage capacity](https://docs.aws.amazon.com/fsx/latest/APIReference/API_CreateFileSystem.html#FSx-CreateFileSystem-request-StorageCapacity). Update is allowed only for `SCRATCH_2`, `PERSISTENT_1` and `PERSISTENT_2` deployment types, See more details at [Fsx Storage Capacity Update](https://docs.aws.amazon.com/fsx/latest/APIReference/API_UpdateFileSystem.html#FSx-UpdateFileSystem-request-StorageCapacity). Required when not creating filesystem for a backup.
        #[builder(into, default)]
        pub storage_capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// The filesystem storage type. Either `SSD` or `HDD`, defaults to `SSD`. `HDD` is only supported on `PERSISTENT_1` deployment types.
        #[builder(into, default)]
        pub storage_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from. File systems currently support only one subnet. The file server is also launched in that subnet's Availability Zone.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        #[builder(into, default)]
        pub weekly_maintenance_start_time: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LustreFileSystemResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// How Amazon FSx keeps your file and directory listings up to date as you add or modify objects in your linked S3 bucket. see [Auto Import Data Repo](https://docs.aws.amazon.com/fsx/latest/LustreGuide/autoimport-data-repo.html) for more details. Only supported on `PERSISTENT_1` deployment types.
        pub auto_import_policy: pulumi_wasm_rust::Output<String>,
        /// The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days. only valid for `PERSISTENT_1` and `PERSISTENT_2` deployment_type.
        pub automatic_backup_retention_days: pulumi_wasm_rust::Output<i32>,
        /// The ID of the source backup to create the filesystem from.
        pub backup_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A boolean flag indicating whether tags for the file system should be copied to backups. Applicable for `PERSISTENT_1` and `PERSISTENT_2` deployment_type. The default value is false.
        pub copy_tags_to_backups: pulumi_wasm_rust::Output<Option<bool>>,
        /// A recurring daily time, in the format HH:MM. HH is the zero-padded hour of the day (0-23), and MM is the zero-padded minute of the hour. For example, 05:00 specifies 5 AM daily. only valid for `PERSISTENT_1` and `PERSISTENT_2` deployment_type. Requires `automatic_backup_retention_days` to be set.
        pub daily_automatic_backup_start_time: pulumi_wasm_rust::Output<String>,
        /// Sets the data compression configuration for the file system. Valid values are `LZ4` and `NONE`. Default value is `NONE`. Unsetting this value reverts the compression type back to `NONE`.
        pub data_compression_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The filesystem deployment type. One of: `SCRATCH_1`, `SCRATCH_2`, `PERSISTENT_1`, `PERSISTENT_2`.
        pub deployment_type: pulumi_wasm_rust::Output<Option<String>>,
        /// DNS name for the file system, e.g., `fs-12345678.fsx.us-west-2.amazonaws.com`
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// The type of drive cache used by `PERSISTENT_1` filesystems that are provisioned with `HDD` storage_type. Required for `HDD` storage_type, set to either `READ` or `NONE`.
        pub drive_cache_type: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 URI (with optional prefix) where the root of your Amazon FSx file system is exported. Can only be specified with `import_path` argument and the path must use the same Amazon S3 bucket as specified in `import_path`. Set equal to `import_path` to overwrite files on export. Defaults to `s3://{IMPORT BUCKET}/FSxLustre{CREATION TIMESTAMP}`. Only supported on `PERSISTENT_1` deployment types.
        pub export_path: pulumi_wasm_rust::Output<String>,
        /// Sets the Lustre version for the file system that you're creating. Valid values are 2.10 for `SCRATCH_1`, `SCRATCH_2` and `PERSISTENT_1` deployment types. Valid values for 2.12 include all deployment types.
        pub file_system_type_version: pulumi_wasm_rust::Output<String>,
        /// A map of tags to apply to the file system's final backup.
        ///
        /// **Note:** If the filesystem uses a Scratch deployment type, final backup during delete will always be skipped and this argument will not be used even when set.
        pub final_backup_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// S3 URI (with optional prefix) that you're using as the data repository for your FSx for Lustre file system. For example, `s3://example-bucket/optional-prefix/`. Only supported on `PERSISTENT_1` deployment types.
        pub import_path: pulumi_wasm_rust::Output<Option<String>>,
        /// For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. Can only be specified with `import_path` argument. Defaults to `1024`. Minimum of `1` and maximum of `512000`. Only supported on `PERSISTENT_1` deployment types.
        pub imported_file_chunk_size: pulumi_wasm_rust::Output<i32>,
        /// ARN for the KMS Key to encrypt the file system at rest, applicable for `PERSISTENT_1` and `PERSISTENT_2` deployment_type. Defaults to an AWS managed KMS Key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The Lustre logging configuration used when creating an Amazon FSx for Lustre file system. When logging is enabled, Lustre logs error and warning events for data repositories associated with your file system to Amazon CloudWatch Logs. See `log_configuration` Block for details.
        pub log_configuration: pulumi_wasm_rust::Output<
            super::super::types::fsx::LustreFileSystemLogConfiguration,
        >,
        /// The Lustre metadata configuration used when creating an Amazon FSx for Lustre file system. This can be used to specify a user provisioned metadata scale. This is only supported when `deployment_type` is set to `PERSISTENT_2`. See `metadata_configuration` Block for details.
        pub metadata_configuration: pulumi_wasm_rust::Output<
            super::super::types::fsx::LustreFileSystemMetadataConfiguration,
        >,
        /// The value to be used when mounting the filesystem.
        pub mount_name: pulumi_wasm_rust::Output<String>,
        /// Set of Elastic Network Interface identifiers from which the file system is accessible. As explained in the [documentation](https://docs.aws.amazon.com/fsx/latest/LustreGuide/mounting-on-premises.html), the first network interface returned is the primary network interface.
        pub network_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Describes the amount of read and write throughput for each 1 tebibyte of storage, in MB/s/TiB, required for the `PERSISTENT_1` and `PERSISTENT_2` deployment_type. Valid values for `PERSISTENT_1` deployment_type and `SSD` storage_type are 50, 100, 200. Valid values for `PERSISTENT_1` deployment_type and `HDD` storage_type are 12, 40. Valid values for `PERSISTENT_2` deployment_type and ` SSD` storage_type are 125, 250, 500, 1000.
        pub per_unit_storage_throughput: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Lustre root squash configuration used when creating an Amazon FSx for Lustre file system. When enabled, root squash restricts root-level access from clients that try to access your file system as a root user. See `root_squash_configuration` Block for details.
        pub root_squash_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::LustreFileSystemRootSquashConfiguration>,
        >,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// When enabled, will skip the default final backup taken when the file system is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `true`.
        ///
        /// **Note:** If the filesystem uses a Scratch deployment type, final backup during delete will always be skipped and this argument will not be used even when set.
        pub skip_final_backup: pulumi_wasm_rust::Output<Option<bool>>,
        /// The storage capacity (GiB) of the file system. Minimum of `1200`. See more details at [Allowed values for Fsx storage capacity](https://docs.aws.amazon.com/fsx/latest/APIReference/API_CreateFileSystem.html#FSx-CreateFileSystem-request-StorageCapacity). Update is allowed only for `SCRATCH_2`, `PERSISTENT_1` and `PERSISTENT_2` deployment types, See more details at [Fsx Storage Capacity Update](https://docs.aws.amazon.com/fsx/latest/APIReference/API_UpdateFileSystem.html#FSx-UpdateFileSystem-request-StorageCapacity). Required when not creating filesystem for a backup.
        pub storage_capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// The filesystem storage type. Either `SSD` or `HDD`, defaults to `SSD`. `HDD` is only supported on `PERSISTENT_1` deployment types.
        pub storage_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from. File systems currently support only one subnet. The file server is also launched in that subnet's Availability Zone.
        ///
        /// The following arguments are optional:
        pub subnet_ids: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier of the Virtual Private Cloud for the file system.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        pub weekly_maintenance_start_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LustreFileSystemArgs) -> LustreFileSystemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_import_policy_binding = args.auto_import_policy.get_inner();
        let automatic_backup_retention_days_binding = args
            .automatic_backup_retention_days
            .get_inner();
        let backup_id_binding = args.backup_id.get_inner();
        let copy_tags_to_backups_binding = args.copy_tags_to_backups.get_inner();
        let daily_automatic_backup_start_time_binding = args
            .daily_automatic_backup_start_time
            .get_inner();
        let data_compression_type_binding = args.data_compression_type.get_inner();
        let deployment_type_binding = args.deployment_type.get_inner();
        let drive_cache_type_binding = args.drive_cache_type.get_inner();
        let export_path_binding = args.export_path.get_inner();
        let file_system_type_version_binding = args.file_system_type_version.get_inner();
        let final_backup_tags_binding = args.final_backup_tags.get_inner();
        let import_path_binding = args.import_path.get_inner();
        let imported_file_chunk_size_binding = args.imported_file_chunk_size.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let log_configuration_binding = args.log_configuration.get_inner();
        let metadata_configuration_binding = args.metadata_configuration.get_inner();
        let per_unit_storage_throughput_binding = args
            .per_unit_storage_throughput
            .get_inner();
        let root_squash_configuration_binding = args
            .root_squash_configuration
            .get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let skip_final_backup_binding = args.skip_final_backup.get_inner();
        let storage_capacity_binding = args.storage_capacity.get_inner();
        let storage_type_binding = args.storage_type.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let weekly_maintenance_start_time_binding = args
            .weekly_maintenance_start_time
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/lustreFileSystem:LustreFileSystem".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoImportPolicy".into(),
                    value: &auto_import_policy_binding,
                },
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
                    name: "dailyAutomaticBackupStartTime".into(),
                    value: &daily_automatic_backup_start_time_binding,
                },
                register_interface::ObjectField {
                    name: "dataCompressionType".into(),
                    value: &data_compression_type_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentType".into(),
                    value: &deployment_type_binding,
                },
                register_interface::ObjectField {
                    name: "driveCacheType".into(),
                    value: &drive_cache_type_binding,
                },
                register_interface::ObjectField {
                    name: "exportPath".into(),
                    value: &export_path_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemTypeVersion".into(),
                    value: &file_system_type_version_binding,
                },
                register_interface::ObjectField {
                    name: "finalBackupTags".into(),
                    value: &final_backup_tags_binding,
                },
                register_interface::ObjectField {
                    name: "importPath".into(),
                    value: &import_path_binding,
                },
                register_interface::ObjectField {
                    name: "importedFileChunkSize".into(),
                    value: &imported_file_chunk_size_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "logConfiguration".into(),
                    value: &log_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "metadataConfiguration".into(),
                    value: &metadata_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "perUnitStorageThroughput".into(),
                    value: &per_unit_storage_throughput_binding,
                },
                register_interface::ObjectField {
                    name: "rootSquashConfiguration".into(),
                    value: &root_squash_configuration_binding,
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
                    name: "weeklyMaintenanceStartTime".into(),
                    value: &weekly_maintenance_start_time_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoImportPolicy".into(),
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
                    name: "dailyAutomaticBackupStartTime".into(),
                },
                register_interface::ResultField {
                    name: "dataCompressionType".into(),
                },
                register_interface::ResultField {
                    name: "deploymentType".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "driveCacheType".into(),
                },
                register_interface::ResultField {
                    name: "exportPath".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemTypeVersion".into(),
                },
                register_interface::ResultField {
                    name: "finalBackupTags".into(),
                },
                register_interface::ResultField {
                    name: "importPath".into(),
                },
                register_interface::ResultField {
                    name: "importedFileChunkSize".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "logConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "metadataConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "mountName".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceIds".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "perUnitStorageThroughput".into(),
                },
                register_interface::ResultField {
                    name: "rootSquashConfiguration".into(),
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
        LustreFileSystemResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_import_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoImportPolicy").unwrap(),
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
            daily_automatic_backup_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyAutomaticBackupStartTime").unwrap(),
            ),
            data_compression_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataCompressionType").unwrap(),
            ),
            deployment_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentType").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            drive_cache_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driveCacheType").unwrap(),
            ),
            export_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportPath").unwrap(),
            ),
            file_system_type_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemTypeVersion").unwrap(),
            ),
            final_backup_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalBackupTags").unwrap(),
            ),
            import_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importPath").unwrap(),
            ),
            imported_file_chunk_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importedFileChunkSize").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            log_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logConfiguration").unwrap(),
            ),
            metadata_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataConfiguration").unwrap(),
            ),
            mount_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mountName").unwrap(),
            ),
            network_interface_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceIds").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            per_unit_storage_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("perUnitStorageThroughput").unwrap(),
            ),
            root_squash_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootSquashConfiguration").unwrap(),
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
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            weekly_maintenance_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeklyMaintenanceStartTime").unwrap(),
            ),
        }
    }
}