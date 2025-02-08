/// Provides an S3 object resource.
///
/// ## Example Usage
///
/// ### Uploading a file to a bucket
///
/// ```yaml
/// resources:
///   object:
///     type: aws:s3:BucketObject
///     properties:
///       bucket: your_bucket_name
///       key: new_object_key
///       source:
///         fn::FileAsset: path/to/file
///       etag:
///         fn::invoke:
///           function: std:filemd5
///           arguments:
///             input: path/to/file
///           return: result
/// ```
///
/// ### Encrypting with KMS Key
///
/// ```yaml
/// resources:
///   examplekms:
///     type: aws:kms:Key
///     properties:
///       description: KMS key 1
///       deletionWindowInDays: 7
///   examplebucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: examplebuckettftest
///   example:
///     type: aws:s3:BucketAclV2
///     properties:
///       bucket: ${examplebucket.id}
///       acl: private
///   exampleBucketObject:
///     type: aws:s3:BucketObject
///     name: example
///     properties:
///       key: someobject
///       bucket: ${examplebucket.id}
///       source:
///         fn::FileAsset: index.html
///       kmsKeyId: ${examplekms.arn}
/// ```
///
/// ### Server Side Encryption with S3 Default Master Key
///
/// ```yaml
/// resources:
///   examplebucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: examplebuckettftest
///   example:
///     type: aws:s3:BucketAclV2
///     properties:
///       bucket: ${examplebucket.id}
///       acl: private
///   exampleBucketObject:
///     type: aws:s3:BucketObject
///     name: example
///     properties:
///       key: someobject
///       bucket: ${examplebucket.id}
///       source:
///         fn::FileAsset: index.html
///       serverSideEncryption: aws:kms
/// ```
///
/// ### Server Side Encryption with AWS-Managed Key
///
/// ```yaml
/// resources:
///   examplebucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: examplebuckettftest
///   example:
///     type: aws:s3:BucketAclV2
///     properties:
///       bucket: ${examplebucket.id}
///       acl: private
///   exampleBucketObject:
///     type: aws:s3:BucketObject
///     name: example
///     properties:
///       key: someobject
///       bucket: ${examplebucket.id}
///       source:
///         fn::FileAsset: index.html
///       serverSideEncryption: AES256
/// ```
///
/// ### S3 Object Lock
///
/// ```yaml
/// resources:
///   examplebucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: examplebuckettftest
///       objectLockEnabled: true
///   example:
///     type: aws:s3:BucketAclV2
///     properties:
///       bucket: ${examplebucket.id}
///       acl: private
///   exampleBucketVersioningV2:
///     type: aws:s3:BucketVersioningV2
///     name: example
///     properties:
///       bucket: ${examplebucket.id}
///       versioningConfiguration:
///         status: Enabled
///   exampleBucketObject:
///     type: aws:s3:BucketObject
///     name: example
///     properties:
///       key: someobject
///       bucket: ${examplebucket.id}
///       source:
///         fn::FileAsset: important.txt
///       objectLockLegalHoldStatus: ON
///       objectLockMode: GOVERNANCE
///       objectLockRetainUntilDate: 2021-12-31T23:59:60Z
///       forceDestroy: true
///     options:
///       dependsOn:
///         - ${exampleBucketVersioningV2}
/// ```
///
/// ## Import
///
/// Import using S3 URL syntax:
///
/// __Using `pulumi import` to import__ objects using the `id` or S3 URL. For example:
///
/// Import using the `id`, which is the bucket name and the key together:
///
/// ```sh
/// $ pulumi import aws:s3/bucketObject:BucketObject example some-bucket-name/some/key.txt
/// ```
/// Import using S3 URL syntax:
///
/// ```sh
/// $ pulumi import aws:s3/bucketObject:BucketObject example s3://some-bucket-name/some/key.txt
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_object {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketObjectArgs {
        /// [Canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `aws-exec-read`, `authenticated-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Defaults to `private`.
        #[builder(into, default)]
        pub acl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the bucket to put the file in. Alternatively, an [S3 access point](https://docs.aws.amazon.com/AmazonS3/latest/dev/using-access-points.html) ARN can be specified.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether or not to use [Amazon S3 Bucket Keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html) for SSE-KMS.
        #[builder(into, default)]
        pub bucket_key_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Caching behavior along the request/reply chain Read [w3c cache_control](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9) for further details.
        #[builder(into, default)]
        pub cache_control: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Literal string value to use as the object content, which will be uploaded as UTF-8-encoded text.
        #[builder(into, default)]
        pub content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Base64-encoded data that will be decoded and uploaded as raw bytes for the object content. This allows safely uploading non-UTF8 binary data, but is recommended only for small content such as the result of the `gzipbase64` function with small text strings. For larger objects, use `source` to stream the content from a disk file.
        #[builder(into, default)]
        pub content_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Presentational information for the object. Read [w3c content_disposition](http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1) for further information.
        #[builder(into, default)]
        pub content_disposition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Content encodings that have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. Read [w3c content encoding](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11) for further information.
        #[builder(into, default)]
        pub content_encoding: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Language the content is in e.g., en-US or en-GB.
        #[builder(into, default)]
        pub content_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Standard MIME type describing the format of the object data, e.g., application/octet-stream. All Valid MIME Types are valid for this input.
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Triggers updates when the value changes. This attribute is not compatible with KMS encryption, `kms_key_id` or `server_side_encryption = "aws:kms"` (see `source_hash` instead).
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to allow the object to be deleted by removing any legal hold on any object version. Default is `false`. This value should be set to `true` only if the bucket has S3 object lock enabled.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the object once it is in the bucket.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the KMS Key to use for object encryption. If the S3 Bucket has server-side encryption enabled, that value will automatically be used. If referencing the `aws.kms.Key` resource, use the `arn` attribute. If referencing the `aws.kms.Alias` data source or resource, use the `target_key_arn` attribute. The provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of keys/values to provision metadata (will be automatically prefixed by `x-amz-meta-`, note that only lowercase label are currently supported by the AWS Go API).
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// [Legal hold](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-legal-holds) status that you want to apply to the specified object. Valid values are `ON` and `OFF`.
        #[builder(into, default)]
        pub object_lock_legal_hold_status: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Object lock [retention mode](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-modes) that you want to apply to this object. Valid values are `GOVERNANCE` and `COMPLIANCE`.
        #[builder(into, default)]
        pub object_lock_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Date and time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8), when this object's object lock will [expire](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-periods).
        #[builder(into, default)]
        pub object_lock_retain_until_date: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Server-side encryption of the object in S3. Valid values are "`AES256`" and "`aws:kms`".
        #[builder(into, default)]
        pub server_side_encryption: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path to a file that will be read and uploaded as raw bytes for the object content.
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Triggers updates like `etag` but useful to address `etag` encryption limitations.
        #[builder(into, default)]
        pub source_hash: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// [Storage Class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html#AmazonS3-PutObject-request-header-StorageClass) for the object. Defaults to "`STANDARD`".
        #[builder(into, default)]
        pub storage_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target URL for [website redirect](http://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html).
        ///
        /// If no content is provided through `source`, `content` or `content_base64`, then the object will be empty.
        #[builder(into, default)]
        pub website_redirect: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketObjectResult {
        /// [Canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `aws-exec-read`, `authenticated-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Defaults to `private`.
        pub acl: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the object.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the bucket to put the file in. Alternatively, an [S3 access point](https://docs.aws.amazon.com/AmazonS3/latest/dev/using-access-points.html) ARN can be specified.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Whether or not to use [Amazon S3 Bucket Keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html) for SSE-KMS.
        pub bucket_key_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Caching behavior along the request/reply chain Read [w3c cache_control](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9) for further details.
        pub cache_control: pulumi_gestalt_rust::Output<Option<String>>,
        /// Literal string value to use as the object content, which will be uploaded as UTF-8-encoded text.
        pub content: pulumi_gestalt_rust::Output<Option<String>>,
        /// Base64-encoded data that will be decoded and uploaded as raw bytes for the object content. This allows safely uploading non-UTF8 binary data, but is recommended only for small content such as the result of the `gzipbase64` function with small text strings. For larger objects, use `source` to stream the content from a disk file.
        pub content_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// Presentational information for the object. Read [w3c content_disposition](http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1) for further information.
        pub content_disposition: pulumi_gestalt_rust::Output<Option<String>>,
        /// Content encodings that have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. Read [w3c content encoding](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11) for further information.
        pub content_encoding: pulumi_gestalt_rust::Output<Option<String>>,
        /// Language the content is in e.g., en-US or en-GB.
        pub content_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Standard MIME type describing the format of the object data, e.g., application/octet-stream. All Valid MIME Types are valid for this input.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// Triggers updates when the value changes. This attribute is not compatible with KMS encryption, `kms_key_id` or `server_side_encryption = "aws:kms"` (see `source_hash` instead).
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Whether to allow the object to be deleted by removing any legal hold on any object version. Default is `false`. This value should be set to `true` only if the bucket has S3 object lock enabled.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the object once it is in the bucket.
        ///
        /// The following arguments are optional:
        pub key: pulumi_gestalt_rust::Output<String>,
        /// ARN of the KMS Key to use for object encryption. If the S3 Bucket has server-side encryption enabled, that value will automatically be used. If referencing the `aws.kms.Key` resource, use the `arn` attribute. If referencing the `aws.kms.Alias` data source or resource, use the `target_key_arn` attribute. The provider will only perform drift detection if a configuration value is provided.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Map of keys/values to provision metadata (will be automatically prefixed by `x-amz-meta-`, note that only lowercase label are currently supported by the AWS Go API).
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// [Legal hold](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-legal-holds) status that you want to apply to the specified object. Valid values are `ON` and `OFF`.
        pub object_lock_legal_hold_status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Object lock [retention mode](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-modes) that you want to apply to this object. Valid values are `GOVERNANCE` and `COMPLIANCE`.
        pub object_lock_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Date and time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8), when this object's object lock will [expire](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-periods).
        pub object_lock_retain_until_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// Server-side encryption of the object in S3. Valid values are "`AES256`" and "`aws:kms`".
        pub server_side_encryption: pulumi_gestalt_rust::Output<String>,
        /// Path to a file that will be read and uploaded as raw bytes for the object content.
        pub source: pulumi_gestalt_rust::Output<Option<String>>,
        /// Triggers updates like `etag` but useful to address `etag` encryption limitations.
        pub source_hash: pulumi_gestalt_rust::Output<Option<String>>,
        /// [Storage Class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html#AmazonS3-PutObject-request-header-StorageClass) for the object. Defaults to "`STANDARD`".
        pub storage_class: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Unique version ID value for the object, if bucket versioning is enabled.
        pub version_id: pulumi_gestalt_rust::Output<String>,
        /// Target URL for [website redirect](http://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html).
        ///
        /// If no content is provided through `source`, `content` or `content_base64`, then the object will be empty.
        pub website_redirect: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BucketObjectArgs,
    ) -> BucketObjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let acl_binding = args.acl.get_output(context).get_inner();
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let bucket_key_enabled_binding = args
            .bucket_key_enabled
            .get_output(context)
            .get_inner();
        let cache_control_binding = args.cache_control.get_output(context).get_inner();
        let content_binding = args.content.get_output(context).get_inner();
        let content_base64_binding = args.content_base64.get_output(context).get_inner();
        let content_disposition_binding = args
            .content_disposition
            .get_output(context)
            .get_inner();
        let content_encoding_binding = args
            .content_encoding
            .get_output(context)
            .get_inner();
        let content_language_binding = args
            .content_language
            .get_output(context)
            .get_inner();
        let content_type_binding = args.content_type.get_output(context).get_inner();
        let etag_binding = args.etag.get_output(context).get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
        let key_binding = args.key.get_output(context).get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let object_lock_legal_hold_status_binding = args
            .object_lock_legal_hold_status
            .get_output(context)
            .get_inner();
        let object_lock_mode_binding = args
            .object_lock_mode
            .get_output(context)
            .get_inner();
        let object_lock_retain_until_date_binding = args
            .object_lock_retain_until_date
            .get_output(context)
            .get_inner();
        let server_side_encryption_binding = args
            .server_side_encryption
            .get_output(context)
            .get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let source_hash_binding = args.source_hash.get_output(context).get_inner();
        let storage_class_binding = args.storage_class.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let website_redirect_binding = args
            .website_redirect
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketObject:BucketObject".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acl".into(),
                    value: &acl_binding,
                },
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "bucketKeyEnabled".into(),
                    value: &bucket_key_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "cacheControl".into(),
                    value: &cache_control_binding,
                },
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "contentBase64".into(),
                    value: &content_base64_binding,
                },
                register_interface::ObjectField {
                    name: "contentDisposition".into(),
                    value: &content_disposition_binding,
                },
                register_interface::ObjectField {
                    name: "contentEncoding".into(),
                    value: &content_encoding_binding,
                },
                register_interface::ObjectField {
                    name: "contentLanguage".into(),
                    value: &content_language_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "objectLockLegalHoldStatus".into(),
                    value: &object_lock_legal_hold_status_binding,
                },
                register_interface::ObjectField {
                    name: "objectLockMode".into(),
                    value: &object_lock_mode_binding,
                },
                register_interface::ObjectField {
                    name: "objectLockRetainUntilDate".into(),
                    value: &object_lock_retain_until_date_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryption".into(),
                    value: &server_side_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "sourceHash".into(),
                    value: &source_hash_binding,
                },
                register_interface::ObjectField {
                    name: "storageClass".into(),
                    value: &storage_class_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "websiteRedirect".into(),
                    value: &website_redirect_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BucketObjectResult {
            acl: pulumi_gestalt_rust::__private::into_domain(o.extract_field("acl")),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            bucket_key_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucketKeyEnabled"),
            ),
            cache_control: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheControl"),
            ),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            content_base64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentBase64"),
            ),
            content_disposition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentDisposition"),
            ),
            content_encoding: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentEncoding"),
            ),
            content_language: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentLanguage"),
            ),
            content_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            force_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            object_lock_legal_hold_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectLockLegalHoldStatus"),
            ),
            object_lock_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectLockMode"),
            ),
            object_lock_retain_until_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectLockRetainUntilDate"),
            ),
            server_side_encryption: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverSideEncryption"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            source_hash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceHash"),
            ),
            storage_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageClass"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionId"),
            ),
            website_redirect: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("websiteRedirect"),
            ),
        }
    }
}
