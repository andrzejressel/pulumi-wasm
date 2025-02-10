/// Provides an IAM role inline policy.
///
/// > **NOTE:** For a given role, this resource is incompatible with using the `aws.iam.Role` resource `inline_policy` argument. When using that argument and this resource, both will attempt to manage the role's inline policies and the provider will show a permanent difference.
///
/// > **NOTE:** We suggest using explicit JSON encoding or `aws.iam.getPolicyDocument` when assigning a value to `policy`. They seamlessly translate configuration to JSON, enabling you to maintain consistency within your configuration without the need for context switches. Also, you can sidestep potential complications arising from formatting discrepancies, whitespace inconsistencies, and other nuances inherent to JSON.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testPolicy:
///     type: aws:iam:RolePolicy
///     name: test_policy
///     properties:
///       name: test_policy
///       role: ${testRole.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - ec2:Describe*
///               Effect: Allow
///               Resource: '*'
///   testRole:
///     type: aws:iam:Role
///     name: test_role
///     properties:
///       name: test_role
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: ec2.amazonaws.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Role Policies using the `role_name:role_policy_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/rolePolicy:RolePolicy mypolicy role_of_mypolicy_name:mypolicy_name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod role_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RolePolicyArgs {
        /// The name of the role policy. If omitted, this provider will
        /// assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The inline policy document. This is a JSON formatted string. For more information about building IAM policy documents with the provider, see the AWS IAM Policy Document Guide
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the IAM role to attach to the policy.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RolePolicyResult {
        /// The name of the role policy. If omitted, this provider will
        /// assign a random, unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The inline policy document. This is a JSON formatted string. For more information about building IAM policy documents with the provider, see the AWS IAM Policy Document Guide
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The name of the IAM role to attach to the policy.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RolePolicyArgs,
    ) -> RolePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/rolePolicy:RolePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RolePolicyResult {
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            policy: o.get_field("policy"),
            role: o.get_field("role"),
        }
    }
}
