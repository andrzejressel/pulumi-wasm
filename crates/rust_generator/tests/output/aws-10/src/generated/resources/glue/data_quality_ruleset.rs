/// Provides a Glue Data Quality Ruleset Resource. You can refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/glue-data-quality.html) for a full explanation of the Glue Data Quality Ruleset functionality
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_quality_ruleset::create(
///         "example",
///         DataQualityRulesetArgs::builder()
///             .name("example")
///             .ruleset("Rules = [Completeness \"colA\" between 0.4 and 0.8]")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With description
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_quality_ruleset::create(
///         "example",
///         DataQualityRulesetArgs::builder()
///             .description("example")
///             .name("example")
///             .ruleset("Rules = [Completeness \"colA\" between 0.4 and 0.8]")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With tags
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:DataQualityRuleset
///     properties:
///       name: example
///       ruleset: Rules = [Completeness "colA" between 0.4 and 0.8]
///       tags:
///         hello: world
/// ```
///
/// ### With target_table
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_quality_ruleset::create(
///         "example",
///         DataQualityRulesetArgs::builder()
///             .name("example")
///             .ruleset("Rules = [Completeness \"colA\" between 0.4 and 0.8]")
///             .target_table(
///                 DataQualityRulesetTargetTable::builder()
///                     .databaseName("${exampleAwsGlueCatalogDatabase.name}")
///                     .tableName("${exampleAwsGlueCatalogTable.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Data Quality Ruleset using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/dataQualityRuleset:DataQualityRuleset example exampleName
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_quality_ruleset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataQualityRulesetArgs {
        /// Description of the data quality ruleset.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the data quality ruleset.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A Data Quality Definition Language (DQDL) ruleset. For more information, see the AWS Glue developer guide.
        #[builder(into)]
        pub ruleset: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A Configuration block specifying a target table associated with the data quality ruleset. See `target_table` below.
        #[builder(into, default)]
        pub target_table: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::DataQualityRulesetTargetTable>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataQualityRulesetResult {
        /// ARN of the Glue Data Quality Ruleset.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The time and date that this data quality ruleset was created.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// Description of the data quality ruleset.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time and date that this data quality ruleset was created.
        pub last_modified_on: pulumi_gestalt_rust::Output<String>,
        /// Name of the data quality ruleset.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// When a ruleset was created from a recommendation run, this run ID is generated to link the two together.
        pub recommendation_run_id: pulumi_gestalt_rust::Output<String>,
        /// A Data Quality Definition Language (DQDL) ruleset. For more information, see the AWS Glue developer guide.
        pub ruleset: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A Configuration block specifying a target table associated with the data quality ruleset. See `target_table` below.
        pub target_table: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::DataQualityRulesetTargetTable>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataQualityRulesetArgs,
    ) -> DataQualityRulesetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let ruleset_binding = args.ruleset.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_table_binding = args.target_table.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/dataQualityRuleset:DataQualityRuleset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleset".into(),
                    value: ruleset_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetTable".into(),
                    value: target_table_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataQualityRulesetResult {
            arn: o.get_field("arn"),
            created_on: o.get_field("createdOn"),
            description: o.get_field("description"),
            last_modified_on: o.get_field("lastModifiedOn"),
            name: o.get_field("name"),
            recommendation_run_id: o.get_field("recommendationRunId"),
            ruleset: o.get_field("ruleset"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_table: o.get_field("targetTable"),
        }
    }
}
