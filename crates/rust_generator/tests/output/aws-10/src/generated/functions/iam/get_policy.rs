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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPolicyArgs,
    ) -> GetPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let path_prefix_binding = args.path_prefix.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getPolicy:getPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pathPrefix".into(),
                    value: &path_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPolicyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attachment_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachmentCount"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            path_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pathPrefix"),
            ),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
