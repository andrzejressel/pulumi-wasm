/// Manages a Cosmos DB Mongo User Definition.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod mongo_user_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MongoUserDefinitionArgs {
        /// The resource ID of the Mongo DB. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cosmos_mongo_database_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of Mongo Roles that are inherited to the Mongo User Definition.
        ///
        /// > **Note:** The role that needs to be inherited should exist in the Mongo DB of `cosmos_mongo_database_id`.
        #[builder(into, default)]
        pub inherited_role_names: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The password for the Mongo User Definition.
        #[builder(into)]
        pub password: pulumi_wasm_rust::InputOrOutput<String>,
        /// The username for the Mongo User Definition. Changing this forces a new resource to be created.
        #[builder(into)]
        pub username: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MongoUserDefinitionResult {
        /// The resource ID of the Mongo DB. Changing this forces a new resource to be created.
        pub cosmos_mongo_database_id: pulumi_wasm_rust::Output<String>,
        /// A list of Mongo Roles that are inherited to the Mongo User Definition.
        ///
        /// > **Note:** The role that needs to be inherited should exist in the Mongo DB of `cosmos_mongo_database_id`.
        pub inherited_role_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The password for the Mongo User Definition.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The username for the Mongo User Definition. Changing this forces a new resource to be created.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MongoUserDefinitionArgs,
    ) -> MongoUserDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cosmos_mongo_database_id_binding = args
            .cosmos_mongo_database_id
            .get_output(context)
            .get_inner();
        let inherited_role_names_binding = args
            .inherited_role_names
            .get_output(context)
            .get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/mongoUserDefinition:MongoUserDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cosmosMongoDatabaseId".into(),
                    value: &cosmos_mongo_database_id_binding,
                },
                register_interface::ObjectField {
                    name: "inheritedRoleNames".into(),
                    value: &inherited_role_names_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MongoUserDefinitionResult {
            cosmos_mongo_database_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cosmosMongoDatabaseId"),
            ),
            inherited_role_names: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inheritedRoleNames"),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
