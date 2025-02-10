/// Provides a resource to manage an S3 Object Lambda Access Point resource policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///   exampleAccessPoint:
///     type: aws:s3:AccessPoint
///     name: example
///     properties:
///       bucket: ${example.id}
///       name: example
///   exampleObjectLambdaAccessPoint:
///     type: aws:s3control:ObjectLambdaAccessPoint
///     name: example
///     properties:
///       name: example
///       configuration:
///         supportingAccessPoint: ${exampleAccessPoint.arn}
///         transformationConfigurations:
///           - actions:
///               - GetObject
///             contentTransformation:
///               awsLambda:
///                 functionArn: ${exampleAwsLambdaFunction.arn}
///   exampleObjectLambdaAccessPointPolicy:
///     type: aws:s3control:ObjectLambdaAccessPointPolicy
///     name: example
///     properties:
///       name: ${exampleObjectLambdaAccessPoint.name}
///       policy:
///         fn::toJSON:
///           Version: 2008-10-17
///           Statement:
///             - Effect: Allow
///               Action: s3-object-lambda:GetObject
///               Principal:
///                 AWS: ${current.accountId}
///               Resource: ${exampleObjectLambdaAccessPoint.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Object Lambda Access Point policies using the `account_id` and `name`, separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/objectLambdaAccessPointPolicy:ObjectLambdaAccessPointPolicy example 123456789012:example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod object_lambda_access_point_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectLambdaAccessPointPolicyArgs {
        /// The AWS account ID for the account that owns the Object Lambda Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Object Lambda Access Point.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Object Lambda Access Point resource policy document.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ObjectLambdaAccessPointPolicyResult {
        /// The AWS account ID for the account that owns the Object Lambda Access Point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether this access point currently has a policy that allows public access.
        pub has_public_access_policy: pulumi_gestalt_rust::Output<bool>,
        /// The name of the Object Lambda Access Point.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Object Lambda Access Point resource policy document.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObjectLambdaAccessPointPolicyArgs,
    ) -> ObjectLambdaAccessPointPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/objectLambdaAccessPointPolicy:ObjectLambdaAccessPointPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ObjectLambdaAccessPointPolicyResult {
            account_id: o.get_field("accountId"),
            has_public_access_policy: o.get_field("hasPublicAccessPolicy"),
            name: o.get_field("name"),
            policy: o.get_field("policy"),
        }
    }
}
