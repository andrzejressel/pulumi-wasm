#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// Specifies the ID of the Key Vault instance where the Secret resides, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Certificate.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the version of the certificate to look up.  (Defaults to latest)
        ///
        /// **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// The raw Key Vault Certificate data represented as a hexadecimal string.
        pub certificate_data: pulumi_gestalt_rust::Output<String>,
        /// The raw Key Vault Certificate data represented as a base64 string.
        pub certificate_data_base64: pulumi_gestalt_rust::Output<String>,
        /// A `certificate_policy` block as defined below.
        pub certificate_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::keyvault::GetCertificateCertificatePolicy>,
        >,
        /// Expiry date of certificate in RFC3339 format.
        pub expires: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Certificate Issuer.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Not Before date of certificate in RFC3339 format.
        pub not_before: pulumi_gestalt_rust::Output<String>,
        /// The (Versioned) ID for this Key Vault Certificate. This property points to a specific version of a Key Vault Certificate, as such using this won't auto-rotate values if used in other Azure Services.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        /// The Versionless ID of the Key Vault Certificate. This property allows other Azure Services (that support it) to auto-rotate their value when the Key Vault Certificate is updated.
        pub resource_manager_versionless_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the associated Key Vault Secret.
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The X509 Thumbprint of the Key Vault Certificate represented as a hexadecimal string.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
        /// The current version of the Key Vault Certificate.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The Base ID of the Key Vault Certificate.
        pub versionless_id: pulumi_gestalt_rust::Output<String>,
        /// The Base ID of the Key Vault Secret.
        pub versionless_secret_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getCertificate:getCertificate".into(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificateResult {
            certificate_data: o.get_field("certificateData"),
            certificate_data_base64: o.get_field("certificateDataBase64"),
            certificate_policies: o.get_field("certificatePolicies"),
            expires: o.get_field("expires"),
            id: o.get_field("id"),
            key_vault_id: o.get_field("keyVaultId"),
            name: o.get_field("name"),
            not_before: o.get_field("notBefore"),
            resource_manager_id: o.get_field("resourceManagerId"),
            resource_manager_versionless_id: o.get_field("resourceManagerVersionlessId"),
            secret_id: o.get_field("secretId"),
            tags: o.get_field("tags"),
            thumbprint: o.get_field("thumbprint"),
            version: o.get_field("version"),
            versionless_id: o.get_field("versionlessId"),
            versionless_secret_id: o.get_field("versionlessSecretId"),
        }
    }
}
