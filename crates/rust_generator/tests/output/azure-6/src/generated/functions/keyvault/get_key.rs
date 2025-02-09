#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyArgs {
        /// Specifies the ID of the Key Vault instance where the Secret resides, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        ///
        /// **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Key.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKeyResult {
        /// The EC Curve name of this Key Vault Key.
        pub curve: pulumi_gestalt_rust::Output<String>,
        /// The RSA public exponent of this Key Vault Key.
        pub e: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of JSON web key operations assigned to this Key Vault Key
        pub key_opts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the Size of this Key Vault Key.
        pub key_size: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the Key Type of this Key Vault Key
        pub key_type: pulumi_gestalt_rust::Output<String>,
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The RSA modulus of this Key Vault Key.
        pub n: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The OpenSSH encoded public key of this Key Vault Key.
        pub public_key_openssh: pulumi_gestalt_rust::Output<String>,
        /// The PEM encoded public key of this Key Vault Key.
        pub public_key_pem: pulumi_gestalt_rust::Output<String>,
        /// The (Versioned) ID for this Key Vault Key. This property points to a specific version of a Key Vault Key, as such using this won't auto-rotate values if used in other Azure Services.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// The Versionless ID of the Key Vault Key. This property allows other Azure Services (that support it) to auto-rotate their value when the Key Vault Key is updated.
        pub resource_versionless_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to this Key Vault Key.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The current version of the Key Vault Key.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The Base ID of the Key Vault Key.
        pub versionless_id: pulumi_gestalt_rust::Output<String>,
        /// The EC X component of this Key Vault Key.
        pub x: pulumi_gestalt_rust::Output<String>,
        /// The EC Y component of this Key Vault Key.
        pub y: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetKeyArgs,
    ) -> GetKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding_1 = args.key_vault_id.get_output(context);
        let key_vault_id_binding = key_vault_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetKeyResult {
            curve: pulumi_gestalt_rust::__private::into_domain(o.extract_field("curve")),
            e: pulumi_gestalt_rust::__private::into_domain(o.extract_field("e")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_opts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyOpts"),
            ),
            key_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keySize"),
            ),
            key_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyType"),
            ),
            key_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultId"),
            ),
            n: pulumi_gestalt_rust::__private::into_domain(o.extract_field("n")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            public_key_openssh: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicKeyOpenssh"),
            ),
            public_key_pem: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicKeyPem"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            resource_versionless_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceVersionlessId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            versionless_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionlessId"),
            ),
            x: pulumi_gestalt_rust::__private::into_domain(o.extract_field("x")),
            y: pulumi_gestalt_rust::__private::into_domain(o.extract_field("y")),
        }
    }
}
