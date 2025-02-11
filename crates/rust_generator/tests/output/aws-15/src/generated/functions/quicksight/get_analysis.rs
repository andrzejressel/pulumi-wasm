#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_analysis {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAnalysisArgs {
        /// Identifier for the analysis.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub analysis_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAnalysisResult {
        pub analysis_id: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub last_published_time: pulumi_gestalt_rust::Output<String>,
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub permissions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::quicksight::GetAnalysisPermission>,
        >,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub theme_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAnalysisArgs,
    ) -> GetAnalysisResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let analysis_id_binding = args.analysis_id.get_output(context);
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:quicksight/getAnalysis:getAnalysis".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analysisId".into(),
                    value: &analysis_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAnalysisResult {
            analysis_id: o.get_field("analysisId"),
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            created_time: o.get_field("createdTime"),
            id: o.get_field("id"),
            last_published_time: o.get_field("lastPublishedTime"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            theme_arn: o.get_field("themeArn"),
        }
    }
}
