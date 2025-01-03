pub mod get_bundle {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBundleArgs {
        /// ID of the bundle.
        #[builder(into, default)]
        pub bundle_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the bundle. You cannot combine this parameter with `bundle_id`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Owner of the bundles. You have to leave it blank for own bundles. You cannot combine this parameter with `bundle_id`.
        #[builder(into, default)]
        pub owner: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBundleResult {
        /// The ID of the bundle.
        pub bundle_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The compute type. See supported fields below.
        pub compute_types: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleComputeType>,
        >,
        /// The description of the bundle.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the compute type.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The owner of the bundle.
        pub owner: pulumi_wasm_rust::Output<Option<String>>,
        /// The root volume. See supported fields below.
        pub root_storages: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleRootStorage>,
        >,
        /// The user storage. See supported fields below.
        pub user_storages: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleUserStorage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBundleArgs) -> GetBundleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bundle_id_binding = args.bundle_id.get_inner();
        let name_binding = args.name.get_inner();
        let owner_binding = args.owner.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:workspaces/getBundle:getBundle".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "computeTypes".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "rootStorages".into(),
                },
                register_interface::ResultField {
                    name: "userStorages".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBundleResult {
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            compute_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeTypes").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            root_storages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootStorages").unwrap(),
            ),
            user_storages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userStorages").unwrap(),
            ),
        }
    }
}
