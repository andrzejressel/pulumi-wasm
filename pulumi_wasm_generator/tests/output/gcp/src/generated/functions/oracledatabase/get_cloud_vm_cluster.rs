pub mod get_cloud_vm_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCloudVmClusterArgs {
        /// The ID of the VM Cluster.
        #[builder(into)]
        pub cloud_vm_cluster_id: pulumi_wasm_rust::Output<String>,
        /// The location of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCloudVmClusterResult {
        pub backup_subnet_cidr: pulumi_wasm_rust::Output<String>,
        pub cidr: pulumi_wasm_rust::Output<String>,
        pub cloud_vm_cluster_id: pulumi_wasm_rust::Output<String>,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub exadata_infrastructure: pulumi_wasm_rust::Output<String>,
        pub gcp_oracle_zone: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub properties: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::oracledatabase::GetCloudVmClusterProperty>,
        >,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCloudVmClusterArgs) -> GetCloudVmClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_vm_cluster_id_binding = args.cloud_vm_cluster_id.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:oracledatabase/getCloudVmCluster:getCloudVmCluster".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudVmClusterId".into(),
                    value: &cloud_vm_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupSubnetCidr".into(),
                },
                register_interface::ResultField {
                    name: "cidr".into(),
                },
                register_interface::ResultField {
                    name: "cloudVmClusterId".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "exadataInfrastructure".into(),
                },
                register_interface::ResultField {
                    name: "gcpOracleZone".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCloudVmClusterResult {
            backup_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupSubnetCidr").unwrap(),
            ),
            cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidr").unwrap(),
            ),
            cloud_vm_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudVmClusterId").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            exadata_infrastructure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exadataInfrastructure").unwrap(),
            ),
            gcp_oracle_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gcpOracleZone").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
        }
    }
}
