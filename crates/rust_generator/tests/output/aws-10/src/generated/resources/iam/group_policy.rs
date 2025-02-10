/// Provides an IAM policy attached to a group.
///
/// > **NOTE:** We suggest using explicit JSON encoding or `aws.iam.getPolicyDocument` when assigning a value to `policy`. They seamlessly translate configuration to JSON, enabling you to maintain consistency within your configuration without the need for context switches. Also, you can sidestep potential complications arising from formatting discrepancies, whitespace inconsistencies, and other nuances inherent to JSON.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myDeveloperPolicy:
///     type: aws:iam:GroupPolicy
///     name: my_developer_policy
///     properties:
///       name: my_developer_policy
///       group: ${myDevelopers.name}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - ec2:Describe*
///               Effect: Allow
///               Resource: '*'
///   myDevelopers:
///     type: aws:iam:Group
///     name: my_developers
///     properties:
///       name: developers
///       path: /users/
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Group Policies using the `group_name:group_policy_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/groupPolicy:GroupPolicy mypolicy group_of_mypolicy_name:mypolicy_name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyArgs {
        /// The IAM group to attach to the policy.
        #[builder(into)]
        pub group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the policy. If omitted, the provider will
        /// assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy document. This is a JSON formatted string.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupPolicyResult {
        /// The IAM group to attach to the policy.
        pub group: pulumi_gestalt_rust::Output<String>,
        /// The name of the policy. If omitted, the provider will
        /// assign a random, unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The policy document. This is a JSON formatted string.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupPolicyArgs,
    ) -> GroupPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_binding = args.group.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/groupPolicy:GroupPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupPolicyResult {
            group: o.get_field("group"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            policy: o.get_field("policy"),
        }
    }
}
