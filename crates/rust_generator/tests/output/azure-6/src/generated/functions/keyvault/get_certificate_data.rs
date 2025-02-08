#[allow(clippy::doc_lazy_continuation)]
pub mod get_certificate_data {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateDataArgs {
        /// Specifies the ID of the Key Vault instance where the Secret resides, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Secret.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the version of the certificate to look up.  (Defaults to latest)
        ///
        /// > **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateDataResult {
        /// Amount of certificates in the chain in case Key Vault Certificate is a bundle (e.g. has an intermediate certificate).
        pub certificates_count: pulumi_gestalt_rust::Output<i32>,
        /// Expiry date of certificate in RFC3339 format.
        pub expires: pulumi_gestalt_rust::Output<String>,
        /// The raw Key Vault Certificate data represented as a hexadecimal string.
        pub hex: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Key Vault Certificate Key.
        pub key: pulumi_gestalt_rust::Output<String>,
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Not Before date of certificate in RFC3339 format.
        pub not_before: pulumi_gestalt_rust::Output<String>,
        /// The Key Vault Certificate in PEM format.
        pub pem: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCertificateDataArgs,
    ) -> GetCertificateDataResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getCertificateData:getCertificateData".into(),
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
        GetCertificateDataResult {
            certificates_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificatesCount"),
            ),
            expires: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expires"),
            ),
            hex: pulumi_gestalt_rust::__private::into_domain(o.extract_field("hex")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            key_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            not_before: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notBefore"),
            ),
            pem: pulumi_gestalt_rust::__private::into_domain(o.extract_field("pem")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
