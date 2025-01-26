pub mod list_storage_account_keys {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListStorageAccountKeysArgs {
        /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies type of the key to be listed. Possible value is kerb.
        #[builder(into, default)]
        pub expand: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group within the user's subscription. The name is case insensitive.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: ListStorageAccountKeysArgs,
    ) -> ListStorageAccountKeysResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let expand_binding = args.expand.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        ListStorageAccountKeysResult {
            keys: pulumi_wasm_rust::__private::into_domain(o.extract_field("keys")),
        }
    }
}
