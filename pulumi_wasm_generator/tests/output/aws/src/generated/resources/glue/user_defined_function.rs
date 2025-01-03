/// Provides a Glue User Defined Function Resource.
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
///         CatalogDatabaseArgs::builder().name("my_database").build_struct(),
///     );
///     let exampleUserDefinedFunction = user_defined_function::create(
///         "exampleUserDefinedFunction",
///         UserDefinedFunctionArgs::builder()
///             .catalog_id("${example.catalogId}")
///             .class_name("class")
///             .database_name("${example.name}")
///             .name("my_func")
///             .owner_name("owner")
///             .owner_type("GROUP")
///             .resource_uris(
///                 vec![
///                     UserDefinedFunctionResourceUri::builder().resourceType("ARCHIVE")
///                     .uri("uri").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue User Defined Functions using the `catalog_id:database_name:function_name`. If you have not set a Catalog ID specify the AWS Account ID that the database is in. For example:
///
/// ```sh
/// $ pulumi import aws:glue/userDefinedFunction:UserDefinedFunction func 123456789012:my_database:my_func
/// ```
pub mod user_defined_function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserDefinedFunctionArgs {
        /// ID of the Glue Catalog to create the function in. If omitted, this defaults to the AWS Account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Java class that contains the function code.
        #[builder(into)]
        pub class_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Database to create the Function.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The name of the function.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The owner of the function.
        #[builder(into)]
        pub owner_name: pulumi_wasm_rust::Output<String>,
        /// The owner type. can be one of `USER`, `ROLE`, and `GROUP`.
        #[builder(into)]
        pub owner_type: pulumi_wasm_rust::Output<String>,
        /// The configuration block for Resource URIs. See resource uris below for more details.
        #[builder(into, default)]
        pub resource_uris: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::UserDefinedFunctionResourceUri>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserDefinedFunctionResult {
        /// The ARN of the Glue User Defined Function.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ID of the Glue Catalog to create the function in. If omitted, this defaults to the AWS Account ID.
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Java class that contains the function code.
        pub class_name: pulumi_wasm_rust::Output<String>,
        /// The time at which the function was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The name of the Database to create the Function.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The name of the function.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The owner of the function.
        pub owner_name: pulumi_wasm_rust::Output<String>,
        /// The owner type. can be one of `USER`, `ROLE`, and `GROUP`.
        pub owner_type: pulumi_wasm_rust::Output<String>,
        /// The configuration block for Resource URIs. See resource uris below for more details.
        pub resource_uris: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::UserDefinedFunctionResourceUri>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: UserDefinedFunctionArgs,
    ) -> UserDefinedFunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let class_name_binding = args.class_name.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let name_binding = args.name.get_inner();
        let owner_name_binding = args.owner_name.get_inner();
        let owner_type_binding = args.owner_type.get_inner();
        let resource_uris_binding = args.resource_uris.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/userDefinedFunction:UserDefinedFunction".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "className".into(),
                    value: &class_name_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ownerName".into(),
                    value: &owner_name_binding,
                },
                register_interface::ObjectField {
                    name: "ownerType".into(),
                    value: &owner_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceUris".into(),
                    value: &resource_uris_binding,
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
                    name: "className".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerName".into(),
                },
                register_interface::ResultField {
                    name: "ownerType".into(),
                },
                register_interface::ResultField {
                    name: "resourceUris".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserDefinedFunctionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            class_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("className").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerName").unwrap(),
            ),
            owner_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerType").unwrap(),
            ),
            resource_uris: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceUris").unwrap(),
            ),
        }
    }
}
