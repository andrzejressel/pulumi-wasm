/// Resource for managing an AWS Lake Formation Data Cells Filter.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_cells_filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCellsFilterArgs {
        /// Information about the data cells filter. See Table Data below for details.
        #[builder(into, default)]
        pub table_data: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::DataCellsFilterTableData>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::DataCellsFilterTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataCellsFilterResult {
        /// Information about the data cells filter. See Table Data below for details.
        pub table_data: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::DataCellsFilterTableData>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::DataCellsFilterTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataCellsFilterArgs,
    ) -> DataCellsFilterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let table_data_binding = args.table_data.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lakeformation/dataCellsFilter:DataCellsFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableData".into(),
                    value: &table_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataCellsFilterResult {
            table_data: o.get_field("tableData"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
