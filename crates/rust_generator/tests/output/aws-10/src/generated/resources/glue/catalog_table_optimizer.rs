/// Resource for managing an AWS Glue Catalog Table Optimizer.
///
/// ## Example Usage
///
/// ### Compaction Optimizer
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod catalog_table_optimizer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogTableOptimizerArgs {
        /// The Catalog ID of the table.
        #[builder(into)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A configuration block that defines the table optimizer settings. See Configuration for additional details.
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CatalogTableOptimizerConfiguration>,
        >,
        /// The name of the database in the catalog in which the table resides.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the table.
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of table optimizer. Valid values are `compaction`, `retention`, and `orphan_file_deletion`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CatalogTableOptimizerResult {
        /// The Catalog ID of the table.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// A configuration block that defines the table optimizer settings. See Configuration for additional details.
        pub configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CatalogTableOptimizerConfiguration>,
        >,
        /// The name of the database in the catalog in which the table resides.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the table.
        pub table_name: pulumi_gestalt_rust::Output<String>,
        /// The type of table optimizer. Valid values are `compaction`, `retention`, and `orphan_file_deletion`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CatalogTableOptimizerArgs,
    ) -> CatalogTableOptimizerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding_1 = args.catalog_id.get_output(context);
        let catalog_id_binding = catalog_id_binding_1.get_inner();
        let configuration_binding_1 = args.configuration.get_output(context);
        let configuration_binding = configuration_binding_1.get_inner();
        let database_name_binding_1 = args.database_name.get_output(context);
        let database_name_binding = database_name_binding_1.get_inner();
        let table_name_binding_1 = args.table_name.get_output(context);
        let table_name_binding = table_name_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/catalogTableOptimizer:CatalogTableOptimizer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        CatalogTableOptimizerResult {
            catalog_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("catalogId"),
            ),
            configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configuration"),
            ),
            database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            table_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableName"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
