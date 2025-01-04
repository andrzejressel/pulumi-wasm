pub mod get_flexible_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFlexibleServerArgs {
        /// Specifies the name of the MySQL Flexible Server.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group for the MySQL Flexible Server.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFlexibleServerResult {
        /// The Administrator login of the MySQL Flexible Server.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// The backup retention days of the MySQL Flexible Server.
        pub backup_retention_days: pulumi_wasm_rust::Output<i32>,
        /// The ID of the virtual network subnet the MySQL Flexible Server is created in.
        pub delegated_subnet_id: pulumi_wasm_rust::Output<String>,
        /// The fully qualified domain name of the MySQL Flexible Server.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// Is geo redundant backup enabled?
        pub geo_redundant_backup_enabled: pulumi_wasm_rust::Output<bool>,
        /// A `high_availability` block for this MySQL Flexible Server as defined below.
        pub high_availabilities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mysql::GetFlexibleServerHighAvailability>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region of the MySQL Flexible Server.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `maintenance_window` block for this MySQL Flexible Server as defined below.
        pub maintenance_windows: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mysql::GetFlexibleServerMaintenanceWindow>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Private DNS zone of the MySQL Flexible Server.
        pub private_dns_zone_id: pulumi_wasm_rust::Output<String>,
        /// Is the public network access enabled?
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// The maximum number of replicas that a primary MySQL Flexible Server can have.
        pub replica_capacity: pulumi_wasm_rust::Output<i32>,
        /// The replication role of the MySQL Flexible Server.
        pub replication_role: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub restore_point_in_time: pulumi_wasm_rust::Output<String>,
        /// The SKU Name of the MySQL Flexible Server.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A `storage` block for this MySQL Flexible Server as defined below.
        pub storages: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mysql::GetFlexibleServerStorage>,
        >,
        /// A mapping of tags which are assigned to the MySQL Flexible Server.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The version of the MySQL Flexible Server.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The Availability Zones where this MySQL Flexible Server is located.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFlexibleServerArgs) -> GetFlexibleServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mysql/getFlexibleServer:getFlexibleServer".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorLogin".into(),
                },
                register_interface::ResultField {
                    name: "backupRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "delegatedSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "geoRedundantBackupEnabled".into(),
                },
                register_interface::ResultField {
                    name: "highAvailabilities".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindows".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsZoneId".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "replicaCapacity".into(),
                },
                register_interface::ResultField {
                    name: "replicationRole".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "restorePointInTime".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storages".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFlexibleServerResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLogin").unwrap(),
            ),
            backup_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRetentionDays").unwrap(),
            ),
            delegated_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegatedSubnetId").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            geo_redundant_backup_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("geoRedundantBackupEnabled").unwrap(),
            ),
            high_availabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("highAvailabilities").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_windows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindows").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_dns_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsZoneId").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            replica_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaCapacity").unwrap(),
            ),
            replication_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationRole").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            restore_point_in_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restorePointInTime").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storages").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
