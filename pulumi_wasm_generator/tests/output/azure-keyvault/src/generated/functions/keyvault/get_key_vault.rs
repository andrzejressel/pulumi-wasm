pub mod get_key_vault {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyVaultArgs {
        /// Specifies the name of the Key Vault.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Key Vault exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetKeyVaultResult {
        /// One or more `access_policy` blocks as defined below.
        pub access_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::keyvault::GetKeyVaultAccessPolicy>,
        >,
        /// Is Role Based Access Control (RBAC) for authorization of data actions enabled on this Key Vault?
        pub enable_rbac_authorization: pulumi_wasm_rust::Output<bool>,
        /// Can Azure Virtual Machines retrieve certificates stored as secrets from the Key Vault?
        pub enabled_for_deployment: pulumi_wasm_rust::Output<bool>,
        /// Can Azure Disk Encryption retrieve secrets from the Key Vault?
        pub enabled_for_disk_encryption: pulumi_wasm_rust::Output<bool>,
        /// Can Azure Resource Manager retrieve secrets from the Key Vault?
        pub enabled_for_template_deployment: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region in which the Key Vault exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_acls: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::keyvault::GetKeyVaultNetworkAcl>,
        >,
        /// Is public network access enabled on this Key Vault?
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// Is purge protection enabled on this Key Vault?
        pub purge_protection_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the SKU used for this Key Vault.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Key Vault.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Azure Active Directory Tenant ID used to authenticate requests for this Key Vault.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
        /// The URI of the vault for performing operations on keys and secrets.
        pub vault_uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetKeyVaultArgs) -> GetKeyVaultResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getKeyVault:getKeyVault".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPolicies".into(),
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
                    name: "id".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKeyVaultResult {
            access_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicies").unwrap(),
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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