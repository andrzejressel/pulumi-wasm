/// Associates a CodeCommit Approval Rule Template with a Repository.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod approval_rule_template_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApprovalRuleTemplateAssociationArgs {
        /// The name for the approval rule template.
        #[builder(into)]
        pub approval_rule_template_name: pulumi_wasm_rust::Output<String>,
        /// The name of the repository that you want to associate with the template.
        #[builder(into)]
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApprovalRuleTemplateAssociationResult {
        /// The name for the approval rule template.
        pub approval_rule_template_name: pulumi_wasm_rust::Output<String>,
        /// The name of the repository that you want to associate with the template.
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ApprovalRuleTemplateAssociationArgs,
    ) -> ApprovalRuleTemplateAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let approval_rule_template_name_binding = args
            .approval_rule_template_name
            .get_inner();
        let repository_name_binding = args.repository_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecommit/approvalRuleTemplateAssociation:ApprovalRuleTemplateAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "approvalRuleTemplateName".into(),
                    value: &approval_rule_template_name_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "approvalRuleTemplateName".into(),
                },
                register_interface::ResultField {
                    name: "repositoryName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApprovalRuleTemplateAssociationResult {
            approval_rule_template_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalRuleTemplateName").unwrap(),
            ),
            repository_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryName").unwrap(),
            ),
        }
    }
}