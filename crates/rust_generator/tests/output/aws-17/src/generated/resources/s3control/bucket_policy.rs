/// Provides a resource to manage an S3 Control Bucket Policy.
///
/// > This functionality is for managing [S3 on Outposts](https://docs.aws.amazon.com/AmazonS3/latest/dev/S3onOutposts.html). To manage S3 Bucket Policies in an AWS Partition, see the `aws.s3.BucketPolicy` resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3control:BucketPolicy
///     properties:
///       bucket: ${exampleAwsS3controlBucket.arn}
///       policy:
///         fn::toJSON:
///           Id: testBucketPolicy
///           Statement:
///             - Action: s3-outposts:PutBucketLifecycleConfiguration
///               Effect: Deny
///               Principal:
///                 AWS: '*'
///               Resource: ${exampleAwsS3controlBucket.arn}
///               Sid: statement1
///           Version: 2012-10-17
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Control Bucket Policies using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/bucketPolicy:BucketPolicy example arn:aws:s3-outposts:us-east-1:123456789012:outpost/op-12345678/bucket/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketPolicyArgs {
        /// Amazon Resource Name (ARN) of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// JSON string of the resource policy.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketPolicyResult {
        /// Amazon Resource Name (ARN) of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// JSON string of the resource policy.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketPolicyArgs,
    ) -> BucketPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/bucketPolicy:BucketPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketPolicyResult {
            bucket: o.get_field("bucket"),
            policy: o.get_field("policy"),
        }
    }
}
