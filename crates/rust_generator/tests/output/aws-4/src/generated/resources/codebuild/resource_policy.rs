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
pub mod resource_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// A JSON-formatted resource policy. For more information, see [Sharing a Projec](https://docs.aws.amazon.com/codebuild/latest/userguide/project-sharing.html#project-sharing-share) and [Sharing a Report Group](https://docs.aws.amazon.com/codebuild/latest/userguide/report-groups-sharing.html#report-groups-sharing-share).
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ARN of the Project or ReportGroup resource you want to associate with a resource policy.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// A JSON-formatted resource policy. For more information, see [Sharing a Projec](https://docs.aws.amazon.com/codebuild/latest/userguide/project-sharing.html#project-sharing-share) and [Sharing a Report Group](https://docs.aws.amazon.com/codebuild/latest/userguide/report-groups-sharing.html#report-groups-sharing-share).
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The ARN of the Project or ReportGroup resource you want to associate with a resource policy.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResourcePolicyArgs,
    ) -> ResourcePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codebuild/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourcePolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
        }
    }
}
