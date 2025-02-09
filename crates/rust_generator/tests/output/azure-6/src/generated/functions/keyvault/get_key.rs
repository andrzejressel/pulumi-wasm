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
        context: &pulumi_gestalt_rust::Context,
        args: GetKeyArgs,
    ) -> GetKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getKey:getKey".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: key_vault_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKeyResult {
            curve: o.get_field("curve"),
            e: o.get_field("e"),
            id: o.get_field("id"),
            key_opts: o.get_field("keyOpts"),
            key_size: o.get_field("keySize"),
            key_type: o.get_field("keyType"),
            key_vault_id: o.get_field("keyVaultId"),
            n: o.get_field("n"),
            name: o.get_field("name"),
            public_key_openssh: o.get_field("publicKeyOpenssh"),
            public_key_pem: o.get_field("publicKeyPem"),
            resource_id: o.get_field("resourceId"),
            resource_versionless_id: o.get_field("resourceVersionlessId"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
            versionless_id: o.get_field("versionlessId"),
            x: o.get_field("x"),
            y: o.get_field("y"),
        }
    }
}
