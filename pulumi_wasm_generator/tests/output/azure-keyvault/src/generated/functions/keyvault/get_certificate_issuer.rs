pub mod get_certificate_issuer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateIssuerArgs {
        /// The ID of the Key Vault in which to locate the Certificate Issuer.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Key Vault Certificate Issuer.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateIssuerResult {
        /// The account number with the third-party Certificate Issuer.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// A list of `admin` blocks as defined below.
        pub admins: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::keyvault::GetCertificateIssuerAdmin>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The organization ID with the third-party Certificate Issuer.
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// The name of the third-party Certificate Issuer.
        pub provider_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCertificateIssuerArgs) -> GetCertificateIssuerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getCertificateIssuer:getCertificateIssuer".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "admins".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "providerName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCertificateIssuerResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            admins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("admins").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerName").unwrap(),
            ),
        }
    }
}
