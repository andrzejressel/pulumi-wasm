pub mod get_account_alias {
    #[allow(dead_code)]
    pub struct GetAccountAliasResult {
        /// Alias associated with the AWS account.
        pub account_alias: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke() -> GetAccountAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getAccountAlias:getAccountAlias".into(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountAlias".into(),
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
        GetAccountAliasResult {
            account_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountAlias").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
