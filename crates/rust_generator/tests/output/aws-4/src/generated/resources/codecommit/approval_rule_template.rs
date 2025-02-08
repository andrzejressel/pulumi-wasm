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
#[allow(clippy::doc_lazy_continuation)]
pub mod approval_rule_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApprovalRuleTemplateArgs {
        /// The content of the approval rule template. Maximum of 3000 characters.
        #[builder(into)]
        pub content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the approval rule template. Maximum of 1000 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name for the approval rule template. Maximum of 100 characters.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApprovalRuleTemplateResult {
        /// The ID of the approval rule template
        pub approval_rule_template_id: pulumi_gestalt_rust::Output<String>,
        /// The content of the approval rule template. Maximum of 3000 characters.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The date the approval rule template was created, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// The description of the approval rule template. Maximum of 1000 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The date the approval rule template was most recently changed, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified_date: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the user who made the most recent changes to the approval rule template.
        pub last_modified_user: pulumi_gestalt_rust::Output<String>,
        /// The name for the approval rule template. Maximum of 100 characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The SHA-256 hash signature for the content of the approval rule template.
        pub rule_content_sha256: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApprovalRuleTemplateArgs,
    ) -> ApprovalRuleTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApprovalRuleTemplateResult {
            approval_rule_template_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("approvalRuleTemplateId"),
            ),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            creation_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationDate"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            last_modified_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedDate"),
            ),
            last_modified_user: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedUser"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            rule_content_sha256: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleContentSha256"),
            ),
        }
    }
}
