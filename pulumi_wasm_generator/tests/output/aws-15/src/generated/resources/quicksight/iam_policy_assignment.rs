/// Resource for managing an AWS QuickSight IAM Policy Assignment.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IamPolicyAssignmentArgs {
        /// Name of the assignment.
        #[builder(into)]
        pub assignment_name: pulumi_wasm_rust::Output<String>,
        /// Status of the assignment. Valid values are `ENABLED`, `DISABLED`, and `DRAFT`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub assignment_status: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon QuickSight users, groups, or both to assign the policy to. See `identities` block.
        #[builder(into, default)]
        pub identities: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::IamPolicyAssignmentIdentities>,
        >,
        /// Namespace that contains the assignment. Defaults to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the IAM policy to apply to the Amazon QuickSight users and groups specified in this assignment.
        #[builder(into, default)]
        pub policy_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IamPolicyAssignmentResult {
        /// Assignment ID.
        pub assignment_id: pulumi_wasm_rust::Output<String>,
        /// Name of the assignment.
        pub assignment_name: pulumi_wasm_rust::Output<String>,
        /// Status of the assignment. Valid values are `ENABLED`, `DISABLED`, and `DRAFT`.
        ///
        /// The following arguments are optional:
        pub assignment_status: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// Amazon QuickSight users, groups, or both to assign the policy to. See `identities` block.
        pub identities: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::IamPolicyAssignmentIdentities>,
        >,
        /// Namespace that contains the assignment. Defaults to `default`.
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM policy to apply to the Amazon QuickSight users and groups specified in this assignment.
        pub policy_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IamPolicyAssignmentArgs,
    ) -> IamPolicyAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assignment_name_binding = args.assignment_name.get_inner();
        let assignment_status_binding = args.assignment_status.get_inner();
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let identities_binding = args.identities.get_inner();
        let namespace_binding = args.namespace.get_inner();
        let policy_arn_binding = args.policy_arn.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "assignmentId".into(),
                },
                register_interface::ResultField {
                    name: "assignmentName".into(),
                },
                register_interface::ResultField {
                    name: "assignmentStatus".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
                },
                register_interface::ResultField {
                    name: "policyArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IamPolicyAssignmentResult {
            assignment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignmentId").unwrap(),
            ),
            assignment_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignmentName").unwrap(),
            ),
            assignment_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignmentStatus").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
            policy_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyArn").unwrap(),
            ),
        }
    }
}
