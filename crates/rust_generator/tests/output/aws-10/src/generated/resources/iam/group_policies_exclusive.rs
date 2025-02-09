///
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of inline policy assignments using the `group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/groupPoliciesExclusive:GroupPoliciesExclusive example MyGroup
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_policies_exclusive {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPoliciesExclusiveArgs {
        /// IAM group name.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of inline policy names to be assigned to the group. Policies attached to this group but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_names: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupPoliciesExclusiveResult {
        /// IAM group name.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of inline policy names to be assigned to the group. Policies attached to this group but not configured in this argument will be removed.
        pub policy_names: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GroupPoliciesExclusiveArgs,
    ) -> GroupPoliciesExclusiveResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let group_name_binding_1 = args.group_name.get_output(context);
        let group_name_binding = group_name_binding_1.get_inner();
        let policy_names_binding_1 = args.policy_names.get_output(context);
        let policy_names_binding = policy_names_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/groupPoliciesExclusive:GroupPoliciesExclusive".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "policyNames".into(),
                    value: &policy_names_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupPoliciesExclusiveResult {
            group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupName"),
            ),
            policy_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyNames"),
            ),
        }
    }
}
