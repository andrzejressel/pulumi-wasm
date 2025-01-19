/// Provides a AWS Clean Rooms configured table. Configured tables are used to represent references to existing tables in the AWS Glue Data Catalog.
///
/// ## Example Usage
///
/// ### Configured table with tags
///
/// ```yaml
/// resources:
///   testConfiguredTable:
///     type: aws:cleanrooms:ConfiguredTable
///     name: test_configured_table
///     properties:
///       name: pulumi-example-table
///       description: I made this table with Pulumi!
///       analysisMethod: DIRECT_QUERY
///       allowedColumns:
///         - column1
///         - column2
///         - column3
///       tableReference:
///         databaseName: example_database
///         tableName: example_table
///       tags:
///         Project: Pulumi
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_cleanrooms_configured_table` using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cleanrooms/configuredTable:ConfiguredTable table 1234abcd-12ab-34cd-56ef-1234567890ab
/// ```
pub mod configured_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfiguredTableArgs {
        /// The columns of the references table which will be included in the configured table.
        #[builder(into)]
        pub allowed_columns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The analysis method for the configured table. The only valid value is currently `DIRECT_QUERY`.
        #[builder(into)]
        pub analysis_method: pulumi_wasm_rust::Output<String>,
        /// A description for the configured table.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the configured table.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the AWS Glue table which will be used to create the configured table.
        /// * `table_reference.database_name` - (Required - Forces new resource) - The name of the AWS Glue database which contains the table.
        /// * `table_reference.table_name` - (Required - Forces new resource) - The name of the AWS Glue table which will be used to create the configured table.
        #[builder(into)]
        pub table_reference: pulumi_wasm_rust::Output<
            super::super::types::cleanrooms::ConfiguredTableTableReference,
        >,
        /// Key value pairs which tag the configured table.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfiguredTableResult {
        /// The columns of the references table which will be included in the configured table.
        pub allowed_columns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The analysis method for the configured table. The only valid value is currently `DIRECT_QUERY`.
        pub analysis_method: pulumi_wasm_rust::Output<String>,
        /// The ARN of the configured table.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date and time the configured table was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A description for the configured table.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the configured table.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A reference to the AWS Glue table which will be used to create the configured table.
        /// * `table_reference.database_name` - (Required - Forces new resource) - The name of the AWS Glue database which contains the table.
        /// * `table_reference.table_name` - (Required - Forces new resource) - The name of the AWS Glue table which will be used to create the configured table.
        pub table_reference: pulumi_wasm_rust::Output<
            super::super::types::cleanrooms::ConfiguredTableTableReference,
        >,
        /// Key value pairs which tag the configured table.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The date and time the configured table was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConfiguredTableArgs) -> ConfiguredTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_columns_binding = args.allowed_columns.get_inner();
        let analysis_method_binding = args.analysis_method.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let table_reference_binding = args.table_reference.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cleanrooms/configuredTable:ConfiguredTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedColumns".into(),
                    value: &allowed_columns_binding,
                },
                register_interface::ObjectField {
                    name: "analysisMethod".into(),
                    value: &analysis_method_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tableReference".into(),
                    value: &table_reference_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowedColumns".into(),
                },
                register_interface::ResultField {
                    name: "analysisMethod".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tableReference".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfiguredTableResult {
            allowed_columns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedColumns").unwrap(),
            ),
            analysis_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analysisMethod").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            table_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableReference").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
