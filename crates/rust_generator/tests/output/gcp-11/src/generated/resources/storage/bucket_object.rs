/// Creates a new object inside an existing bucket in Google cloud storage service (GCS).
/// [ACLs](https://cloud.google.com/storage/docs/access-control/lists) can be applied using the `gcp.storage.ObjectACL` resource.
///  For more information see
/// [the official documentation](https://cloud.google.com/storage/docs/key-terms#objects)
/// and
/// [API](https://cloud.google.com/storage/docs/json_api/v1/objects).
///
///
/// ## Example Usage
///
/// Example creating a public object in an existing `image-store` bucket.
///
/// ```yaml
/// resources:
///   picture:
///     type: gcp:storage:BucketObject
///     properties:
///       name: butterfly01
///       source:
///         fn::FileAsset: /images/nature/garden-tiger-moth.jpg
///       bucket: image-store
/// ```
///
/// Example creating an empty folder in an existing `image-store` bucket.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let emptyFolder = bucket_object::create(
///         "emptyFolder",
///         BucketObjectArgs::builder()
///             .bucket("image-store")
///             .content(" ")
///             .name("empty_folder/")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_object {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketObjectArgs {
        /// The name of the containing bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// [Cache-Control](https://tools.ietf.org/html/rfc7234#section-5.2)
        /// directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600
        #[builder(into, default)]
        pub cache_control: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Data as `string` to be uploaded. Must be defined if `source` is not. **Note**: The `content` field is marked as sensitive.
        #[builder(into, default)]
        pub content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// [Content-Disposition](https://tools.ietf.org/html/rfc6266) of the object data.
        #[builder(into, default)]
        pub content_disposition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// [Content-Encoding](https://tools.ietf.org/html/rfc7231#section-3.1.2.2) of the object data.
        #[builder(into, default)]
        pub content_encoding: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// [Content-Language](https://tools.ietf.org/html/rfc7231#section-3.1.3.2) of the object data.
        #[builder(into, default)]
        pub content_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// [Content-Type](https://tools.ietf.org/html/rfc7231#section-3.1.1.5) of the object data. Defaults to "application/octet-stream" or "text/plain; charset=utf-8".
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables object encryption with Customer-Supplied Encryption Key (CSEK). Google [documentation about CSEK.](https://cloud.google.com/storage/docs/encryption/customer-supplied-keys)
        /// Structure is documented below.
        #[builder(into, default)]
        pub customer_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketObjectCustomerEncryption>,
        >,
        #[builder(into, default)]
        pub detect_md5hash: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether an object is under [event-based hold](https://cloud.google.com/storage/docs/object-holds#hold-types). Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any).
        #[builder(into, default)]
        pub event_based_hold: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The resource name of the Cloud KMS key that will be used to [encrypt](https://cloud.google.com/storage/docs/encryption/using-customer-managed-keys) the object.
        #[builder(into, default)]
        pub kms_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-provided metadata, in key/value pairs.
        ///
        /// One of the following is required:
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the object. If you're interpolating the name of this object, see `output_name` instead.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [object retention](http://cloud.google.com/storage/docs/object-lock) settings for the object. The retention settings allow an object to be retained until a provided date. Structure is documented below.
        #[builder(into, default)]
        pub retention: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketObjectRetention>,
        >,
        /// A path to the data you want to upload. Must be defined
        /// if `content` is not.
        ///
        /// - - -
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [StorageClass](https://cloud.google.com/storage/docs/storage-classes) of the new bucket object.
        /// Supported values include: `MULTI_REGIONAL`, `REGIONAL`, `NEARLINE`, `COLDLINE`, `ARCHIVE`. If not provided, this defaults to the bucket's default
        /// storage class or to a [standard](https://cloud.google.com/storage/docs/storage-classes#standard) class.
        #[builder(into, default)]
        pub storage_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether an object is under [temporary hold](https://cloud.google.com/storage/docs/object-holds#hold-types). While this flag is set to true, the object is protected against deletion and overwrites.
        #[builder(into, default)]
        pub temporary_hold: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct BucketObjectResult {
        /// The name of the containing bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// [Cache-Control](https://tools.ietf.org/html/rfc7234#section-5.2)
        /// directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600
        pub cache_control: pulumi_gestalt_rust::Output<Option<String>>,
        /// Data as `string` to be uploaded. Must be defined if `source` is not. **Note**: The `content` field is marked as sensitive.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// [Content-Disposition](https://tools.ietf.org/html/rfc6266) of the object data.
        pub content_disposition: pulumi_gestalt_rust::Output<Option<String>>,
        /// [Content-Encoding](https://tools.ietf.org/html/rfc7231#section-3.1.2.2) of the object data.
        pub content_encoding: pulumi_gestalt_rust::Output<Option<String>>,
        /// [Content-Language](https://tools.ietf.org/html/rfc7231#section-3.1.3.2) of the object data.
        pub content_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// [Content-Type](https://tools.ietf.org/html/rfc7231#section-3.1.1.5) of the object data. Defaults to "application/octet-stream" or "text/plain; charset=utf-8".
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// (Computed) Base 64 CRC32 hash of the uploaded data.
        pub crc32c: pulumi_gestalt_rust::Output<String>,
        /// Enables object encryption with Customer-Supplied Encryption Key (CSEK). Google [documentation about CSEK.](https://cloud.google.com/storage/docs/encryption/customer-supplied-keys)
        /// Structure is documented below.
        pub customer_encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketObjectCustomerEncryption>,
        >,
        pub detect_md5hash: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether an object is under [event-based hold](https://cloud.google.com/storage/docs/object-holds#hold-types). Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any).
        pub event_based_hold: pulumi_gestalt_rust::Output<Option<bool>>,
        /// (Computed) The content generation of this object. Used for object [versioning](https://cloud.google.com/storage/docs/object-versioning) and [soft delete](https://cloud.google.com/storage/docs/soft-delete).
        pub generation: pulumi_gestalt_rust::Output<i32>,
        /// The resource name of the Cloud KMS key that will be used to [encrypt](https://cloud.google.com/storage/docs/encryption/using-customer-managed-keys) the object.
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        /// (Computed) Base 64 MD5 hash of the uploaded data.
        pub md5hash: pulumi_gestalt_rust::Output<String>,
        /// (Computed) A url reference to download this object.
        pub media_link: pulumi_gestalt_rust::Output<String>,
        /// User-provided metadata, in key/value pairs.
        ///
        /// One of the following is required:
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the object. If you're interpolating the name of this object, see `output_name` instead.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The name of the object. Use this field in interpolations with `gcp.storage.ObjectACL` to recreate
        /// `gcp.storage.ObjectACL` resources when your `gcp.storage.BucketObject` is recreated.
        pub output_name: pulumi_gestalt_rust::Output<String>,
        /// The [object retention](http://cloud.google.com/storage/docs/object-lock) settings for the object. The retention settings allow an object to be retained until a provided date. Structure is documented below.
        pub retention: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketObjectRetention>,
        >,
        /// (Computed) A url reference to this object.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A path to the data you want to upload. Must be defined
        /// if `content` is not.
        ///
        /// - - -
        pub source: pulumi_gestalt_rust::Output<Option<String>>,
        /// The [StorageClass](https://cloud.google.com/storage/docs/storage-classes) of the new bucket object.
        /// Supported values include: `MULTI_REGIONAL`, `REGIONAL`, `NEARLINE`, `COLDLINE`, `ARCHIVE`. If not provided, this defaults to the bucket's default
        /// storage class or to a [standard](https://cloud.google.com/storage/docs/storage-classes#standard) class.
        pub storage_class: pulumi_gestalt_rust::Output<String>,
        /// Whether an object is under [temporary hold](https://cloud.google.com/storage/docs/object-holds#hold-types). While this flag is set to true, the object is protected against deletion and overwrites.
        pub temporary_hold: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketObjectArgs,
    ) -> BucketObjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let cache_control_binding = args.cache_control.get_output(context);
        let content_binding = args.content.get_output(context);
        let content_disposition_binding = args.content_disposition.get_output(context);
        let content_encoding_binding = args.content_encoding.get_output(context);
        let content_language_binding = args.content_language.get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let customer_encryption_binding = args.customer_encryption.get_output(context);
        let detect_md5hash_binding = args.detect_md5hash.get_output(context);
        let event_based_hold_binding = args.event_based_hold.get_output(context);
        let kms_key_name_binding = args.kms_key_name.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let retention_binding = args.retention.get_output(context);
        let source_binding = args.source.get_output(context);
        let storage_class_binding = args.storage_class.get_output(context);
        let temporary_hold_binding = args.temporary_hold.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/bucketObject:BucketObject".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheControl".into(),
                    value: cache_control_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "content".into(),
                    value: content_binding.get_id(),
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
                    name: "customerEncryption".into(),
                    value: customer_encryption_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectMd5hash".into(),
                    value: detect_md5hash_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventBasedHold".into(),
                    value: event_based_hold_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyName".into(),
                    value: kms_key_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retention".into(),
                    value: retention_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageClass".into(),
                    value: storage_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "temporaryHold".into(),
                    value: temporary_hold_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketObjectResult {
            bucket: o.get_field("bucket"),
            cache_control: o.get_field("cacheControl"),
            content: o.get_field("content"),
            content_disposition: o.get_field("contentDisposition"),
            content_encoding: o.get_field("contentEncoding"),
            content_language: o.get_field("contentLanguage"),
            content_type: o.get_field("contentType"),
            crc32c: o.get_field("crc32c"),
            customer_encryption: o.get_field("customerEncryption"),
            detect_md5hash: o.get_field("detectMd5hash"),
            event_based_hold: o.get_field("eventBasedHold"),
            generation: o.get_field("generation"),
            kms_key_name: o.get_field("kmsKeyName"),
            md5hash: o.get_field("md5hash"),
            media_link: o.get_field("mediaLink"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            output_name: o.get_field("outputName"),
            retention: o.get_field("retention"),
            self_link: o.get_field("selfLink"),
            source: o.get_field("source"),
            storage_class: o.get_field("storageClass"),
            temporary_hold: o.get_field("temporaryHold"),
        }
    }
}
