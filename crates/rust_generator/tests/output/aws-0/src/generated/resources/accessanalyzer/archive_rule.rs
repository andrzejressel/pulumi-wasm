/// Resource for managing an AWS AccessAnalyzer Archive Rule.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:accessanalyzer:ArchiveRule
///     properties:
///       analyzerName: example-analyzer
///       ruleName: example-rule
///       filters:
///         - criteria: condition.aws:UserId
///           eqs:
///             - userid
///         - criteria: error
///           exists: true
///         - criteria: isPublic
///           eqs:
///             - 'false'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AccessAnalyzer ArchiveRule using the `analyzer_name/rule_name`. For example:
///
/// ```sh
/// $ pulumi import aws:accessanalyzer/archiveRule:ArchiveRule example example-analyzer/example-rule
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod archive_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ArchiveRuleArgs {
        /// Analyzer name.
        #[builder(into)]
        pub analyzer_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Filter criteria for the archive rule. See Filter for more details.
        #[builder(into)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::accessanalyzer::ArchiveRuleFilter>,
        >,
        /// Rule name.
        #[builder(into)]
        pub rule_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ArchiveRuleResult {
        /// Analyzer name.
        pub analyzer_name: pulumi_gestalt_rust::Output<String>,
        /// Filter criteria for the archive rule. See Filter for more details.
        pub filters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::accessanalyzer::ArchiveRuleFilter>,
        >,
        /// Rule name.
        pub rule_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ArchiveRuleArgs,
    ) -> ArchiveRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let analyzer_name_binding = args.analyzer_name.get_output(context).get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let rule_name_binding = args.rule_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:accessanalyzer/archiveRule:ArchiveRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "analyzerName".into(),
                    value: &analyzer_name_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "ruleName".into(),
                    value: &rule_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ArchiveRuleResult {
            analyzer_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("analyzerName"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            rule_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleName"),
            ),
        }
    }
}
