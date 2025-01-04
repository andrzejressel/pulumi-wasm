pub mod get_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// The name of the NetApp Account where the NetApp Pool exists.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp Snapshot.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp Pool where the NetApp Volume exists.
        #[builder(into)]
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the NetApp Snapshot exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp Volume where the NetApp Snapshot exists.
        #[builder(into)]
        pub volume_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the NetApp Snapshot exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub pool_name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub volume_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSnapshotArgs) -> GetSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let name_binding = args.name.get_inner();
        let pool_name_binding = args.pool_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let volume_name_binding = args.volume_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:netapp/getSnapshot:getSnapshot".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "poolName".into(),
                    value: &pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "volumeName".into(),
                    value: &volume_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "poolName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "volumeName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSnapshotResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pool_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            volume_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeName").unwrap(),
            ),
        }
    }
}
