#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_serverless_security_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessSecurityPolicyArgs {
        /// Name of the policy
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of security policy. One of `encryption` or `network`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServerlessSecurityPolicyResult {
        /// The date the security policy was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the security policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The date the security policy was last modified.
        pub last_modified_date: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The JSON policy document without any whitespaces.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Version of the policy.
        pub policy_version: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServerlessSecurityPolicyArgs,
    ) -> GetServerlessSecurityPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:opensearch/getServerlessSecurityPolicy:getServerlessSecurityPolicy"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServerlessSecurityPolicyResult {
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            last_modified_date: o.get_field("lastModifiedDate"),
            name: o.get_field("name"),
            policy: o.get_field("policy"),
            policy_version: o.get_field("policyVersion"),
            type_: o.get_field("type"),
        }
    }
}
