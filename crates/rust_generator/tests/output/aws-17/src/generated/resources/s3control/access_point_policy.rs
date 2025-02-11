/// Provides a resource to manage an S3 Access Point resource policy.
///
/// > **NOTE on Access Points and Access Point Policies:** The provider provides both a standalone Access Point Policy resource and an Access Point resource with a resource policy defined in-line. You cannot use an Access Point with in-line resource policy in conjunction with an Access Point Policy resource. Doing so will cause a conflict of policies and will overwrite the access point's resource policy.
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
///       publicAccessBlockConfiguration:
///         blockPublicAcls: true
///         blockPublicPolicy: false
///         ignorePublicAcls: true
///         restrictPublicBuckets: false
///   exampleAccessPointPolicy:
///     type: aws:s3control:AccessPointPolicy
///     name: example
///     properties:
///       accessPointArn: ${exampleAccessPoint.arn}
///       policy:
///         fn::toJSON:
///           Version: 2008-10-17
///           Statement:
///             - Effect: Allow
///               Action: s3:GetObjectTagging
///               Principal:
///                 AWS: '*'
///               Resource: ${exampleAccessPoint.arn}/object/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Access Point policies using the `access_point_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessPointPolicy:AccessPointPolicy example arn:aws:s3:us-west-2:123456789012:accesspoint/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_point_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPointPolicyArgs {
        /// The ARN of the access point that you want to associate with the specified policy.
        #[builder(into)]
        pub access_point_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The policy that you want to apply to the specified access point.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessPointPolicyResult {
        /// The ARN of the access point that you want to associate with the specified policy.
        pub access_point_arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether this access point currently has a policy that allows public access.
        pub has_public_access_policy: pulumi_gestalt_rust::Output<bool>,
        /// The policy that you want to apply to the specified access point.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessPointPolicyArgs,
    ) -> AccessPointPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_point_arn_binding = args.access_point_arn.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/accessPointPolicy:AccessPointPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessPointArn".into(),
                    value: &access_point_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessPointPolicyResult {
            access_point_arn: o.get_field("accessPointArn"),
            has_public_access_policy: o.get_field("hasPublicAccessPolicy"),
            policy: o.get_field("policy"),
        }
    }
}
