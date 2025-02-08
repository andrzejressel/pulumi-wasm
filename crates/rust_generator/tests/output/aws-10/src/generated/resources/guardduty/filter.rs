/// Provides a resource to manage a GuardDuty filter.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myFilter = filter::create(
///         "myFilter",
///         FilterArgs::builder()
///             .action("ARCHIVE")
///             .detector_id("${example.id}")
///             .finding_criteria(
///                 FilterFindingCriteria::builder()
///                     .criterions(
///                         vec![
///                             FilterFindingCriteriaCriterion::builder()
///                             .equals(vec!["eu-west-1",]).field("region").build_struct(),
///                             FilterFindingCriteriaCriterion::builder()
///                             .field("service.additionalInfo.threatListName")
///                             .notEquals(vec!["some-threat", "another-threat",])
///                             .build_struct(), FilterFindingCriteriaCriterion::builder()
///                             .field("updatedAt").greaterThan("2020-01-01T00:00:00Z")
///                             .lessThan("2020-02-01T00:00:00Z").build_struct(),
///                             FilterFindingCriteriaCriterion::builder().field("severity")
///                             .greaterThanOrEqual("4").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("MyFilter")
///             .rank(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GuardDuty filters using the detector ID and filter's name separated by a colon. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/filter:Filter MyFilter 00b00fd5aecc0ab60a708659477e9617:MyFilter
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FilterArgs {
        /// Specifies the action that is to be applied to the findings that match the filter. Can be one of `ARCHIVE` or `NOOP`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the filter.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of a GuardDuty detector, attached to your account.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Represents the criteria to be used in the filter for querying findings. Contains one or more `criterion` blocks, documented below.
        #[builder(into)]
        pub finding_criteria: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::guardduty::FilterFindingCriteria,
        >,
        /// The name of your filter.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.
        #[builder(into)]
        pub rank: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The tags that you want to add to the Filter resource. A tag consists of a key and a value. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FilterResult {
        /// Specifies the action that is to be applied to the findings that match the filter. Can be one of `ARCHIVE` or `NOOP`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the GuardDuty filter.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the filter.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of a GuardDuty detector, attached to your account.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// Represents the criteria to be used in the filter for querying findings. Contains one or more `criterion` blocks, documented below.
        pub finding_criteria: pulumi_gestalt_rust::Output<
            super::super::types::guardduty::FilterFindingCriteria,
        >,
        /// The name of your filter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.
        pub rank: pulumi_gestalt_rust::Output<i32>,
        /// The tags that you want to add to the Filter resource. A tag consists of a key and a value. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FilterArgs,
    ) -> FilterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let detector_id_binding = args.detector_id.get_output(context).get_inner();
        let finding_criteria_binding = args
            .finding_criteria
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let rank_binding = args.rank.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/filter:Filter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
                register_interface::ObjectField {
                    name: "findingCriteria".into(),
                    value: &finding_criteria_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rank".into(),
                    value: &rank_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FilterResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            detector_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("detectorId"),
            ),
            finding_criteria: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("findingCriteria"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            rank: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rank")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
