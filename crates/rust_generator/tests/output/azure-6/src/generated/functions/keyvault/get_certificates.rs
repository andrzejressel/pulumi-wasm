#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificates {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificatesArgs {
        /// Specifies whether to include certificates which are not completely provisioned. Defaults to true.
        #[builder(into, default)]
        pub include_pending: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ID of the Key Vault instance to fetch certificate names from, available on the `azure.keyvault.KeyVault` Data Source / Resource.
        ///
        /// **NOTE:** The vault must be in the same subscription as the provider. If the vault is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificatesResult {
        /// One or more `certificates` blocks as defined below.
        pub certificates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::keyvault::GetCertificatesCertificate>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_pending: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Key Vault ID.
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// List containing names of certificates that exist in this Key Vault.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificatesArgs,
    ) -> GetCertificatesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let include_pending_binding = args.include_pending.get_output(context);
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getCertificates:getCertificates".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includePending".into(),
                    value: include_pending_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: key_vault_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificatesResult {
            certificates: o.get_field("certificates"),
            id: o.get_field("id"),
            include_pending: o.get_field("includePending"),
            key_vault_id: o.get_field("keyVaultId"),
            names: o.get_field("names"),
        }
    }
}
