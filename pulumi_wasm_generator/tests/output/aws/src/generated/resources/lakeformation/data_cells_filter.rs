/// Resource for managing an AWS Lake Formation Data Cells Filter.
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
///     let example = data_cells_filter::create(
///         "example",
///         DataCellsFilterArgs::builder()
///             .table_data(
///                 DataCellsFilterTableData::builder()
///                     .columnNames(vec!["my_column",])
///                     .databaseName("${test.name}")
///                     .name("example")
///                     .rowFilter(
///                         DataCellsFilterTableDataRowFilter::builder()
///                             .filterExpression("my_column='example'")
///                             .build_struct(),
///                     )
///                     .tableCatalogId("${current.accountId}")
///                     .tableName("${testAwsGlueCatalogTable.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lake Formation Data Cells Filter using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:lakeformation/dataCellsFilter:DataCellsFilter example database_name,name,table_catalog_id,table_name
/// ```
pub mod data_cells_filter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCellsFilterArgs {
        /// Information about the data cells filter. See Table Data below for details.
        #[builder(into, default)]
        pub table_data: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::DataCellsFilterTableData>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::DataCellsFilterTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataCellsFilterResult {
        /// Information about the data cells filter. See Table Data below for details.
        pub table_data: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::DataCellsFilterTableData>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lakeformation::DataCellsFilterTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataCellsFilterArgs) -> DataCellsFilterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let table_data_binding = args.table_data.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lakeformation/dataCellsFilter:DataCellsFilter".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tableData".into(),
                    value: &table_data_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "tableData".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataCellsFilterResult {
            table_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableData").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}