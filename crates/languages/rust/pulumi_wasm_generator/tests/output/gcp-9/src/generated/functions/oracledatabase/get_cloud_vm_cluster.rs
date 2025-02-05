pub mod get_cloud_vm_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCloudVmClusterArgs {
        /// The ID of the VM Cluster.
        #[builder(into)]
        pub cloud_vm_cluster_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCloudVmClusterArgs,
    ) -> GetCloudVmClusterResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_vm_cluster_id_binding = args
            .cloud_vm_cluster_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:oracledatabase/getCloudVmCluster:getCloudVmCluster".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCloudVmClusterResult {
            backup_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupSubnetCidr"),
            ),
            cidr: pulumi_wasm_rust::__private::into_domain(o.extract_field("cidr")),
            cloud_vm_cluster_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudVmClusterId"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            exadata_infrastructure: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exadataInfrastructure"),
            ),
            gcp_oracle_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gcpOracleZone"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("properties"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
        }
    }
}
