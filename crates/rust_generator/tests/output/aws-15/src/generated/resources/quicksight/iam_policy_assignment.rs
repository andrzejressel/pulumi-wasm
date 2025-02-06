/// Resource for managing an AWS QuickSight IAM Policy Assignment.
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
///     let example = iam_policy_assignment::create(
///         "example",
///         IamPolicyAssignmentArgs::builder()
///             .assignment_name("example")
///             .assignment_status("ENABLED")
///             .identities(
///                 IamPolicyAssignmentIdentities::builder()
///                     .users(vec!["${exampleAwsQuicksightUser.userName}",])
///                     .build_struct(),
///             )
///             .policy_arn("${exampleAwsIamPolicy.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight IAM Policy Assignment using the AWS account ID, namespace, and assignment name separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/iamPolicyAssignment:IamPolicyAssignment example 123456789012,default,example
/// ```
pub mod iam_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IamPolicyAssignmentArgs {
        /// Name of the assignment.
        #[builder(into)]
        pub assignment_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Status of the assignment. Valid values are `ENABLED`, `DISABLED`, and `DRAFT`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub assignment_status: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon QuickSight users, groups, or both to assign the policy to. See `identities` block.
        #[builder(into, default)]
        pub identities: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::IamPolicyAssignmentIdentities>,
        >,
        /// Namespace that contains the assignment. Defaults to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the IAM policy to apply to the Amazon QuickSight users and groups specified in this assignment.
        #[builder(into, default)]
        pub policy_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IamPolicyAssignmentResult {
        /// Assignment ID.
        pub assignment_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the assignment.
        pub assignment_name: pulumi_gestalt_rust::Output<String>,
        /// Status of the assignment. Valid values are `ENABLED`, `DISABLED`, and `DRAFT`.
        ///
        /// The following arguments are optional:
        pub assignment_status: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon QuickSight users, groups, or both to assign the policy to. See `identities` block.
        pub identities: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::IamPolicyAssignmentIdentities>,
        >,
        /// Namespace that contains the assignment. Defaults to `default`.
        pub namespace: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM policy to apply to the Amazon QuickSight users and groups specified in this assignment.
        pub policy_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IamPolicyAssignmentArgs,
    ) -> IamPolicyAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let assignment_name_binding = args
            .assignment_name
            .get_output(context)
            .get_inner();
        let assignment_status_binding = args
            .assignment_status
            .get_output(context)
            .get_inner();
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let identities_binding = args.identities.get_output(context).get_inner();
        let namespace_binding = args.namespace.get_output(context).get_inner();
        let policy_arn_binding = args.policy_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/iamPolicyAssignment:IamPolicyAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assignmentName".into(),
                    value: &assignment_name_binding,
                },
                register_interface::ObjectField {
                    name: "assignmentStatus".into(),
                    value: &assignment_status_binding,
                },
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "identities".into(),
                    value: &identities_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "policyArn".into(),
                    value: &policy_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IamPolicyAssignmentResult {
            assignment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assignmentId"),
            ),
            assignment_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assignmentName"),
            ),
            assignment_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assignmentStatus"),
            ),
            aws_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            namespace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespace"),
            ),
            policy_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyArn"),
            ),
        }
    }
}
