pub mod get_access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessPolicyArgs {
        /// Specifies the name of the Management Template. Possible values are: `Key Management`,
        /// `Secret Management`, `Certificate Management`, `Key & Secret Management`, `Key & Certificate Management`,
        /// `Secret & Certificate Management`,  `Key, Secret, & Certificate Management`
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccessPolicyResult {
        /// the certificate permissions for the access policy
        pub certificate_permissions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// the key permissions for the access policy
        pub key_permissions: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// the secret permissions for the access policy
        pub secret_permissions: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccessPolicyArgs) -> GetAccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getAccessPolicy:getAccessPolicy".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificatePermissions".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyPermissions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "secretPermissions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccessPolicyResult {
            certificate_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificatePermissions").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyPermissions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            secret_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretPermissions").unwrap(),
            ),
        }
    }
}
