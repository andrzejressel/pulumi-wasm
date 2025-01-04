/// Resource for managing an AWS Audit Manager Assessment Delegation.
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
///     let example = assessment_delegation::create(
///         "example",
///         AssessmentDelegationArgs::builder()
///             .assessment_id("${exampleAwsAuditmanagerAssessment.id}")
///             .control_set_id("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .role_type("RESOURCE_OWNER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Assessment Delegation using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/assessmentDelegation:AssessmentDelegation example abcdef-123456,arn:aws:iam::123456789012:role/example,example
/// ```
pub mod assessment_delegation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentDelegationArgs {
        /// Identifier for the assessment.
        #[builder(into)]
        pub assessment_id: pulumi_wasm_rust::Output<String>,
        /// Comment describing the delegation request.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Assessment control set name. This value is the control set name used during assessment creation (not the AWS-generated ID). The `_id` suffix on this attribute has been preserved to be consistent with the underlying AWS API.
        #[builder(into)]
        pub control_set_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM role.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Type of customer persona. For assessment delegation, type must always be `RESOURCE_OWNER`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub role_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AssessmentDelegationResult {
        /// Identifier for the assessment.
        pub assessment_id: pulumi_wasm_rust::Output<String>,
        /// Comment describing the delegation request.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Assessment control set name. This value is the control set name used during assessment creation (not the AWS-generated ID). The `_id` suffix on this attribute has been preserved to be consistent with the underlying AWS API.
        pub control_set_id: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the delegation.
        pub delegation_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM role.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Type of customer persona. For assessment delegation, type must always be `RESOURCE_OWNER`.
        ///
        /// The following arguments are optional:
        pub role_type: pulumi_wasm_rust::Output<String>,
        /// Status of the delegation.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AssessmentDelegationArgs,
    ) -> AssessmentDelegationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assessment_id_binding = args.assessment_id.get_inner();
        let comment_binding = args.comment.get_inner();
        let control_set_id_binding = args.control_set_id.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let role_type_binding = args.role_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/assessmentDelegation:AssessmentDelegation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assessmentId".into(),
                    value: &assessment_id_binding,
                },
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "controlSetId".into(),
                    value: &control_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "roleType".into(),
                    value: &role_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "assessmentId".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "controlSetId".into(),
                },
                register_interface::ResultField {
                    name: "delegationId".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "roleType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssessmentDelegationResult {
            assessment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assessmentId").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            control_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlSetId").unwrap(),
            ),
            delegation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegationId").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            role_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
