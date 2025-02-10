/// Provides a resource to manage an [Amazon Macie Findings Filter](https://docs.aws.amazon.com/macie/latest/APIReference/findingsfilters-id.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let test = findings_filter::create(
///         "test",
///         FindingsFilterArgs::builder()
///             .action("ARCHIVE")
///             .description("DESCRIPTION")
///             .finding_criteria(
///                 FindingsFilterFindingCriteria::builder()
///                     .criterions(
///                         vec![
///                             FindingsFilterFindingCriteriaCriterion::builder()
///                             .eqs(vec!["${current.name}",]).field("region")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("NAME OF THE FINDINGS FILTER")
///             .position(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_findings_filter` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:macie/findingsFilter:FindingsFilter example abcd1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod findings_filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FindingsFilterArgs {
        /// The action to perform on findings that meet the filter criteria (`finding_criteria`). Valid values are: `ARCHIVE`, suppress (automatically archive) the findings; and, `NOOP`, don't perform any action on the findings.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A custom description of the filter. The description can contain as many as 512 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The criteria to use to filter findings.
        #[builder(into)]
        pub finding_criteria: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::macie::FindingsFilterFindingCriteria,
        >,
        /// A custom name for the filter. The name must contain at least 3 characters and can contain as many as 64 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The position of the filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter is applied to findings, relative to other filters that are also applied to the findings.
        #[builder(into, default)]
        pub position: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A map of key-value pairs that specifies the tags to associate with the filter.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FindingsFilterResult {
        /// The action to perform on findings that meet the filter criteria (`finding_criteria`). Valid values are: `ARCHIVE`, suppress (automatically archive) the findings; and, `NOOP`, don't perform any action on the findings.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Findings Filter.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A custom description of the filter. The description can contain as many as 512 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The criteria to use to filter findings.
        pub finding_criteria: pulumi_gestalt_rust::Output<
            super::super::types::macie::FindingsFilterFindingCriteria,
        >,
        /// A custom name for the filter. The name must contain at least 3 characters and can contain as many as 64 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The position of the filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter is applied to findings, relative to other filters that are also applied to the findings.
        pub position: pulumi_gestalt_rust::Output<i32>,
        /// A map of key-value pairs that specifies the tags to associate with the filter.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FindingsFilterArgs,
    ) -> FindingsFilterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let finding_criteria_binding = args.finding_criteria.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let position_binding = args.position.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:macie/findingsFilter:FindingsFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "findingCriteria".into(),
                    value: finding_criteria_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "position".into(),
                    value: position_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FindingsFilterResult {
            action: o.get_field("action"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            finding_criteria: o.get_field("findingCriteria"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            position: o.get_field("position"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
