pub mod get_bucket_object_content {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketObjectContentArgs {
        /// The name of the containing bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// (Computed) [Content-Language](https://tools.ietf.org/html/rfc7231#section-3.1.3.2) of the object content.
        #[builder(into, default)]
        pub content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the object.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBucketObjectContentResult {
        pub bucket: pulumi_gestalt_rust::Output<String>,
        pub cache_control: pulumi_gestalt_rust::Output<String>,
        /// (Computed) [Content-Language](https://tools.ietf.org/html/rfc7231#section-3.1.3.2) of the object content.
        pub content: pulumi_gestalt_rust::Output<Option<String>>,
        pub content_disposition: pulumi_gestalt_rust::Output<String>,
        pub content_encoding: pulumi_gestalt_rust::Output<String>,
        pub content_language: pulumi_gestalt_rust::Output<String>,
        pub content_type: pulumi_gestalt_rust::Output<String>,
        pub crc32c: pulumi_gestalt_rust::Output<String>,
        pub customer_encryptions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::storage::GetBucketObjectContentCustomerEncryption,
            >,
        >,
        pub detect_md5hash: pulumi_gestalt_rust::Output<String>,
        pub event_based_hold: pulumi_gestalt_rust::Output<bool>,
        pub generation: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        pub md5hash: pulumi_gestalt_rust::Output<String>,
        pub media_link: pulumi_gestalt_rust::Output<String>,
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub output_name: pulumi_gestalt_rust::Output<String>,
        pub retentions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketObjectContentRetention>,
        >,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub source: pulumi_gestalt_rust::Output<String>,
        pub storage_class: pulumi_gestalt_rust::Output<String>,
        pub temporary_hold: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBucketObjectContentArgs,
    ) -> GetBucketObjectContentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let content_binding = args.content.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:storage/getBucketObjectContent:getBucketObjectContent".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBucketObjectContentResult {
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            cache_control: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheControl"),
            ),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
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
            crc32c: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("crc32c"),
            ),
            customer_encryptions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerEncryptions"),
            ),
            detect_md5hash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("detectMd5hash"),
            ),
            event_based_hold: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventBasedHold"),
            ),
            generation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("generation"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            md5hash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("md5hash"),
            ),
            media_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mediaLink"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            output_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputName"),
            ),
            retentions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentions"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            storage_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageClass"),
            ),
            temporary_hold: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("temporaryHold"),
            ),
        }
    }
}
