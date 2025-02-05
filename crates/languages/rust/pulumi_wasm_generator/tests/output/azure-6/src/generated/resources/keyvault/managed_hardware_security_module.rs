/// Manages a Key Vault Managed Hardware Security Module.
///
/// > **Note:** The Azure Provider includes a Feature Toggle which will purge a Key Vault Managed Hardware Security Module resource on destroy, rather than the default soft-delete. See `purge_soft_deleted_hardware_security_modules_on_destroy` for more information.
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
///   exampleManagedHardwareSecurityModule:
///     type: azure:keyvault:ManagedHardwareSecurityModule
///     name: example
///     properties:
///       name: exampleKVHsm
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: Standard_B1
///       purgeProtectionEnabled: false
///       softDeleteRetentionDays: 90
///       tenantId: ${current.tenantId}
///       adminObjectIds:
///         - ${current.objectId}
///       tags:
///         Env: Test
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Key Vault Managed Hardware Security Module can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/managedHardwareSecurityModule:ManagedHardwareSecurityModule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.KeyVault/managedHSMs/hsm1
/// ```
///
pub mod managed_hardware_security_module {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleArgs {
        /// Specifies a list of administrators object IDs for the key vault Managed Hardware Security Module. Changing this forces a new resource to be created.
        #[builder(into)]
        pub admin_object_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Key Vault Managed Hardware Security Module. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `network_acls` block as defined below.
        #[builder(into, default)]
        pub network_acls: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::keyvault::ManagedHardwareSecurityModuleNetworkAcls,
            >,
        >,
        /// Whether traffic from public networks is permitted. Defaults to `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Is Purge Protection enabled for this Key Vault Managed Hardware Security Module? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub purge_protection_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group in which to create the Key Vault Managed Hardware Security Module. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of KeyVault certificates resource IDs (minimum of three and up to a maximum of 10) to activate this Managed HSM. More information see [activate-your-managed-hsm](https://learn.microsoft.com/azure/key-vault/managed-hsm/quick-create-cli#activate-your-managed-hsm)
        #[builder(into, default)]
        pub security_domain_key_vault_certificate_ids: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the minimum number of shares required to decrypt the security domain for recovery. This is required when `security_domain_key_vault_certificate_ids` is specified. Valid values are between 2 and 10.
        #[builder(into, default)]
        pub security_domain_quorum: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The Name of the SKU used for this Key Vault Managed Hardware Security Module. Possible value is `Standard_B1`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of days that items should be retained for once soft-deleted. This value can be between `7` and `90` days. Defaults to `90`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub soft_delete_retention_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Azure Active Directory Tenant ID that should be used for authenticating requests to the key vault Managed Hardware Security Module. Changing this forces a new resource to be created.
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleResult {
        /// Specifies a list of administrators object IDs for the key vault Managed Hardware Security Module. Changing this forces a new resource to be created.
        pub admin_object_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The URI of the Key Vault Managed Hardware Security Module, used for performing operations on keys.
        pub hsm_uri: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Key Vault Managed Hardware Security Module. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_acls` block as defined below.
        pub network_acls: pulumi_wasm_rust::Output<
            super::super::types::keyvault::ManagedHardwareSecurityModuleNetworkAcls,
        >,
        /// Whether traffic from public networks is permitted. Defaults to `true`. Changing this forces a new resource to be created.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is Purge Protection enabled for this Key Vault Managed Hardware Security Module? Changing this forces a new resource to be created.
        pub purge_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Key Vault Managed Hardware Security Module. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// This attribute can be used for disaster recovery or when creating another Managed HSM that shares the same security domain.
        pub security_domain_encrypted_data: pulumi_wasm_rust::Output<String>,
        /// A list of KeyVault certificates resource IDs (minimum of three and up to a maximum of 10) to activate this Managed HSM. More information see [activate-your-managed-hsm](https://learn.microsoft.com/azure/key-vault/managed-hsm/quick-create-cli#activate-your-managed-hsm)
        pub security_domain_key_vault_certificate_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// Specifies the minimum number of shares required to decrypt the security domain for recovery. This is required when `security_domain_key_vault_certificate_ids` is specified. Valid values are between 2 and 10.
        pub security_domain_quorum: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Name of the SKU used for this Key Vault Managed Hardware Security Module. Possible value is `Standard_B1`. Changing this forces a new resource to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The number of days that items should be retained for once soft-deleted. This value can be between `7` and `90` days. Defaults to `90`. Changing this forces a new resource to be created.
        pub soft_delete_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Azure Active Directory Tenant ID that should be used for authenticating requests to the key vault Managed Hardware Security Module. Changing this forces a new resource to be created.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ManagedHardwareSecurityModuleArgs,
    ) -> ManagedHardwareSecurityModuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_object_ids_binding = args
            .admin_object_ids
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_acls_binding = args.network_acls.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let purge_protection_enabled_binding = args
            .purge_protection_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let security_domain_key_vault_certificate_ids_binding = args
            .security_domain_key_vault_certificate_ids
            .get_output(context)
            .get_inner();
        let security_domain_quorum_binding = args
            .security_domain_quorum
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let soft_delete_retention_days_binding = args
            .soft_delete_retention_days
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tenant_id_binding = args.tenant_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/managedHardwareSecurityModule:ManagedHardwareSecurityModule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminObjectIds".into(),
                    value: &admin_object_ids_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkAcls".into(),
                    value: &network_acls_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "purgeProtectionEnabled".into(),
                    value: &purge_protection_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "securityDomainKeyVaultCertificateIds".into(),
                    value: &security_domain_key_vault_certificate_ids_binding,
                },
                register_interface::ObjectField {
                    name: "securityDomainQuorum".into(),
                    value: &security_domain_quorum_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "softDeleteRetentionDays".into(),
                    value: &soft_delete_retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagedHardwareSecurityModuleResult {
            admin_object_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminObjectIds"),
            ),
            hsm_uri: pulumi_wasm_rust::__private::into_domain(o.extract_field("hsmUri")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network_acls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkAcls"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            purge_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("purgeProtectionEnabled"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            security_domain_encrypted_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityDomainEncryptedData"),
            ),
            security_domain_key_vault_certificate_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityDomainKeyVaultCertificateIds"),
            ),
            security_domain_quorum: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityDomainQuorum"),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            soft_delete_retention_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("softDeleteRetentionDays"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
