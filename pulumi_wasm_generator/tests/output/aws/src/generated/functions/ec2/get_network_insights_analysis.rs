pub mod get_network_insights_analysis {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInsightsAnalysisArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::ec2::GetNetworkInsightsAnalysisFilter>,
            >,
        >,
        /// ID of the Network Insights Analysis to select.
        #[builder(into, default)]
        pub network_insights_analysis_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
        args: GetNetworkInsightsAnalysisArgs,
    ) -> GetNetworkInsightsAnalysisResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let network_insights_analysis_id_binding = args
            .network_insights_analysis_id
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getNetworkInsightsAnalysis:getNetworkInsightsAnalysis"
                .into(),
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
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "forwardPathComponents".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "networkInsightsAnalysisId".into(),
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
                    name: "warningMessage".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkInsightsAnalysisResult {
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
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            forward_path_components: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardPathComponents").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            network_insights_analysis_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInsightsAnalysisId").unwrap(),
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
            warning_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("warningMessage").unwrap(),
            ),
        }
    }
}