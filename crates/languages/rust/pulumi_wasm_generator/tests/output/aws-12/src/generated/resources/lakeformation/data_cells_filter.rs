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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCellsFilterArgs {
        /// Information about the data cells filter. See Table Data below for details.
        #[builder(into, default)]
        pub table_data: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lakeformation::DataCellsFilterTableData>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DataCellsFilterArgs,
    ) -> DataCellsFilterResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let table_data_binding = args.table_data.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lakeformation/dataCellsFilter:DataCellsFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataCellsFilterResult {
            table_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tableData"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
