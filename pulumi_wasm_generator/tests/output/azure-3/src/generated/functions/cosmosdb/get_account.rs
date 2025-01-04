pub mod get_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// Specifies the name of the CosmosDB Account.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group in which the CosmosDB Account resides.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountResult {
        /// If automatic failover is enabled for this CosmosDB Account.
        pub automatic_failover_enabled: pulumi_wasm_rust::Output<bool>,
        /// Capabilities enabled on this Cosmos DB account.
        pub capabilities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetAccountCapability>,
        >,
        pub consistency_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetAccountConsistencyPolicy>,
        >,
        /// The endpoint used to connect to the CosmosDB account.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// If Free Tier pricing option is enabled for this CosmosDB Account. You can have up to one free tier Azure Cosmos DB account per Azure subscription.
        pub free_tier_enabled: pulumi_wasm_rust::Output<bool>,
        pub geo_locations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetAccountGeoLocation>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The current IP Filter for this CosmosDB account
        pub ip_range_filter: pulumi_wasm_rust::Output<String>,
        /// If virtual network filtering is enabled for this Cosmos DB account.
        pub is_virtual_network_filter_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Key Vault key URI for CMK encryption.
        pub key_vault_key_id: pulumi_wasm_rust::Output<String>,
        /// The Kind of the CosmosDB account.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The name of the Azure region hosting replicated data.
        pub location: pulumi_wasm_rust::Output<String>,
        /// If multiple write locations are enabled for this Cosmos DB account.
        pub multiple_write_locations_enabled: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Offer Type to used by this CosmosDB Account.
        pub offer_type: pulumi_wasm_rust::Output<String>,
        /// The primary key for the CosmosDB account.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        /// The primary Mongodb connection string for the CosmosDB account.
        pub primary_mongodb_connection_string: pulumi_wasm_rust::Output<String>,
        /// The primary read-only Key for the CosmosDB account.
        pub primary_readonly_key: pulumi_wasm_rust::Output<String>,
        /// The primary readonly Mongodb connection string for the CosmosDB account.
        pub primary_readonly_mongodb_connection_string: pulumi_wasm_rust::Output<String>,
        /// The primary read-only SQL connection string for the CosmosDB account.
        pub primary_readonly_sql_connection_string: pulumi_wasm_rust::Output<String>,
        /// The primary SQL connection string for the CosmosDB Account.
        pub primary_sql_connection_string: pulumi_wasm_rust::Output<String>,
        /// A list of read endpoints available for this CosmosDB account.
        pub read_endpoints: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary key for the CosmosDB account.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// The secondary Mongodb connection string for the CosmosDB account.
        pub secondary_mongodb_connection_string: pulumi_wasm_rust::Output<String>,
        /// The secondary read-only key for the CosmosDB account.
        pub secondary_readonly_key: pulumi_wasm_rust::Output<String>,
        /// The secondary readonly Mongodb connection string for the CosmosDB account.
        pub secondary_readonly_mongodb_connection_string: pulumi_wasm_rust::Output<
            String,
        >,
        /// The secondary read-only SQL connection string for the CosmosDB account.
        pub secondary_readonly_sql_connection_string: pulumi_wasm_rust::Output<String>,
        /// The secondary SQL connection string for the CosmosDB Account.
        pub secondary_sql_connection_string: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Subnets that are allowed to access this CosmosDB account.
        pub virtual_network_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetAccountVirtualNetworkRule>,
        >,
        /// A list of write endpoints available for this CosmosDB account.
        pub write_endpoints: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccountArgs) -> GetAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cosmosdb/getAccount:getAccount".into(),
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
                    name: "automaticFailoverEnabled".into(),
                },
                register_interface::ResultField {
                    name: "capabilities".into(),
                },
                register_interface::ResultField {
                    name: "consistencyPolicies".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "freeTierEnabled".into(),
                },
                register_interface::ResultField {
                    name: "geoLocations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipRangeFilter".into(),
                },
                register_interface::ResultField {
                    name: "isVirtualNetworkFilterEnabled".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultKeyId".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "multipleWriteLocationsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "offerType".into(),
                },
                register_interface::ResultField {
                    name: "primaryKey".into(),
                },
                register_interface::ResultField {
                    name: "primaryMongodbConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "primaryReadonlyKey".into(),
                },
                register_interface::ResultField {
                    name: "primaryReadonlyMongodbConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "primaryReadonlySqlConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "primarySqlConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "readEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryKey".into(),
                },
                register_interface::ResultField {
                    name: "secondaryMongodbConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "secondaryReadonlyKey".into(),
                },
                register_interface::ResultField {
                    name: "secondaryReadonlyMongodbConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "secondaryReadonlySqlConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "secondarySqlConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkRules".into(),
                },
                register_interface::ResultField {
                    name: "writeEndpoints".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountResult {
            automatic_failover_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticFailoverEnabled").unwrap(),
            ),
            capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capabilities").unwrap(),
            ),
            consistency_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consistencyPolicies").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            free_tier_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("freeTierEnabled").unwrap(),
            ),
            geo_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("geoLocations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_range_filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipRangeFilter").unwrap(),
            ),
            is_virtual_network_filter_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isVirtualNetworkFilterEnabled").unwrap(),
            ),
            key_vault_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultKeyId").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            multiple_write_locations_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multipleWriteLocationsEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            offer_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offerType").unwrap(),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryKey").unwrap(),
            ),
            primary_mongodb_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryMongodbConnectionString").unwrap(),
            ),
            primary_readonly_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryReadonlyKey").unwrap(),
            ),
            primary_readonly_mongodb_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryReadonlyMongodbConnectionString").unwrap(),
            ),
            primary_readonly_sql_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryReadonlySqlConnectionString").unwrap(),
            ),
            primary_sql_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primarySqlConnectionString").unwrap(),
            ),
            read_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readEndpoints").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryKey").unwrap(),
            ),
            secondary_mongodb_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryMongodbConnectionString").unwrap(),
            ),
            secondary_readonly_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryReadonlyKey").unwrap(),
            ),
            secondary_readonly_mongodb_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryReadonlyMongodbConnectionString").unwrap(),
            ),
            secondary_readonly_sql_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryReadonlySqlConnectionString").unwrap(),
            ),
            secondary_sql_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondarySqlConnectionString").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_network_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkRules").unwrap(),
            ),
            write_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writeEndpoints").unwrap(),
            ),
        }
    }
}
