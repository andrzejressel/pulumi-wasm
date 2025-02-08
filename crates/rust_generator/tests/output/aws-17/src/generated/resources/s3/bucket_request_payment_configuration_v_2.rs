/// Provides an S3 bucket request payment configuration resource. For more information, see [Requester Pays Buckets](https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html).
///
/// > **NOTE:** Destroying an `aws.s3.BucketRequestPaymentConfigurationV2` resource resets the bucket's `payer` to the S3 default: the bucket owner.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_request_payment_configuration_v_2::create(
///         "example",
///         BucketRequestPaymentConfigurationV2Args::builder()
///             .bucket("${exampleAwsS3Bucket.id}")
///             .payer("Requester")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// __Using `pulumi import` to import__ S3 bucket request payment configuration using the `bucket` or using the `bucket` and `expected_bucket_owner` separated by a comma (`,`). For example:
///
/// If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketRequestPaymentConfigurationV2:BucketRequestPaymentConfigurationV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketRequestPaymentConfigurationV2:BucketRequestPaymentConfigurationV2 example bucket-name,123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_request_payment_configuration_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketRequestPaymentConfigurationV2Args {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies who pays for the download and request fees. Valid values: `BucketOwner`, `Requester`.
        #[builder(into)]
        pub payer: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketRequestPaymentConfigurationV2Result {
        /// Name of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies who pays for the download and request fees. Valid values: `BucketOwner`, `Requester`.
        pub payer: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BucketRequestPaymentConfigurationV2Args,
    ) -> BucketRequestPaymentConfigurationV2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context)
            .get_inner();
        let payer_binding = args.payer.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketRequestPaymentConfigurationV2:BucketRequestPaymentConfigurationV2"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding,
                },
                register_interface::ObjectField {
                    name: "payer".into(),
                    value: &payer_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BucketRequestPaymentConfigurationV2Result {
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            expected_bucket_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expectedBucketOwner"),
            ),
            payer: pulumi_gestalt_rust::__private::into_domain(o.extract_field("payer")),
        }
    }
}
