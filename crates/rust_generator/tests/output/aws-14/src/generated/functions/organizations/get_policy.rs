#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyArgs {
        /// The unique identifier (ID) of the policy that you want more details on. Policy id starts with a "p-" followed by 8-28 lowercase or uppercase letters, digits, and underscores.
        #[builder(into)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyResult {
        /// The Amazon Resource Name of the policy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates if a policy is an AWS managed policy.
        pub aws_managed: pulumi_gestalt_rust::Output<bool>,
        /// The text content of the policy.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The description of the policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The friendly name of the policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// The type of policy values can be `AISERVICES_OPT_OUT_POLICY | BACKUP_POLICY | RESOURCE_CONTROL_POLICY | SERVICE_CONTROL_POLICY | TAG_POLICY`
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPolicyArgs,
    ) -> GetPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_id_binding = args.policy_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:organizations/getPolicy:getPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: policy_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPolicyResult {
            arn: o.get_field("arn"),
            aws_managed: o.get_field("awsManaged"),
            content: o.get_field("content"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            policy_id: o.get_field("policyId"),
            type_: o.get_field("type"),
        }
    }
}
