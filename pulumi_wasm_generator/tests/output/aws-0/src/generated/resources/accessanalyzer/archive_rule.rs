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
pub mod archive_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ArchiveRuleArgs {
        /// Analyzer name.
        #[builder(into)]
        pub analyzer_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Filter criteria for the archive rule. See Filter for more details.
        #[builder(into)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::accessanalyzer::ArchiveRuleFilter>,
        >,
        /// Rule name.
        #[builder(into)]
        pub rule_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ArchiveRuleResult {
        /// Analyzer name.
        pub analyzer_name: pulumi_wasm_rust::Output<String>,
        /// Filter criteria for the archive rule. See Filter for more details.
        pub filters: pulumi_wasm_rust::Output<
            Vec<super::super::types::accessanalyzer::ArchiveRuleFilter>,
        >,
        /// Rule name.
        pub rule_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ArchiveRuleArgs,
    ) -> ArchiveRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "analyzerName".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "ruleName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ArchiveRuleResult {
            analyzer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analyzerName").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            rule_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleName").unwrap(),
            ),
        }
    }
}
