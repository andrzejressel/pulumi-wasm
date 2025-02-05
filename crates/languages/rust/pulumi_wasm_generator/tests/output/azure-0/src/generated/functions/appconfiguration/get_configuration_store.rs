pub mod get_configuration_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationStoreArgs {
        /// The Name of this App Configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the App Configuration exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationStoreResult {
        /// An `encryption` block as defined below.
        pub encryptions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreEncryption,
            >,
        >,
        /// The URL of the App Configuration Replica.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identities: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreIdentity,
            >,
        >,
        /// Whether local authentication methods is enabled.
        pub local_auth_enabled: pulumi_wasm_rust::Output<bool>,
        /// The supported Azure location where the App Configuration Replica exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the App Configuration Replica.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `primary_read_key` block as defined below containing the primary read access key.
        pub primary_read_keys: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStorePrimaryReadKey,
            >,
        >,
        /// A `primary_write_key` block as defined below containing the primary write access key.
        pub primary_write_keys: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStorePrimaryWriteKey,
            >,
        >,
        /// The Public Network Access setting of this App Configuration.
        pub public_network_access: pulumi_wasm_rust::Output<String>,
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether Purge Protection is enabled.
        pub purge_protection_enabled: pulumi_wasm_rust::Output<bool>,
        /// One or more `replica` blocks as defined below.
        pub replicas: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreReplica,
            >,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `secondary_read_key` block as defined below containing the secondary read access key.
        pub secondary_read_keys: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreSecondaryReadKey,
            >,
        >,
        /// A `secondary_write_key` block as defined below containing the secondary write access key.
        pub secondary_write_keys: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::appconfiguration::GetConfigurationStoreSecondaryWriteKey,
            >,
        >,
        /// The name of the SKU used for this App Configuration.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// The number of days that items should be retained for once soft-deleted.
        pub soft_delete_retention_days: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags assigned to the App Configuration.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetConfigurationStoreArgs,
    ) -> GetConfigurationStoreResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appconfiguration/getConfigurationStore:getConfigurationStore"
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
        GetConfigurationStoreResult {
            encryptions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptions"),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localAuthEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            primary_read_keys: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryReadKeys"),
            ),
            primary_write_keys: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryWriteKeys"),
            ),
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccess"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            purge_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("purgeProtectionEnabled"),
            ),
            replicas: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicas"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_read_keys: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryReadKeys"),
            ),
            secondary_write_keys: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryWriteKeys"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            soft_delete_retention_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("softDeleteRetentionDays"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
