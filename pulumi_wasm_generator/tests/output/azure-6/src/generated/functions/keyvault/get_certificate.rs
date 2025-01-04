pub mod get_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// Specifies the ID of the Key Vault instance where the Secret resides, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Key Vault Certificate.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the certificate to look up.  (Defaults to latest)
        ///
        /// **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// The raw Key Vault Certificate data represented as a hexadecimal string.
        pub certificate_data: pulumi_wasm_rust::Output<String>,
        /// The raw Key Vault Certificate data represented as a base64 string.
        pub certificate_data_base64: pulumi_wasm_rust::Output<String>,
        /// A `certificate_policy` block as defined below.
        pub certificate_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::keyvault::GetCertificateCertificatePolicy>,
        >,
        /// Expiry date of certificate in RFC3339 format.
        pub expires: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Certificate Issuer.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Not Before date of certificate in RFC3339 format.
        pub not_before: pulumi_wasm_rust::Output<String>,
        /// The (Versioned) ID for this Key Vault Certificate. This property points to a specific version of a Key Vault Certificate, as such using this won't auto-rotate values if used in other Azure Services.
        pub resource_manager_id: pulumi_wasm_rust::Output<String>,
        /// The Versionless ID of the Key Vault Certificate. This property allows other Azure Services (that support it) to auto-rotate their value when the Key Vault Certificate is updated.
        pub resource_manager_versionless_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the associated Key Vault Secret.
        pub secret_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The X509 Thumbprint of the Key Vault Certificate represented as a hexadecimal string.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
        /// The current version of the Key Vault Certificate.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The Base ID of the Key Vault Certificate.
        pub versionless_id: pulumi_wasm_rust::Output<String>,
        /// The Base ID of the Key Vault Secret.
        pub versionless_secret_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCertificateArgs) -> GetCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_inner();
        let name_binding = args.name.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getCertificate:getCertificate".into(),
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
                    name: "certificateData".into(),
                },
                register_interface::ResultField {
                    name: "certificateDataBase64".into(),
                },
                register_interface::ResultField {
                    name: "certificatePolicies".into(),
                },
                register_interface::ResultField {
                    name: "expires".into(),
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
                    name: "notBefore".into(),
                },
                register_interface::ResultField {
                    name: "resourceManagerId".into(),
                },
                register_interface::ResultField {
                    name: "resourceManagerVersionlessId".into(),
                },
                register_interface::ResultField {
                    name: "secretId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "thumbprint".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionlessId".into(),
                },
                register_interface::ResultField {
                    name: "versionlessSecretId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCertificateResult {
            certificate_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateData").unwrap(),
            ),
            certificate_data_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateDataBase64").unwrap(),
            ),
            certificate_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificatePolicies").unwrap(),
            ),
            expires: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expires").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            not_before: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notBefore").unwrap(),
            ),
            resource_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceManagerId").unwrap(),
            ),
            resource_manager_versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceManagerVersionlessId").unwrap(),
            ),
            secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprint").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionlessId").unwrap(),
            ),
            versionless_secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionlessSecretId").unwrap(),
            ),
        }
    }
}
