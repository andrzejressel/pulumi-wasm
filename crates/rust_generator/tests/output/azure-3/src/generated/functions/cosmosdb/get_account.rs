pub mod get_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// Specifies the name of the CosmosDB Account.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group in which the CosmosDB Account resides.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountResult {
        /// If automatic failover is enabled for this CosmosDB Account.
        pub automatic_failover_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Capabilities enabled on this Cosmos DB account.
        pub capabilities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetAccountCapability>,
        >,
        pub consistency_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetAccountConsistencyPolicy>,
        >,
        /// The endpoint used to connect to the CosmosDB account.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// If Free Tier pricing option is enabled for this CosmosDB Account. You can have up to one free tier Azure Cosmos DB account per Azure subscription.
        pub free_tier_enabled: pulumi_gestalt_rust::Output<bool>,
        pub geo_locations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetAccountGeoLocation>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The current IP Filter for this CosmosDB account
        pub ip_range_filter: pulumi_gestalt_rust::Output<String>,
        /// If virtual network filtering is enabled for this Cosmos DB account.
        pub is_virtual_network_filter_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Key Vault key URI for CMK encryption.
        pub key_vault_key_id: pulumi_gestalt_rust::Output<String>,
        /// The Kind of the CosmosDB account.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure region hosting replicated data.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// If multiple write locations are enabled for this Cosmos DB account.
        pub multiple_write_locations_enabled: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Offer Type to used by this CosmosDB Account.
        pub offer_type: pulumi_gestalt_rust::Output<String>,
        /// The primary key for the CosmosDB account.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// The primary Mongodb connection string for the CosmosDB account.
        pub primary_mongodb_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The primary read-only Key for the CosmosDB account.
        pub primary_readonly_key: pulumi_gestalt_rust::Output<String>,
        /// The primary readonly Mongodb connection string for the CosmosDB account.
        pub primary_readonly_mongodb_connection_string: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The primary read-only SQL connection string for the CosmosDB account.
        pub primary_readonly_sql_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The primary SQL connection string for the CosmosDB Account.
        pub primary_sql_connection_string: pulumi_gestalt_rust::Output<String>,
        /// A list of read endpoints available for this CosmosDB account.
        pub read_endpoints: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary key for the CosmosDB account.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary Mongodb connection string for the CosmosDB account.
        pub secondary_mongodb_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary read-only key for the CosmosDB account.
        pub secondary_readonly_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary readonly Mongodb connection string for the CosmosDB account.
        pub secondary_readonly_mongodb_connection_string: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The secondary read-only SQL connection string for the CosmosDB account.
        pub secondary_readonly_sql_connection_string: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The secondary SQL connection string for the CosmosDB Account.
        pub secondary_sql_connection_string: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Subnets that are allowed to access this CosmosDB account.
        pub virtual_network_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetAccountVirtualNetworkRule>,
        >,
        /// A list of write endpoints available for this CosmosDB account.
        pub write_endpoints: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAccountArgs,
    ) -> GetAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cosmosdb/getAccount:getAccount".into(),
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
        GetAccountResult {
            automatic_failover_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticFailoverEnabled"),
            ),
            capabilities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capabilities"),
            ),
            consistency_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("consistencyPolicies"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            free_tier_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("freeTierEnabled"),
            ),
            geo_locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("geoLocations"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_range_filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipRangeFilter"),
            ),
            is_virtual_network_filter_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isVirtualNetworkFilterEnabled"),
            ),
            key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultKeyId"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            multiple_write_locations_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multipleWriteLocationsEnabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            offer_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("offerType"),
            ),
            primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            primary_mongodb_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryMongodbConnectionString"),
            ),
            primary_readonly_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryReadonlyKey"),
            ),
            primary_readonly_mongodb_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryReadonlyMongodbConnectionString"),
            ),
            primary_readonly_sql_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryReadonlySqlConnectionString"),
            ),
            primary_sql_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primarySqlConnectionString"),
            ),
            read_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readEndpoints"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
            secondary_mongodb_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryMongodbConnectionString"),
            ),
            secondary_readonly_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryReadonlyKey"),
            ),
            secondary_readonly_mongodb_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryReadonlyMongodbConnectionString"),
            ),
            secondary_readonly_sql_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryReadonlySqlConnectionString"),
            ),
            secondary_sql_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondarySqlConnectionString"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            virtual_network_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkRules"),
            ),
            write_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("writeEndpoints"),
            ),
        }
    }
}
