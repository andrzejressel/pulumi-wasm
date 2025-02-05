pub mod get_certificate_issuer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateIssuerArgs {
        /// The ID of the Key Vault in which to locate the Certificate Issuer.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Key Vault Certificate Issuer.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateIssuerResult {
        /// The account number with the third-party Certificate Issuer.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// A list of `admin` blocks as defined below.
        pub admins: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::keyvault::GetCertificateIssuerAdmin>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The organization ID with the third-party Certificate Issuer.
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// The name of the third-party Certificate Issuer.
        pub provider_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCertificateIssuerArgs,
    ) -> GetCertificateIssuerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getCertificateIssuer:getCertificateIssuer".into(),
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
        GetCertificateIssuerResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            admins: pulumi_wasm_rust::__private::into_domain(o.extract_field("admins")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            org_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("orgId")),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("providerName"),
            ),
        }
    }
}
