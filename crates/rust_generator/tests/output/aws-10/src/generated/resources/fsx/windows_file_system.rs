/// Manages a FSx Windows File System. See the [FSx Windows Guide](https://docs.aws.amazon.com/fsx/latest/WindowsGuide/what-is.html) for more information.
///
/// > **NOTE:** Either the `active_directory_id` argument or `self_managed_active_directory` configuration block must be specified.
///
/// ## Example Usage
///
/// ### Using AWS Directory Service
///
/// Additional information for using AWS Directory Service with Windows File Systems can be found in the [FSx Windows Guide](https://docs.aws.amazon.com/fsx/latest/WindowsGuide/fsx-aws-managed-ad.html).
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = windows_file_system::create(
///         "example",
///         WindowsFileSystemArgs::builder()
///             .active_directory_id("${exampleAwsDirectoryServiceDirectory.id}")
///             .kms_key_id("${exampleAwsKmsKey.arn}")
///             .storage_capacity(300)
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .throughput_capacity(1024)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using a Self-Managed Microsoft Active Directory
///
/// Additional information for using AWS Directory Service with Windows File Systems can be found in the [FSx Windows Guide](https://docs.aws.amazon.com/fsx/latest/WindowsGuide/self-managed-AD.html).
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = windows_file_system::create(
///         "example",
///         WindowsFileSystemArgs::builder()
///             .kms_key_id("${exampleAwsKmsKey.arn}")
///             .self_managed_active_directory(
///                 WindowsFileSystemSelfManagedActiveDirectory::builder()
///                     .dnsIps(vec!["10.0.0.111", "10.0.0.222",])
///                     .domainName("corp.example.com")
///                     .password("avoid-plaintext-passwords")
///                     .username("Admin")
///                     .build_struct(),
///             )
///             .storage_capacity(300)
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .throughput_capacity(1024)
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
/// $ pulumi import aws:fsx/windowsFileSystem:WindowsFileSystem example fs-543ab12b1ca672f33
/// ```
/// Certain resource arguments, like `security_group_ids` and the `self_managed_active_directory` configuation block `password`, do not have a FSx API method for reading the information after creation. If these arguments are set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod windows_file_system {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WindowsFileSystemArgs {
        /// The ID for an existing Microsoft Active Directory instance that the file system should join when it's created. Cannot be specified with `self_managed_active_directory`.
        #[builder(into, default)]
        pub active_directory_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array DNS alias names that you want to associate with the Amazon FSx file system.  For more information, see [Working with DNS Aliases](https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html)
        #[builder(into, default)]
        pub aliases: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The configuration that Amazon FSx for Windows File Server uses to audit and log user accesses of files, folders, and file shares on the Amazon FSx for Windows File Server file system. See `audit_log_configuration` Block for details.
        #[builder(into, default)]
        pub audit_log_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::WindowsFileSystemAuditLogConfiguration>,
        >,
        /// The number of days to retain automatic backups. Minimum of `0` and maximum of `90`. Defaults to `7`. Set to `0` to disable.
        #[builder(into, default)]
        pub automatic_backup_retention_days: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The ID of the source backup to create the filesystem from.
        #[builder(into, default)]
        pub backup_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean flag indicating whether tags on the file system should be copied to backups. Defaults to `false`.
        #[builder(into, default)]
        pub copy_tags_to_backups: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The preferred time (in `HH:MM` format) to take daily automatic backups, in the UTC time zone.
        #[builder(into, default)]
        pub daily_automatic_backup_start_time: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the file system deployment type, valid values are `MULTI_AZ_1`, `SINGLE_AZ_1` and `SINGLE_AZ_2`. Default value is `SINGLE_AZ_1`.
        #[builder(into, default)]
        pub deployment_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SSD IOPS configuration for the Amazon FSx for Windows File Server file system. See `disk_iops_configuration` Block for details.
        #[builder(into, default)]
        pub disk_iops_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::WindowsFileSystemDiskIopsConfiguration>,
        >,
        /// A map of tags to apply to the file system's final backup.
        #[builder(into, default)]
        pub final_backup_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN for the KMS Key to encrypt the file system at rest. Defaults to an AWS managed KMS Key.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the subnet in which you want the preferred file server to be located. Required for when deployment type is `MULTI_AZ_1`.
        #[builder(into, default)]
        pub preferred_subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Configuration block that Amazon FSx uses to join the Windows File Server instance to your self-managed (including on-premises) Microsoft Active Directory (AD) directory. Cannot be specified with `active_directory_id`. See `self_managed_active_directory` Block for details.
        #[builder(into, default)]
        pub self_managed_active_directory: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::WindowsFileSystemSelfManagedActiveDirectory>,
        >,
        /// When enabled, will skip the default final backup taken when the file system is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        #[builder(into, default)]
        pub skip_final_backup: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Storage capacity (GiB) of the file system. Minimum of 32 and maximum of 65536. If the storage type is set to `HDD` the minimum value is 2000. Required when not creating filesystem for a backup.
        #[builder(into, default)]
        pub storage_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the storage type, Valid values are `SSD` and `HDD`. `HDD` is supported on `SINGLE_AZ_2` and `MULTI_AZ_1` Windows file system deployment types. Default value is `SSD`.
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from. To specify more than a single subnet set `deployment_type` to `MULTI_AZ_1`.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Throughput (megabytes per second) of the file system. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/fsx/latest/WindowsGuide/performance.html).
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
    pub struct WindowsFileSystemResult {
        /// The ID for an existing Microsoft Active Directory instance that the file system should join when it's created. Cannot be specified with `self_managed_active_directory`.
        pub active_directory_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// An array DNS alias names that you want to associate with the Amazon FSx file system.  For more information, see [Working with DNS Aliases](https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html)
        pub aliases: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration that Amazon FSx for Windows File Server uses to audit and log user accesses of files, folders, and file shares on the Amazon FSx for Windows File Server file system. See `audit_log_configuration` Block for details.
        pub audit_log_configuration: pulumi_gestalt_rust::Output<
            super::super::types::fsx::WindowsFileSystemAuditLogConfiguration,
        >,
        /// The number of days to retain automatic backups. Minimum of `0` and maximum of `90`. Defaults to `7`. Set to `0` to disable.
        pub automatic_backup_retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the source backup to create the filesystem from.
        pub backup_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A boolean flag indicating whether tags on the file system should be copied to backups. Defaults to `false`.
        pub copy_tags_to_backups: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The preferred time (in `HH:MM` format) to take daily automatic backups, in the UTC time zone.
        pub daily_automatic_backup_start_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies the file system deployment type, valid values are `MULTI_AZ_1`, `SINGLE_AZ_1` and `SINGLE_AZ_2`. Default value is `SINGLE_AZ_1`.
        pub deployment_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SSD IOPS configuration for the Amazon FSx for Windows File Server file system. See `disk_iops_configuration` Block for details.
        pub disk_iops_configuration: pulumi_gestalt_rust::Output<
            super::super::types::fsx::WindowsFileSystemDiskIopsConfiguration,
        >,
        /// DNS name for the file system, e.g., `fs-12345678.corp.example.com` (domain name matching the Active Directory domain name)
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to apply to the file system's final backup.
        pub final_backup_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN for the KMS Key to encrypt the file system at rest. Defaults to an AWS managed KMS Key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Set of Elastic Network Interface identifiers from which the file system is accessible.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The IP address of the primary, or preferred, file server.
        pub preferred_file_server_ip: pulumi_gestalt_rust::Output<String>,
        /// Specifies the subnet in which you want the preferred file server to be located. Required for when deployment type is `MULTI_AZ_1`.
        pub preferred_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// For `MULTI_AZ_1` deployment types, use this endpoint when performing administrative tasks on the file system using Amazon FSx Remote PowerShell. For `SINGLE_AZ_1` deployment types, this is the DNS name of the file system.
        pub remote_administration_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Configuration block that Amazon FSx uses to join the Windows File Server instance to your self-managed (including on-premises) Microsoft Active Directory (AD) directory. Cannot be specified with `active_directory_id`. See `self_managed_active_directory` Block for details.
        pub self_managed_active_directory: pulumi_gestalt_rust::Output<
            Option<super::super::types::fsx::WindowsFileSystemSelfManagedActiveDirectory>,
        >,
        /// When enabled, will skip the default final backup taken when the file system is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        pub skip_final_backup: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Storage capacity (GiB) of the file system. Minimum of 32 and maximum of 65536. If the storage type is set to `HDD` the minimum value is 2000. Required when not creating filesystem for a backup.
        pub storage_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the storage type, Valid values are `SSD` and `HDD`. `HDD` is supported on `SINGLE_AZ_2` and `MULTI_AZ_1` Windows file system deployment types. Default value is `SSD`.
        pub storage_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from. To specify more than a single subnet set `deployment_type` to `MULTI_AZ_1`.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Throughput (megabytes per second) of the file system. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/fsx/latest/WindowsGuide/performance.html).
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WindowsFileSystemArgs,
    ) -> WindowsFileSystemResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let active_directory_id_binding = args
            .active_directory_id
            .get_output(context)
            .get_inner();
        let aliases_binding = args.aliases.get_output(context).get_inner();
        let audit_log_configuration_binding = args
            .audit_log_configuration
            .get_output(context)
            .get_inner();
        let automatic_backup_retention_days_binding = args
            .automatic_backup_retention_days
            .get_output(context)
            .get_inner();
        let backup_id_binding = args.backup_id.get_output(context).get_inner();
        let copy_tags_to_backups_binding = args
            .copy_tags_to_backups
            .get_output(context)
            .get_inner();
        let daily_automatic_backup_start_time_binding = args
            .daily_automatic_backup_start_time
            .get_output(context)
            .get_inner();
        let deployment_type_binding = args
            .deployment_type
            .get_output(context)
            .get_inner();
        let disk_iops_configuration_binding = args
            .disk_iops_configuration
            .get_output(context)
            .get_inner();
        let final_backup_tags_binding = args
            .final_backup_tags
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let preferred_subnet_id_binding = args
            .preferred_subnet_id
            .get_output(context)
            .get_inner();
        let security_group_ids_binding = args
            .security_group_ids
            .get_output(context)
            .get_inner();
        let self_managed_active_directory_binding = args
            .self_managed_active_directory
            .get_output(context)
            .get_inner();
        let skip_final_backup_binding = args
            .skip_final_backup
            .get_output(context)
            .get_inner();
        let storage_capacity_binding = args
            .storage_capacity
            .get_output(context)
            .get_inner();
        let storage_type_binding = args.storage_type.get_output(context).get_inner();
        let subnet_ids_binding = args.subnet_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let throughput_capacity_binding = args
            .throughput_capacity
            .get_output(context)
            .get_inner();
        let weekly_maintenance_start_time_binding = args
            .weekly_maintenance_start_time
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/windowsFileSystem:WindowsFileSystem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activeDirectoryId".into(),
                    value: &active_directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "aliases".into(),
                    value: &aliases_binding,
                },
                register_interface::ObjectField {
                    name: "auditLogConfiguration".into(),
                    value: &audit_log_configuration_binding,
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
                    name: "deploymentType".into(),
                    value: &deployment_type_binding,
                },
                register_interface::ObjectField {
                    name: "diskIopsConfiguration".into(),
                    value: &disk_iops_configuration_binding,
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
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "selfManagedActiveDirectory".into(),
                    value: &self_managed_active_directory_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        WindowsFileSystemResult {
            active_directory_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activeDirectoryId"),
            ),
            aliases: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aliases"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            audit_log_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("auditLogConfiguration"),
            ),
            automatic_backup_retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticBackupRetentionDays"),
            ),
            backup_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupId"),
            ),
            copy_tags_to_backups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("copyTagsToBackups"),
            ),
            daily_automatic_backup_start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dailyAutomaticBackupStartTime"),
            ),
            deployment_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentType"),
            ),
            disk_iops_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskIopsConfiguration"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            final_backup_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("finalBackupTags"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            network_interface_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceIds"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            preferred_file_server_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredFileServerIp"),
            ),
            preferred_subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredSubnetId"),
            ),
            remote_administration_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("remoteAdministrationEndpoint"),
            ),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            self_managed_active_directory: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfManagedActiveDirectory"),
            ),
            skip_final_backup: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipFinalBackup"),
            ),
            storage_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageCapacity"),
            ),
            storage_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            throughput_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("throughputCapacity"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcId"),
            ),
            weekly_maintenance_start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("weeklyMaintenanceStartTime"),
            ),
        }
    }
}
