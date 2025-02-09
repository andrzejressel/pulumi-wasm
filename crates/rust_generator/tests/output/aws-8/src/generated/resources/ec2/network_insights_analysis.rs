/// Provides a Network Insights Analysis resource. Part of the "Reachability Analyzer" service in the AWS VPC console.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_insights_analysis {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInsightsAnalysisArgs {
        /// A list of ARNs for resources the path must traverse.
        #[builder(into, default)]
        pub filter_in_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// ID of the Network Insights Path to run an analysis on.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub network_insights_path_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If enabled, the resource will wait for the Network Insights Analysis status to change to `succeeded` or `failed`. Setting this to `false` will skip the process. Default: `true`.
        #[builder(into, default)]
        pub wait_for_completion: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct NetworkInsightsAnalysisResult {
        /// Potential intermediate components of a feasible path. Described below.
        pub alternate_path_hints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::NetworkInsightsAnalysisAlternatePathHint>,
        >,
        /// ARN of the Network Insights Analysis.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Explanation codes for an unreachable path. See the [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_Explanation.html) for details.
        pub explanations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanation>,
        >,
        /// A list of ARNs for resources the path must traverse.
        pub filter_in_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The components in the path from source to destination. See the [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_PathComponent.html) for details.
        pub forward_path_components: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponent>,
        >,
        /// ID of the Network Insights Path to run an analysis on.
        ///
        /// The following arguments are optional:
        pub network_insights_path_id: pulumi_gestalt_rust::Output<String>,
        /// Set to `true` if the destination was reachable.
        pub path_found: pulumi_gestalt_rust::Output<bool>,
        /// The components in the path from destination to source. See the [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_PathComponent.html) for details.
        pub return_path_components: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponent>,
        >,
        /// The date/time the analysis was started.
        pub start_date: pulumi_gestalt_rust::Output<String>,
        /// The status of the analysis. `succeeded` means the analysis was completed, not that a path was found, for that see `path_found`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A message to provide more context when the `status` is `failed`.
        pub status_message: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If enabled, the resource will wait for the Network Insights Analysis status to change to `succeeded` or `failed`. Setting this to `false` will skip the process. Default: `true`.
        pub wait_for_completion: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The warning message.
        pub warning_message: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkInsightsAnalysisArgs,
    ) -> NetworkInsightsAnalysisResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_in_arns_binding = args.filter_in_arns.get_output(context);
        let network_insights_path_id_binding = args
            .network_insights_path_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let wait_for_completion_binding = args.wait_for_completion.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/networkInsightsAnalysis:NetworkInsightsAnalysis".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterInArns".into(),
                    value: filter_in_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInsightsPathId".into(),
                    value: network_insights_path_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitForCompletion".into(),
                    value: wait_for_completion_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkInsightsAnalysisResult {
            alternate_path_hints: o.get_field("alternatePathHints"),
            arn: o.get_field("arn"),
            explanations: o.get_field("explanations"),
            filter_in_arns: o.get_field("filterInArns"),
            forward_path_components: o.get_field("forwardPathComponents"),
            network_insights_path_id: o.get_field("networkInsightsPathId"),
            path_found: o.get_field("pathFound"),
            return_path_components: o.get_field("returnPathComponents"),
            start_date: o.get_field("startDate"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            wait_for_completion: o.get_field("waitForCompletion"),
            warning_message: o.get_field("warningMessage"),
        }
    }
}
