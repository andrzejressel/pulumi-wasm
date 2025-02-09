#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_insights_analysis {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInsightsAnalysisArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::ec2::GetNetworkInsightsAnalysisFilter>,
            >,
        >,
        /// ID of the Network Insights Analysis to select.
        #[builder(into, default)]
        pub network_insights_analysis_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNetworkInsightsAnalysisResult {
        /// Potential intermediate components of a feasible path.
        pub alternate_path_hints: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2::GetNetworkInsightsAnalysisAlternatePathHint,
            >,
        >,
        /// ARN of the selected Network Insights Analysis.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Explanation codes for an unreachable path.
        pub explanations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetNetworkInsightsAnalysisExplanation>,
        >,
        /// ARNs of the AWS resources that the path must traverse.
        pub filter_in_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::ec2::GetNetworkInsightsAnalysisFilter>,
            >,
        >,
        /// The components in the path from source to destination.
        pub forward_path_components: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponent,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub network_insights_analysis_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the path.
        pub network_insights_path_id: pulumi_gestalt_rust::Output<String>,
        /// Set to `true` if the destination was reachable.
        pub path_found: pulumi_gestalt_rust::Output<bool>,
        /// The components in the path from destination to source.
        pub return_path_components: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponent,
            >,
        >,
        /// Date/time the analysis was started.
        pub start_date: pulumi_gestalt_rust::Output<String>,
        /// Status of the analysis. `succeeded` means the analysis was completed, not that a path was found, for that see `path_found`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Message to provide more context when the `status` is `failed`.
        pub status_message: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Warning message.
        pub warning_message: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkInsightsAnalysisArgs,
    ) -> GetNetworkInsightsAnalysisResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let network_insights_analysis_id_binding = args
            .network_insights_analysis_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getNetworkInsightsAnalysis:getNetworkInsightsAnalysis"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInsightsAnalysisId".into(),
                    value: network_insights_analysis_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkInsightsAnalysisResult {
            alternate_path_hints: o.get_field("alternatePathHints"),
            arn: o.get_field("arn"),
            explanations: o.get_field("explanations"),
            filter_in_arns: o.get_field("filterInArns"),
            filters: o.get_field("filters"),
            forward_path_components: o.get_field("forwardPathComponents"),
            id: o.get_field("id"),
            network_insights_analysis_id: o.get_field("networkInsightsAnalysisId"),
            network_insights_path_id: o.get_field("networkInsightsPathId"),
            path_found: o.get_field("pathFound"),
            return_path_components: o.get_field("returnPathComponents"),
            start_date: o.get_field("startDate"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
            tags: o.get_field("tags"),
            warning_message: o.get_field("warningMessage"),
        }
    }
}
