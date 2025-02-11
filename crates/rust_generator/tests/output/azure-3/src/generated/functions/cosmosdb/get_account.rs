#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountArgs,
    ) -> GetAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:cosmosdb/getAccount:getAccount".into(),
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
        GetAccountResult {
            automatic_failover_enabled: o.get_field("automaticFailoverEnabled"),
            capabilities: o.get_field("capabilities"),
            consistency_policies: o.get_field("consistencyPolicies"),
            endpoint: o.get_field("endpoint"),
            free_tier_enabled: o.get_field("freeTierEnabled"),
            geo_locations: o.get_field("geoLocations"),
            id: o.get_field("id"),
            ip_range_filter: o.get_field("ipRangeFilter"),
            is_virtual_network_filter_enabled: o
                .get_field("isVirtualNetworkFilterEnabled"),
            key_vault_key_id: o.get_field("keyVaultKeyId"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            multiple_write_locations_enabled: o
                .get_field("multipleWriteLocationsEnabled"),
            name: o.get_field("name"),
            offer_type: o.get_field("offerType"),
            primary_key: o.get_field("primaryKey"),
            primary_mongodb_connection_string: o
                .get_field("primaryMongodbConnectionString"),
            primary_readonly_key: o.get_field("primaryReadonlyKey"),
            primary_readonly_mongodb_connection_string: o
                .get_field("primaryReadonlyMongodbConnectionString"),
            primary_readonly_sql_connection_string: o
                .get_field("primaryReadonlySqlConnectionString"),
            primary_sql_connection_string: o.get_field("primarySqlConnectionString"),
            read_endpoints: o.get_field("readEndpoints"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_key: o.get_field("secondaryKey"),
            secondary_mongodb_connection_string: o
                .get_field("secondaryMongodbConnectionString"),
            secondary_readonly_key: o.get_field("secondaryReadonlyKey"),
            secondary_readonly_mongodb_connection_string: o
                .get_field("secondaryReadonlyMongodbConnectionString"),
            secondary_readonly_sql_connection_string: o
                .get_field("secondaryReadonlySqlConnectionString"),
            secondary_sql_connection_string: o.get_field("secondarySqlConnectionString"),
            tags: o.get_field("tags"),
            virtual_network_rules: o.get_field("virtualNetworkRules"),
            write_endpoints: o.get_field("writeEndpoints"),
        }
    }
}
