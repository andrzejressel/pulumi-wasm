#[allow(clippy::doc_lazy_continuation)]
pub mod get_managed_hardware_security_module {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleArgs {
        /// The name of the Key Vault Managed Hardware Security Module.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Key Vault Managed Hardware Security Module exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleResult {
        /// Specifies a list of administrators object IDs for the key vault Managed Hardware Security Module.
        pub admin_object_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The URI of the Hardware Security Module for performing operations on keys and secrets.
        pub hsm_uri: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which the Key Vault managed Hardware Security Module exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is purge protection enabled on this Key Vault Managed Hardware Security Module?
        pub purge_protection_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Name of the SKU used for this Key Vault Managed Hardware Security Module.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The number of days that items should be retained for soft-deleted.
        pub soft_delete_retention_days: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags assigned to the Key Vault Managed Hardware Security Module.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Azure Active Directory Tenant ID used for authenticating requests to the Key Vault Managed Hardware Security Module.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetManagedHardwareSecurityModuleArgs,
    ) -> GetManagedHardwareSecurityModuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getManagedHardwareSecurityModule:getManagedHardwareSecurityModule"
                .into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedHardwareSecurityModuleResult {
            admin_object_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("adminObjectIds"),
            ),
            hsm_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hsmUri"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            purge_protection_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("purgeProtectionEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            soft_delete_retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("softDeleteRetentionDays"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
