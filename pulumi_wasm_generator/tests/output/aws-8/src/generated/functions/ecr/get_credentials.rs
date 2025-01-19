pub mod get_credentials {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCredentialsArgs {
        #[builder(into)]
        pub registry_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetCredentialsResult {
        pub authorization_token: pulumi_wasm_rust::Output<String>,
        pub expires_at: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub proxy_endpoint: pulumi_wasm_rust::Output<String>,
        pub registry_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCredentialsArgs) -> GetCredentialsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let registry_id_binding = args.registry_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getCredentials:getCredentials".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "registryId".into(),
                    value: &registry_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizationToken".into(),
                },
                register_interface::ResultField {
                    name: "expiresAt".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "proxyEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCredentialsResult {
            authorization_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationToken").unwrap(),
            ),
            expires_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresAt").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            proxy_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyEndpoint").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
        }
    }
}
