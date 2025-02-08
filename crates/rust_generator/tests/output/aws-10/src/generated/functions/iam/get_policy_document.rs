#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPolicyDocumentArgs,
    ) -> GetPolicyDocumentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let override_json_binding = args.override_json.get_output(context).get_inner();
        let override_policy_documents_binding = args
            .override_policy_documents
            .get_output(context)
            .get_inner();
        let policy_id_binding = args.policy_id.get_output(context).get_inner();
        let source_json_binding = args.source_json.get_output(context).get_inner();
        let source_policy_documents_binding = args
            .source_policy_documents
            .get_output(context)
            .get_inner();
        let statements_binding = args.statements.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getPolicyDocument:getPolicyDocument".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "overrideJson".into(),
                    value: &override_json_binding,
                },
                register_interface::ObjectField {
                    name: "overridePolicyDocuments".into(),
                    value: &override_policy_documents_binding,
                },
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "sourceJson".into(),
                    value: &source_json_binding,
                },
                register_interface::ObjectField {
                    name: "sourcePolicyDocuments".into(),
                    value: &source_policy_documents_binding,
                },
                register_interface::ObjectField {
                    name: "statements".into(),
                    value: &statements_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPolicyDocumentResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            json: pulumi_gestalt_rust::__private::into_domain(o.extract_field("json")),
            minified_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minifiedJson"),
            ),
            override_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("overrideJson"),
            ),
            override_policy_documents: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("overridePolicyDocuments"),
            ),
            policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyId"),
            ),
            source_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceJson"),
            ),
            source_policy_documents: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourcePolicyDocuments"),
            ),
            statements: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statements"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
