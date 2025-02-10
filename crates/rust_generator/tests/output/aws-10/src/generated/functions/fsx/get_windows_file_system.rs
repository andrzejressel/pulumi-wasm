#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_windows_file_system {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWindowsFileSystemArgs {
        /// Identifier of the file system (e.g. `fs-12345678`).
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The tags to associate with the file system.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetWindowsFileSystemResult {
        /// The ID for Microsoft Active Directory instance that the file system is join to.
        pub active_directory_id: pulumi_gestalt_rust::Output<String>,
        /// An array DNS alias names associated with the Amazon FSx file system.
        pub aliases: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration that Amazon FSx for Windows File Server uses to audit and log user accesses of files, folders, and file shares on the Amazon FSx for Windows File Server file system.
        pub audit_log_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::fsx::GetWindowsFileSystemAuditLogConfiguration,
            >,
        >,
        /// The number of days to retain automatic backups.
        pub automatic_backup_retention_days: pulumi_gestalt_rust::Output<i32>,
        pub backup_id: pulumi_gestalt_rust::Output<String>,
        /// A boolean flag indicating whether tags on the file system should be copied to backups.
        pub copy_tags_to_backups: pulumi_gestalt_rust::Output<bool>,
        /// The preferred time (in `HH:MM` format) to take daily automatic backups, in the UTC time zone.
        pub daily_automatic_backup_start_time: pulumi_gestalt_rust::Output<String>,
        /// The file system deployment type.
        pub deployment_type: pulumi_gestalt_rust::Output<String>,
        /// The SSD IOPS configuration for the file system.
        pub disk_iops_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::fsx::GetWindowsFileSystemDiskIopsConfiguration,
            >,
        >,
        /// DNS name for the file system (e.g. `fs-12345678.corp.example.com`).
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the file system (e.g. `fs-12345678`).
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the KMS Key to encrypt the file system at rest.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The IP address of the primary, or preferred, file server.
        pub preferred_file_server_ip: pulumi_gestalt_rust::Output<String>,
        /// Specifies the subnet in which you want the preferred file server to be located.
        pub preferred_subnet_id: pulumi_gestalt_rust::Output<String>,
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub skip_final_backup: pulumi_gestalt_rust::Output<bool>,
        /// The storage capacity of the file system in gibibytes (GiB).
        pub storage_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The type of storage the file system is using. If set to `SSD`, the file system uses solid state drive storage. If set to `HDD`, the file system uses hard disk drive storage.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the IDs of the subnets that the file system is accessible from.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The tags to associate with the file system.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Throughput (megabytes per second) of the file system in power of 2 increments. Minimum of `8` and maximum of `2048`.
        pub throughput_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the primary virtual private cloud (VPC) for the file system.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        pub weekly_maintenance_start_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWindowsFileSystemArgs,
    ) -> GetWindowsFileSystemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:fsx/getWindowsFileSystem:getWindowsFileSystem".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWindowsFileSystemResult {
            active_directory_id: o.get_field("activeDirectoryId"),
            aliases: o.get_field("aliases"),
            arn: o.get_field("arn"),
            audit_log_configurations: o.get_field("auditLogConfigurations"),
            automatic_backup_retention_days: o.get_field("automaticBackupRetentionDays"),
            backup_id: o.get_field("backupId"),
            copy_tags_to_backups: o.get_field("copyTagsToBackups"),
            daily_automatic_backup_start_time: o
                .get_field("dailyAutomaticBackupStartTime"),
            deployment_type: o.get_field("deploymentType"),
            disk_iops_configurations: o.get_field("diskIopsConfigurations"),
            dns_name: o.get_field("dnsName"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            network_interface_ids: o.get_field("networkInterfaceIds"),
            owner_id: o.get_field("ownerId"),
            preferred_file_server_ip: o.get_field("preferredFileServerIp"),
            preferred_subnet_id: o.get_field("preferredSubnetId"),
            security_group_ids: o.get_field("securityGroupIds"),
            skip_final_backup: o.get_field("skipFinalBackup"),
            storage_capacity: o.get_field("storageCapacity"),
            storage_type: o.get_field("storageType"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            throughput_capacity: o.get_field("throughputCapacity"),
            vpc_id: o.get_field("vpcId"),
            weekly_maintenance_start_time: o.get_field("weeklyMaintenanceStartTime"),
        }
    }
}
