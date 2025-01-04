pub mod get_managed_hardware_security_module {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleArgs {
        /// The name of the Key Vault Managed Hardware Security Module.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Key Vault Managed Hardware Security Module exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleResult {
        /// Specifies a list of administrators object IDs for the key vault Managed Hardware Security Module.
        pub admin_object_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The URI of the Hardware Security Module for performing operations on keys and secrets.
        pub hsm_uri: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region in which the Key Vault managed Hardware Security Module exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Is purge protection enabled on this Key Vault Managed Hardware Security Module?
        pub purge_protection_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the SKU used for this Key Vault Managed Hardware Security Module.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The number of days that items should be retained for soft-deleted.
        pub soft_delete_retention_days: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags assigned to the Key Vault Managed Hardware Security Module.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Azure Active Directory Tenant ID used for authenticating requests to the Key Vault Managed Hardware Security Module.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetManagedHardwareSecurityModuleArgs,
    ) -> GetManagedHardwareSecurityModuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getManagedHardwareSecurityModule:getManagedHardwareSecurityModule"
                .into(),
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
                    name: "adminObjectIds".into(),
                },
                register_interface::ResultField {
                    name: "hsmUri".into(),
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
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetManagedHardwareSecurityModuleResult {
            admin_object_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminObjectIds").unwrap(),
            ),
            hsm_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmUri").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
        }
    }
}
