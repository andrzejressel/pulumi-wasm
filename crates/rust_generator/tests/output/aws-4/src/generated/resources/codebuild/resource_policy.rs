/// Provides a CodeBuild Resource Policy Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:codebuild:ReportGroup
///     properties:
///       name: example
///       type: TEST
///       exportConfig:
///         type: NO_EXPORT
///   exampleResourcePolicy:
///     type: aws:codebuild:ResourcePolicy
///     name: example
///     properties:
///       resourceArn: ${example.arn}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Id: default
///           Statement:
///             - Sid: default
///               Effect: Allow
///               Principal:
///                 AWS: arn:${current.partition}:iam::${currentGetCallerIdentity.accountId}:root
///               Action:
///                 - codebuild:BatchGetReportGroups
///                 - codebuild:BatchGetReports
///                 - codebuild:ListReportsForReportGroup
///                 - codebuild:DescribeTestCases
///               Resource: ${example.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeBuild Resource Policy using the CodeBuild Resource Policy arn. For example:
///
/// ```sh
/// $ pulumi import aws:codebuild/resourcePolicy:ResourcePolicy example arn:aws:codebuild:us-west-2:123456789:report-group/report-group-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// A JSON-formatted resource policy. For more information, see [Sharing a Projec](https://docs.aws.amazon.com/codebuild/latest/userguide/project-sharing.html#project-sharing-share) and [Sharing a Report Group](https://docs.aws.amazon.com/codebuild/latest/userguide/report-groups-sharing.html#report-groups-sharing-share).
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the Project or ReportGroup resource you want to associate with a resource policy.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// A JSON-formatted resource policy. For more information, see [Sharing a Projec](https://docs.aws.amazon.com/codebuild/latest/userguide/project-sharing.html#project-sharing-share) and [Sharing a Report Group](https://docs.aws.amazon.com/codebuild/latest/userguide/report-groups-sharing.html#report-groups-sharing-share).
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Project or ReportGroup resource you want to associate with a resource policy.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourcePolicyArgs,
    ) -> ResourcePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codebuild/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: resource_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourcePolicyResult {
            policy: o.get_field("policy"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}
