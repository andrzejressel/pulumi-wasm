/// Provides a DynamoDB contributor insights resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod contributor_insights {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContributorInsightsArgs {
        /// The global secondary index name
        #[builder(into, default)]
        pub index_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the table to enable contributor insights
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ContributorInsightsResult {
        /// The global secondary index name
        pub index_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the table to enable contributor insights
        pub table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContributorInsightsArgs,
    ) -> ContributorInsightsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let index_name_binding = args.index_name.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dynamodb/contributorInsights:ContributorInsights".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexName".into(),
                    value: index_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: table_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContributorInsightsResult {
            index_name: o.get_field("indexName"),
            table_name: o.get_field("tableName"),
        }
    }
}
