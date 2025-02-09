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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AnalysisArgs,
    ) -> AnalysisResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let analysis_id_binding_1 = args.analysis_id.get_output(context);
        let analysis_id_binding = analysis_id_binding_1.get_inner();
        let aws_account_id_binding_1 = args.aws_account_id.get_output(context);
        let aws_account_id_binding = aws_account_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let permissions_binding_1 = args.permissions.get_output(context);
        let permissions_binding = permissions_binding_1.get_inner();
        let recovery_window_in_days_binding_1 = args
            .recovery_window_in_days
            .get_output(context);
        let recovery_window_in_days_binding = recovery_window_in_days_binding_1
            .get_inner();
        let source_entity_binding_1 = args.source_entity.get_output(context);
        let source_entity_binding = source_entity_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let theme_arn_binding_1 = args.theme_arn.get_output(context);
        let theme_arn_binding = theme_arn_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/analysis:Analysis".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "analysisId".into(),
                    value: &analysis_id_binding,
                },
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryWindowInDays".into(),
                    value: &recovery_window_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "sourceEntity".into(),
                    value: &source_entity_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "themeArn".into(),
                    value: &theme_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AnalysisResult {
            analysis_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("analysisId"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            last_published_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastPublishedTime"),
            ),
            last_updated_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            recovery_window_in_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryWindowInDays"),
            ),
            source_entity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceEntity"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            theme_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("themeArn"),
            ),
        }
    }
}
