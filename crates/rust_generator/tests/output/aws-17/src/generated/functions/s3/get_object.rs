#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_object {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetObjectArgs {
        /// Name of the bucket to read the object from. Alternatively, an [S3 access point](https://docs.aws.amazon.com/AmazonS3/latest/dev/using-access-points.html) ARN can be specified
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// To retrieve the object's checksum, this argument must be `ENABLED`. If you enable `checksum_mode` and the object is encrypted with KMS, you must have permission to use the `kms:Decrypt` action. Valid values: `ENABLED`
        #[builder(into, default)]
        pub checksum_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Full path to the object inside the bucket
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the object.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specific version ID of the object returned (defaults to latest version)
        #[builder(into, default)]
        pub version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetObjectResult {
        /// ARN of the object.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Object data (see **limitations above** to understand cases in which this field is actually available)
        pub body: pulumi_gestalt_rust::Output<String>,
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// (Optional) Whether or not to use [Amazon S3 Bucket Keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html) for SSE-KMS.
        pub bucket_key_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Caching behavior along the request/reply chain.
        pub cache_control: pulumi_gestalt_rust::Output<String>,
        /// The base64-encoded, 32-bit CRC32 checksum of the object.
        pub checksum_crc32: pulumi_gestalt_rust::Output<String>,
        /// The base64-encoded, 32-bit CRC32C checksum of the object.
        pub checksum_crc32c: pulumi_gestalt_rust::Output<String>,
        pub checksum_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The base64-encoded, 160-bit SHA-1 digest of the object.
        pub checksum_sha1: pulumi_gestalt_rust::Output<String>,
        /// The base64-encoded, 256-bit SHA-256 digest of the object.
        pub checksum_sha256: pulumi_gestalt_rust::Output<String>,
        /// Presentational information for the object.
        pub content_disposition: pulumi_gestalt_rust::Output<String>,
        /// What content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.
        pub content_encoding: pulumi_gestalt_rust::Output<String>,
        /// Language the content is in.
        pub content_language: pulumi_gestalt_rust::Output<String>,
        /// Size of the body in bytes.
        pub content_length: pulumi_gestalt_rust::Output<i32>,
        /// Standard MIME type describing the format of the object data.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// [ETag](https://en.wikipedia.org/wiki/HTTP_ETag) generated for the object (an MD5 sum of the object content in case it's not encrypted)
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// If the object expiration is configured (see [object lifecycle management](http://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html)), the field includes this header. It includes the expiry-date and rule-id key value pairs providing object expiration information. The value of the rule-id is URL encoded.
        pub expiration: pulumi_gestalt_rust::Output<String>,
        /// Date and time at which the object is no longer cacheable.
        pub expires: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key: pulumi_gestalt_rust::Output<String>,
        /// Last modified date of the object in RFC1123 format (e.g., `Mon, 02 Jan 2006 15:04:05 MST`)
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// Map of metadata stored with the object in S3. Keys are always returned in lowercase.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates whether this object has an active [legal hold](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-legal-holds). This field is only returned if you have permission to view an object's legal hold status.
        pub object_lock_legal_hold_status: pulumi_gestalt_rust::Output<String>,
        /// Object lock [retention mode](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-modes) currently in place for this object.
        pub object_lock_mode: pulumi_gestalt_rust::Output<String>,
        /// The date and time when this object's object lock will expire.
        pub object_lock_retain_until_date: pulumi_gestalt_rust::Output<String>,
        pub range: pulumi_gestalt_rust::Output<Option<String>>,
        /// If the object is stored using server-side encryption (KMS or Amazon S3-managed encryption key), this field includes the chosen encryption and algorithm used.
        pub server_side_encryption: pulumi_gestalt_rust::Output<String>,
        /// If present, specifies the ID of the Key Management Service (KMS) master encryption key that was used for the object.
        pub sse_kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// [Storage class](http://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html) information of the object. Available for all objects except for `Standard` storage class objects.
        pub storage_class: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the object.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Latest version ID of the object returned.
        pub version_id: pulumi_gestalt_rust::Output<String>,
        /// If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.
        pub website_redirect_location: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetObjectArgs,
    ) -> GetObjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let checksum_mode_binding = args.checksum_mode.get_output(context);
        let key_binding = args.key.get_output(context);
        let range_binding = args.range.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let version_id_binding = args.version_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:s3/getObject:getObject".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "checksumMode".into(),
                    value: &checksum_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: &key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "range".into(),
                    value: &range_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetObjectResult {
            arn: o.get_field("arn"),
            body: o.get_field("body"),
            bucket: o.get_field("bucket"),
            bucket_key_enabled: o.get_field("bucketKeyEnabled"),
            cache_control: o.get_field("cacheControl"),
            checksum_crc32: o.get_field("checksumCrc32"),
            checksum_crc32c: o.get_field("checksumCrc32c"),
            checksum_mode: o.get_field("checksumMode"),
            checksum_sha1: o.get_field("checksumSha1"),
            checksum_sha256: o.get_field("checksumSha256"),
            content_disposition: o.get_field("contentDisposition"),
            content_encoding: o.get_field("contentEncoding"),
            content_language: o.get_field("contentLanguage"),
            content_length: o.get_field("contentLength"),
            content_type: o.get_field("contentType"),
            etag: o.get_field("etag"),
            expiration: o.get_field("expiration"),
            expires: o.get_field("expires"),
            id: o.get_field("id"),
            key: o.get_field("key"),
            last_modified: o.get_field("lastModified"),
            metadata: o.get_field("metadata"),
            object_lock_legal_hold_status: o.get_field("objectLockLegalHoldStatus"),
            object_lock_mode: o.get_field("objectLockMode"),
            object_lock_retain_until_date: o.get_field("objectLockRetainUntilDate"),
            range: o.get_field("range"),
            server_side_encryption: o.get_field("serverSideEncryption"),
            sse_kms_key_id: o.get_field("sseKmsKeyId"),
            storage_class: o.get_field("storageClass"),
            tags: o.get_field("tags"),
            version_id: o.get_field("versionId"),
            website_redirect_location: o.get_field("websiteRedirectLocation"),
        }
    }
}
