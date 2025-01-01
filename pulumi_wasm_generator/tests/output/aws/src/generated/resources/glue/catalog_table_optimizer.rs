/// Resource for managing an AWS Glue Catalog Table Optimizer.
///
/// ## Example Usage
///
/// ### Compaction Optimizer
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_table_optimizer::create(
///         "example",
///         CatalogTableOptimizerArgs::builder()
///             .catalog_id("123456789012")
///             .configuration(
///                 CatalogTableOptimizerConfiguration::builder()
///                     .enabled(true)
///                     .roleArn("arn:aws:iam::123456789012:role/example-role")
///                     .build_struct(),
///             )
///             .database_name("example_database")
///             .table_name("example_table")
///             .type_("compaction")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Snapshot Retention Optimizer
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_table_optimizer::create(
///         "example",
///         CatalogTableOptimizerArgs::builder()
///             .catalog_id("123456789012")
///             .configuration(
///                 CatalogTableOptimizerConfiguration::builder()
///                     .enabled(true)
///                     .retentionConfiguration(
///                         CatalogTableOptimizerConfigurationRetentionConfiguration::builder()
///                             .icebergConfiguration(
///                                 CatalogTableOptimizerConfigurationRetentionConfigurationIcebergConfiguration::builder()
///                                     .cleanExpiredFiles(true)
///                                     .numberOfSnapshotsToRetain(3)
///                                     .snapshotRetentionPeriodInDays(7)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .roleArn("arn:aws:iam::123456789012:role/example-role")
///                     .build_struct(),
///             )
///             .database_name("example_database")
///             .table_name("example_table")
///             .type_("retention")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Orphan File Deletion Optimizer
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_table_optimizer::create(
///         "example",
///         CatalogTableOptimizerArgs::builder()
///             .catalog_id("123456789012")
///             .configuration(
///                 CatalogTableOptimizerConfiguration::builder()
///                     .enabled(true)
///                     .orphanFileDeletionConfiguration(
///                         CatalogTableOptimizerConfigurationOrphanFileDeletionConfiguration::builder()
///                             .icebergConfiguration(
///                                 CatalogTableOptimizerConfigurationOrphanFileDeletionConfigurationIcebergConfiguration::builder()
///                                     .location("s3://example-bucket/example_table/")
///                                     .orphanFileRetentionPeriodInDays(7)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .roleArn("arn:aws:iam::123456789012:role/example-role")
///                     .build_struct(),
///             )
///             .database_name("example_database")
///             .table_name("example_table")
///             .type_("orphan_file_deletion")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Catalog Table Optimizer using the `catalog_id,database_name,table_name,type`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/catalogTableOptimizer:CatalogTableOptimizer example 123456789012,example_database,example_table,compaction
/// ```
pub mod catalog_table_optimizer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogTableOptimizerArgs {
        /// The Catalog ID of the table.
        #[builder(into)]
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// A configuration block that defines the table optimizer settings. See Configuration for additional details.
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CatalogTableOptimizerConfiguration>,
        >,
        /// The name of the database in the catalog in which the table resides.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The name of the table.
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::Output<String>,
        /// The type of table optimizer. Valid values are `compaction`, `retention`, and `orphan_file_deletion`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CatalogTableOptimizerResult {
        /// The Catalog ID of the table.
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// A configuration block that defines the table optimizer settings. See Configuration for additional details.
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CatalogTableOptimizerConfiguration>,
        >,
        /// The name of the database in the catalog in which the table resides.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The name of the table.
        pub table_name: pulumi_wasm_rust::Output<String>,
        /// The type of table optimizer. Valid values are `compaction`, `retention`, and `orphan_file_deletion`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: CatalogTableOptimizerArgs,
    ) -> CatalogTableOptimizerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let configuration_binding = args.configuration.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let table_name_binding = args.table_name.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/catalogTableOptimizer:CatalogTableOptimizer".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CatalogTableOptimizerResult {
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
