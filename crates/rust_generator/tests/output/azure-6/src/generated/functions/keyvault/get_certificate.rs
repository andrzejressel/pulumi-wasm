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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getCertificate:getCertificate".into(),
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
        GetCertificateResult {
            certificate_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateData"),
            ),
            certificate_data_base64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateDataBase64"),
            ),
            certificate_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificatePolicies"),
            ),
            expires: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expires"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            not_before: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notBefore"),
            ),
            resource_manager_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceManagerId"),
            ),
            resource_manager_versionless_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceManagerVersionlessId"),
            ),
            secret_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            versionless_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionlessId"),
            ),
            versionless_secret_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionlessSecretId"),
            ),
        }
    }
}
