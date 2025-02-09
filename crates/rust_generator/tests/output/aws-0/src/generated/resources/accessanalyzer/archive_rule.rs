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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ArchiveRuleArgs,
    ) -> ArchiveRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let analyzer_name_binding = args.analyzer_name.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let rule_name_binding = args.rule_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:accessanalyzer/archiveRule:ArchiveRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analyzerName".into(),
                    value: analyzer_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleName".into(),
                    value: rule_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ArchiveRuleResult {
            analyzer_name: o.get_field("analyzerName"),
            filters: o.get_field("filters"),
            rule_name: o.get_field("ruleName"),
        }
    }
}
