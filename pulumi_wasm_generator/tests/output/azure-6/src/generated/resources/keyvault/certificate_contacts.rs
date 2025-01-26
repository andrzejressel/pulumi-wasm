/// Manages Key Vault Certificate Contacts.
///
/// ## Disclaimers
///
/// <!-- TODO: Remove Note in 4.0 -->
/// > **Note:** It's possible to define Key Vault Certificate Contacts both within the `azure.keyvault.KeyVault` resource via the `contact` block and by using the `azure.keyvault.CertificateContacts` resource. However it's not possible to use both methods to manage Certificate Contacts within a KeyVault, since there'll be conflicts.
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
///       tenantId: ${current.tenantId}
///       skuName: premium
///   exampleAccessPolicy:
///     type: azure:keyvault:AccessPolicy
///     name: example
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       certificatePermissions:
///         - ManageContacts
///       keyPermissions:
///         - Create
///       secretPermissions:
///         - Set
///   exampleCertificateContacts:
///     type: azure:keyvault:CertificateContacts
///     name: example
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       contacts:
///         - email: example@example.com
///           name: example
///           phone: '01234567890'
///         - email: example2@example.com
///     options:
///       dependsOn:
///         - ${exampleAccessPolicy}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Key Vault Certificate Contacts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/certificateContacts:CertificateContacts example https://example-keyvault.vault.azure.net/certificates/contacts
/// ```
///
pub mod certificate_contacts {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateContactsArgs {
        /// One or more `contact` blocks as defined below.
        /// -->
        #[builder(into, default)]
        pub contacts: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::keyvault::CertificateContactsContact>>,
        >,
        /// The ID of the Key Vault. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateContactsResult {
        /// One or more `contact` blocks as defined below.
        /// -->
        pub contacts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::keyvault::CertificateContactsContact>>,
        >,
        /// The ID of the Key Vault. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CertificateContactsArgs,
    ) -> CertificateContactsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contacts_binding = args.contacts.get_output(context).get_inner();
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/certificateContacts:CertificateContacts".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contacts".into(),
                    value: &contacts_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contacts".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateContactsResult {
            contacts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contacts").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
        }
    }
}
