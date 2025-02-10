/// > **NOTE:**: To reliably detect drift between customer managed inline policies listed in this resource and actual policies attached to the role in the cloud, you currently need to run Pulumi with `pulumi up --refresh`. See [#4766](https://github.com/pulumi/pulumi-aws/issues/4766) for tracking making this work with regular `pulumi up` invocations.
///
/// Resource for maintaining exclusive management of inline policies assigned to an AWS IAM (Identity & Access Management) role.
///
/// !> This resource takes exclusive ownership over inline policies assigned to a role. This includes removal of inline policies which are not explicitly configured. To prevent persistent drift, ensure any `aws.iam.RolePolicy` resources managed alongside this resource are included in the `policy_names` argument.
///
/// > Destruction of this resource means Pulumi will no longer manage reconciliation of the configured inline policy assignments. It __will not__ delete the configured policies from the role.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = role_policies_exclusive::create(
///         "example",
///         RolePoliciesExclusiveArgs::builder()
///             .policy_names(vec!["${exampleAwsIamRolePolicy.name}",])
///             .role_name("${exampleAwsIamRole.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Disallow Inline Policies
///
/// To automatically remove any configured inline policies, set the `policy_names` argument to an empty list.
///
/// > This will not __prevent__ inline policies from being assigned to a role via Pulumi (or any other interface). This resource enables bringing inline policy assignments into a configured state, however, this reconciliation happens only when `apply` is proactively run.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = role_policies_exclusive::create(
///         "example",
///         RolePoliciesExclusiveArgs::builder()
///             .policy_names(vec![])
///             .role_name("${exampleAwsIamRole.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of inline policy assignments using the `role_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/rolePoliciesExclusive:RolePoliciesExclusive example MyRole
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod role_policies_exclusive {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RolePoliciesExclusiveArgs {
        /// A list of inline policy names to be assigned to the role. Policies attached to this role but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_names: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// IAM role name.
        #[builder(into)]
        pub role_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RolePoliciesExclusiveResult {
        /// A list of inline policy names to be assigned to the role. Policies attached to this role but not configured in this argument will be removed.
        pub policy_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// IAM role name.
        pub role_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RolePoliciesExclusiveArgs,
    ) -> RolePoliciesExclusiveResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_names_binding = args.policy_names.get_output(context);
        let role_name_binding = args.role_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/rolePoliciesExclusive:RolePoliciesExclusive".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyNames".into(),
                    value: policy_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleName".into(),
                    value: role_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RolePoliciesExclusiveResult {
            policy_names: o.get_field("policyNames"),
            role_name: o.get_field("roleName"),
        }
    }
}
