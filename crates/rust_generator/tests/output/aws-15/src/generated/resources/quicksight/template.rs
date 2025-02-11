/// Resource for managing a QuickSight Template.
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
///     let example = template::create(
///         "example",
///         TemplateArgs::builder()
///             .name("example-name")
///             .source_entity(
///                 TemplateSourceEntity::builder()
///                     .sourceTemplate(
///                         TemplateSourceEntitySourceTemplate::builder()
///                             .arn("${source.arn}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .template_id("example-id")
///             .version_description("version")
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
///     type: aws:quicksight:Template
///     properties:
///       templateId: example-id
///       name: example-name
///       versionDescription: version
///       definition:
///         dataSetConfigurations:
///           - dataSetSchema:
///               columnSchemaLists:
///                 - name: Column1
///                   dataType: STRING
///                 - name: Column2
///                   dataType: INTEGER
///             placeholder: '1'
///         sheets:
///           - title: Test
///             sheetId: Test1
///             visuals:
///               - barChartVisual:
///                   visualId: BarChart
///                   chartConfiguration:
///                     fieldWells:
///                       barChartAggregatedFieldWells:
///                         categories:
///                           - categoricalDimensionField:
///                               fieldId: '1'
///                               column:
///                                 columnName: Column1
///                                 dataSetIdentifier: '1'
///                         values:
///                           - numericalMeasureField:
///                               fieldId: '2'
///                               column:
///                                 columnName: Column2
///                                 dataSetIdentifier: '1'
///                               aggregationFunction:
///                                 simpleNumericalAggregation: SUM
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight Template using the AWS account ID and template ID separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/template:Template example 123456789012,example-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display name for the template.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of resource permissions on the template. Maximum of 64 items. See permissions.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::quicksight::TemplatePermission>>,
        >,
        /// The entity that you are using as a source when you create the template (analysis or template). Only one of `definition` or `source_entity` should be configured. See source_entity.
        #[builder(into, default)]
        pub source_entity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::TemplateSourceEntity>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier for the template.
        #[builder(into)]
        pub template_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of the current template version being created/updated.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub version_description: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TemplateResult {
        /// ARN of the template.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The time that the template was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The time that the template was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Display name for the template.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A set of resource permissions on the template. Maximum of 64 items. See permissions.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::quicksight::TemplatePermission>>,
        >,
        /// The entity that you are using as a source when you create the template (analysis or template). Only one of `definition` or `source_entity` should be configured. See source_entity.
        pub source_entity: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::TemplateSourceEntity>,
        >,
        /// Amazon Resource Name (ARN) of an analysis or template that was used to create this template.
        pub source_entity_arn: pulumi_gestalt_rust::Output<String>,
        /// The template creation status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier for the template.
        pub template_id: pulumi_gestalt_rust::Output<String>,
        /// A description of the current template version being created/updated.
        ///
        /// The following arguments are optional:
        pub version_description: pulumi_gestalt_rust::Output<String>,
        /// The version number of the template version.
        pub version_number: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TemplateArgs,
    ) -> TemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let permissions_binding = args.permissions.get_output(context);
        let source_entity_binding = args.source_entity.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_id_binding = args.template_id.get_output(context);
        let version_description_binding = args.version_description.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/template:Template".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding.drop_type(),
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
                    name: "templateId".into(),
                    value: &template_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionDescription".into(),
                    value: &version_description_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TemplateResult {
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            created_time: o.get_field("createdTime"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            source_entity: o.get_field("sourceEntity"),
            source_entity_arn: o.get_field("sourceEntityArn"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            template_id: o.get_field("templateId"),
            version_description: o.get_field("versionDescription"),
            version_number: o.get_field("versionNumber"),
        }
    }
}
