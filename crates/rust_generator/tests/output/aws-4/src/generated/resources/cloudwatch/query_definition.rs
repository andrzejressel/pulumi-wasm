/// Provides a CloudWatch Logs query definition resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = query_definition::create(
///         "example",
///         QueryDefinitionArgs::builder()
///             .log_group_names(vec!["/aws/logGroup1", "/aws/logGroup2",])
///             .name("custom_query")
///             .query_string(
///                 "fields @timestamp, @message\n| sort @timestamp desc\n| limit 25",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch query definitions using the query definition ARN. The ARN can be found on the "Edit Query" page for the query in the AWS Console. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/queryDefinition:QueryDefinition example arn:aws:logs:us-west-2:123456789012:query-definition:269951d7-6f75-496d-9d7b-6b7a5486bdbd
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod query_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueryDefinitionArgs {
        /// Specific log groups to use with the query.
        #[builder(into, default)]
        pub log_group_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the query.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The query to save. You can read more about CloudWatch Logs Query Syntax in the [documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html).
        #[builder(into)]
        pub query_string: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct QueryDefinitionResult {
        /// Specific log groups to use with the query.
        pub log_group_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the query.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The query definition ID.
        pub query_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The query to save. You can read more about CloudWatch Logs Query Syntax in the [documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html).
        pub query_string: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueryDefinitionArgs,
    ) -> QueryDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_group_names_binding = args.log_group_names.get_output(context);
        let name_binding = args.name.get_output(context);
        let query_string_binding = args.query_string.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/queryDefinition:QueryDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupNames".into(),
                    value: log_group_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryString".into(),
                    value: query_string_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        QueryDefinitionResult {
            log_group_names: o.get_field("logGroupNames"),
            name: o.get_field("name"),
            query_definition_id: o.get_field("queryDefinitionId"),
            query_string: o.get_field("queryString"),
        }
    }
}
