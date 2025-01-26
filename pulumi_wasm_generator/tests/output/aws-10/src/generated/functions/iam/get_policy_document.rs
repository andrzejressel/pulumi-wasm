pub mod get_policy_document {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyDocumentArgs {
        #[builder(into, default)]
        pub override_json: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of IAM policy documents that are merged together into the exported document. In merging, statements with non-blank `sid`s will override statements with the same `sid` from earlier documents in the list. Statements with non-blank `sid`s will also override statements with the same `sid` from `source_policy_documents`.  Non-overriding statements will be added to the exported document.
        #[builder(into, default)]
        pub override_policy_documents: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// ID for the policy document.
        #[builder(into, default)]
        pub policy_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub source_json: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of IAM policy documents that are merged together into the exported document. Statements defined in `source_policy_documents` must have unique `sid`s. Statements with the same `sid` from `override_policy_documents` will override source statements.
        #[builder(into, default)]
        pub source_policy_documents: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Configuration block for a policy statement. Detailed below.
        #[builder(into, default)]
        pub statements: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::iam::GetPolicyDocumentStatement>>,
        >,
        /// IAM policy document version. Valid values are `2008-10-17` and `2012-10-17`. Defaults to `2012-10-17`. For more information, see the [AWS IAM User Guide](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_version.html).
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyDocumentResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Standard JSON policy document rendered based on the arguments above.
        pub json: pulumi_wasm_rust::Output<String>,
        /// Minified JSON policy document rendered based on the arguments above.
        pub minified_json: pulumi_wasm_rust::Output<String>,
        pub override_json: pulumi_wasm_rust::Output<Option<String>>,
        pub override_policy_documents: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
        pub source_json: pulumi_wasm_rust::Output<Option<String>>,
        pub source_policy_documents: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub statements: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::iam::GetPolicyDocumentStatement>>,
        >,
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPolicyDocumentArgs,
    ) -> GetPolicyDocumentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "json".into(),
                },
                register_interface::ResultField {
                    name: "minifiedJson".into(),
                },
                register_interface::ResultField {
                    name: "overrideJson".into(),
                },
                register_interface::ResultField {
                    name: "overridePolicyDocuments".into(),
                },
                register_interface::ResultField {
                    name: "policyId".into(),
                },
                register_interface::ResultField {
                    name: "sourceJson".into(),
                },
                register_interface::ResultField {
                    name: "sourcePolicyDocuments".into(),
                },
                register_interface::ResultField {
                    name: "statements".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPolicyDocumentResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("json").unwrap(),
            ),
            minified_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minifiedJson").unwrap(),
            ),
            override_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overrideJson").unwrap(),
            ),
            override_policy_documents: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overridePolicyDocuments").unwrap(),
            ),
            policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyId").unwrap(),
            ),
            source_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceJson").unwrap(),
            ),
            source_policy_documents: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourcePolicyDocuments").unwrap(),
            ),
            statements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statements").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
