#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ontap_file_system {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOntapFileSystemArgs {
        /// Identifier of the file system (e.g. `fs-12345678`).
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The tags associated with the file system.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOntapFileSystemResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The number of days to retain automatic backups.
        pub automatic_backup_retention_days: pulumi_gestalt_rust::Output<i32>,
        /// The preferred time (in `HH:MM` format) to take daily automatic backups, in the UTC time zone.
        pub daily_automatic_backup_start_time: pulumi_gestalt_rust::Output<String>,
        /// The file system deployment type.
        pub deployment_type: pulumi_gestalt_rust::Output<String>,
        /// The SSD IOPS configuration for the Amazon FSx for NetApp ONTAP file system, specifying the number of provisioned IOPS and the provision mode. See Disk IOPS Below.
        pub disk_iops_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::fsx::GetOntapFileSystemDiskIopsConfiguration>,
        >,
        /// DNS name for the file system.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// (Multi-AZ only) Specifies the IP address range in which the endpoints to access your file system exist.
        pub endpoint_ip_address_range: pulumi_gestalt_rust::Output<String>,
        /// The Management and Intercluster FileSystemEndpoints that are used to access data or to manage the file system using the NetApp ONTAP CLI, REST API, or NetApp SnapMirror. See FileSystemEndpoints below.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::fsx::GetOntapFileSystemEndpoint>,
        >,
        /// The number of HA pairs for the file system.
        pub ha_pairs: pulumi_gestalt_rust::Output<i32>,
        /// Identifier of the file system (e.g. `fs-12345678`).
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the KMS Key to encrypt the file system at rest.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The IDs of the elastic network interfaces from which a specific file system is accessible.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the subnet in which you want the preferred file server to be located.
        pub preferred_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// (Multi-AZ only) The VPC route tables in which your file system's endpoints exist.
        pub route_table_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The storage capacity of the file system in gibibytes (GiB).
        pub storage_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The type of storage the file system is using. If set to `SSD`, the file system uses solid state drive storage. If set to `HDD`, the file system uses hard disk drive storage.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the IDs of the subnets that the file system is accessible from. For the MULTI_AZ_1 file system deployment type, there are two subnet IDs, one for the preferred file server and one for the standby file server. The preferred file server subnet identified in the `preferred_subnet_id` property.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The tags associated with the file system.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The sustained throughput of an Amazon FSx file system in Megabytes per second (MBps). If the file system uses multiple HA pairs this will equal throuthput_capacity_per_ha_pair x ha_pairs
        pub throughput_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The sustained throughput of each HA pair for an Amazon FSx file system in Megabytes per second (MBps).
        pub throughput_capacity_per_ha_pair: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the primary virtual private cloud (VPC) for the file system.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The preferred start time (in `D:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        pub weekly_maintenance_start_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetOntapFileSystemArgs,
    ) -> GetOntapFileSystemResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:fsx/getOntapFileSystem:getOntapFileSystem".into(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOntapFileSystemResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            automatic_backup_retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticBackupRetentionDays"),
            ),
            daily_automatic_backup_start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dailyAutomaticBackupStartTime"),
            ),
            deployment_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentType"),
            ),
            disk_iops_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskIopsConfigurations"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            endpoint_ip_address_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointIpAddressRange"),
            ),
            endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoints"),
            ),
            ha_pairs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("haPairs"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            network_interface_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceIds"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            preferred_subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredSubnetId"),
            ),
            route_table_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routeTableIds"),
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
            throughput_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("throughputCapacity"),
            ),
            throughput_capacity_per_ha_pair: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("throughputCapacityPerHaPair"),
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
