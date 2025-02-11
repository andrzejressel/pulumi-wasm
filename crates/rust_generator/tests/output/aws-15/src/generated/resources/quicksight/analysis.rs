/// Resource for managing a QuickSight Analysis.
///
/// ## Example Usage
///
/// ### From Source Template
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = analysis::create(
///         "example",
///         AnalysisArgs::builder()
///             .analysis_id("example-id")
///             .name("example-name")
///             .source_entity(
///                 AnalysisSourceEntity::builder()
///                     .sourceTemplate(
///                         AnalysisSourceEntitySourceTemplate::builder()
///                             .arn("${source.arn}")
///                             .dataSetReferences(
///                                 vec![
///                                     AnalysisSourceEntitySourceTemplateDataSetReference::builder()
///                                     .dataSetArn("${dataset.arn}").dataSetPlaceholder("1")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Definition
///
/// ```yaml
/// resources:
///   example:
///     type: aws:quicksight:Analysis
///     properties:
///       analysisId: example-id
///       name: example-name
///       definition:
///         dataSetIdentifiersDeclarations:
///           - dataSetArn: ${dataset.arn}
///             identifier: '1'
///         sheets:
///           - title: Example
///             sheetId: Example1
///             visuals:
///               - lineChartVisual:
///                   visualId: LineChart
///                   title:
///                     formatText:
///                       plainText: Line Chart Example
///                   chartConfiguration:
///                     fieldWells:
///                       lineChartAggregatedFieldWells:
///                         categories:
///                           - categoricalDimensionField:
///                               fieldId: '1'
///                               column:
///                                 dataSetIdentifier: '1'
///                                 columnName: Column1
///                         values:
///                           - categoricalMeasureField:
///                               fieldId: '2'
///                               column:
///                                 dataSetIdentifier: '1'
///                                 columnName: Column1
///                               aggregationFunction: COUNT
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight Analysis using the AWS account ID and analysis ID separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/analysis:Analysis example 123456789012,example-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod analysis {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnalysisArgs {
        /// Identifier for the analysis.
        #[builder(into)]
        pub analysis_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display name for the analysis.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parameters for the creation of the analysis, which you want to use to override the default settings. An analysis can have any type of parameters, and some parameters might accept multiple values. See parameters.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::AnalysisParameters>,
        >,
        /// A set of resource permissions on the analysis. Maximum of 64 items. See permissions.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::quicksight::AnalysisPermission>>,
        >,
        /// A value that specifies the number of days that Amazon QuickSight waits before it deletes the analysis. Use `0` to force deletion without recovery. Minimum value of `7`. Maximum value of `30`. Default to `30`.
        #[builder(into, default)]
        pub recovery_window_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The entity that you are using as a source when you create the analysis (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
        #[builder(into, default)]
        pub source_entity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::AnalysisSourceEntity>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon Resource Name (ARN) of the theme that is being used for this analysis. The theme ARN must exist in the same AWS account where you create the analysis.
        #[builder(into, default)]
        pub theme_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AnalysisResult {
        /// Identifier for the analysis.
        pub analysis_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the analysis.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The time that the analysis was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        pub last_published_time: pulumi_gestalt_rust::Output<String>,
        /// The time that the analysis was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Display name for the analysis.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parameters for the creation of the analysis, which you want to use to override the default settings. An analysis can have any type of parameters, and some parameters might accept multiple values. See parameters.
        pub parameters: pulumi_gestalt_rust::Output<
            super::super::types::quicksight::AnalysisParameters,
        >,
        /// A set of resource permissions on the analysis. Maximum of 64 items. See permissions.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::quicksight::AnalysisPermission>>,
        >,
        /// A value that specifies the number of days that Amazon QuickSight waits before it deletes the analysis. Use `0` to force deletion without recovery. Minimum value of `7`. Maximum value of `30`. Default to `30`.
        pub recovery_window_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The entity that you are using as a source when you create the analysis (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
        pub source_entity: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::AnalysisSourceEntity>,
        >,
        /// The analysis creation status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) of the theme that is being used for this analysis. The theme ARN must exist in the same AWS account where you create the analysis.
        pub theme_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AnalysisArgs,
    ) -> AnalysisResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let analysis_id_binding = args.analysis_id.get_output(context);
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let permissions_binding = args.permissions.get_output(context);
        let recovery_window_in_days_binding = args
            .recovery_window_in_days
            .get_output(context);
        let source_entity_binding = args.source_entity.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let theme_arn_binding = args.theme_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/analysis:Analysis".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analysisId".into(),
                    value: &analysis_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryWindowInDays".into(),
                    value: &recovery_window_in_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceEntity".into(),
                    value: &source_entity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "themeArn".into(),
                    value: &theme_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AnalysisResult {
            analysis_id: o.get_field("analysisId"),
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            created_time: o.get_field("createdTime"),
            last_published_time: o.get_field("lastPublishedTime"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            permissions: o.get_field("permissions"),
            recovery_window_in_days: o.get_field("recoveryWindowInDays"),
            source_entity: o.get_field("sourceEntity"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            theme_arn: o.get_field("themeArn"),
        }
    }
}
