/// Provides a DynamoDB contributor insights resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = contributor_insights::create(
///         "test",
///         ContributorInsightsArgs::builder().table_name("ExampleTableName").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_dynamodb_contributor_insights` using the format `name:table_name/index:index_name`, followed by the account number. For example:
///
/// ```sh
/// $ pulumi import aws:dynamodb/contributorInsights:ContributorInsights test name:ExampleTableName/index:ExampleIndexName/123456789012
/// ```
pub mod contributor_insights {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContributorInsightsArgs {
        /// The global secondary index name
        #[builder(into, default)]
        pub index_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the table to enable contributor insights
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ContributorInsightsResult {
        /// The global secondary index name
        pub index_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the table to enable contributor insights
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ContributorInsightsArgs,
    ) -> ContributorInsightsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let index_name_binding = args.index_name.get_inner();
        let table_name_binding = args.table_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dynamodb/contributorInsights:ContributorInsights".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "indexName".into(),
                    value: &index_name_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "indexName".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContributorInsightsResult {
            index_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexName").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
        }
    }
}
