/// Provides a Glue Catalog Database Resource. You can refer to the [Glue Developer Guide](http://docs.aws.amazon.com/glue/latest/dg/populate-data-catalog.html) for a full explanation of the Glue Data Catalog functionality
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_database::create(
///         "example",
///         CatalogDatabaseArgs::builder().name("MyCatalogDatabase").build_struct(),
///     );
/// }
/// ```
///
/// ### Create Table Default Permissions
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_database::create(
///         "example",
///         CatalogDatabaseArgs::builder()
///             .create_table_default_permissions(
///                 vec![
///                     CatalogDatabaseCreateTableDefaultPermission::builder()
///                     .permissions(vec!["SELECT",])
///                     .principal(CatalogDatabaseCreateTableDefaultPermissionPrincipal::builder()
///                     .dataLakePrincipalIdentifier("IAM_ALLOWED_PRINCIPALS")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .name("MyCatalogDatabase")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Catalog Databases using the `catalog_id:name`. If you have not set a Catalog ID specify the AWS Account ID that the database is in. For example:
///
/// ```sh
/// $ pulumi import aws:glue/catalogDatabase:CatalogDatabase database 123456789012:my_database
/// ```
pub mod catalog_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogDatabaseArgs {
        /// ID of the Glue Catalog to create the database in. If omitted, this defaults to the AWS Account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a set of default permissions on the table for principals. See `create_table_default_permission` below.
        #[builder(into, default)]
        pub create_table_default_permissions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::glue::CatalogDatabaseCreateTableDefaultPermission,
                >,
            >,
        >,
        /// Description of the database.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block that references an entity outside the AWS Glue Data Catalog. See `federated_database` below.
        #[builder(into, default)]
        pub federated_database: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CatalogDatabaseFederatedDatabase>,
        >,
        /// Location of the database (for example, an HDFS path).
        #[builder(into, default)]
        pub location_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the database. The acceptable characters are lowercase letters, numbers, and the underscore character.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// List of key-value pairs that define parameters and properties of the database.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for a target database for resource linking. See `target_database` below.
        #[builder(into, default)]
        pub target_database: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CatalogDatabaseTargetDatabase>,
        >,
    }
    #[allow(dead_code)]
    pub struct CatalogDatabaseResult {
        /// ARN of the Glue Catalog Database.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ID of the Glue Catalog to create the database in. If omitted, this defaults to the AWS Account ID.
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// Creates a set of default permissions on the table for principals. See `create_table_default_permission` below.
        pub create_table_default_permissions: pulumi_wasm_rust::Output<
            Vec<super::super::types::glue::CatalogDatabaseCreateTableDefaultPermission>,
        >,
        /// Description of the database.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block that references an entity outside the AWS Glue Data Catalog. See `federated_database` below.
        pub federated_database: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CatalogDatabaseFederatedDatabase>,
        >,
        /// Location of the database (for example, an HDFS path).
        pub location_uri: pulumi_wasm_rust::Output<String>,
        /// Name of the database. The acceptable characters are lowercase letters, numbers, and the underscore character.
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of key-value pairs that define parameters and properties of the database.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for a target database for resource linking. See `target_database` below.
        pub target_database: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CatalogDatabaseTargetDatabase>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CatalogDatabaseArgs) -> CatalogDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let create_table_default_permissions_binding = args
            .create_table_default_permissions
            .get_inner();
        let description_binding = args.description.get_inner();
        let federated_database_binding = args.federated_database.get_inner();
        let location_uri_binding = args.location_uri.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_database_binding = args.target_database.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/catalogDatabase:CatalogDatabase".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "createTableDefaultPermissions".into(),
                    value: &create_table_default_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "federatedDatabase".into(),
                    value: &federated_database_binding,
                },
                register_interface::ObjectField {
                    name: "locationUri".into(),
                    value: &location_uri_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetDatabase".into(),
                    value: &target_database_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "createTableDefaultPermissions".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "federatedDatabase".into(),
                },
                register_interface::ResultField {
                    name: "locationUri".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetDatabase".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CatalogDatabaseResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            create_table_default_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTableDefaultPermissions").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            federated_database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("federatedDatabase").unwrap(),
            ),
            location_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationUri").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetDatabase").unwrap(),
            ),
        }
    }
}