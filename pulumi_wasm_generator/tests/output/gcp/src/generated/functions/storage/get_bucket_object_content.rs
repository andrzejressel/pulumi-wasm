pub mod get_bucket_object_content {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketObjectContentArgs {
        /// The name of the containing bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// (Computed) [Content-Language](https://tools.ietf.org/html/rfc7231#section-3.1.3.2) of the object content.
        #[builder(into, default)]
        pub content: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the object.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetBucketObjectContentResult {
        pub bucket: pulumi_wasm_rust::Output<String>,
        pub cache_control: pulumi_wasm_rust::Output<String>,
        /// (Computed) [Content-Language](https://tools.ietf.org/html/rfc7231#section-3.1.3.2) of the object content.
        pub content: pulumi_wasm_rust::Output<Option<String>>,
        pub content_disposition: pulumi_wasm_rust::Output<String>,
        pub content_encoding: pulumi_wasm_rust::Output<String>,
        pub content_language: pulumi_wasm_rust::Output<String>,
        pub content_type: pulumi_wasm_rust::Output<String>,
        pub crc32c: pulumi_wasm_rust::Output<String>,
        pub customer_encryptions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::storage::GetBucketObjectContentCustomerEncryption,
            >,
        >,
        pub detect_md5hash: pulumi_wasm_rust::Output<String>,
        pub event_based_hold: pulumi_wasm_rust::Output<bool>,
        pub generation: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        pub md5hash: pulumi_wasm_rust::Output<String>,
        pub media_link: pulumi_wasm_rust::Output<String>,
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub output_name: pulumi_wasm_rust::Output<String>,
        pub retentions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketObjectContentRetention>,
        >,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub source: pulumi_wasm_rust::Output<String>,
        pub storage_class: pulumi_wasm_rust::Output<String>,
        pub temporary_hold: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBucketObjectContentArgs) -> GetBucketObjectContentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let content_binding = args.content.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:storage/getBucketObjectContent:getBucketObjectContent".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "cacheControl".into(),
                },
                register_interface::ResultField {
                    name: "content".into(),
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
                    name: "crc32c".into(),
                },
                register_interface::ResultField {
                    name: "customerEncryptions".into(),
                },
                register_interface::ResultField {
                    name: "detectMd5hash".into(),
                },
                register_interface::ResultField {
                    name: "eventBasedHold".into(),
                },
                register_interface::ResultField {
                    name: "generation".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "md5hash".into(),
                },
                register_interface::ResultField {
                    name: "mediaLink".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outputName".into(),
                },
                register_interface::ResultField {
                    name: "retentions".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "storageClass".into(),
                },
                register_interface::ResultField {
                    name: "temporaryHold".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBucketObjectContentResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            cache_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheControl").unwrap(),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
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
            crc32c: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("crc32c").unwrap(),
            ),
            customer_encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerEncryptions").unwrap(),
            ),
            detect_md5hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectMd5hash").unwrap(),
            ),
            event_based_hold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventBasedHold").unwrap(),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generation").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            md5hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("md5hash").unwrap(),
            ),
            media_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mediaLink").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            output_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputName").unwrap(),
            ),
            retentions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentions").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            storage_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageClass").unwrap(),
            ),
            temporary_hold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("temporaryHold").unwrap(),
            ),
        }
    }
}
