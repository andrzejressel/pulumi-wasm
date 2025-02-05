pub mod get_cloud_vm_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCloudVmClusterArgs {
        /// The name of this Cloud VM Cluster.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Cloud VM Cluster exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCloudVmClusterResult {
        /// Client OCI backup subnet CIDR, default is `192.168.252.0/22`.
        pub backup_subnet_cidr: pulumi_wasm_rust::Output<String>,
        /// The Cloud Exadata Infrastructure ID.
        pub cloud_exadata_infrastructure_id: pulumi_wasm_rust::Output<String>,
        /// The cluster name for Cloud VM Cluster.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The OCID of the compartment.
        pub compartment_id: pulumi_wasm_rust::Output<String>,
        /// A `compute_nodes` block as defined below.
        pub compute_nodes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The number of CPU cores enabled on the Cloud VM Cluster.
        pub cpu_core_count: pulumi_wasm_rust::Output<i32>,
        /// A `data_collection_options` block as defined below.
        pub data_collection_options: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::oracle::GetCloudVmClusterDataCollectionOption,
            >,
        >,
        /// The percentage assigned to DATA storage (user data and database files). The remaining percentage is assigned to RECO storage (database redo logs, archive logs, and recovery manager backups). Accepted values are `35`, `40`, `60`, and 80. The default is `80` percent assigned to DATA storage. See [Storage Configuration](https://docs.oracle.com/en-us/iaas/exadatacloud/index.html#Exadata) in the Exadata documentation for details on the impact of the configuration settings on storage.
        pub data_storage_percentage: pulumi_wasm_rust::Output<i32>,
        /// The data disk group size to be allocated in TBs.
        pub data_storage_size_in_tbs: pulumi_wasm_rust::Output<f64>,
        /// The local node storage to be allocated in GBs.
        pub db_node_storage_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// A `db_servers` block as defined below.
        pub db_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The type of redundancy configured for the Cloud Vm Cluster. `NORMAL` is 2-way redundancy. `HIGH` is 3-way redundancy.
        pub disk_redundancy: pulumi_wasm_rust::Output<String>,
        /// The user-friendly name for the Cloud VM Cluster. The name does not need to be unique.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The domain name for the Cloud VM Cluster.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// A valid Oracle Grid Infrastructure (GI) software version.
        pub gi_version: pulumi_wasm_rust::Output<String>,
        /// The hostname for the Cloud VM Cluster without suffix.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The hostname for the Cloud VM Cluster with suffix.
        pub hostname_actual: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `iorm_config_cache` block as defined below.
        pub iorm_config_caches: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::oracle::GetCloudVmClusterIormConfigCach>,
        >,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the last maintenance update history entry. This value is updated when a maintenance update starts.
        pub last_update_history_entry_id: pulumi_wasm_rust::Output<String>,
        /// The Oracle license model that applies to the Cloud VM Cluster.
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// Additional information about the current `lifecycleState`.
        pub lifecycle_details: pulumi_wasm_rust::Output<String>,
        /// The current state of IORM configuration for the Exadata DB system.
        pub lifecycle_state: pulumi_wasm_rust::Output<String>,
        /// The port number configured for the listener on the Cloud VM Cluster.
        pub listener_port: pulumi_wasm_rust::Output<i32>,
        /// If true, database backup on local Exadata storage is configured for the Cloud VM Cluster. If false, database backup on local Exadata storage is not available in the Cloud VM Cluster.
        pub local_backup_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure Region where the Cloud VM Cluster exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The memory to be allocated in GBs.
        pub memory_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of nodes in the Cloud VM Cluster.
        pub node_count: pulumi_wasm_rust::Output<i32>,
        /// The list of [OCIDs](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) for the network security groups (NSGs) to which this resource belongs. Setting this to an empty list removes all resources from all NSGs. For more information about NSGs, see [Security Rules](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/securityrules.htm). NsgIds restrictions:
        /// * A network security group (NSG) is optional for Autonomous Databases with private access. The nsgIds list can be empty.
        pub nsg_url: pulumi_wasm_rust::Output<String>,
        /// The URL of the resource in the OCI console.
        pub oci_url: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Cloud VM Cluster.
        pub ocid: pulumi_wasm_rust::Output<String>,
        /// The number of OCPU cores to enable on the Cloud VM Cluster. Only 1 decimal place is allowed for the fractional part.
        pub ocpu_count: pulumi_wasm_rust::Output<f64>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The FQDN of the DNS record for the SCAN IP addresses that are associated with the Cloud VM Cluster.
        pub scan_dns_name: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the DNS record for the SCAN IP addresses that are associated with the Cloud VM Cluster.
        pub scan_dns_record_id: pulumi_wasm_rust::Output<String>,
        /// A `scan_ip_ids` block as defined below.
        pub scan_ip_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The TCP Single Client Access Name (SCAN) port. The default port is 1521.
        pub scan_listener_port_tcp: pulumi_wasm_rust::Output<i32>,
        /// The TCPS Single Client Access Name (SCAN) port. The default port is 2484.
        pub scan_listener_port_tcp_ssl: pulumi_wasm_rust::Output<i32>,
        /// The model name of the Exadata hardware running the Cloud VM Cluster.
        pub shape: pulumi_wasm_rust::Output<String>,
        /// If true, sparse disk group is configured for the Cloud VM Cluster. If false, sparse disk group is not created.
        pub sparse_diskgroup_enabled: pulumi_wasm_rust::Output<bool>,
        /// The public key portion of one or more key pairs used for SSH access to the Cloud VM Cluster.
        pub ssh_public_keys: pulumi_wasm_rust::Output<Vec<String>>,
        /// The storage allocation for the disk group, in gigabytes (GB).
        pub storage_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The ID of the Azure Resource Manager subnet resource.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the subnet associated with the Cloud VM Cluster.
        pub subnet_ocid: pulumi_wasm_rust::Output<String>,
        /// Operating system version of the image.
        pub system_version: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Cloud VM Cluster.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The date and time that the Cloud VM Cluster was created.
        pub time_created: pulumi_wasm_rust::Output<String>,
        /// The time zone of the Cloud VM Cluster. For details, see [Exadata Infrastructure Time Zones](https://docs.oracle.com/en-us/iaas/base-database/doc/manage-time-zone.html).
        pub time_zone: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the virtual IP (VIP) addresses associated with the Cloud VM Cluster. The Cluster Ready Services (CRS) creates and maintains one VIP address for each node in the Exadata Cloud Service instance to enable failover. If one node fails, the VIP is reassigned to another active node in the Cluster.
        pub vip_ods: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID to an Azure Resource Manager Virtual Network resource.
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the zone the Cloud VM Cluster is associated with.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCloudVmClusterArgs,
    ) -> GetCloudVmClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:oracle/getCloudVmCluster:getCloudVmCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCloudVmClusterResult {
            backup_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupSubnetCidr"),
            ),
            cloud_exadata_infrastructure_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudExadataInfrastructureId"),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            compartment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compartmentId"),
            ),
            compute_nodes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("computeNodes"),
            ),
            cpu_core_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cpuCoreCount"),
            ),
            data_collection_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataCollectionOptions"),
            ),
            data_storage_percentage: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataStoragePercentage"),
            ),
            data_storage_size_in_tbs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataStorageSizeInTbs"),
            ),
            db_node_storage_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dbNodeStorageSizeInGbs"),
            ),
            db_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dbServers"),
            ),
            disk_redundancy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("diskRedundancy"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            gi_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("giVersion"),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            hostname_actual: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostnameActual"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            iorm_config_caches: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iormConfigCaches"),
            ),
            last_update_history_entry_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdateHistoryEntryId"),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("licenseModel"),
            ),
            lifecycle_details: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecycleDetails"),
            ),
            lifecycle_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecycleState"),
            ),
            listener_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("listenerPort"),
            ),
            local_backup_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localBackupEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            memory_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("memorySizeInGbs"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            node_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeCount"),
            ),
            nsg_url: pulumi_wasm_rust::__private::into_domain(o.extract_field("nsgUrl")),
            oci_url: pulumi_wasm_rust::__private::into_domain(o.extract_field("ociUrl")),
            ocid: pulumi_wasm_rust::__private::into_domain(o.extract_field("ocid")),
            ocpu_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ocpuCount"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scan_dns_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scanDnsName"),
            ),
            scan_dns_record_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scanDnsRecordId"),
            ),
            scan_ip_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scanIpIds"),
            ),
            scan_listener_port_tcp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scanListenerPortTcp"),
            ),
            scan_listener_port_tcp_ssl: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scanListenerPortTcpSsl"),
            ),
            shape: pulumi_wasm_rust::__private::into_domain(o.extract_field("shape")),
            sparse_diskgroup_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sparseDiskgroupEnabled"),
            ),
            ssh_public_keys: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sshPublicKeys"),
            ),
            storage_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageSizeInGbs"),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            subnet_ocid: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetOcid"),
            ),
            system_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("systemVersion"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            time_created: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeCreated"),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeZone"),
            ),
            vip_ods: pulumi_wasm_rust::__private::into_domain(o.extract_field("vipOds")),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkId"),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
