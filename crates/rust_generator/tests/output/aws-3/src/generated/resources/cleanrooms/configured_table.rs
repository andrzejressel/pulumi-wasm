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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configured_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfiguredTableArgs {
        /// The columns of the references table which will be included in the configured table.
        #[builder(into)]
        pub allowed_columns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The analysis method for the configured table. The only valid value is currently `DIRECT_QUERY`.
        #[builder(into)]
        pub analysis_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description for the configured table.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the configured table.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the AWS Glue table which will be used to create the configured table.
        /// * `table_reference.database_name` - (Required - Forces new resource) - The name of the AWS Glue database which contains the table.
        /// * `table_reference.table_name` - (Required - Forces new resource) - The name of the AWS Glue table which will be used to create the configured table.
        #[builder(into)]
        pub table_reference: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cleanrooms::ConfiguredTableTableReference,
        >,
        /// Key value pairs which tag the configured table.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfiguredTableResult {
        /// The columns of the references table which will be included in the configured table.
        pub allowed_columns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The analysis method for the configured table. The only valid value is currently `DIRECT_QUERY`.
        pub analysis_method: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the configured table.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date and time the configured table was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A description for the configured table.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the configured table.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A reference to the AWS Glue table which will be used to create the configured table.
        /// * `table_reference.database_name` - (Required - Forces new resource) - The name of the AWS Glue database which contains the table.
        /// * `table_reference.table_name` - (Required - Forces new resource) - The name of the AWS Glue table which will be used to create the configured table.
        pub table_reference: pulumi_gestalt_rust::Output<
            super::super::types::cleanrooms::ConfiguredTableTableReference,
        >,
        /// Key value pairs which tag the configured table.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The date and time the configured table was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfiguredTableArgs,
    ) -> ConfiguredTableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allowed_columns_binding = args.allowed_columns.get_output(context);
        let analysis_method_binding = args.analysis_method.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let table_reference_binding = args.table_reference.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cleanrooms/configuredTable:ConfiguredTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedColumns".into(),
                    value: allowed_columns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analysisMethod".into(),
                    value: analysis_method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableReference".into(),
                    value: table_reference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfiguredTableResult {
            allowed_columns: o.get_field("allowedColumns"),
            analysis_method: o.get_field("analysisMethod"),
            arn: o.get_field("arn"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            table_reference: o.get_field("tableReference"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_time: o.get_field("updateTime"),
        }
    }
}
