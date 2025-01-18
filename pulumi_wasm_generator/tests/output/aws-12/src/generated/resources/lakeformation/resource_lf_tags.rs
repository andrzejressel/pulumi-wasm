/// Manages an attachment between one or more existing LF-tags and an existing Lake Formation resource.
///
/// ## Example Usage
///
/// ### Database Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = lf_tag::create(
///         "example",
///         LfTagArgs::builder()
///             .key("right")
///             .values(
///                 vec![
///                     "abbey", "village", "luffield", "woodcote", "copse", "chapel",
///                     "stowe", "club",
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleResourceLfTags = resource_lf_tags::create(
///         "exampleResourceLfTags",
///         ResourceLfTagsArgs::builder()
///             .database(
///                 ResourceLfTagsDatabase::builder()
///                     .name("${exampleAwsGlueCatalogDatabase.name}")
///                     .build_struct(),
///             )
///             .lf_tags(
///                 vec![
///                     ResourceLfTagsLfTag::builder().key("${example.key}").value("stowe")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Multiple Tags Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = lf_tag::create(
///         "example",
///         LfTagArgs::builder()
///             .key("right")
///             .values(
///                 vec![
///                     "abbey", "village", "luffield", "woodcote", "copse", "chapel",
///                     "stowe", "club",
///                 ],
///             )
///             .build_struct(),
///     );
///     let example2 = lf_tag::create(
///         "example2",
///         LfTagArgs::builder()
///             .key("left")
///             .values(
///                 vec![
///                     "farm", "theloop", "aintree", "brooklands", "maggotts", "becketts",
///                     "vale",
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleResourceLfTags = resource_lf_tags::create(
///         "exampleResourceLfTags",
///         ResourceLfTagsArgs::builder()
///             .database(
///                 ResourceLfTagsDatabase::builder()
///                     .name("${exampleAwsGlueCatalogDatabase.name}")
///                     .build_struct(),
///             )
///             .lf_tags(
///                 vec![
///                     ResourceLfTagsLfTag::builder().key("right").value("luffield")
///                     .build_struct(), ResourceLfTagsLfTag::builder().key("left")
///                     .value("aintree").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
pub mod resource_lf_tags {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceLfTagsArgs {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for a database resource. See below.
        #[builder(into, default)]
        pub database: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagsDatabase>,
        >,
        /// Set of LF-tags to attach to the resource. See below.
        ///
        /// Exactly one of the following is required:
        #[builder(into)]
        pub lf_tags: pulumi_wasm_rust::Output<
            Vec<super::super::types::lakeformation::ResourceLfTagsLfTag>,
        >,
        /// Configuration block for a table resource. See below.
        #[builder(into, default)]
        pub table: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagsTable>,
        >,
        /// Configuration block for a table with columns resource. See below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub table_with_columns: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagsTableWithColumns>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceLfTagsResult {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// Configuration block for a database resource. See below.
        pub database: pulumi_wasm_rust::Output<
            super::super::types::lakeformation::ResourceLfTagsDatabase,
        >,
        /// Set of LF-tags to attach to the resource. See below.
        ///
        /// Exactly one of the following is required:
        pub lf_tags: pulumi_wasm_rust::Output<
            Vec<super::super::types::lakeformation::ResourceLfTagsLfTag>,
        >,
        /// Configuration block for a table resource. See below.
        pub table: pulumi_wasm_rust::Output<
            super::super::types::lakeformation::ResourceLfTagsTable,
        >,
        /// Configuration block for a table with columns resource. See below.
        ///
        /// The following arguments are optional:
        pub table_with_columns: pulumi_wasm_rust::Output<
            super::super::types::lakeformation::ResourceLfTagsTableWithColumns,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceLfTagsArgs) -> ResourceLfTagsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let database_binding = args.database.get_inner();
        let lf_tags_binding = args.lf_tags.get_inner();
        let table_binding = args.table.get_inner();
        let table_with_columns_binding = args.table_with_columns.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lakeformation/resourceLfTags:ResourceLfTags".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "lfTags".into(),
                    value: &lf_tags_binding,
                },
                register_interface::ObjectField {
                    name: "table".into(),
                    value: &table_binding,
                },
                register_interface::ObjectField {
                    name: "tableWithColumns".into(),
                    value: &table_with_columns_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "lfTags".into(),
                },
                register_interface::ResultField {
                    name: "table".into(),
                },
                register_interface::ResultField {
                    name: "tableWithColumns".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceLfTagsResult {
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            lf_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lfTags").unwrap(),
            ),
            table: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("table").unwrap(),
            ),
            table_with_columns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableWithColumns").unwrap(),
            ),
        }
    }
}
