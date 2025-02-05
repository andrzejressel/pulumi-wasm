pub mod get_network_insights_analysis {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInsightsAnalysisArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::ec2::GetNetworkInsightsAnalysisFilter>,
            >,
        >,
        /// ID of the Network Insights Analysis to select.
        #[builder(into, default)]
        pub network_insights_analysis_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNetworkInsightsAnalysisResult {
        /// Potential intermediate components of a feasible path.
        pub alternate_path_hints: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2::GetNetworkInsightsAnalysisAlternatePathHint,
            >,
        >,
        /// ARN of the selected Network Insights Analysis.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Explanation codes for an unreachable path.
        pub explanations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetNetworkInsightsAnalysisExplanation>,
        >,
        /// ARNs of the AWS resources that the path must traverse.
        pub filter_in_arns: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::ec2::GetNetworkInsightsAnalysisFilter>,
            >,
        >,
        /// The components in the path from source to destination.
        pub forward_path_components: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponent,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub network_insights_analysis_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the path.
        pub network_insights_path_id: pulumi_wasm_rust::Output<String>,
        /// Set to `true` if the destination was reachable.
        pub path_found: pulumi_wasm_rust::Output<bool>,
        /// The components in the path from destination to source.
        pub return_path_components: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponent,
            >,
        >,
        /// Date/time the analysis was started.
        pub start_date: pulumi_wasm_rust::Output<String>,
        /// Status of the analysis. `succeeded` means the analysis was completed, not that a path was found, for that see `path_found`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Message to provide more context when the `status` is `failed`.
        pub status_message: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Warning message.
        pub warning_message: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkInsightsAnalysisArgs,
    ) -> GetNetworkInsightsAnalysisResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let network_insights_analysis_id_binding = args
            .network_insights_analysis_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getNetworkInsightsAnalysis:getNetworkInsightsAnalysis"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "networkInsightsAnalysisId".into(),
                    value: &network_insights_analysis_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkInsightsAnalysisResult {
            alternate_path_hints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("alternatePathHints"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            explanations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("explanations"),
            ),
            filter_in_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filterInArns"),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            forward_path_components: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forwardPathComponents"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            network_insights_analysis_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkInsightsAnalysisId"),
            ),
            network_insights_path_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkInsightsPathId"),
            ),
            path_found: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pathFound"),
            ),
            return_path_components: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("returnPathComponents"),
            ),
            start_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startDate"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            status_message: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statusMessage"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            warning_message: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("warningMessage"),
            ),
        }
    }
}
