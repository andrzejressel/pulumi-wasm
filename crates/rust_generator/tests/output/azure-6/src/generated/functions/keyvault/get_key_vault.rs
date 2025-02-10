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
        context: &pulumi_gestalt_rust::Context,
        args: GetKeyVaultArgs,
    ) -> GetKeyVaultResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getKeyVault:getKeyVault".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKeyVaultResult {
            access_policies: o.get_field("accessPolicies"),
            enable_rbac_authorization: o.get_field("enableRbacAuthorization"),
            enabled_for_deployment: o.get_field("enabledForDeployment"),
            enabled_for_disk_encryption: o.get_field("enabledForDiskEncryption"),
            enabled_for_template_deployment: o.get_field("enabledForTemplateDeployment"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_acls: o.get_field("networkAcls"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            purge_protection_enabled: o.get_field("purgeProtectionEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            tenant_id: o.get_field("tenantId"),
            vault_uri: o.get_field("vaultUri"),
        }
    }
}
