#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificate_issuer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateIssuerArgs {
        /// The ID of the Key Vault in which to locate the Certificate Issuer.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Key Vault Certificate Issuer.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateIssuerResult {
        /// The account number with the third-party Certificate Issuer.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A list of `admin` blocks as defined below.
        pub admins: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::keyvault::GetCertificateIssuerAdmin>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The organization ID with the third-party Certificate Issuer.
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the third-party Certificate Issuer.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificateIssuerArgs,
    ) -> GetCertificateIssuerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getCertificateIssuer:getCertificateIssuer".into(),
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
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificateIssuerResult {
            account_id: o.get_field("accountId"),
            admins: o.get_field("admins"),
            id: o.get_field("id"),
            key_vault_id: o.get_field("keyVaultId"),
            name: o.get_field("name"),
            org_id: o.get_field("orgId"),
            provider_name: o.get_field("providerName"),
        }
    }
}
