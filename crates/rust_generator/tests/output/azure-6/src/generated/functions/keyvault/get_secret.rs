#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretArgs {
        /// Specifies the ID of the Key Vault instance to fetch secret names from, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Secret.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the version of the Key Vault Secret. Defaults to the current version of the Key Vault Secret.
        ///
        /// **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecretResult {
        /// The content type for the Key Vault Secret.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// The date and time at which the Key Vault Secret expires and is no longer valid.
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The earliest date at which the Key Vault Secret can be used.
        pub not_before_date: pulumi_gestalt_rust::Output<String>,
        /// The (Versioned) ID for this Key Vault Secret. This property points to a specific version of a Key Vault Secret, as such using this won't auto-rotate values if used in other Azure Services.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// The Versionless ID of the Key Vault Secret. This property allows other Azure Services (that support it) to auto-rotate their value when the Key Vault Secret is updated.
        pub resource_versionless_id: pulumi_gestalt_rust::Output<String>,
        /// Any tags assigned to this resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The value of the Key Vault Secret.
        pub value: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Versionless ID of the Key Vault Secret. This can be used to always get latest secret value, and enable fetching automatically rotating secrets.
        pub versionless_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSecretArgs,
    ) -> GetSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getSecret:getSecret".into(),
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
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSecretResult {
            content_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            expiration_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expirationDate"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            not_before_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notBeforeDate"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            resource_versionless_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceVersionlessId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            versionless_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionlessId"),
            ),
        }
    }
}
