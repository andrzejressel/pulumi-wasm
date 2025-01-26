/// Manages a Cosmos DB Mongo Role Definition.
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
///     let exampleMongoRoleDefinition = mongo_role_definition::create(
///         "exampleMongoRoleDefinition",
///         MongoRoleDefinitionArgs::builder()
///             .cosmos_mongo_database_id("${exampleMongoDatabase.id}")
///             .role_name("example-roledefinition")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cosmos DB Mongo Role Definitions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/mongoRoleDefinition:MongoRoleDefinition example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1/mongodbRoleDefinitions/dbname1.rolename1
/// ```
///
pub mod mongo_role_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MongoRoleDefinitionArgs {
        /// The resource ID of the Mongo DB. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cosmos_mongo_database_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of Mongo Roles which are inherited to the Mongo Role Definition.
        ///
        /// > **Note:** The role that needs to be inherited should exist in the Mongo DB of `cosmos_mongo_database_id`.
        #[builder(into, default)]
        pub inherited_role_names: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `privilege` block as defined below.
        #[builder(into, default)]
        pub privileges: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::cosmosdb::MongoRoleDefinitionPrivilege>>,
        >,
        /// The user-friendly name for the Mongo Role Definition. It must be unique for the database account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub role_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MongoRoleDefinitionResult {
        /// The resource ID of the Mongo DB. Changing this forces a new resource to be created.
        pub cosmos_mongo_database_id: pulumi_wasm_rust::Output<String>,
        /// A list of Mongo Roles which are inherited to the Mongo Role Definition.
        ///
        /// > **Note:** The role that needs to be inherited should exist in the Mongo DB of `cosmos_mongo_database_id`.
        pub inherited_role_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `privilege` block as defined below.
        pub privileges: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cosmosdb::MongoRoleDefinitionPrivilege>>,
        >,
        /// The user-friendly name for the Mongo Role Definition. It must be unique for the database account. Changing this forces a new resource to be created.
        pub role_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MongoRoleDefinitionArgs,
    ) -> MongoRoleDefinitionResult {
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
        let privileges_binding = args.privileges.get_output(context).get_inner();
        let role_name_binding = args.role_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/mongoRoleDefinition:MongoRoleDefinition".into(),
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
                    name: "privileges".into(),
                    value: &privileges_binding,
                },
                register_interface::ObjectField {
                    name: "roleName".into(),
                    value: &role_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cosmosMongoDatabaseId".into(),
                },
                register_interface::ResultField {
                    name: "inheritedRoleNames".into(),
                },
                register_interface::ResultField {
                    name: "privileges".into(),
                },
                register_interface::ResultField {
                    name: "roleName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MongoRoleDefinitionResult {
            cosmos_mongo_database_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cosmosMongoDatabaseId").unwrap(),
            ),
            inherited_role_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inheritedRoleNames").unwrap(),
            ),
            privileges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privileges").unwrap(),
            ),
            role_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleName").unwrap(),
            ),
        }
    }
}
