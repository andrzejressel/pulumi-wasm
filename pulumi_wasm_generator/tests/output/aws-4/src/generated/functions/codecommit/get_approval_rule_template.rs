pub mod get_approval_rule_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApprovalRuleTemplateArgs {
        /// Name for the approval rule template. This needs to be less than 100 characters.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetApprovalRuleTemplateResult {
        /// The ID of the approval rule template.
        pub approval_rule_template_id: pulumi_wasm_rust::Output<String>,
        /// Content of the approval rule template.
        pub content: pulumi_wasm_rust::Output<String>,
        /// Date the approval rule template was created, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// Description of the approval rule template.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Date the approval rule template was most recently changed, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified_date: pulumi_wasm_rust::Output<String>,
        /// ARN of the user who made the most recent changes to the approval rule template.
        pub last_modified_user: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// SHA-256 hash signature for the content of the approval rule template.
        pub rule_content_sha256: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetApprovalRuleTemplateArgs,
    ) -> GetApprovalRuleTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codecommit/getApprovalRuleTemplate:getApprovalRuleTemplate"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "approvalRuleTemplateId".into(),
                },
                register_interface::ResultField {
                    name: "content".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedDate".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedUser".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ruleContentSha256".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApprovalRuleTemplateResult {
            approval_rule_template_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalRuleTemplateId").unwrap(),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedDate").unwrap(),
            ),
            last_modified_user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedUser").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rule_content_sha256: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleContentSha256").unwrap(),
            ),
        }
    }
}
