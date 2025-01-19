pub mod get_client_config {
    #[allow(dead_code)]
    pub struct GetClientConfigResult {
        /// Azure Client ID (Application Object ID).
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// Azure Object ID of the current user or service principal.
        pub object_id: pulumi_wasm_rust::Output<String>,
        /// Azure Subscription ID
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// Azure Tenant ID
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke() -> GetClientConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::getClientConfig".into(),
            version: super::super::get_version(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clientId".into(),
                },
                register_interface::ResultField {
                    name: "objectId".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionId".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClientConfigResult {
            client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientId").unwrap(),
            ),
            object_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectId").unwrap(),
            ),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionId").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
        }
    }
}
