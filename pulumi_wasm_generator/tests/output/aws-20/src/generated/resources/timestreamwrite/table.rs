/// Provides a Timestream table resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = table::create(
///         "example",
///         TableArgs::builder()
///             .database_name("${exampleAwsTimestreamwriteDatabase.databaseName}")
///             .table_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Full usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:timestreamwrite:Table
///     properties:
///       databaseName: ${exampleAwsTimestreamwriteDatabase.databaseName}
///       tableName: example
///       retentionProperties:
///         magneticStoreRetentionPeriodInDays: 30
///         memoryStoreRetentionPeriodInHours: 8
///       tags:
///         Name: example-timestream-table
/// ```
///
/// ### Customer-defined Partition Key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:timestreamwrite:Table
///     properties:
///       databaseName: ${exampleAwsTimestreamwriteDatabase.databaseName}
///       tableName: example
///       schema:
///         compositePartitionKey:
///           enforcementInRecord: REQUIRED
///           name: attr1
///           type: DIMENSION
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Timestream tables using the `table_name` and `database_name` separate by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:timestreamwrite/table:Table example ExampleTable:ExampleDatabase
/// ```
pub mod table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// The name of the Timestream database.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Contains properties to set on the table when enabling magnetic store writes. See Magnetic Store Write Properties below for more details.
        #[builder(into, default)]
        pub magnetic_store_write_properties: pulumi_wasm_rust::Output<
            Option<
                super::super::types::timestreamwrite::TableMagneticStoreWriteProperties,
            >,
        >,
        /// The retention duration for the memory store and magnetic store. See Retention Properties below for more details. If not provided, `magnetic_store_retention_period_in_days` default to 73000 and `memory_store_retention_period_in_hours` defaults to 6.
        #[builder(into, default)]
        pub retention_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::timestreamwrite::TableRetentionProperties>,
        >,
        /// The schema of the table. See Schema below for more details.
        #[builder(into, default)]
        pub schema: pulumi_wasm_rust::Output<
            Option<super::super::types::timestreamwrite::TableSchema>,
        >,
        /// The name of the Timestream table.
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// The ARN that uniquely identifies this table.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Timestream database.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Contains properties to set on the table when enabling magnetic store writes. See Magnetic Store Write Properties below for more details.
        pub magnetic_store_write_properties: pulumi_wasm_rust::Output<
            super::super::types::timestreamwrite::TableMagneticStoreWriteProperties,
        >,
        /// The retention duration for the memory store and magnetic store. See Retention Properties below for more details. If not provided, `magnetic_store_retention_period_in_days` default to 73000 and `memory_store_retention_period_in_hours` defaults to 6.
        pub retention_properties: pulumi_wasm_rust::Output<
            super::super::types::timestreamwrite::TableRetentionProperties,
        >,
        /// The schema of the table. See Schema below for more details.
        pub schema: pulumi_wasm_rust::Output<
            super::super::types::timestreamwrite::TableSchema,
        >,
        /// The name of the Timestream table.
        pub table_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TableArgs) -> TableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_name_binding = args.database_name.get_inner();
        let magnetic_store_write_properties_binding = args
            .magnetic_store_write_properties
            .get_inner();
        let retention_properties_binding = args.retention_properties.get_inner();
        let schema_binding = args.schema.get_inner();
        let table_name_binding = args.table_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:timestreamwrite/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "magneticStoreWriteProperties".into(),
                    value: &magnetic_store_write_properties_binding,
                },
                register_interface::ObjectField {
                    name: "retentionProperties".into(),
                    value: &retention_properties_binding,
                },
                register_interface::ObjectField {
                    name: "schema".into(),
                    value: &schema_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "magneticStoreWriteProperties".into(),
                },
                register_interface::ResultField {
                    name: "retentionProperties".into(),
                },
                register_interface::ResultField {
                    name: "schema".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TableResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            magnetic_store_write_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("magneticStoreWriteProperties").unwrap(),
            ),
            retention_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionProperties").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
