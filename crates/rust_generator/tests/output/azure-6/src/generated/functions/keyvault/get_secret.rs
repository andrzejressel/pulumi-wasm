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
        context: &pulumi_gestalt_rust::Context,
        args: GetSecretArgs,
    ) -> GetSecretResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getSecret:getSecret".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecretResult {
            content_type: o.get_field("contentType"),
            expiration_date: o.get_field("expirationDate"),
            id: o.get_field("id"),
            key_vault_id: o.get_field("keyVaultId"),
            name: o.get_field("name"),
            not_before_date: o.get_field("notBeforeDate"),
            resource_id: o.get_field("resourceId"),
            resource_versionless_id: o.get_field("resourceVersionlessId"),
            tags: o.get_field("tags"),
            value: o.get_field("value"),
            version: o.get_field("version"),
            versionless_id: o.get_field("versionlessId"),
        }
    }
}
