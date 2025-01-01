pub mod get_configuration_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationStoreArgs {
        /// The Name of this App Configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the App Configuration exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetConfigurationStoreArgs) -> GetConfigurationStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appconfiguration/getConfigurationStore:getConfigurationStore"
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
                    name: "encryptions".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "localAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryReadKeys".into(),
                },
                register_interface::ResultField {
                    name: "primaryWriteKeys".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccess".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "purgeProtectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "replicas".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryReadKeys".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWriteKeys".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "softDeleteRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConfigurationStoreResult {
            encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptions").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_read_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryReadKeys").unwrap(),
            ),
            primary_write_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryWriteKeys").unwrap(),
            ),
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccess").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            purge_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purgeProtectionEnabled").unwrap(),
            ),
            replicas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicas").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_read_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryReadKeys").unwrap(),
            ),
            secondary_write_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWriteKeys").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            soft_delete_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("softDeleteRetentionDays").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
