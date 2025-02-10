/// Associates a CodeCommit Approval Rule Template with a Repository.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = approval_rule_template_association::create(
///         "example",
///         ApprovalRuleTemplateAssociationArgs::builder()
///             .approval_rule_template_name(
///                 "${exampleAwsCodecommitApprovalRuleTemplate.name}",
///             )
///             .repository_name("${exampleAwsCodecommitRepository.repositoryName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeCommit approval rule template associations using the `approval_rule_template_name` and `repository_name` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:codecommit/approvalRuleTemplateAssociation:ApprovalRuleTemplateAssociation example approver-rule-for-example,MyExampleRepo
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod approval_rule_template_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApprovalRuleTemplateAssociationArgs {
        /// The name for the approval rule template.
        #[builder(into)]
        pub approval_rule_template_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the repository that you want to associate with the template.
        #[builder(into)]
        pub repository_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApprovalRuleTemplateAssociationResult {
        /// The name for the approval rule template.
        pub approval_rule_template_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the repository that you want to associate with the template.
        pub repository_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApprovalRuleTemplateAssociationArgs,
    ) -> ApprovalRuleTemplateAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let approval_rule_template_name_binding = args
            .approval_rule_template_name
            .get_output(context);
        let repository_name_binding = args.repository_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codecommit/approvalRuleTemplateAssociation:ApprovalRuleTemplateAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvalRuleTemplateName".into(),
                    value: approval_rule_template_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryName".into(),
                    value: repository_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApprovalRuleTemplateAssociationResult {
            approval_rule_template_name: o.get_field("approvalRuleTemplateName"),
            repository_name: o.get_field("repositoryName"),
        }
    }
}
