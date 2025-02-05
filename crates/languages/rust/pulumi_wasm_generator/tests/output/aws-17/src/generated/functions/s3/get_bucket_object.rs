pub mod get_bucket_object {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketObjectArgs {
        /// Name of the bucket to read the object from. Alternatively, an [S3 access point](https://docs.aws.amazon.com/AmazonS3/latest/dev/using-access-points.html) ARN can be specified
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::InputOrOutput<String>,
        /// Full path to the object inside the bucket
        #[builder(into)]
        pub key: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub range: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the object.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specific version ID of the object returned (defaults to latest version)
        #[builder(into, default)]
        pub version_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBucketObjectResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Object data (see **limitations above** to understand cases in which this field is actually available)
        pub body: pulumi_wasm_rust::Output<String>,
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// (Optional) Whether or not to use [Amazon S3 Bucket Keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html) for SSE-KMS.
        pub bucket_key_enabled: pulumi_wasm_rust::Output<bool>,
        /// Caching behavior along the request/reply chain.
        pub cache_control: pulumi_wasm_rust::Output<String>,
        /// Presentational information for the object.
        pub content_disposition: pulumi_wasm_rust::Output<String>,
        /// What content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.
        pub content_encoding: pulumi_wasm_rust::Output<String>,
        /// Language the content is in.
        pub content_language: pulumi_wasm_rust::Output<String>,
        /// Size of the body in bytes.
        pub content_length: pulumi_wasm_rust::Output<i32>,
        /// Standard MIME type describing the format of the object data.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// [ETag](https://en.wikipedia.org/wiki/HTTP_ETag) generated for the object (an MD5 sum of the object content in case it's not encrypted)
        pub etag: pulumi_wasm_rust::Output<String>,
        /// If the object expiration is configured (see [object lifecycle management](http://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html)), the field includes this header. It includes the expiry-date and rule-id key value pairs providing object expiration information. The value of the rule-id is URL encoded.
        pub expiration: pulumi_wasm_rust::Output<String>,
        /// Date and time at which the object is no longer cacheable.
        pub expires: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key: pulumi_wasm_rust::Output<String>,
        /// Last modified date of the object in RFC1123 format (e.g., `Mon, 02 Jan 2006 15:04:05 MST`)
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// Map of metadata stored with the object in S3. Keys are always returned in lowercase.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates whether this object has an active [legal hold](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-legal-holds). This field is only returned if you have permission to view an object's legal hold status.
        pub object_lock_legal_hold_status: pulumi_wasm_rust::Output<String>,
        /// Object lock [retention mode](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-modes) currently in place for this object.
        pub object_lock_mode: pulumi_wasm_rust::Output<String>,
        /// The date and time when this object's object lock will expire.
        pub object_lock_retain_until_date: pulumi_wasm_rust::Output<String>,
        pub range: pulumi_wasm_rust::Output<Option<String>>,
        /// If the object is stored using server-side encryption (KMS or Amazon S3-managed encryption key), this field includes the chosen encryption and algorithm used.
        pub server_side_encryption: pulumi_wasm_rust::Output<String>,
        /// If present, specifies the ID of the Key Management Service (KMS) master encryption key that was used for the object.
        pub sse_kms_key_id: pulumi_wasm_rust::Output<String>,
        /// [Storage class](http://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html) information of the object. Available for all objects except for `Standard` storage class objects.
        pub storage_class: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the object.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Latest version ID of the object returned.
        pub version_id: pulumi_wasm_rust::Output<String>,
        /// If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.
        pub website_redirect_location: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBucketObjectArgs,
    ) -> GetBucketObjectResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let key_binding = args.key.get_output(context).get_inner();
        let range_binding = args.range.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let version_id_binding = args.version_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3/getBucketObject:getBucketObject".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "range".into(),
                    value: &range_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBucketObjectResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            body: pulumi_wasm_rust::__private::into_domain(o.extract_field("body")),
            bucket: pulumi_wasm_rust::__private::into_domain(o.extract_field("bucket")),
            bucket_key_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bucketKeyEnabled"),
            ),
            cache_control: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cacheControl"),
            ),
            content_disposition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentDisposition"),
            ),
            content_encoding: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentEncoding"),
            ),
            content_language: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentLanguage"),
            ),
            content_length: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentLength"),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            expiration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expiration"),
            ),
            expires: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expires"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key: pulumi_wasm_rust::__private::into_domain(o.extract_field("key")),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModified"),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            object_lock_legal_hold_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("objectLockLegalHoldStatus"),
            ),
            object_lock_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("objectLockMode"),
            ),
            object_lock_retain_until_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("objectLockRetainUntilDate"),
            ),
            range: pulumi_wasm_rust::__private::into_domain(o.extract_field("range")),
            server_side_encryption: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverSideEncryption"),
            ),
            sse_kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sseKmsKeyId"),
            ),
            storage_class: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageClass"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            version_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionId"),
            ),
            website_redirect_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("websiteRedirectLocation"),
            ),
        }
    }
}
