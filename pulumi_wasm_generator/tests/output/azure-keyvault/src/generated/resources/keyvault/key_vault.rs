/// Manages a Key Vault.
///
/// ## Disclaimers
///
/// > **Note:** It's possible to define Key Vault Access Policies both within the `azure.keyvault.KeyVault` resource via the `access_policy` block and by using the `azure.keyvault.AccessPolicy` resource. However it's not possible to use both methods to manage Access Policies within a KeyVault, since there'll be conflicts.
///
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
///       enabledForDiskEncryption: true
///       tenantId: ${current.tenantId}
///       softDeleteRetentionDays: 7
///       purgeProtectionEnabled: false
///       skuName: standard
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Get
///           secretPermissions:
///             - Get
///           storagePermissions:
///             - Get
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Key Vault's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/keyVault:KeyVault example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.KeyVault/vaults/vault1
/// ```
///
pub mod key_vault {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyVaultArgs {
        /// A list of up to 1024 objects describing access policies, as described below.
        ///
        /// > **NOTE** Since `access_policy` can be configured both inline and via the separate `azure.keyvault.AccessPolicy` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub access_policies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::keyvault::KeyVaultAccessPolicy>>,
        >,
        /// One or more `contact` block as defined below.
        ///
        /// > **Note:** This field can only be set once user has `managecontacts` certificate permission.
        ///
        /// > **Note:** This field can only be set when `public_network_access_enabled` is set to `true`. To manage the `contact` with `public_network_access_enabled` set to `false`, please use the `azure.keyvault.CertificateContacts` resource instead of this property, and remove this property from the configuration. Especially for existing `azure.keyvault.KeyVault`, this means you'll need to import the `azure.keyvault.CertificateContacts` manually.
        #[builder(into, default)]
        pub contacts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::keyvault::KeyVaultContact>>,
        >,
        /// Boolean flag to specify whether Azure Key Vault uses Role Based Access Control (RBAC) for authorization of data actions.
        #[builder(into, default)]
        pub enable_rbac_authorization: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag to specify whether Azure Virtual Machines are permitted to retrieve certificates stored as secrets from the key vault.
        #[builder(into, default)]
        pub enabled_for_deployment: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag to specify whether Azure Disk Encryption is permitted to retrieve secrets from the vault and unwrap keys.
        #[builder(into, default)]
        pub enabled_for_disk_encryption: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag to specify whether Azure Resource Manager is permitted to retrieve secrets from the key vault.
        #[builder(into, default)]
        pub enabled_for_template_deployment: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Key Vault. Changing this forces a new resource to be created. The name must be globally unique. If the vault is in a recoverable state then the vault will need to be purged before reusing the name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network_acls` block as defined below.
        #[builder(into, default)]
        pub network_acls: pulumi_wasm_rust::Output<
            Option<super::super::types::keyvault::KeyVaultNetworkAcls>,
        >,
        /// Whether public network access is allowed for this Key Vault. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is Purge Protection enabled for this Key Vault?
        ///
        /// !> **Note:** Once Purge Protection has been Enabled it's not possible to Disable it. Support for [disabling purge protection is being tracked in this Azure API issue](https://github.com/Azure/azure-rest-api-specs/issues/8075). Deleting the Key Vault with Purge Protection Enabled will schedule the Key Vault to be deleted (which will happen by Azure in the configured number of days, currently 90 days).
        #[builder(into, default)]
        pub purge_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Key Vault. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the SKU used for this Key Vault. Possible values are `standard` and `premium`.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The number of days that items should be retained for once soft-deleted. This value can be between `7` and `90` (the default) days.
        ///
        /// > **Note:** This field can only be configured one time and cannot be updated.
        ///
        /// <!-- TODO: Remove `contact` and Notes in 4.0 -->
        #[builder(into, default)]
        pub soft_delete_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault.
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct KeyVaultResult {
        /// A list of up to 1024 objects describing access policies, as described below.
        ///
        /// > **NOTE** Since `access_policy` can be configured both inline and via the separate `azure.keyvault.AccessPolicy` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        pub access_policies: pulumi_wasm_rust::Output<
            Vec<super::super::types::keyvault::KeyVaultAccessPolicy>,
        >,
        /// One or more `contact` block as defined below.
        ///
        /// > **Note:** This field can only be set once user has `managecontacts` certificate permission.
        ///
        /// > **Note:** This field can only be set when `public_network_access_enabled` is set to `true`. To manage the `contact` with `public_network_access_enabled` set to `false`, please use the `azure.keyvault.CertificateContacts` resource instead of this property, and remove this property from the configuration. Especially for existing `azure.keyvault.KeyVault`, this means you'll need to import the `azure.keyvault.CertificateContacts` manually.
        pub contacts: pulumi_wasm_rust::Output<
            Vec<super::super::types::keyvault::KeyVaultContact>,
        >,
        /// Boolean flag to specify whether Azure Key Vault uses Role Based Access Control (RBAC) for authorization of data actions.
        pub enable_rbac_authorization: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag to specify whether Azure Virtual Machines are permitted to retrieve certificates stored as secrets from the key vault.
        pub enabled_for_deployment: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag to specify whether Azure Disk Encryption is permitted to retrieve secrets from the vault and unwrap keys.
        pub enabled_for_disk_encryption: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag to specify whether Azure Resource Manager is permitted to retrieve secrets from the key vault.
        pub enabled_for_template_deployment: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Key Vault. Changing this forces a new resource to be created. The name must be globally unique. If the vault is in a recoverable state then the vault will need to be purged before reusing the name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_acls` block as defined below.
        pub network_acls: pulumi_wasm_rust::Output<
            super::super::types::keyvault::KeyVaultNetworkAcls,
        >,
        /// Whether public network access is allowed for this Key Vault. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is Purge Protection enabled for this Key Vault?
        ///
        /// !> **Note:** Once Purge Protection has been Enabled it's not possible to Disable it. Support for [disabling purge protection is being tracked in this Azure API issue](https://github.com/Azure/azure-rest-api-specs/issues/8075). Deleting the Key Vault with Purge Protection Enabled will schedule the Key Vault to be deleted (which will happen by Azure in the configured number of days, currently 90 days).
        pub purge_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Key Vault. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the SKU used for this Key Vault. Possible values are `standard` and `premium`.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The number of days that items should be retained for once soft-deleted. This value can be between `7` and `90` (the default) days.
        ///
        /// > **Note:** This field can only be configured one time and cannot be updated.
        ///
        /// <!-- TODO: Remove `contact` and Notes in 4.0 -->
        pub soft_delete_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
        /// The URI of the Key Vault, used for performing operations on keys and secrets.
        pub vault_uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KeyVaultArgs) -> KeyVaultResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_policies_binding = args.access_policies.get_inner();
        let contacts_binding = args.contacts.get_inner();
        let enable_rbac_authorization_binding = args
            .enable_rbac_authorization
            .get_inner();
        let enabled_for_deployment_binding = args.enabled_for_deployment.get_inner();
        let enabled_for_disk_encryption_binding = args
            .enabled_for_disk_encryption
            .get_inner();
        let enabled_for_template_deployment_binding = args
            .enabled_for_template_deployment
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_acls_binding = args.network_acls.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let purge_protection_enabled_binding = args.purge_protection_enabled.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let soft_delete_retention_days_binding = args
            .soft_delete_retention_days
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let tenant_id_binding = args.tenant_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/keyVault:KeyVault".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicies".into(),
                    value: &access_policies_binding,
                },
                register_interface::ObjectField {
                    name: "contacts".into(),
                    value: &contacts_binding,
                },
                register_interface::ObjectField {
                    name: "enableRbacAuthorization".into(),
                    value: &enable_rbac_authorization_binding,
                },
                register_interface::ObjectField {
                    name: "enabledForDeployment".into(),
                    value: &enabled_for_deployment_binding,
                },
                register_interface::ObjectField {
                    name: "enabledForDiskEncryption".into(),
                    value: &enabled_for_disk_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "enabledForTemplateDeployment".into(),
                    value: &enabled_for_template_deployment_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPolicies".into(),
                },
                register_interface::ResultField {
                    name: "contacts".into(),
                },
                register_interface::ResultField {
                    name: "enableRbacAuthorization".into(),
                },
                register_interface::ResultField {
                    name: "enabledForDeployment".into(),
                },
                register_interface::ResultField {
                    name: "enabledForDiskEncryption".into(),
                },
                register_interface::ResultField {
                    name: "enabledForTemplateDeployment".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkAcls".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "purgeProtectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "softDeleteRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
                register_interface::ResultField {
                    name: "vaultUri".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KeyVaultResult {
            access_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicies").unwrap(),
            ),
            contacts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contacts").unwrap(),
            ),
            enable_rbac_authorization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableRbacAuthorization").unwrap(),
            ),
            enabled_for_deployment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledForDeployment").unwrap(),
            ),
            enabled_for_disk_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledForDiskEncryption").unwrap(),
            ),
            enabled_for_template_deployment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledForTemplateDeployment").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_acls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAcls").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            purge_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purgeProtectionEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            soft_delete_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("softDeleteRetentionDays").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
            vault_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultUri").unwrap(),
            ),
        }
    }
}