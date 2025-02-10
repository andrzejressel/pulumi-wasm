/// Manages a Key Vault Certificate Issuer.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: standard
///       tenantId: ${current.tenantId}
///   exampleCertificateIssuer:
///     type: azure:keyvault:CertificateIssuer
///     name: example
///     properties:
///       name: example-issuer
///       orgId: ExampleOrgName
///       keyVaultId: ${exampleKeyVault.id}
///       providerName: DigiCert
///       accountId: '0000'
///       password: example-password
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Key Vault Certificate Issuers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/certificateIssuer:CertificateIssuer example "https://key-vault-name.vault.azure.net/certificates/issuers/example"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate_issuer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateIssuerArgs {
        /// The account number with the third-party Certificate Issuer.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `admin` blocks as defined below.
        #[builder(into, default)]
        pub admins: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::keyvault::CertificateIssuerAdmin>>,
        >,
        /// The ID of the Key Vault in which to create the Certificate Issuer. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Key Vault Certificate Issuer. Changing this forces a new Key Vault Certificate Issuer to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the organization as provided to the issuer.
        #[builder(into, default)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password associated with the account and organization ID at the third-party Certificate Issuer. If not specified, will not overwrite any previous value.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the third-party Certificate Issuer. Possible values are: `DigiCert`, `GlobalSign`, `OneCertV2-PrivateCA`, `OneCertV2-PublicCA` and `SslAdminV2`.
        #[builder(into)]
        pub provider_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateIssuerResult {
        /// The account number with the third-party Certificate Issuer.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `admin` blocks as defined below.
        pub admins: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::keyvault::CertificateIssuerAdmin>>,
        >,
        /// The ID of the Key Vault in which to create the Certificate Issuer. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Key Vault Certificate Issuer. Changing this forces a new Key Vault Certificate Issuer to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the organization as provided to the issuer.
        pub org_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The password associated with the account and organization ID at the third-party Certificate Issuer. If not specified, will not overwrite any previous value.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the third-party Certificate Issuer. Possible values are: `DigiCert`, `GlobalSign`, `OneCertV2-PrivateCA`, `OneCertV2-PublicCA` and `SslAdminV2`.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateIssuerArgs,
    ) -> CertificateIssuerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let admins_binding = args.admins.get_output(context);
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let password_binding = args.password.get_output(context);
        let provider_name_binding = args.provider_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:keyvault/certificateIssuer:CertificateIssuer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "admins".into(),
                    value: admins_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: key_vault_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: org_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerName".into(),
                    value: provider_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateIssuerResult {
            account_id: o.get_field("accountId"),
            admins: o.get_field("admins"),
            key_vault_id: o.get_field("keyVaultId"),
            name: o.get_field("name"),
            org_id: o.get_field("orgId"),
            password: o.get_field("password"),
            provider_name: o.get_field("providerName"),
        }
    }
}
