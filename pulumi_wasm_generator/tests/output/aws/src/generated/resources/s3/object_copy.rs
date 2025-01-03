/// Provides a resource for copying an S3 object.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod object_copy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectCopyArgs {
        /// [Canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `authenticated-read`, `aws-exec-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Conflicts with `grant`.
        #[builder(into, default)]
        pub acl: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the bucket to put the file in.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub bucket_key_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies caching behavior along the request/reply chain Read [w3c cache_control](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9) for further details.
        #[builder(into, default)]
        pub cache_control: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates the algorithm used to create the checksum for the object. If a value is specified and the object is encrypted with KMS, you must have permission to use the `kms:Decrypt` action. Valid values: `CRC32`, `CRC32C`, `SHA1`, `SHA256`.
        #[builder(into, default)]
        pub checksum_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies presentational information for the object. Read [w3c content_disposition](http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1) for further information.
        #[builder(into, default)]
        pub content_disposition: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. Read [w3c content encoding](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11) for further information.
        #[builder(into, default)]
        pub content_encoding: pulumi_wasm_rust::Output<Option<String>>,
        /// Language the content is in e.g., en-US or en-GB.
        #[builder(into, default)]
        pub content_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Standard MIME type describing the format of the object data, e.g., `application/octet-stream`. All Valid MIME Types are valid for this input.
        #[builder(into, default)]
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Copies the object if its entity tag (ETag) matches the specified tag.
        #[builder(into, default)]
        pub copy_if_match: pulumi_wasm_rust::Output<Option<String>>,
        /// Copies the object if it has been modified since the specified time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        #[builder(into, default)]
        pub copy_if_modified_since: pulumi_wasm_rust::Output<Option<String>>,
        /// Copies the object if its entity tag (ETag) is different than the specified ETag.
        #[builder(into, default)]
        pub copy_if_none_match: pulumi_wasm_rust::Output<Option<String>>,
        /// Copies the object if it hasn't been modified since the specified time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        #[builder(into, default)]
        pub copy_if_unmodified_since: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the algorithm to use to when encrypting the object (for example, AES256).
        #[builder(into, default)]
        pub customer_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side-encryption-customer-algorithm header.
        #[builder(into, default)]
        pub customer_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
        #[builder(into, default)]
        pub customer_key_md5: pulumi_wasm_rust::Output<Option<String>>,
        /// Account id of the expected destination bucket owner. If the destination bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Account id of the expected source bucket owner. If the source bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        #[builder(into, default)]
        pub expected_source_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Date and time at which the object is no longer cacheable, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        #[builder(into, default)]
        pub expires: pulumi_wasm_rust::Output<Option<String>>,
        /// Allow the object to be deleted by removing any legal hold on any object version. Default is `false`. This value should be set to `true` only if the bucket has S3 object lock enabled.
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for header grants. Documented below. Conflicts with `acl`.
        #[builder(into, default)]
        pub grants: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::ObjectCopyGrant>>,
        >,
        /// Name of the object once it is in the bucket.
        #[builder(into)]
        pub key: pulumi_wasm_rust::Output<String>,
        /// Specifies the AWS KMS Encryption Context to use for object encryption. The value is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.
        #[builder(into, default)]
        pub kms_encryption_context: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the AWS KMS Key ARN to use for object encryption. This value is a fully qualified **ARN** of the KMS Key. If using `aws.kms.Key`, use the exported `arn` attribute: `kms_key_id = aws_kms_key.foo.arn`
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of keys/values to provision metadata (will be automatically prefixed by `x-amz-meta-`, note that only lowercase label are currently supported by the AWS Go API).
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies whether the metadata is copied from the source object or replaced with metadata provided in the request. Valid values are `COPY` and `REPLACE`.
        #[builder(into, default)]
        pub metadata_directive: pulumi_wasm_rust::Output<Option<String>>,
        /// The [legal hold](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-legal-holds) status that you want to apply to the specified object. Valid values are `ON` and `OFF`.
        #[builder(into, default)]
        pub object_lock_legal_hold_status: pulumi_wasm_rust::Output<Option<String>>,
        /// Object lock [retention mode](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-modes) that you want to apply to this object. Valid values are `GOVERNANCE` and `COMPLIANCE`.
        #[builder(into, default)]
        pub object_lock_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Date and time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8), when this object's object lock will [expire](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-periods).
        #[builder(into, default)]
        pub object_lock_retain_until_date: pulumi_wasm_rust::Output<Option<String>>,
        /// Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from requester pays buckets, see Downloading Objects in Requestor Pays Buckets (https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html) in the Amazon S3 Developer Guide. If included, the only valid value is `requester`.
        #[builder(into, default)]
        pub request_payer: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies server-side encryption of the object in S3. Valid values are `AES256` and `aws:kms`.
        #[builder(into, default)]
        pub server_side_encryption: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the source object for the copy operation. You specify the value in one of two formats. For objects not accessed through an access point, specify the name of the source bucket and the key of the source object, separated by a slash (`/`). For example, `testbucket/test1.json`. For objects accessed through access points, specify the ARN of the object as accessed through the access point, in the format `arn:aws:s3:<Region>:<account-id>:accesspoint/<access-point-name>/object/<key>`. For example, `arn:aws:s3:us-west-2:9999912999:accesspoint/my-access-point/object/testbucket/test1.json`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<String>,
        /// Specifies the algorithm to use when decrypting the source object (for example, AES256).
        #[builder(into, default)]
        pub source_customer_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.
        #[builder(into, default)]
        pub source_customer_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
        #[builder(into, default)]
        pub source_customer_key_md5: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the desired [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CopyObject.html#AmazonS3-CopyObject-request-header-StorageClass) for the object. Defaults to `STANDARD`.
        #[builder(into, default)]
        pub storage_class: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the object tag-set are copied from the source object or replaced with tag-set provided in the request. Valid values are `COPY` and `REPLACE`.
        #[builder(into, default)]
        pub tagging_directive: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a target URL for [website redirect](http://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html).
        #[builder(into, default)]
        pub website_redirect: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ObjectCopyResult {
        /// [Canned ACL](https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl) to apply. Valid values are `private`, `public-read`, `public-read-write`, `authenticated-read`, `aws-exec-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Conflicts with `grant`.
        pub acl: pulumi_wasm_rust::Output<String>,
        /// ARN of the object.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the bucket to put the file in.
        pub bucket: pulumi_wasm_rust::Output<String>,
        pub bucket_key_enabled: pulumi_wasm_rust::Output<bool>,
        /// Specifies caching behavior along the request/reply chain Read [w3c cache_control](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9) for further details.
        pub cache_control: pulumi_wasm_rust::Output<String>,
        /// Indicates the algorithm used to create the checksum for the object. If a value is specified and the object is encrypted with KMS, you must have permission to use the `kms:Decrypt` action. Valid values: `CRC32`, `CRC32C`, `SHA1`, `SHA256`.
        pub checksum_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// The base64-encoded, 32-bit CRC32 checksum of the object.
        pub checksum_crc32: pulumi_wasm_rust::Output<String>,
        /// The base64-encoded, 32-bit CRC32C checksum of the object.
        pub checksum_crc32c: pulumi_wasm_rust::Output<String>,
        /// The base64-encoded, 160-bit SHA-1 digest of the object.
        pub checksum_sha1: pulumi_wasm_rust::Output<String>,
        /// The base64-encoded, 256-bit SHA-256 digest of the object.
        pub checksum_sha256: pulumi_wasm_rust::Output<String>,
        /// Specifies presentational information for the object. Read [w3c content_disposition](http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1) for further information.
        pub content_disposition: pulumi_wasm_rust::Output<String>,
        /// Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. Read [w3c content encoding](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11) for further information.
        pub content_encoding: pulumi_wasm_rust::Output<String>,
        /// Language the content is in e.g., en-US or en-GB.
        pub content_language: pulumi_wasm_rust::Output<String>,
        /// Standard MIME type describing the format of the object data, e.g., `application/octet-stream`. All Valid MIME Types are valid for this input.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// Copies the object if its entity tag (ETag) matches the specified tag.
        pub copy_if_match: pulumi_wasm_rust::Output<Option<String>>,
        /// Copies the object if it has been modified since the specified time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub copy_if_modified_since: pulumi_wasm_rust::Output<Option<String>>,
        /// Copies the object if its entity tag (ETag) is different than the specified ETag.
        pub copy_if_none_match: pulumi_wasm_rust::Output<Option<String>>,
        /// Copies the object if it hasn't been modified since the specified time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub copy_if_unmodified_since: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the algorithm to use to when encrypting the object (for example, AES256).
        pub customer_algorithm: pulumi_wasm_rust::Output<String>,
        /// Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side-encryption-customer-algorithm header.
        pub customer_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
        pub customer_key_md5: pulumi_wasm_rust::Output<String>,
        /// ETag generated for the object (an MD5 sum of the object content). For plaintext objects or objects encrypted with an AWS-managed key, the hash is an MD5 digest of the object data. For objects encrypted with a KMS key or objects created by either the Multipart Upload or Part Copy operation, the hash is not an MD5 digest, regardless of the method of encryption. More information on possible values can be found on [Common Response Headers](https://docs.aws.amazon.com/AmazonS3/latest/API/RESTCommonResponseHeaders.html).
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Account id of the expected destination bucket owner. If the destination bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Account id of the expected source bucket owner. If the source bucket is owned by a different account, the request will fail with an HTTP 403 (Access Denied) error.
        pub expected_source_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// If the object expiration is configured, this attribute will be set.
        pub expiration: pulumi_wasm_rust::Output<String>,
        /// Date and time at which the object is no longer cacheable, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub expires: pulumi_wasm_rust::Output<Option<String>>,
        /// Allow the object to be deleted by removing any legal hold on any object version. Default is `false`. This value should be set to `true` only if the bucket has S3 object lock enabled.
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for header grants. Documented below. Conflicts with `acl`.
        pub grants: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::ObjectCopyGrant>>,
        >,
        /// Name of the object once it is in the bucket.
        pub key: pulumi_wasm_rust::Output<String>,
        /// Specifies the AWS KMS Encryption Context to use for object encryption. The value is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.
        pub kms_encryption_context: pulumi_wasm_rust::Output<String>,
        /// Specifies the AWS KMS Key ARN to use for object encryption. This value is a fully qualified **ARN** of the KMS Key. If using `aws.kms.Key`, use the exported `arn` attribute: `kms_key_id = aws_kms_key.foo.arn`
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Returns the date that the object was last modified, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// Map of keys/values to provision metadata (will be automatically prefixed by `x-amz-meta-`, note that only lowercase label are currently supported by the AWS Go API).
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies whether the metadata is copied from the source object or replaced with metadata provided in the request. Valid values are `COPY` and `REPLACE`.
        pub metadata_directive: pulumi_wasm_rust::Output<Option<String>>,
        /// The [legal hold](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-legal-holds) status that you want to apply to the specified object. Valid values are `ON` and `OFF`.
        pub object_lock_legal_hold_status: pulumi_wasm_rust::Output<String>,
        /// Object lock [retention mode](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-modes) that you want to apply to this object. Valid values are `GOVERNANCE` and `COMPLIANCE`.
        pub object_lock_mode: pulumi_wasm_rust::Output<String>,
        /// Date and time, in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8), when this object's object lock will [expire](https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock-overview.html#object-lock-retention-periods).
        pub object_lock_retain_until_date: pulumi_wasm_rust::Output<String>,
        /// If present, indicates that the requester was successfully charged for the request.
        pub request_charged: pulumi_wasm_rust::Output<bool>,
        /// Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from requester pays buckets, see Downloading Objects in Requestor Pays Buckets (https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html) in the Amazon S3 Developer Guide. If included, the only valid value is `requester`.
        pub request_payer: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies server-side encryption of the object in S3. Valid values are `AES256` and `aws:kms`.
        pub server_side_encryption: pulumi_wasm_rust::Output<String>,
        /// Specifies the source object for the copy operation. You specify the value in one of two formats. For objects not accessed through an access point, specify the name of the source bucket and the key of the source object, separated by a slash (`/`). For example, `testbucket/test1.json`. For objects accessed through access points, specify the ARN of the object as accessed through the access point, in the format `arn:aws:s3:<Region>:<account-id>:accesspoint/<access-point-name>/object/<key>`. For example, `arn:aws:s3:us-west-2:9999912999:accesspoint/my-access-point/object/testbucket/test1.json`.
        ///
        /// The following arguments are optional:
        pub source: pulumi_wasm_rust::Output<String>,
        /// Specifies the algorithm to use when decrypting the source object (for example, AES256).
        pub source_customer_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.
        pub source_customer_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
        pub source_customer_key_md5: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the copied object in the source bucket.
        pub source_version_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the desired [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_CopyObject.html#AmazonS3-CopyObject-request-header-StorageClass) for the object. Defaults to `STANDARD`.
        pub storage_class: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the object tag-set are copied from the source object or replaced with tag-set provided in the request. Valid values are `COPY` and `REPLACE`.
        pub tagging_directive: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Version ID of the newly created copy.
        pub version_id: pulumi_wasm_rust::Output<String>,
        /// Specifies a target URL for [website redirect](http://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html).
        pub website_redirect: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ObjectCopyArgs) -> ObjectCopyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let acl_binding = args.acl.get_inner();
        let bucket_binding = args.bucket.get_inner();
        let bucket_key_enabled_binding = args.bucket_key_enabled.get_inner();
        let cache_control_binding = args.cache_control.get_inner();
        let checksum_algorithm_binding = args.checksum_algorithm.get_inner();
        let content_disposition_binding = args.content_disposition.get_inner();
        let content_encoding_binding = args.content_encoding.get_inner();
        let content_language_binding = args.content_language.get_inner();
        let content_type_binding = args.content_type.get_inner();
        let copy_if_match_binding = args.copy_if_match.get_inner();
        let copy_if_modified_since_binding = args.copy_if_modified_since.get_inner();
        let copy_if_none_match_binding = args.copy_if_none_match.get_inner();
        let copy_if_unmodified_since_binding = args.copy_if_unmodified_since.get_inner();
        let customer_algorithm_binding = args.customer_algorithm.get_inner();
        let customer_key_binding = args.customer_key.get_inner();
        let customer_key_md5_binding = args.customer_key_md5.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let expected_source_bucket_owner_binding = args
            .expected_source_bucket_owner
            .get_inner();
        let expires_binding = args.expires.get_inner();
        let force_destroy_binding = args.force_destroy.get_inner();
        let grants_binding = args.grants.get_inner();
        let key_binding = args.key.get_inner();
        let kms_encryption_context_binding = args.kms_encryption_context.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let metadata_directive_binding = args.metadata_directive.get_inner();
        let object_lock_legal_hold_status_binding = args
            .object_lock_legal_hold_status
            .get_inner();
        let object_lock_mode_binding = args.object_lock_mode.get_inner();
        let object_lock_retain_until_date_binding = args
            .object_lock_retain_until_date
            .get_inner();
        let request_payer_binding = args.request_payer.get_inner();
        let server_side_encryption_binding = args.server_side_encryption.get_inner();
        let source_binding = args.source.get_inner();
        let source_customer_algorithm_binding = args
            .source_customer_algorithm
            .get_inner();
        let source_customer_key_binding = args.source_customer_key.get_inner();
        let source_customer_key_md5_binding = args.source_customer_key_md5.get_inner();
        let storage_class_binding = args.storage_class.get_inner();
        let tagging_directive_binding = args.tagging_directive.get_inner();
        let tags_binding = args.tags.get_inner();
        let website_redirect_binding = args.website_redirect.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/objectCopy:ObjectCopy".into(),
            name: name.to_string(),
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
                    name: "checksumAlgorithm".into(),
                    value: &checksum_algorithm_binding,
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
                    name: "copyIfMatch".into(),
                    value: &copy_if_match_binding,
                },
                register_interface::ObjectField {
                    name: "copyIfModifiedSince".into(),
                    value: &copy_if_modified_since_binding,
                },
                register_interface::ObjectField {
                    name: "copyIfNoneMatch".into(),
                    value: &copy_if_none_match_binding,
                },
                register_interface::ObjectField {
                    name: "copyIfUnmodifiedSince".into(),
                    value: &copy_if_unmodified_since_binding,
                },
                register_interface::ObjectField {
                    name: "customerAlgorithm".into(),
                    value: &customer_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "customerKey".into(),
                    value: &customer_key_binding,
                },
                register_interface::ObjectField {
                    name: "customerKeyMd5".into(),
                    value: &customer_key_md5_binding,
                },
                register_interface::ObjectField {
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding,
                },
                register_interface::ObjectField {
                    name: "expectedSourceBucketOwner".into(),
                    value: &expected_source_bucket_owner_binding,
                },
                register_interface::ObjectField {
                    name: "expires".into(),
                    value: &expires_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "grants".into(),
                    value: &grants_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "kmsEncryptionContext".into(),
                    value: &kms_encryption_context_binding,
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
                    name: "metadataDirective".into(),
                    value: &metadata_directive_binding,
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
                    name: "requestPayer".into(),
                    value: &request_payer_binding,
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
                    name: "sourceCustomerAlgorithm".into(),
                    value: &source_customer_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "sourceCustomerKey".into(),
                    value: &source_customer_key_binding,
                },
                register_interface::ObjectField {
                    name: "sourceCustomerKeyMd5".into(),
                    value: &source_customer_key_md5_binding,
                },
                register_interface::ObjectField {
                    name: "storageClass".into(),
                    value: &storage_class_binding,
                },
                register_interface::ObjectField {
                    name: "taggingDirective".into(),
                    value: &tagging_directive_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "acl".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "bucketKeyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "cacheControl".into(),
                },
                register_interface::ResultField {
                    name: "checksumAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "checksumCrc32".into(),
                },
                register_interface::ResultField {
                    name: "checksumCrc32c".into(),
                },
                register_interface::ResultField {
                    name: "checksumSha1".into(),
                },
                register_interface::ResultField {
                    name: "checksumSha256".into(),
                },
                register_interface::ResultField {
                    name: "contentDisposition".into(),
                },
                register_interface::ResultField {
                    name: "contentEncoding".into(),
                },
                register_interface::ResultField {
                    name: "contentLanguage".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "copyIfMatch".into(),
                },
                register_interface::ResultField {
                    name: "copyIfModifiedSince".into(),
                },
                register_interface::ResultField {
                    name: "copyIfNoneMatch".into(),
                },
                register_interface::ResultField {
                    name: "copyIfUnmodifiedSince".into(),
                },
                register_interface::ResultField {
                    name: "customerAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "customerKey".into(),
                },
                register_interface::ResultField {
                    name: "customerKeyMd5".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "expectedBucketOwner".into(),
                },
                register_interface::ResultField {
                    name: "expectedSourceBucketOwner".into(),
                },
                register_interface::ResultField {
                    name: "expiration".into(),
                },
                register_interface::ResultField {
                    name: "expires".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "grants".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "kmsEncryptionContext".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "metadataDirective".into(),
                },
                register_interface::ResultField {
                    name: "objectLockLegalHoldStatus".into(),
                },
                register_interface::ResultField {
                    name: "objectLockMode".into(),
                },
                register_interface::ResultField {
                    name: "objectLockRetainUntilDate".into(),
                },
                register_interface::ResultField {
                    name: "requestCharged".into(),
                },
                register_interface::ResultField {
                    name: "requestPayer".into(),
                },
                register_interface::ResultField {
                    name: "serverSideEncryption".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "sourceCustomerAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "sourceCustomerKey".into(),
                },
                register_interface::ResultField {
                    name: "sourceCustomerKeyMd5".into(),
                },
                register_interface::ResultField {
                    name: "sourceVersionId".into(),
                },
                register_interface::ResultField {
                    name: "storageClass".into(),
                },
                register_interface::ResultField {
                    name: "taggingDirective".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "versionId".into(),
                },
                register_interface::ResultField {
                    name: "websiteRedirect".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ObjectCopyResult {
            acl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acl").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            bucket_key_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketKeyEnabled").unwrap(),
            ),
            cache_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheControl").unwrap(),
            ),
            checksum_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksumAlgorithm").unwrap(),
            ),
            checksum_crc32: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksumCrc32").unwrap(),
            ),
            checksum_crc32c: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksumCrc32c").unwrap(),
            ),
            checksum_sha1: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksumSha1").unwrap(),
            ),
            checksum_sha256: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksumSha256").unwrap(),
            ),
            content_disposition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentDisposition").unwrap(),
            ),
            content_encoding: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentEncoding").unwrap(),
            ),
            content_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentLanguage").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            copy_if_match: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyIfMatch").unwrap(),
            ),
            copy_if_modified_since: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyIfModifiedSince").unwrap(),
            ),
            copy_if_none_match: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyIfNoneMatch").unwrap(),
            ),
            copy_if_unmodified_since: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyIfUnmodifiedSince").unwrap(),
            ),
            customer_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerAlgorithm").unwrap(),
            ),
            customer_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerKey").unwrap(),
            ),
            customer_key_md5: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerKeyMd5").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
            expected_source_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedSourceBucketOwner").unwrap(),
            ),
            expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiration").unwrap(),
            ),
            expires: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expires").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            grants: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grants").unwrap(),
            ),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            kms_encryption_context: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsEncryptionContext").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            metadata_directive: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataDirective").unwrap(),
            ),
            object_lock_legal_hold_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectLockLegalHoldStatus").unwrap(),
            ),
            object_lock_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectLockMode").unwrap(),
            ),
            object_lock_retain_until_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectLockRetainUntilDate").unwrap(),
            ),
            request_charged: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestCharged").unwrap(),
            ),
            request_payer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestPayer").unwrap(),
            ),
            server_side_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverSideEncryption").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            source_customer_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCustomerAlgorithm").unwrap(),
            ),
            source_customer_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCustomerKey").unwrap(),
            ),
            source_customer_key_md5: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCustomerKeyMd5").unwrap(),
            ),
            source_version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceVersionId").unwrap(),
            ),
            storage_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageClass").unwrap(),
            ),
            tagging_directive: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taggingDirective").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionId").unwrap(),
            ),
            website_redirect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("websiteRedirect").unwrap(),
            ),
        }
    }
}
