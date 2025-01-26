pub mod get_bucket_object {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketObjectArgs {
        /// The name of the containing bucket.
        #[builder(into, default)]
        pub bucket: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the object.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBucketObjectResult {
        pub bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// (Computed) [Cache-Control](https://tools.ietf.org/html/rfc7234#section-5.2)
        /// directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600
        pub cache_control: pulumi_wasm_rust::Output<String>,
        pub content: pulumi_wasm_rust::Output<String>,
        /// (Computed) [Content-Disposition](https://tools.ietf.org/html/rfc6266) of the object data.
        pub content_disposition: pulumi_wasm_rust::Output<String>,
        /// (Computed) [Content-Encoding](https://tools.ietf.org/html/rfc7231#section-3.1.2.2) of the object data.
        pub content_encoding: pulumi_wasm_rust::Output<String>,
        /// (Computed) [Content-Language](https://tools.ietf.org/html/rfc7231#section-3.1.3.2) of the object data.
        pub content_language: pulumi_wasm_rust::Output<String>,
        /// (Computed) [Content-Type](https://tools.ietf.org/html/rfc7231#section-3.1.1.5) of the object data. Defaults to "application/octet-stream" or "text/plain; charset=utf-8".
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// (Computed) Base 64 CRC32 hash of the uploaded data.
        pub crc32c: pulumi_wasm_rust::Output<String>,
        pub customer_encryptions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketObjectCustomerEncryption>,
        >,
        pub detect_md5hash: pulumi_wasm_rust::Output<String>,
        /// (Computed) Whether an object is under [event-based hold](https://cloud.google.com/storage/docs/object-holds#hold-types). Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any).
        pub event_based_hold: pulumi_wasm_rust::Output<bool>,
        /// (Computed) The content generation of this object. Used for object [versioning](https://cloud.google.com/storage/docs/object-versioning) and [soft delete](https://cloud.google.com/storage/docs/soft-delete).
        pub generation: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        /// (Computed) Base 64 MD5 hash of the uploaded data.
        pub md5hash: pulumi_wasm_rust::Output<String>,
        /// (Computed) A url reference to download this object.
        pub media_link: pulumi_wasm_rust::Output<String>,
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub output_name: pulumi_wasm_rust::Output<String>,
        pub retentions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketObjectRetention>,
        >,
        /// (Computed) A url reference to this object.
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub source: pulumi_wasm_rust::Output<String>,
        /// (Computed) The [StorageClass](https://cloud.google.com/storage/docs/storage-classes) of the new bucket object.
        /// Supported values include: `MULTI_REGIONAL`, `REGIONAL`, `NEARLINE`, `COLDLINE`, `ARCHIVE`. If not provided, this defaults to the bucket's default
        /// storage class or to a [standard](https://cloud.google.com/storage/docs/storage-classes#standard) class.
        pub storage_class: pulumi_wasm_rust::Output<String>,
        /// (Computed) Whether an object is under [temporary hold](https://cloud.google.com/storage/docs/object-holds#hold-types). While this flag is set to true, the object is protected against deletion and overwrites.
        pub temporary_hold: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBucketObjectArgs,
    ) -> GetBucketObjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:storage/getBucketObject:getBucketObject".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBucketObjectResult {
            bucket: pulumi_wasm_rust::__private::into_domain(o.extract_field("bucket")),
            cache_control: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cacheControl"),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("content"),
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
            content_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            crc32c: pulumi_wasm_rust::__private::into_domain(o.extract_field("crc32c")),
            customer_encryptions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerEncryptions"),
            ),
            detect_md5hash: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("detectMd5hash"),
            ),
            event_based_hold: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventBasedHold"),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("generation"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            md5hash: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("md5hash"),
            ),
            media_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mediaLink"),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            output_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outputName"),
            ),
            retentions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retentions"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            source: pulumi_wasm_rust::__private::into_domain(o.extract_field("source")),
            storage_class: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageClass"),
            ),
            temporary_hold: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("temporaryHold"),
            ),
        }
    }
}
