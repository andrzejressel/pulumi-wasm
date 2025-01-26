pub mod get_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyArgs {
        /// Specifies the ID of the Key Vault instance where the Secret resides, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        ///
        /// **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Key.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKeyResult {
        /// The EC Curve name of this Key Vault Key.
        pub curve: pulumi_wasm_rust::Output<String>,
        /// The RSA public exponent of this Key Vault Key.
        pub e: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of JSON web key operations assigned to this Key Vault Key
        pub key_opts: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the Size of this Key Vault Key.
        pub key_size: pulumi_wasm_rust::Output<i32>,
        /// Specifies the Key Type of this Key Vault Key
        pub key_type: pulumi_wasm_rust::Output<String>,
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The RSA modulus of this Key Vault Key.
        pub n: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The OpenSSH encoded public key of this Key Vault Key.
        pub public_key_openssh: pulumi_wasm_rust::Output<String>,
        /// The PEM encoded public key of this Key Vault Key.
        pub public_key_pem: pulumi_wasm_rust::Output<String>,
        /// The (Versioned) ID for this Key Vault Key. This property points to a specific version of a Key Vault Key, as such using this won't auto-rotate values if used in other Azure Services.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The Versionless ID of the Key Vault Key. This property allows other Azure Services (that support it) to auto-rotate their value when the Key Vault Key is updated.
        pub resource_versionless_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to this Key Vault Key.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The current version of the Key Vault Key.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The Base ID of the Key Vault Key.
        pub versionless_id: pulumi_wasm_rust::Output<String>,
        /// The EC X component of this Key Vault Key.
        pub x: pulumi_wasm_rust::Output<String>,
        /// The EC Y component of this Key Vault Key.
        pub y: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetKeyArgs,
    ) -> GetKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getKey:getKey".into(),
            version: super::super::super::get_version(),
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
                    name: "curve".into(),
                },
                register_interface::ResultField {
                    name: "e".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyOpts".into(),
                },
                register_interface::ResultField {
                    name: "keySize".into(),
                },
                register_interface::ResultField {
                    name: "keyType".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "n".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicKeyOpenssh".into(),
                },
                register_interface::ResultField {
                    name: "publicKeyPem".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "resourceVersionlessId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionlessId".into(),
                },
                register_interface::ResultField {
                    name: "x".into(),
                },
                register_interface::ResultField {
                    name: "y".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKeyResult {
            curve: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("curve").unwrap(),
            ),
            e: pulumi_wasm_rust::__private::into_domain(hashmap.remove("e").unwrap()),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_opts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyOpts").unwrap(),
            ),
            key_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keySize").unwrap(),
            ),
            key_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyType").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            n: pulumi_wasm_rust::__private::into_domain(hashmap.remove("n").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_key_openssh: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKeyOpenssh").unwrap(),
            ),
            public_key_pem: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKeyPem").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            resource_versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceVersionlessId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionlessId").unwrap(),
            ),
            x: pulumi_wasm_rust::__private::into_domain(hashmap.remove("x").unwrap()),
            y: pulumi_wasm_rust::__private::into_domain(hashmap.remove("y").unwrap()),
        }
    }
}
