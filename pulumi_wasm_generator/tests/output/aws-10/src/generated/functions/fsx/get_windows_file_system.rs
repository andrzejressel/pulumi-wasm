pub mod get_windows_file_system {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWindowsFileSystemArgs {
        /// Identifier of the file system (e.g. `fs-12345678`).
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The tags to associate with the file system.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetWindowsFileSystemResult {
        /// The ID for Microsoft Active Directory instance that the file system is join to.
        pub active_directory_id: pulumi_wasm_rust::Output<String>,
        /// An array DNS alias names associated with the Amazon FSx file system.
        pub aliases: pulumi_wasm_rust::Output<Vec<String>>,
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The configuration that Amazon FSx for Windows File Server uses to audit and log user accesses of files, folders, and file shares on the Amazon FSx for Windows File Server file system.
        pub audit_log_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::fsx::GetWindowsFileSystemAuditLogConfiguration,
            >,
        >,
        /// The number of days to retain automatic backups.
        pub automatic_backup_retention_days: pulumi_wasm_rust::Output<i32>,
        pub backup_id: pulumi_wasm_rust::Output<String>,
        /// A boolean flag indicating whether tags on the file system should be copied to backups.
        pub copy_tags_to_backups: pulumi_wasm_rust::Output<bool>,
        /// The preferred time (in `HH:MM` format) to take daily automatic backups, in the UTC time zone.
        pub daily_automatic_backup_start_time: pulumi_wasm_rust::Output<String>,
        /// The file system deployment type.
        pub deployment_type: pulumi_wasm_rust::Output<String>,
        /// The SSD IOPS configuration for the file system.
        pub disk_iops_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::fsx::GetWindowsFileSystemDiskIopsConfiguration,
            >,
        >,
        /// DNS name for the file system (e.g. `fs-12345678.corp.example.com`).
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Identifier of the file system (e.g. `fs-12345678`).
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN for the KMS Key to encrypt the file system at rest.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        pub network_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// The IP address of the primary, or preferred, file server.
        pub preferred_file_server_ip: pulumi_wasm_rust::Output<String>,
        /// Specifies the subnet in which you want the preferred file server to be located.
        pub preferred_subnet_id: pulumi_wasm_rust::Output<String>,
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub skip_final_backup: pulumi_wasm_rust::Output<bool>,
        /// The storage capacity of the file system in gibibytes (GiB).
        pub storage_capacity: pulumi_wasm_rust::Output<i32>,
        /// The type of storage the file system is using. If set to `SSD`, the file system uses solid state drive storage. If set to `HDD`, the file system uses hard disk drive storage.
        pub storage_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the IDs of the subnets that the file system is accessible from.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The tags to associate with the file system.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Throughput (megabytes per second) of the file system in power of 2 increments. Minimum of `8` and maximum of `2048`.
        pub throughput_capacity: pulumi_wasm_rust::Output<i32>,
        /// The ID of the primary virtual private cloud (VPC) for the file system.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        pub weekly_maintenance_start_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetWindowsFileSystemArgs,
    ) -> GetWindowsFileSystemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:fsx/getWindowsFileSystem:getWindowsFileSystem".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activeDirectoryId".into(),
                },
                register_interface::ResultField {
                    name: "aliases".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "auditLogConfigurations".into(),
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
                    name: "deploymentType".into(),
                },
                register_interface::ResultField {
                    name: "diskIopsConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "preferredFileServerIp".into(),
                },
                register_interface::ResultField {
                    name: "preferredSubnetId".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWindowsFileSystemResult {
            active_directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeDirectoryId").unwrap(),
            ),
            aliases: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliases").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            audit_log_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auditLogConfigurations").unwrap(),
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
            deployment_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentType").unwrap(),
            ),
            disk_iops_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskIopsConfigurations").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            network_interface_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceIds").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            preferred_file_server_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredFileServerIp").unwrap(),
            ),
            preferred_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredSubnetId").unwrap(),
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
