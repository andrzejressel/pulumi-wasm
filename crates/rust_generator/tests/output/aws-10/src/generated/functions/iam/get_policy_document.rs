#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_policy_document {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyDocumentArgs {
        #[builder(into, default)]
        pub override_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of IAM policy documents that are merged together into the exported document. In merging, statements with non-blank `sid`s will override statements with the same `sid` from earlier documents in the list. Statements with non-blank `sid`s will also override statements with the same `sid` from `source_policy_documents`.  Non-overriding statements will be added to the exported document.
        #[builder(into, default)]
        pub override_policy_documents: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// ID for the policy document.
        #[builder(into, default)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub source_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of IAM policy documents that are merged together into the exported document. Statements defined in `source_policy_documents` must have unique `sid`s. Statements with the same `sid` from `override_policy_documents` will override source statements.
        #[builder(into, default)]
        pub source_policy_documents: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Configuration block for a policy statement. Detailed below.
        #[builder(into, default)]
        pub statements: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::iam::GetPolicyDocumentStatement>>,
        >,
        /// IAM policy document version. Valid values are `2008-10-17` and `2012-10-17`. Defaults to `2012-10-17`. For more information, see the [AWS IAM User Guide](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_version.html).
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyDocumentResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Standard JSON policy document rendered based on the arguments above.
        pub json: pulumi_gestalt_rust::Output<String>,
        /// Minified JSON policy document rendered based on the arguments above.
        pub minified_json: pulumi_gestalt_rust::Output<String>,
        pub override_json: pulumi_gestalt_rust::Output<Option<String>>,
        pub override_policy_documents: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub source_json: pulumi_gestalt_rust::Output<Option<String>>,
        pub source_policy_documents: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub statements: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::iam::GetPolicyDocumentStatement>>,
        >,
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPolicyDocumentArgs,
    ) -> GetPolicyDocumentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let override_json_binding = args.override_json.get_output(context);
        let override_policy_documents_binding = args
            .override_policy_documents
            .get_output(context);
        let policy_id_binding = args.policy_id.get_output(context);
        let source_json_binding = args.source_json.get_output(context);
        let source_policy_documents_binding = args
            .source_policy_documents
            .get_output(context);
        let statements_binding = args.statements.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getPolicyDocument:getPolicyDocument".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overrideJson".into(),
                    value: override_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overridePolicyDocuments".into(),
                    value: override_policy_documents_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceJson".into(),
                    value: source_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourcePolicyDocuments".into(),
                    value: source_policy_documents_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statements".into(),
                    value: statements_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPolicyDocumentResult {
            id: o.get_field("id"),
            json: o.get_field("json"),
            minified_json: o.get_field("minifiedJson"),
            override_json: o.get_field("overrideJson"),
            override_policy_documents: o.get_field("overridePolicyDocuments"),
            policy_id: o.get_field("policyId"),
            source_json: o.get_field("sourceJson"),
            source_policy_documents: o.get_field("sourcePolicyDocuments"),
            statements: o.get_field("statements"),
            version: o.get_field("version"),
        }
    }
}
