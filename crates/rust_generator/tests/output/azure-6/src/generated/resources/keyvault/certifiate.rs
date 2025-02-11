/// Manages a Key Vault Certificate.
///
/// ## Example Usage
///
/// ### Importing A PFX)
///
/// > **Note:** this example assumed the PFX file is located in the same directory at `certificate-to-import.pfx`.
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
///       tenantId: ${current.tenantId}
///       skuName: premium
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           certificatePermissions:
///             - Create
///             - Delete
///             - DeleteIssuers
///             - Get
///             - GetIssuers
///             - Import
///             - List
///             - ListIssuers
///             - ManageContacts
///             - ManageIssuers
///             - SetIssuers
///             - Update
///           keyPermissions:
///             - Backup
///             - Create
///             - Decrypt
///             - Delete
///             - Encrypt
///             - Get
///             - Import
///             - List
///             - Purge
///             - Recover
///             - Restore
///             - Sign
///             - UnwrapKey
///             - Update
///             - Verify
///             - WrapKey
///           secretPermissions:
///             - Backup
///             - Delete
///             - Get
///             - List
///             - Purge
///             - Recover
///             - Restore
///             - Set
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: imported-cert
///       keyVaultId: ${exampleKeyVault.id}
///       certificate:
///         contents:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: certificate-to-import.pfx
///             return: result
///         password: ""
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ### Generating a new certificate
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
///       tenantId: ${current.tenantId}
///       skuName: standard
///       softDeleteRetentionDays: 7
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           certificatePermissions:
///             - Create
///             - Delete
///             - DeleteIssuers
///             - Get
///             - GetIssuers
///             - Import
///             - List
///             - ListIssuers
///             - ManageContacts
///             - ManageIssuers
///             - Purge
///             - SetIssuers
///             - Update
///           keyPermissions:
///             - Backup
///             - Create
///             - Decrypt
///             - Delete
///             - Encrypt
///             - Get
///             - Import
///             - List
///             - Purge
///             - Recover
///             - Restore
///             - Sign
///             - UnwrapKey
///             - Update
///             - Verify
///             - WrapKey
///           secretPermissions:
///             - Backup
///             - Delete
///             - Get
///             - List
///             - Purge
///             - Recover
///             - Restore
///             - Set
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: generated-cert
///       keyVaultId: ${exampleKeyVault.id}
///       certificatePolicy:
///         issuerParameters:
///           name: Self
///         keyProperties:
///           exportable: true
///           keySize: 2048
///           keyType: RSA
///           reuseKey: true
///         lifetimeActions:
///           - action:
///               actionType: AutoRenew
///             trigger:
///               daysBeforeExpiry: 30
///         secretProperties:
///           contentType: application/x-pkcs12
///         x509CertificateProperties:
///           extendedKeyUsages:
///             - 1.3.6.1.5.5.7.3.1
///           keyUsages:
///             - cRLSign
///             - dataEncipherment
///             - digitalSignature
///             - keyAgreement
///             - keyCertSign
///             - keyEncipherment
///           subjectAlternativeNames:
///             dnsNames:
///               - internal.contoso.com
///               - domain.hello.world
///           subject: CN=hello-world
///           validityInMonths: 12
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Key Vault Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/certifiate:Certifiate example "https://example-keyvault.vault.azure.net/certificates/example/fdf067c93bbb4b22bff4d8b7a9a56217"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certifiate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertifiateArgs {
        /// A `certificate` block as defined below, used to Import an existing certificate. Changing this will create a new version of the Key Vault Certificate.
        #[builder(into, default)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::keyvault::CertifiateCertificate>,
        >,
        /// A `certificate_policy` block as defined below. Changing this (except the `lifetime_action` field) will create a new version of the Key Vault Certificate.
        ///
        /// > **NOTE:** When creating a Key Vault Certificate, at least one of `certificate` or `certificate_policy` is required. Provide `certificate` to import an existing certificate, `certificate_policy` to generate a new certificate.
        #[builder(into, default)]
        pub certificate_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::keyvault::CertifiateCertificatePolicy>,
        >,
        /// The ID of the Key Vault where the Certificate should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertifiateResult {
        /// A `certificate` block as defined below, used to Import an existing certificate. Changing this will create a new version of the Key Vault Certificate.
        pub certificate: pulumi_gestalt_rust::Output<
            Option<super::super::types::keyvault::CertifiateCertificate>,
        >,
        /// A `certificate_attribute` block as defined below.
        pub certificate_attributes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::keyvault::CertifiateCertificateAttribute>,
        >,
        /// The raw Key Vault Certificate data represented as a hexadecimal string.
        pub certificate_data: pulumi_gestalt_rust::Output<String>,
        /// The Base64 encoded Key Vault Certificate data.
        pub certificate_data_base64: pulumi_gestalt_rust::Output<String>,
        /// A `certificate_policy` block as defined below. Changing this (except the `lifetime_action` field) will create a new version of the Key Vault Certificate.
        ///
        /// > **NOTE:** When creating a Key Vault Certificate, at least one of `certificate` or `certificate_policy` is required. Provide `certificate` to import an existing certificate, `certificate_policy` to generate a new certificate.
        pub certificate_policy: pulumi_gestalt_rust::Output<
            super::super::types::keyvault::CertifiateCertificatePolicy,
        >,
        /// The ID of the Key Vault where the Certificate should be created. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Key Vault Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The (Versioned) ID for this Key Vault Certificate. This property points to a specific version of a Key Vault Certificate, as such using this won't auto-rotate values if used in other Azure Services.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        /// The Versionless ID of the Key Vault Certificate. This property allows other Azure Services (that support it) to auto-rotate their value when the Key Vault Certificate is updated.
        pub resource_manager_versionless_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the associated Key Vault Secret.
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertifiateArgs,
    ) -> CertifiateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_binding = args.certificate.get_output(context);
        let certificate_policy_binding = args.certificate_policy.get_output(context);
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:keyvault/certifiate:Certifiate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificatePolicy".into(),
                    value: &certificate_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertifiateResult {
            certificate: o.get_field("certificate"),
            certificate_attributes: o.get_field("certificateAttributes"),
            certificate_data: o.get_field("certificateData"),
            certificate_data_base64: o.get_field("certificateDataBase64"),
            certificate_policy: o.get_field("certificatePolicy"),
            key_vault_id: o.get_field("keyVaultId"),
            name: o.get_field("name"),
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
