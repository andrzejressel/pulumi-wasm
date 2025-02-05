pub mod get_analysis {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAnalysisArgs {
        /// Identifier for the analysis.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub analysis_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAnalysisResult {
        pub analysis_id: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub last_published_time: pulumi_wasm_rust::Output<String>,
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub permissions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::quicksight::GetAnalysisPermission>,
        >,
        pub status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub theme_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAnalysisArgs,
    ) -> GetAnalysisResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let analysis_id_binding = args.analysis_id.get_output(context).get_inner();
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:quicksight/getAnalysis:getAnalysis".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "analysisId".into(),
                    value: &analysis_id_binding,
                },
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAnalysisResult {
            analysis_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("analysisId"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_published_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastPublishedTime"),
            ),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            theme_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("themeArn"),
            ),
        }
    }
}
