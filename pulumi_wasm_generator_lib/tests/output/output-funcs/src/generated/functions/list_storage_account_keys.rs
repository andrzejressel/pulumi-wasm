#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
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
pub struct ListStorageAccountKeysResult {
    /// Gets the list of storage account keys and their properties for the specified storage account.
    pub keys: pulumi_wasm_rust::Output<
        Vec<super::super::types::StorageAccountKeyResponse>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: ListStorageAccountKeysArgs) -> ListStorageAccountKeysResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_name_binding = args.account_name.get_inner();
    let expand_binding = args.expand.get_inner();
    let resource_group_name_binding = args.resource_group_name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "mypkg::listStorageAccountKeys".into(),
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
        results: vec![register_interface::ResultField { name : "keys".into() },],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    ListStorageAccountKeysResult {
        keys: into_domain(hashmap.remove("keys").unwrap()),
    }
}
