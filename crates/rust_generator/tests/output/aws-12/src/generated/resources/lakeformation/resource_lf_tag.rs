/// Resource for managing an AWS Lake Formation Resource LF Tag.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_lf_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceLfTagArgs {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for a database resource. See Database for more details.
        #[builder(into, default)]
        pub database: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagDatabase>,
        >,
        /// Set of LF-tags to attach to the resource. See LF Tag for more details.
        ///
        /// Exactly one of the following is required:
        #[builder(into, default)]
        pub lf_tag: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagLfTag>,
        >,
        /// Configuration block for a table resource. See Table for more details.
        #[builder(into, default)]
        pub table: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagTable>,
        >,
        /// Configuration block for a table with columns resource. See Table With Columns for more details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub table_with_columns: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagTableWithColumns>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::ResourceLfTagTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceLfTagResult {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        pub catalog_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for a database resource. See Database for more details.
        pub database: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagDatabase>,
        >,
        /// Set of LF-tags to attach to the resource. See LF Tag for more details.
        ///
        /// Exactly one of the following is required:
        pub lf_tag: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagLfTag>,
        >,
        /// Configuration block for a table resource. See Table for more details.
        pub table: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagTable>,
        >,
        /// Configuration block for a table with columns resource. See Table With Columns for more details.
        ///
        /// The following arguments are optional:
        pub table_with_columns: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagTableWithColumns>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::ResourceLfTagTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceLfTagArgs,
    ) -> ResourceLfTagResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let database_binding = args.database.get_output(context);
        let lf_tag_binding = args.lf_tag.get_output(context);
        let table_binding = args.table.get_output(context);
        let table_with_columns_binding = args.table_with_columns.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lakeformation/resourceLfTag:ResourceLfTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "database".into(),
                    value: &database_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lfTag".into(),
                    value: &lf_tag_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "table".into(),
                    value: &table_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableWithColumns".into(),
                    value: &table_with_columns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceLfTagResult {
            catalog_id: o.get_field("catalogId"),
            database: o.get_field("database"),
            lf_tag: o.get_field("lfTag"),
            table: o.get_field("table"),
            table_with_columns: o.get_field("tableWithColumns"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
