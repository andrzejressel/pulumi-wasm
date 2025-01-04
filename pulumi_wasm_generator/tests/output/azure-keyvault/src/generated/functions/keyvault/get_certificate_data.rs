pub mod get_certificate_data {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateDataArgs {
        /// Specifies the ID of the Key Vault instance where the Secret resides, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Key Vault Secret.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the certificate to look up.  (Defaults to latest)
        ///
        /// > **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateDataResult {
        /// Amount of certificates in the chain in case Key Vault Certificate is a bundle (e.g. has an intermediate certificate).
        pub certificates_count: pulumi_wasm_rust::Output<i32>,
        /// Expiry date of certificate in RFC3339 format.
        pub expires: pulumi_wasm_rust::Output<String>,
        /// The raw Key Vault Certificate data represented as a hexadecimal string.
        pub hex: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Key Vault Certificate Key.
        pub key: pulumi_wasm_rust::Output<String>,
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Not Before date of certificate in RFC3339 format.
        pub not_before: pulumi_wasm_rust::Output<String>,
        /// The Key Vault Certificate in PEM format.
        pub pem: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCertificateDataArgs) -> GetCertificateDataResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_inner();
        let name_binding = args.name.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getCertificateData:getCertificateData".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificatesCount".into(),
                },
                register_interface::ResultField {
                    name: "expires".into(),
                },
                register_interface::ResultField {
                    name: "hex".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notBefore".into(),
                },
                register_interface::ResultField {
                    name: "pem".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCertificateDataResult {
            certificates_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificatesCount").unwrap(),
            ),
            expires: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expires").unwrap(),
            ),
            hex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hex").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            not_before: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notBefore").unwrap(),
            ),
            pem: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pem").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
