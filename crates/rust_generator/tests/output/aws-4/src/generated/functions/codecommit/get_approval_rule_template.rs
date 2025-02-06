pub mod get_approval_rule_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApprovalRuleTemplateArgs {
        /// Name for the approval rule template. This needs to be less than 100 characters.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetApprovalRuleTemplateResult {
        /// The ID of the approval rule template.
        pub approval_rule_template_id: pulumi_gestalt_rust::Output<String>,
        /// Content of the approval rule template.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// Date the approval rule template was created, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the approval rule template.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date the approval rule template was most recently changed, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified_date: pulumi_gestalt_rust::Output<String>,
        /// ARN of the user who made the most recent changes to the approval rule template.
        pub last_modified_user: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// SHA-256 hash signature for the content of the approval rule template.
        pub rule_content_sha256: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetApprovalRuleTemplateArgs,
    ) -> GetApprovalRuleTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetApprovalRuleTemplateResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
