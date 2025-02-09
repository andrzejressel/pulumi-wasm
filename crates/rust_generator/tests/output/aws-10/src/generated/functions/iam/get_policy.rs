#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyArgs {
        /// ARN of the IAM policy.
        /// Conflicts with `name` and `path_prefix`.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the IAM policy.
        /// Conflicts with `arn`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Prefix of the path to the IAM policy.
        /// Defaults to a slash (`/`).
        /// Conflicts with `arn`.
        #[builder(into, default)]
        pub path_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of tags for the IAM Policy.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPolicyResult {
        /// ARN of the policy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Number of entities (users, groups, and roles) that the policy is attached to.
        pub attachment_count: pulumi_gestalt_rust::Output<i32>,
        /// Description of the policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Path to the policy.
        pub path: pulumi_gestalt_rust::Output<String>,
        pub path_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// Policy document of the policy.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Policy's ID.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of tags for the IAM Policy.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
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
        let arn_binding = args.arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let path_prefix_binding = args.path_prefix.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getPolicy:getPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pathPrefix".into(),
                    value: path_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPolicyResult {
            arn: o.get_field("arn"),
            attachment_count: o.get_field("attachmentCount"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            path: o.get_field("path"),
            path_prefix: o.get_field("pathPrefix"),
            policy: o.get_field("policy"),
            policy_id: o.get_field("policyId"),
            tags: o.get_field("tags"),
        }
    }
}
