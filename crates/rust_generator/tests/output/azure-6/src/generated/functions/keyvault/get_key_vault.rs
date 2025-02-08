#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_key_vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyVaultArgs {
        /// Specifies the name of the Key Vault.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Key Vault exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKeyVaultResult {
        /// One or more `access_policy` blocks as defined below.
        pub access_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::keyvault::GetKeyVaultAccessPolicy>,
        >,
        /// Is Role Based Access Control (RBAC) for authorization of data actions enabled on this Key Vault?
        pub enable_rbac_authorization: pulumi_gestalt_rust::Output<bool>,
        /// Can Azure Virtual Machines retrieve certificates stored as secrets from the Key Vault?
        pub enabled_for_deployment: pulumi_gestalt_rust::Output<bool>,
        /// Can Azure Disk Encryption retrieve secrets from the Key Vault?
        pub enabled_for_disk_encryption: pulumi_gestalt_rust::Output<bool>,
        /// Can Azure Resource Manager retrieve secrets from the Key Vault?
        pub enabled_for_template_deployment: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which the Key Vault exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_acls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::keyvault::GetKeyVaultNetworkAcl>,
        >,
        /// Is public network access enabled on this Key Vault?
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Is purge protection enabled on this Key Vault?
        pub purge_protection_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Name of the SKU used for this Key Vault.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Key Vault.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Azure Active Directory Tenant ID used to authenticate requests for this Key Vault.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
        /// The URI of the vault for performing operations on keys and secrets.
        pub vault_uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetKeyVaultArgs,
    ) -> GetKeyVaultResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getKeyVault:getKeyVault".into(),
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
        GetKeyVaultResult {
            access_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessPolicies"),
            ),
            enable_rbac_authorization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableRbacAuthorization"),
            ),
            enabled_for_deployment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabledForDeployment"),
            ),
            enabled_for_disk_encryption: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabledForDiskEncryption"),
            ),
            enabled_for_template_deployment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabledForTemplateDeployment"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_acls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkAcls"),
            ),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            purge_protection_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("purgeProtectionEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
            vault_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vaultUri"),
            ),
        }
    }
}
