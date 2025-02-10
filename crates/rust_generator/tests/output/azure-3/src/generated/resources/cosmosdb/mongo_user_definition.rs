/// Manages a Cosmos DB Mongo User Definition.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .capabilities(
///                 vec![
///                     AccountCapability::builder().name("EnableMongo").build_struct(),
///                     AccountCapability::builder()
///                     .name("EnableMongoRoleBasedAccessControl").build_struct(),
///                 ],
///             )
///             .consistency_policy(
///                 AccountConsistencyPolicy::builder()
///                     .consistencyLevel("Strong")
///                     .build_struct(),
///             )
///             .geo_locations(
///                 vec![
///                     AccountGeoLocation::builder().failoverPriority(0)
///                     .location("${example.location}").build_struct(),
///                 ],
///             )
///             .kind("MongoDB")
///             .location("${example.location}")
///             .name("example-ca")
///             .offer_type("Standard")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleMongoDatabase = mongo_database::create(
///         "exampleMongoDatabase",
///         MongoDatabaseArgs::builder()
///             .account_name("${exampleAccount.name}")
///             .name("example-mongodb")
///             .resource_group_name("${exampleAccount.resourceGroupName}")
///             .build_struct(),
///     );
///     let exampleMongoUserDefinition = mongo_user_definition::create(
///         "exampleMongoUserDefinition",
///         MongoUserDefinitionArgs::builder()
///             .cosmos_mongo_database_id("${exampleMongoDatabase.id}")
///             .password("myPassword")
///             .username("myUserName")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cosmos DB Mongo User Definitions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/mongoUserDefinition:MongoUserDefinition example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1/mongodbUserDefinitions/dbname1.username1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mongo_user_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MongoUserDefinitionArgs {
        /// The resource ID of the Mongo DB. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cosmos_mongo_database_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of Mongo Roles that are inherited to the Mongo User Definition.
        ///
        /// > **Note:** The role that needs to be inherited should exist in the Mongo DB of `cosmos_mongo_database_id`.
        #[builder(into, default)]
        pub inherited_role_names: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The password for the Mongo User Definition.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The username for the Mongo User Definition. Changing this forces a new resource to be created.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MongoUserDefinitionResult {
        /// The resource ID of the Mongo DB. Changing this forces a new resource to be created.
        pub cosmos_mongo_database_id: pulumi_gestalt_rust::Output<String>,
        /// A list of Mongo Roles that are inherited to the Mongo User Definition.
        ///
        /// > **Note:** The role that needs to be inherited should exist in the Mongo DB of `cosmos_mongo_database_id`.
        pub inherited_role_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The password for the Mongo User Definition.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// The username for the Mongo User Definition. Changing this forces a new resource to be created.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MongoUserDefinitionArgs,
    ) -> MongoUserDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cosmos_mongo_database_id_binding = args
            .cosmos_mongo_database_id
            .get_output(context);
        let inherited_role_names_binding = args.inherited_role_names.get_output(context);
        let password_binding = args.password.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/mongoUserDefinition:MongoUserDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosMongoDatabaseId".into(),
                    value: cosmos_mongo_database_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inheritedRoleNames".into(),
                    value: inherited_role_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: username_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MongoUserDefinitionResult {
            cosmos_mongo_database_id: o.get_field("cosmosMongoDatabaseId"),
            inherited_role_names: o.get_field("inheritedRoleNames"),
            password: o.get_field("password"),
            username: o.get_field("username"),
        }
    }
}
