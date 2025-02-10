/// Provides a resource for copying an S3 object.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = object_copy::create(
///         "test",
///         ObjectCopyArgs::builder()
///             .bucket("destination_bucket")
///             .grants(
///                 vec![
///                     ObjectCopyGrant::builder().permissions(vec!["READ",]). type ("Group")
///                     .uri("http://acs.amazonaws.com/groups/global/AllUsers")
///                     .build_struct(),
///                 ],
///             )
///             .key("destination_key")
///             .source("source_bucket/source_key")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod object_copy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectCopyArgs {
        /// [Canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `authenticated-read`, `aws-exec-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Conflicts with `grant`.
        #[builder(into, default)]
        pub acl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the bucket to put the file in.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub bucket_key_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies caching behavior along the request/reply chain Read [w3c cache_control](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9) for further details.
        #[builder(into, default)]
        pub cache_control: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates the algorithm used to create the checksum for the object. If a value is specified and the object is encrypted with KMS, you must have permission to use the `kms:Decrypt` action. Valid values: `CRC32`, `CRC32C`, `SHA1`, `SHA256`.
        #[builder(into, default)]
        pub checksum_algorithm: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies presentational information for the object. Read [w3c content_disposition](http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1) for further information.
        #[builder(into, default)]
        pub content_disposition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. Read [w3c content encoding](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11) for further information.
        #[builder(into, default)]
        pub content_encoding: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Language the content is in e.g., en-US or en-GB.
        #[builder(into, default)]
        pub content_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Standard MIME type describing the format of the object data, e.g., `application/octet-stream`. All Valid MIME Types are valid for this input.
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Copies the object if its entity tag (ETag) matches the specified tag.
        #[builder(into, default)]
        pub copy_if_match: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Copies the object if it has been modified since the specified time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        #[builder(into, default)]
        pub copy_if_modified_since: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Copies the object if its entity tag (ETag) is different than the specified ETag.
        #[builder(into, default)]
        pub copy_if_none_match: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Copies the object if it hasn't been modified since the specified time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        #[builder(into, default)]
        pub copy_if_unmodified_since: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the algorithm to use to when encrypting the object (for example, AES256).
        #[builder(into, default)]
        pub customer_algorithm: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side-encryption-customer-algorithm header.
        #[builder(into, default)]
        pub customer_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
        #[builder(into, default)]
        pub customer_key_md5: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Account id of the expected destination bucket owner. If the destination bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Account id of the expected source bucket owner. If the source bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        #[builder(into, default)]
        pub expected_source_bucket_owner: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Date and time at which the object is no longer cacheable, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        #[builder(into, default)]
        pub expires: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Allow the object to be deleted by removing any legal hold on any object version. Default is `false`. This value should be set to `true` only if the bucket has S3 object lock enabled.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for header grants. Documented below. Conflicts with `acl`.
        #[builder(into, default)]
        pub grants: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::ObjectCopyGrant>>,
        >,
        /// Name of the object once it is in the bucket.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the AWS KMS Encryption Context to use for object encryption. The value is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.
        #[builder(into, default)]
        pub kms_encryption_context: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the AWS KMS Key ARN to use for object encryption. This value is a fully qualified **ARN** of the KMS Key. If using `aws.kms.Key`, use the exported `arn` attribute: `kms_key_id = aws_kms_key.foo.arn`
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of keys/values to provision metadata (will be automatically prefixed by `x-amz-meta-`, note that only lowercase label are currently supported by the AWS Go API).
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies whether the metadata is copied from the source object or replaced with metadata provided in the request. Valid values are `COPY` and `REPLACE`.
        #[builder(into, default)]
        pub metadata_directive: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [legal hold](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-legal-holds) status that you want to apply to the specified object. Valid values are `ON` and `OFF`.
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
        /// Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from requester pays buckets, see Downloading Objects in Requestor Pays Buckets (https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html) in the Amazon S3 Developer Guide. If included, the only valid value is `requester`.
        #[builder(into, default)]
        pub request_payer: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies server-side encryption of the object in S3. Valid values are `AES256` and `aws:kms`.
        #[builder(into, default)]
        pub server_side_encryption: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the source object for the copy operation. You specify the value in one of two formats. For objects not accessed through an access point, specify the name of the source bucket and the key of the source object, separated by a slash (`/`). For example, `testbucket/test1.json`. For objects accessed through access points, specify the ARN of the object as accessed through the access point, in the format `arn:aws:s3:<Region>:<account-id>:accesspoint/<access-point-name>/object/<key>`. For example, `arn:aws:s3:us-west-2:9999912999:accesspoint/my-access-point/object/testbucket/test1.json`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the algorithm to use when decrypting the source object (for example, AES256).
        #[builder(into, default)]
        pub source_customer_algorithm: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.
        #[builder(into, default)]
        pub source_customer_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
        #[builder(into, default)]
        pub source_customer_key_md5: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the desired [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CopyObject.html#AmazonS3-CopyObject-request-header-StorageClass) for the object. Defaults to `STANDARD`.
        #[builder(into, default)]
        pub storage_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the object tag-set are copied from the source object or replaced with tag-set provided in the request. Valid values are `COPY` and `REPLACE`.
        #[builder(into, default)]
        pub tagging_directive: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a target URL for [website redirect](http://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html).
        #[builder(into, default)]
        pub website_redirect: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ObjectCopyResult {
        /// [Canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `authenticated-read`, `aws-exec-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Conflicts with `grant`.
        pub acl: pulumi_gestalt_rust::Output<String>,
        /// ARN of the object.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the bucket to put the file in.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        pub bucket_key_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies caching behavior along the request/reply chain Read [w3c cache_control](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9) for further details.
        pub cache_control: pulumi_gestalt_rust::Output<String>,
        /// Indicates the algorithm used to create the checksum for the object. If a value is specified and the object is encrypted with KMS, you must have permission to use the `kms:Decrypt` action. Valid values: `CRC32`, `CRC32C`, `SHA1`, `SHA256`.
        pub checksum_algorithm: pulumi_gestalt_rust::Output<Option<String>>,
        /// The base64-encoded, 32-bit CRC32 checksum of the object.
        pub checksum_crc32: pulumi_gestalt_rust::Output<String>,
        /// The base64-encoded, 32-bit CRC32C checksum of the object.
        pub checksum_crc32c: pulumi_gestalt_rust::Output<String>,
        /// The base64-encoded, 160-bit SHA-1 digest of the object.
        pub checksum_sha1: pulumi_gestalt_rust::Output<String>,
        /// The base64-encoded, 256-bit SHA-256 digest of the object.
        pub checksum_sha256: pulumi_gestalt_rust::Output<String>,
        /// Specifies presentational information for the object. Read [w3c content_disposition](http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1) for further information.
        pub content_disposition: pulumi_gestalt_rust::Output<String>,
        /// Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. Read [w3c content encoding](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11) for further information.
        pub content_encoding: pulumi_gestalt_rust::Output<String>,
        /// Language the content is in e.g., en-US or en-GB.
        pub content_language: pulumi_gestalt_rust::Output<String>,
        /// Standard MIME type describing the format of the object data, e.g., `application/octet-stream`. All Valid MIME Types are valid for this input.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// Copies the object if its entity tag (ETag) matches the specified tag.
        pub copy_if_match: pulumi_gestalt_rust::Output<Option<String>>,
        /// Copies the object if it has been modified since the specified time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub copy_if_modified_since: pulumi_gestalt_rust::Output<Option<String>>,
        /// Copies the object if its entity tag (ETag) is different than the specified ETag.
        pub copy_if_none_match: pulumi_gestalt_rust::Output<Option<String>>,
        /// Copies the object if it hasn't been modified since the specified time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub copy_if_unmodified_since: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the algorithm to use to when encrypting the object (for example, AES256).
        pub customer_algorithm: pulumi_gestalt_rust::Output<String>,
        /// Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side-encryption-customer-algorithm header.
        pub customer_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
        pub customer_key_md5: pulumi_gestalt_rust::Output<String>,
        /// ETag generated for the object (an MD5 sum of the object content). For plaintext objects or objects encrypted with an AWS-managed key, the hash is an MD5 digest of the object data. For objects encrypted with a KMS key or objects created by either the Multipart Upload or Part Copy operation, the hash is not an MD5 digest, regardless of the method of encryption. More information on possible values can be found on [Common Response Headers](https://docs.aws.amazon.com/AmazonS3/latest/API/RESTCommonResponseHeaders.html).
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Account id of the expected destination bucket owner. If the destination bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Account id of the expected source bucket owner. If the source bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        pub expected_source_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// If the object expiration is configured, this attribute will be set.
        pub expiration: pulumi_gestalt_rust::Output<String>,
        /// Date and time at which the object is no longer cacheable, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub expires: pulumi_gestalt_rust::Output<Option<String>>,
        /// Allow the object to be deleted by removing any legal hold on any object version. Default is `false`. This value should be set to `true` only if the bucket has S3 object lock enabled.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block for header grants. Documented below. Conflicts with `acl`.
        pub grants: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::ObjectCopyGrant>>,
        >,
        /// Name of the object once it is in the bucket.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the AWS KMS Encryption Context to use for object encryption. The value is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.
        pub kms_encryption_context: pulumi_gestalt_rust::Output<String>,
        /// Specifies the AWS KMS Key ARN to use for object encryption. This value is a fully qualified **ARN** of the KMS Key. If using `aws.kms.Key`, use the exported `arn` attribute: `kms_key_id = aws_kms_key.foo.arn`
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Returns the date that the object was last modified, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// Map of keys/values to provision metadata (will be automatically prefixed by `x-amz-meta-`, note that only lowercase label are currently supported by the AWS Go API).
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies whether the metadata is copied from the source object or replaced with metadata provided in the request. Valid values are `COPY` and `REPLACE`.
        pub metadata_directive: pulumi_gestalt_rust::Output<Option<String>>,
        /// The [legal hold](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-legal-holds) status that you want to apply to the specified object. Valid values are `ON` and `OFF`.
        pub object_lock_legal_hold_status: pulumi_gestalt_rust::Output<String>,
        /// Object lock [retention mode](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-modes) that you want to apply to this object. Valid values are `GOVERNANCE` and `COMPLIANCE`.
        pub object_lock_mode: pulumi_gestalt_rust::Output<String>,
        /// Date and time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8), when this object's object lock will [expire](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-periods).
        pub object_lock_retain_until_date: pulumi_gestalt_rust::Output<String>,
        /// If present, indicates that the requester was successfully charged for the request.
        pub request_charged: pulumi_gestalt_rust::Output<bool>,
        /// Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from requester pays buckets, see Downloading Objects in Requestor Pays Buckets (https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html) in the Amazon S3 Developer Guide. If included, the only valid value is `requester`.
        pub request_payer: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies server-side encryption of the object in S3. Valid values are `AES256` and `aws:kms`.
        pub server_side_encryption: pulumi_gestalt_rust::Output<String>,
        /// Specifies the source object for the copy operation. You specify the value in one of two formats. For objects not accessed through an access point, specify the name of the source bucket and the key of the source object, separated by a slash (`/`). For example, `testbucket/test1.json`. For objects accessed through access points, specify the ARN of the object as accessed through the access point, in the format `arn:aws:s3:<Region>:<account-id>:accesspoint/<access-point-name>/object/<key>`. For example, `arn:aws:s3:us-west-2:9999912999:accesspoint/my-access-point/object/testbucket/test1.json`.
        ///
        /// The following arguments are optional:
        pub source: pulumi_gestalt_rust::Output<String>,
        /// Specifies the algorithm to use when decrypting the source object (for example, AES256).
        pub source_customer_algorithm: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.
        pub source_customer_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
        pub source_customer_key_md5: pulumi_gestalt_rust::Output<Option<String>>,
        /// Version of the copied object in the source bucket.
        pub source_version_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the desired [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CopyObject.html#AmazonS3-CopyObject-request-header-StorageClass) for the object. Defaults to `STANDARD`.
        pub storage_class: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the object tag-set are copied from the source object or replaced with tag-set provided in the request. Valid values are `COPY` and `REPLACE`.
        pub tagging_directive: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Version ID of the newly created copy.
        pub version_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a target URL for [website redirect](http://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html).
        pub website_redirect: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObjectCopyArgs,
    ) -> ObjectCopyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let acl_binding = args.acl.get_output(context);
        let bucket_binding = args.bucket.get_output(context);
        let bucket_key_enabled_binding = args.bucket_key_enabled.get_output(context);
        let cache_control_binding = args.cache_control.get_output(context);
        let checksum_algorithm_binding = args.checksum_algorithm.get_output(context);
        let content_disposition_binding = args.content_disposition.get_output(context);
        let content_encoding_binding = args.content_encoding.get_output(context);
        let content_language_binding = args.content_language.get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let copy_if_match_binding = args.copy_if_match.get_output(context);
        let copy_if_modified_since_binding = args
            .copy_if_modified_since
            .get_output(context);
        let copy_if_none_match_binding = args.copy_if_none_match.get_output(context);
        let copy_if_unmodified_since_binding = args
            .copy_if_unmodified_since
            .get_output(context);
        let customer_algorithm_binding = args.customer_algorithm.get_output(context);
        let customer_key_binding = args.customer_key.get_output(context);
        let customer_key_md5_binding = args.customer_key_md5.get_output(context);
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context);
        let expected_source_bucket_owner_binding = args
            .expected_source_bucket_owner
            .get_output(context);
        let expires_binding = args.expires.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let grants_binding = args.grants.get_output(context);
        let key_binding = args.key.get_output(context);
        let kms_encryption_context_binding = args
            .kms_encryption_context
            .get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let metadata_directive_binding = args.metadata_directive.get_output(context);
        let object_lock_legal_hold_status_binding = args
            .object_lock_legal_hold_status
            .get_output(context);
        let object_lock_mode_binding = args.object_lock_mode.get_output(context);
        let object_lock_retain_until_date_binding = args
            .object_lock_retain_until_date
            .get_output(context);
        let request_payer_binding = args.request_payer.get_output(context);
        let server_side_encryption_binding = args
            .server_side_encryption
            .get_output(context);
        let source_binding = args.source.get_output(context);
        let source_customer_algorithm_binding = args
            .source_customer_algorithm
            .get_output(context);
        let source_customer_key_binding = args.source_customer_key.get_output(context);
        let source_customer_key_md5_binding = args
            .source_customer_key_md5
            .get_output(context);
        let storage_class_binding = args.storage_class.get_output(context);
        let tagging_directive_binding = args.tagging_directive.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let website_redirect_binding = args.website_redirect.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/objectCopy:ObjectCopy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acl".into(),
                    value: acl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketKeyEnabled".into(),
                    value: bucket_key_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheControl".into(),
                    value: cache_control_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "checksumAlgorithm".into(),
                    value: checksum_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentDisposition".into(),
                    value: content_disposition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentEncoding".into(),
                    value: content_encoding_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentLanguage".into(),
                    value: content_language_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: content_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyIfMatch".into(),
                    value: copy_if_match_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyIfModifiedSince".into(),
                    value: copy_if_modified_since_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyIfNoneMatch".into(),
                    value: copy_if_none_match_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyIfUnmodifiedSince".into(),
                    value: copy_if_unmodified_since_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerAlgorithm".into(),
                    value: customer_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerKey".into(),
                    value: customer_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerKeyMd5".into(),
                    value: customer_key_md5_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedBucketOwner".into(),
                    value: expected_bucket_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedSourceBucketOwner".into(),
                    value: expected_source_bucket_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expires".into(),
                    value: expires_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grants".into(),
                    value: grants_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsEncryptionContext".into(),
                    value: kms_encryption_context_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadataDirective".into(),
                    value: metadata_directive_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectLockLegalHoldStatus".into(),
                    value: object_lock_legal_hold_status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectLockMode".into(),
                    value: object_lock_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectLockRetainUntilDate".into(),
                    value: object_lock_retain_until_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestPayer".into(),
                    value: request_payer_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverSideEncryption".into(),
                    value: server_side_encryption_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceCustomerAlgorithm".into(),
                    value: source_customer_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceCustomerKey".into(),
                    value: source_customer_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceCustomerKeyMd5".into(),
                    value: source_customer_key_md5_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageClass".into(),
                    value: storage_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taggingDirective".into(),
                    value: tagging_directive_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "websiteRedirect".into(),
                    value: website_redirect_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ObjectCopyResult {
            acl: o.get_field("acl"),
            arn: o.get_field("arn"),
            bucket: o.get_field("bucket"),
            bucket_key_enabled: o.get_field("bucketKeyEnabled"),
            cache_control: o.get_field("cacheControl"),
            checksum_algorithm: o.get_field("checksumAlgorithm"),
            checksum_crc32: o.get_field("checksumCrc32"),
            checksum_crc32c: o.get_field("checksumCrc32c"),
            checksum_sha1: o.get_field("checksumSha1"),
            checksum_sha256: o.get_field("checksumSha256"),
            content_disposition: o.get_field("contentDisposition"),
            content_encoding: o.get_field("contentEncoding"),
            content_language: o.get_field("contentLanguage"),
            content_type: o.get_field("contentType"),
            copy_if_match: o.get_field("copyIfMatch"),
            copy_if_modified_since: o.get_field("copyIfModifiedSince"),
            copy_if_none_match: o.get_field("copyIfNoneMatch"),
            copy_if_unmodified_since: o.get_field("copyIfUnmodifiedSince"),
            customer_algorithm: o.get_field("customerAlgorithm"),
            customer_key: o.get_field("customerKey"),
            customer_key_md5: o.get_field("customerKeyMd5"),
            etag: o.get_field("etag"),
            expected_bucket_owner: o.get_field("expectedBucketOwner"),
            expected_source_bucket_owner: o.get_field("expectedSourceBucketOwner"),
            expiration: o.get_field("expiration"),
            expires: o.get_field("expires"),
            force_destroy: o.get_field("forceDestroy"),
            grants: o.get_field("grants"),
            key: o.get_field("key"),
            kms_encryption_context: o.get_field("kmsEncryptionContext"),
            kms_key_id: o.get_field("kmsKeyId"),
            last_modified: o.get_field("lastModified"),
            metadata: o.get_field("metadata"),
            metadata_directive: o.get_field("metadataDirective"),
            object_lock_legal_hold_status: o.get_field("objectLockLegalHoldStatus"),
            object_lock_mode: o.get_field("objectLockMode"),
            object_lock_retain_until_date: o.get_field("objectLockRetainUntilDate"),
            request_charged: o.get_field("requestCharged"),
            request_payer: o.get_field("requestPayer"),
            server_side_encryption: o.get_field("serverSideEncryption"),
            source: o.get_field("source"),
            source_customer_algorithm: o.get_field("sourceCustomerAlgorithm"),
            source_customer_key: o.get_field("sourceCustomerKey"),
            source_customer_key_md5: o.get_field("sourceCustomerKeyMd5"),
            source_version_id: o.get_field("sourceVersionId"),
            storage_class: o.get_field("storageClass"),
            tagging_directive: o.get_field("taggingDirective"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version_id: o.get_field("versionId"),
            website_redirect: o.get_field("websiteRedirect"),
        }
    }
}
