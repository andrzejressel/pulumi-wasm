/// Provides a lightsail bucket access key. This is a set of credentials that allow API requests to be made to the lightsail bucket.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Bucket
///     properties:
///       name: mytestbucket
///       bundleId: small_1_0
///   testLightsailBucketAccessKeyAccessKey:
///     type: aws:lightsailBucketAccessKeyAccessKey
///     name: test
///     properties:
///       bucketName: ${testAwsLightsailBucketAccessKey.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_bucket_access_key` using the `id` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/bucketAccessKey:BucketAccessKey test example-bucket,AKIAIOSFODNN7EXAMPLE
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_access_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketAccessKeyArgs {
        /// The name of the bucket that the new access key will belong to, and grant access to.
        #[builder(into)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketAccessKeyResult {
        /// The ID of the access key.
        pub access_key_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the bucket that the new access key will belong to, and grant access to.
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the access key was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The secret access key used to sign requests. This attribute is not available for imported resources. Note that this will be written to the state file.
        pub secret_access_key: pulumi_gestalt_rust::Output<String>,
        /// The status of the access key.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketAccessKeyArgs,
    ) -> BucketAccessKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_name_binding = args.bucket_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/bucketAccessKey:BucketAccessKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketAccessKeyResult {
            access_key_id: o.get_field("accessKeyId"),
            bucket_name: o.get_field("bucketName"),
            created_at: o.get_field("createdAt"),
            secret_access_key: o.get_field("secretAccessKey"),
            status: o.get_field("status"),
        }
    }
}
