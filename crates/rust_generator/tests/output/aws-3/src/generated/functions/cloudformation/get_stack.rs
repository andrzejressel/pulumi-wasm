#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_stack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStackArgs {
        /// Name of the stack
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags associated with this stack.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetStackResult {
        /// List of capabilities
        pub capabilities: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Description of the stack
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether the rollback of the stack is disabled when stack creation fails
        pub disable_rollback: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the IAM role used to create the stack.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of SNS topic ARNs to publish stack related events
        pub notification_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of outputs from the stack.
        pub outputs: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map of parameters that specify input parameters for the stack.
        pub parameters: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map of tags associated with this stack.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Structure containing the template body.
        pub template_body: pulumi_gestalt_rust::Output<String>,
        /// Amount of time that can pass before the stack status becomes `CREATE_FAILED`
        pub timeout_in_minutes: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetStackArgs,
    ) -> GetStackResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudformation/getStack:getStack".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetStackResult {
            capabilities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capabilities"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disable_rollback: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableRollback"),
            ),
            iam_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamRoleArn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            notification_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notificationArns"),
            ),
            outputs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputs"),
            ),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            template_body: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateBody"),
            ),
            timeout_in_minutes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeoutInMinutes"),
            ),
        }
    }
}
