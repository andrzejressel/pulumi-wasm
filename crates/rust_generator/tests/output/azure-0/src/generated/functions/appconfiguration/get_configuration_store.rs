#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_configuration_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationStoreArgs {
        /// The Name of this App Configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the App Configuration exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationStoreResult {
        /// An `encryption` block as defined below.
        pub encryptions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreEncryption,
            >,
        >,
        /// The URL of the App Configuration Replica.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreIdentity,
            >,
        >,
        /// Whether local authentication methods is enabled.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The supported Azure location where the App Configuration Replica exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the App Configuration Replica.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `primary_read_key` block as defined below containing the primary read access key.
        pub primary_read_keys: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStorePrimaryReadKey,
            >,
        >,
        /// A `primary_write_key` block as defined below containing the primary write access key.
        pub primary_write_keys: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStorePrimaryWriteKey,
            >,
        >,
        /// The Public Network Access setting of this App Configuration.
        pub public_network_access: pulumi_gestalt_rust::Output<String>,
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether Purge Protection is enabled.
        pub purge_protection_enabled: pulumi_gestalt_rust::Output<bool>,
        /// One or more `replica` blocks as defined below.
        pub replicas: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreReplica,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `secondary_read_key` block as defined below containing the secondary read access key.
        pub secondary_read_keys: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreSecondaryReadKey,
            >,
        >,
        /// A `secondary_write_key` block as defined below containing the secondary write access key.
        pub secondary_write_keys: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreSecondaryWriteKey,
            >,
        >,
        /// The name of the SKU used for this App Configuration.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// The number of days that items should be retained for once soft-deleted.
        pub soft_delete_retention_days: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags assigned to the App Configuration.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConfigurationStoreArgs,
    ) -> GetConfigurationStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appconfiguration/getConfigurationStore:getConfigurationStore"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConfigurationStoreResult {
            encryptions: o.get_field("encryptions"),
            endpoint: o.get_field("endpoint"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            local_auth_enabled: o.get_field("localAuthEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_read_keys: o.get_field("primaryReadKeys"),
            primary_write_keys: o.get_field("primaryWriteKeys"),
            public_network_access: o.get_field("publicNetworkAccess"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            purge_protection_enabled: o.get_field("purgeProtectionEnabled"),
            replicas: o.get_field("replicas"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_read_keys: o.get_field("secondaryReadKeys"),
            secondary_write_keys: o.get_field("secondaryWriteKeys"),
            sku: o.get_field("sku"),
            soft_delete_retention_days: o.get_field("softDeleteRetentionDays"),
            tags: o.get_field("tags"),
        }
    }
}
