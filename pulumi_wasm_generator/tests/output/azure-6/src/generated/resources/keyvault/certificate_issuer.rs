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
pub mod certificate_issuer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateIssuerArgs {
        /// The account number with the third-party Certificate Issuer.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `admin` blocks as defined below.
        #[builder(into, default)]
        pub admins: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::keyvault::CertificateIssuerAdmin>>,
        >,
        /// The ID of the Key Vault in which to create the Certificate Issuer. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Key Vault Certificate Issuer. Changing this forces a new Key Vault Certificate Issuer to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the organization as provided to the issuer.
        #[builder(into, default)]
        pub org_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The password associated with the account and organization ID at the third-party Certificate Issuer. If not specified, will not overwrite any previous value.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the third-party Certificate Issuer. Possible values are: `DigiCert`, `GlobalSign`, `OneCertV2-PrivateCA`, `OneCertV2-PublicCA` and `SslAdminV2`.
        #[builder(into)]
        pub provider_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateIssuerResult {
        /// The account number with the third-party Certificate Issuer.
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `admin` blocks as defined below.
        pub admins: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::keyvault::CertificateIssuerAdmin>>,
        >,
        /// The ID of the Key Vault in which to create the Certificate Issuer. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Key Vault Certificate Issuer. Changing this forces a new Key Vault Certificate Issuer to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the organization as provided to the issuer.
        pub org_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The password associated with the account and organization ID at the third-party Certificate Issuer. If not specified, will not overwrite any previous value.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the third-party Certificate Issuer. Possible values are: `DigiCert`, `GlobalSign`, `OneCertV2-PrivateCA`, `OneCertV2-PublicCA` and `SslAdminV2`.
        pub provider_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CertificateIssuerArgs,
    ) -> CertificateIssuerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let admins_binding = args.admins.get_output(context).get_inner();
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let provider_name_binding = args.provider_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/certificateIssuer:CertificateIssuer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "admins".into(),
                    value: &admins_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "providerName".into(),
                    value: &provider_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "admins".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "providerName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateIssuerResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            admins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("admins").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerName").unwrap(),
            ),
        }
    }
}
