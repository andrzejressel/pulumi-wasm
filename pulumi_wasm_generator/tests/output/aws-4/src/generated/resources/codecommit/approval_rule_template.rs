/// Provides a CodeCommit Approval Rule Template Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:codecommit:ApprovalRuleTemplate
///     properties:
///       name: MyExampleApprovalRuleTemplate
///       description: This is an example approval rule template
///       content:
///         fn::toJSON:
///           Version: 2018-11-08
///           DestinationReferences:
///             - refs/heads/master
///           Statements:
///             - Type: Approvers
///               NumberOfApprovalsNeeded: 2
///               ApprovalPoolMembers:
///                 - arn:aws:sts::123456789012:assumed-role/CodeCommitReview/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeCommit approval rule templates using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:codecommit/approvalRuleTemplate:ApprovalRuleTemplate imported ExistingApprovalRuleTemplateName
/// ```
pub mod approval_rule_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApprovalRuleTemplateArgs {
        /// The content of the approval rule template. Maximum of 3000 characters.
        #[builder(into)]
        pub content: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description of the approval rule template. Maximum of 1000 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name for the approval rule template. Maximum of 100 characters.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApprovalRuleTemplateResult {
        /// The ID of the approval rule template
        pub approval_rule_template_id: pulumi_wasm_rust::Output<String>,
        /// The content of the approval rule template. Maximum of 3000 characters.
        pub content: pulumi_wasm_rust::Output<String>,
        /// The date the approval rule template was created, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// The description of the approval rule template. Maximum of 1000 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The date the approval rule template was most recently changed, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified_date: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the user who made the most recent changes to the approval rule template.
        pub last_modified_user: pulumi_wasm_rust::Output<String>,
        /// The name for the approval rule template. Maximum of 100 characters.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The SHA-256 hash signature for the content of the approval rule template.
        pub rule_content_sha256: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ApprovalRuleTemplateArgs,
    ) -> ApprovalRuleTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_binding = args.content.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecommit/approvalRuleTemplate:ApprovalRuleTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApprovalRuleTemplateResult {
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
