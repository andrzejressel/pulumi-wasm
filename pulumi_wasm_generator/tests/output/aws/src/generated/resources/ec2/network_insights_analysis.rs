/// Provides a Network Insights Analysis resource. Part of the "Reachability Analyzer" service in the AWS VPC console.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let analysis = network_insights_analysis::create(
///         "analysis",
///         NetworkInsightsAnalysisArgs::builder()
///             .network_insights_path_id("${path.id}")
///             .build_struct(),
///     );
///     let path = network_insights_path::create(
///         "path",
///         NetworkInsightsPathArgs::builder()
///             .destination("${destination.id}")
///             .protocol("tcp")
///             .source("${source.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Insights Analyzes using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/networkInsightsAnalysis:NetworkInsightsAnalysis test nia-0462085c957f11a55
/// ```
pub mod network_insights_analysis {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInsightsAnalysisArgs {
        /// A list of ARNs for resources the path must traverse.
        #[builder(into, default)]
        pub filter_in_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ID of the Network Insights Path to run an analysis on.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub network_insights_path_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If enabled, the resource will wait for the Network Insights Analysis status to change to `succeeded` or `failed`. Setting this to `false` will skip the process. Default: `true`.
        #[builder(into, default)]
        pub wait_for_completion: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct NetworkInsightsAnalysisResult {
        /// Potential intermediate components of a feasible path. Described below.
        pub alternate_path_hints: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::NetworkInsightsAnalysisAlternatePathHint>,
        >,
        /// ARN of the Network Insights Analysis.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Explanation codes for an unreachable path. See the [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_Explanation.html) for details.
        pub explanations: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanation>,
        >,
        /// A list of ARNs for resources the path must traverse.
        pub filter_in_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The components in the path from source to destination. See the [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_PathComponent.html) for details.
        pub forward_path_components: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponent>,
        >,
        /// ID of the Network Insights Path to run an analysis on.
        ///
        /// The following arguments are optional:
        pub network_insights_path_id: pulumi_wasm_rust::Output<String>,
        /// Set to `true` if the destination was reachable.
        pub path_found: pulumi_wasm_rust::Output<bool>,
        /// The components in the path from destination to source. See the [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_PathComponent.html) for details.
        pub return_path_components: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponent>,
        >,
        /// The date/time the analysis was started.
        pub start_date: pulumi_wasm_rust::Output<String>,
        /// The status of the analysis. `succeeded` means the analysis was completed, not that a path was found, for that see `path_found`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A message to provide more context when the `status` is `failed`.
        pub status_message: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If enabled, the resource will wait for the Network Insights Analysis status to change to `succeeded` or `failed`. Setting this to `false` will skip the process. Default: `true`.
        pub wait_for_completion: pulumi_wasm_rust::Output<Option<bool>>,
        /// The warning message.
        pub warning_message: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkInsightsAnalysisArgs,
    ) -> NetworkInsightsAnalysisResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_in_arns_binding = args.filter_in_arns.get_inner();
        let network_insights_path_id_binding = args.network_insights_path_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let wait_for_completion_binding = args.wait_for_completion.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkInsightsAnalysis:NetworkInsightsAnalysis".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filterInArns".into(),
                    value: &filter_in_arns_binding,
                },
                register_interface::ObjectField {
                    name: "networkInsightsPathId".into(),
                    value: &network_insights_path_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "waitForCompletion".into(),
                    value: &wait_for_completion_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alternatePathHints".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "explanations".into(),
                },
                register_interface::ResultField {
                    name: "filterInArns".into(),
                },
                register_interface::ResultField {
                    name: "forwardPathComponents".into(),
                },
                register_interface::ResultField {
                    name: "networkInsightsPathId".into(),
                },
                register_interface::ResultField {
                    name: "pathFound".into(),
                },
                register_interface::ResultField {
                    name: "returnPathComponents".into(),
                },
                register_interface::ResultField {
                    name: "startDate".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusMessage".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "waitForCompletion".into(),
                },
                register_interface::ResultField {
                    name: "warningMessage".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkInsightsAnalysisResult {
            alternate_path_hints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alternatePathHints").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            explanations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("explanations").unwrap(),
            ),
            filter_in_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filterInArns").unwrap(),
            ),
            forward_path_components: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardPathComponents").unwrap(),
            ),
            network_insights_path_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInsightsPathId").unwrap(),
            ),
            path_found: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathFound").unwrap(),
            ),
            return_path_components: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("returnPathComponents").unwrap(),
            ),
            start_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startDate").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusMessage").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            wait_for_completion: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForCompletion").unwrap(),
            ),
            warning_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("warningMessage").unwrap(),
            ),
        }
    }
}