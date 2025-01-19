pub mod list_storage_account_keys {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListStorageAccountKeysArgs {
        /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Specifies type of the key to be listed. Possible value is kerb.
        #[builder(into, default)]
        pub expand: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group within the user's subscription. The name is case insensitive.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ListStorageAccountKeysResult {
        /// Gets the list of storage account keys and their properties for the specified storage account.
        pub keys: pulumi_wasm_rust::Output<
            Vec<super::super::types::StorageAccountKeyResponse>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: ListStorageAccountKeysArgs) -> ListStorageAccountKeysResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let expand_binding = args.expand.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::listStorageAccountKeys".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "expand".into(),
                    value: &expand_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "keys".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ListStorageAccountKeysResult {
            keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keys").unwrap(),
            ),
        }
    }
}
