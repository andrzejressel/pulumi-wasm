/// Resource for managing an AWS Audit Manager Assessment Delegation.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assessment_delegation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentDelegationArgs {
        /// Identifier for the assessment.
        #[builder(into)]
        pub assessment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Comment describing the delegation request.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Assessment control set name. This value is the control set name used during assessment creation (not the AWS-generated ID). The `_id` suffix on this attribute has been preserved to be consistent with the underlying AWS API.
        #[builder(into)]
        pub control_set_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Resource Name (ARN) of the IAM role.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of customer persona. For assessment delegation, type must always be `RESOURCE_OWNER`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub role_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AssessmentDelegationResult {
        /// Identifier for the assessment.
        pub assessment_id: pulumi_gestalt_rust::Output<String>,
        /// Comment describing the delegation request.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Assessment control set name. This value is the control set name used during assessment creation (not the AWS-generated ID). The `_id` suffix on this attribute has been preserved to be consistent with the underlying AWS API.
        pub control_set_id: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the delegation.
        pub delegation_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM role.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Type of customer persona. For assessment delegation, type must always be `RESOURCE_OWNER`.
        ///
        /// The following arguments are optional:
        pub role_type: pulumi_gestalt_rust::Output<String>,
        /// Status of the delegation.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssessmentDelegationArgs,
    ) -> AssessmentDelegationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assessment_id_binding = args.assessment_id.get_output(context);
        let comment_binding = args.comment.get_output(context);
        let control_set_id_binding = args.control_set_id.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let role_type_binding = args.role_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:auditmanager/assessmentDelegation:AssessmentDelegation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assessmentId".into(),
                    value: &assessment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlSetId".into(),
                    value: &control_set_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleType".into(),
                    value: &role_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssessmentDelegationResult {
            assessment_id: o.get_field("assessmentId"),
            comment: o.get_field("comment"),
            control_set_id: o.get_field("controlSetId"),
            delegation_id: o.get_field("delegationId"),
            role_arn: o.get_field("roleArn"),
            role_type: o.get_field("roleType"),
            status: o.get_field("status"),
        }
    }
}
