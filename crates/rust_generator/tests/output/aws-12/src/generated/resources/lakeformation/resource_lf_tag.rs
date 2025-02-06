/// Resource for managing an AWS Lake Formation Resource LF Tag.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_lf_tag::create(
///         "example",
///         ResourceLfTagArgs::builder()
///             .database(
///                 ResourceLfTagDatabase::builder()
///                     .name("${exampleAwsGlueCatalogDatabase.name}")
///                     .build_struct(),
///             )
///             .lf_tag(
///                 ResourceLfTagLfTag::builder()
///                     .key("${exampleAwsLakeformationLfTag.key}")
///                     .value("stowe")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import this resource.
///
pub mod resource_lf_tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceLfTagArgs {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block for a database resource. See Database for more details.
        #[builder(into, default)]
        pub database: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagDatabase>,
        >,
        /// Set of LF-tags to attach to the resource. See LF Tag for more details.
        ///
        /// Exactly one of the following is required:
        #[builder(into, default)]
        pub lf_tag: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagLfTag>,
        >,
        /// Configuration block for a table resource. See Table for more details.
        #[builder(into, default)]
        pub table: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagTable>,
        >,
        /// Configuration block for a table with columns resource. See Table With Columns for more details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub table_with_columns: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagTableWithColumns>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceLfTagResult {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for a database resource. See Database for more details.
        pub database: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagDatabase>,
        >,
        /// Set of LF-tags to attach to the resource. See LF Tag for more details.
        ///
        /// Exactly one of the following is required:
        pub lf_tag: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagLfTag>,
        >,
        /// Configuration block for a table resource. See Table for more details.
        pub table: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagTable>,
        >,
        /// Configuration block for a table with columns resource. See Table With Columns for more details.
        ///
        /// The following arguments are optional:
        pub table_with_columns: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagTableWithColumns>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResourceLfTagArgs,
    ) -> ResourceLfTagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_output(context).get_inner();
        let database_binding = args.database.get_output(context).get_inner();
        let lf_tag_binding = args.lf_tag.get_output(context).get_inner();
        let table_binding = args.table.get_output(context).get_inner();
        let table_with_columns_binding = args
            .table_with_columns
            .get_output(context)
            .get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lakeformation/resourceLfTag:ResourceLfTag".into(),
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
                    name: "lfTag".into(),
                    value: &lf_tag_binding,
                },
                register_interface::ObjectField {
                    name: "table".into(),
                    value: &table_binding,
                },
                register_interface::ObjectField {
                    name: "tableWithColumns".into(),
                    value: &table_with_columns_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceLfTagResult {
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("catalogId"),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            lf_tag: pulumi_wasm_rust::__private::into_domain(o.extract_field("lfTag")),
            table: pulumi_wasm_rust::__private::into_domain(o.extract_field("table")),
            table_with_columns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tableWithColumns"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
