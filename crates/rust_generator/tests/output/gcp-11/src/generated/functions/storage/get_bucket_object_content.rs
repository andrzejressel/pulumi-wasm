#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetBucketObjectContentArgs,
    ) -> GetBucketObjectContentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let content_binding = args.content.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:storage/getBucketObjectContent:getBucketObjectContent".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "content".into(),
                    value: &content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBucketObjectContentResult {
            bucket: o.get_field("bucket"),
            cache_control: o.get_field("cacheControl"),
            content: o.get_field("content"),
            content_disposition: o.get_field("contentDisposition"),
            content_encoding: o.get_field("contentEncoding"),
            content_language: o.get_field("contentLanguage"),
            content_type: o.get_field("contentType"),
            crc32c: o.get_field("crc32c"),
            customer_encryptions: o.get_field("customerEncryptions"),
            detect_md5hash: o.get_field("detectMd5hash"),
            event_based_hold: o.get_field("eventBasedHold"),
            generation: o.get_field("generation"),
            id: o.get_field("id"),
            kms_key_name: o.get_field("kmsKeyName"),
            md5hash: o.get_field("md5hash"),
            media_link: o.get_field("mediaLink"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            output_name: o.get_field("outputName"),
            retentions: o.get_field("retentions"),
            self_link: o.get_field("selfLink"),
            source: o.get_field("source"),
            storage_class: o.get_field("storageClass"),
            temporary_hold: o.get_field("temporaryHold"),
        }
    }
}
