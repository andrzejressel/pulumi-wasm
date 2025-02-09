///
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of managed IAM policy assignments using the `user_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/userPolicyAttachmentsExclusive:UserPolicyAttachmentsExclusive example MyUser
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_policy_attachments_exclusive {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPolicyAttachmentsExclusiveArgs {
        /// A list of managed IAM policy ARNs to be attached to the user. Policies attached to this user but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// IAM user name.
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserPolicyAttachmentsExclusiveResult {
        /// A list of managed IAM policy ARNs to be attached to the user. Policies attached to this user but not configured in this argument will be removed.
        pub policy_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// IAM user name.
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: UserPolicyAttachmentsExclusiveArgs,
    ) -> UserPolicyAttachmentsExclusiveResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let policy_arns_binding_1 = args.policy_arns.get_output(context);
        let policy_arns_binding = policy_arns_binding_1.get_inner();
        let user_name_binding_1 = args.user_name.get_output(context);
        let user_name_binding = user_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/userPolicyAttachmentsExclusive:UserPolicyAttachmentsExclusive"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyArns".into(),
                    value: &policy_arns_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserPolicyAttachmentsExclusiveResult {
            policy_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyArns"),
            ),
            user_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
        }
    }
}
