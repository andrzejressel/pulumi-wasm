pub mod get_db_nodes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDbNodesArgs {
        /// The id of the Cloud VM cluster.
        #[builder(into)]
        pub cloud_vm_cluster_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDbNodesResult {
        pub cloud_vm_cluster_id: pulumi_wasm_rust::Output<String>,
        /// A `db_nodes` block as defined below.
        pub db_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::oracle::GetDbNodesDbNode>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDbNodesArgs) -> GetDbNodesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_vm_cluster_id_binding = args.cloud_vm_cluster_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:oracle/getDbNodes:getDbNodes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudVmClusterId".into(),
                    value: &cloud_vm_cluster_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cloudVmClusterId".into(),
                },
                register_interface::ResultField {
                    name: "dbNodes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDbNodesResult {
            cloud_vm_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudVmClusterId").unwrap(),
            ),
            db_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbNodes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
