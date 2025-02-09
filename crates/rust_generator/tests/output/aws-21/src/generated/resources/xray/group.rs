/// Creates and manages an AWS XRay Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = group::create(
///         "example",
///         GroupArgs::builder()
///             .filter_expression("responsetime > 5")
///             .group_name("example")
///             .insights_configuration(
///                 GroupInsightsConfiguration::builder()
///                     .insightsEnabled(true)
///                     .notificationsEnabled(true)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import XRay Groups using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:xray/group:Group example arn:aws:xray:us-west-2:1234567890:group/example-group/TNGX7SW5U6QY36T4ZMOUA3HVLBYCZTWDIOOXY3CJAXTHSS3YCWUA
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// The filter expression defining criteria by which to group traces. more info can be found in official [docs](https://docs.aws.amazon.com/xray/latest/devguide/xray-console-filters.html).
        #[builder(into)]
        pub filter_expression: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the group.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration options for enabling insights.
        #[builder(into, default)]
        pub insights_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::xray::GroupInsightsConfiguration>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// The ARN of the Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The filter expression defining criteria by which to group traces. more info can be found in official [docs](https://docs.aws.amazon.com/xray/latest/devguide/xray-console-filters.html).
        pub filter_expression: pulumi_gestalt_rust::Output<String>,
        /// The name of the group.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration options for enabling insights.
        pub insights_configuration: pulumi_gestalt_rust::Output<
            super::super::types::xray::GroupInsightsConfiguration,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
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
        args: GroupArgs,
    ) -> GroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filter_expression_binding_1 = args.filter_expression.get_output(context);
        let filter_expression_binding = filter_expression_binding_1.get_inner();
        let group_name_binding_1 = args.group_name.get_output(context);
        let group_name_binding = group_name_binding_1.get_inner();
        let insights_configuration_binding_1 = args
            .insights_configuration
            .get_output(context);
        let insights_configuration_binding = insights_configuration_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:xray/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filterExpression".into(),
                    value: &filter_expression_binding,
                },
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "insightsConfiguration".into(),
                    value: &insights_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            filter_expression: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filterExpression"),
            ),
            group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupName"),
            ),
            insights_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("insightsConfiguration"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
