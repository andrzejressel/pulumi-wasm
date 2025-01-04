pub mod get_secrets {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretsArgs {
        /// Specifies the ID of the Key Vault instance to fetch secret names from, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        ///
        /// **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSecretsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// List containing names of secrets that exist in this Key Vault.
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more `secrets` blocks as defined below.
        pub secrets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::keyvault::GetSecretsSecret>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSecretsArgs) -> GetSecretsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getSecrets:getSecrets".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "names".into(),
                },
                register_interface::ResultField {
                    name: "secrets".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSecretsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("names").unwrap(),
            ),
            secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secrets").unwrap(),
            ),
        }
    }
}
