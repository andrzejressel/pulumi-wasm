/// Creates an Amazon CloudFront origin access identity.
///
/// For information about CloudFront distributions, see the
/// [Amazon CloudFront Developer Guide](http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html). For more information on generating
/// origin access identities, see
/// [Using an Origin Access Identity to Restrict Access to Your Amazon S3 Content][2].
///
/// ## Example Usage
///
/// The following example below creates a CloudFront origin access identity.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = origin_access_identity::create(
///         "example",
///         OriginAccessIdentityArgs::builder().comment("Some comment").build_struct(),
///     );
/// }
/// ```
///
/// ## Using With CloudFront
///
/// Normally, when referencing an origin access identity in CloudFront, you need to
/// prefix the ID with the `origin-access-identity/cloudfront/` special path.
/// The `cloudfront_access_identity_path` allows this to be circumvented.
/// The below snippet demonstrates use with the `s3_origin_config` structure for the
/// `aws.cloudfront.Distribution` resource:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = distribution::create(
///         "example",
///         DistributionArgs::builder()
///             .origins(
///                 vec![
///                     DistributionOrigin::builder()
///                     .s3OriginConfig(DistributionOriginS3OriginConfig::builder()
///                     .originAccessIdentity("${exampleAwsCloudfrontOriginAccessIdentity.cloudfrontAccessIdentityPath}")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Updating your bucket policy
///
/// Note that the AWS API may translate the `s3_canonical_user_id` `CanonicalUser`
/// principal into an `AWS` IAM ARN principal when supplied in an
/// `aws.s3.BucketV2` bucket policy, causing spurious diffs. If
/// you see this behavior, use the `iam_arn` instead:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let s3Policy = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["s3:GetObject",])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["${exampleAwsCloudfrontOriginAccessIdentity.iamArn}",])
///                     . type ("AWS").build_struct(),])
///                     .resources(vec!["${exampleAwsS3Bucket.arn}/*",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let example = bucket_policy::create(
///         "example",
///         BucketPolicyArgs::builder()
///             .bucket("${exampleAwsS3Bucket.id}")
///             .policy("${s3Policy.json}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// [1]: http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html
/// [2]: http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html
///
/// ## Import
///
/// Using `pulumi import`, import Cloudfront Origin Access Identities using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/originAccessIdentity:OriginAccessIdentity origin_access E74FTE3AEXAMPLE
/// ```
pub mod origin_access_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OriginAccessIdentityArgs {
        /// An optional comment for the origin access identity.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OriginAccessIdentityResult {
        /// Internal value used by CloudFront to allow future
        /// updates to the origin access identity.
        pub caller_reference: pulumi_wasm_rust::Output<String>,
        /// A shortcut to the full path for the
        /// origin access identity to use in CloudFront, see below.
        pub cloudfront_access_identity_path: pulumi_wasm_rust::Output<String>,
        /// An optional comment for the origin access identity.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The current version of the origin access identity's information.
        /// For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// A pre-generated ARN for use in S3 bucket policies (see below).
        /// Example: `arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity
        /// E2QWRUHAPOMQZL`.
        pub iam_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon S3 canonical user ID for the origin
        /// access identity, which you use when giving the origin access identity read
        /// permission to an object in Amazon S3.
        pub s3_canonical_user_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OriginAccessIdentityArgs,
    ) -> OriginAccessIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/originAccessIdentity:OriginAccessIdentity".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "callerReference".into(),
                },
                register_interface::ResultField {
                    name: "cloudfrontAccessIdentityPath".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "iamArn".into(),
                },
                register_interface::ResultField {
                    name: "s3CanonicalUserId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OriginAccessIdentityResult {
            caller_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callerReference").unwrap(),
            ),
            cloudfront_access_identity_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudfrontAccessIdentityPath").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            iam_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamArn").unwrap(),
            ),
            s3_canonical_user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3CanonicalUserId").unwrap(),
            ),
        }
    }
}