pub mod get_bucket_objects {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketObjectsArgs {
        /// Lists object keys in this S3 bucket. Alternatively, an [S3 access point](https://docs.aws.amazon.com/AmazonS3/latest/dev/using-access-points.html) ARN can be specified
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Character used to group keys (Default: none)
        #[builder(into, default)]
        pub delimiter: pulumi_wasm_rust::Output<Option<String>>,
        /// Encodes keys using this method (Default: none; besides none, only "url" can be used)
        #[builder(into, default)]
        pub encoding_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean specifying whether to populate the owner list (Default: false)
        #[builder(into, default)]
        pub fetch_owner: pulumi_wasm_rust::Output<Option<bool>>,
        /// Maximum object keys to return (Default: 1000)
        #[builder(into, default)]
        pub max_keys: pulumi_wasm_rust::Output<Option<i32>>,
        /// Limits results to object keys with this prefix (Default: none)
        #[builder(into, default)]
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Returns key names lexicographically after a specific object key in your bucket (Default: none; S3 lists object keys in UTF-8 character encoding in lexicographical order)
        #[builder(into, default)]
        pub start_after: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBucketObjectsResult {
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// List of any keys between `prefix` and the next occurrence of `delimiter` (i.e., similar to subdirectories of the `prefix` "directory"); the list is only returned when you specify `delimiter`
        pub common_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        pub delimiter: pulumi_wasm_rust::Output<Option<String>>,
        pub encoding_type: pulumi_wasm_rust::Output<Option<String>>,
        pub fetch_owner: pulumi_wasm_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of strings representing object keys
        pub keys: pulumi_wasm_rust::Output<Vec<String>>,
        pub max_keys: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of strings representing object owner IDs (see `fetch_owner` above)
        pub owners: pulumi_wasm_rust::Output<Vec<String>>,
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
        pub start_after: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBucketObjectsArgs) -> GetBucketObjectsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let delimiter_binding = args.delimiter.get_inner();
        let encoding_type_binding = args.encoding_type.get_inner();
        let fetch_owner_binding = args.fetch_owner.get_inner();
        let max_keys_binding = args.max_keys.get_inner();
        let prefix_binding = args.prefix.get_inner();
        let start_after_binding = args.start_after.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3/getBucketObjects:getBucketObjects".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "delimiter".into(),
                    value: &delimiter_binding,
                },
                register_interface::ObjectField {
                    name: "encodingType".into(),
                    value: &encoding_type_binding,
                },
                register_interface::ObjectField {
                    name: "fetchOwner".into(),
                    value: &fetch_owner_binding,
                },
                register_interface::ObjectField {
                    name: "maxKeys".into(),
                    value: &max_keys_binding,
                },
                register_interface::ObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding,
                },
                register_interface::ObjectField {
                    name: "startAfter".into(),
                    value: &start_after_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "commonPrefixes".into(),
                },
                register_interface::ResultField {
                    name: "delimiter".into(),
                },
                register_interface::ResultField {
                    name: "encodingType".into(),
                },
                register_interface::ResultField {
                    name: "fetchOwner".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keys".into(),
                },
                register_interface::ResultField {
                    name: "maxKeys".into(),
                },
                register_interface::ResultField {
                    name: "owners".into(),
                },
                register_interface::ResultField {
                    name: "prefix".into(),
                },
                register_interface::ResultField {
                    name: "startAfter".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBucketObjectsResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            common_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commonPrefixes").unwrap(),
            ),
            delimiter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delimiter").unwrap(),
            ),
            encoding_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encodingType").unwrap(),
            ),
            fetch_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fetchOwner").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keys").unwrap(),
            ),
            max_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxKeys").unwrap(),
            ),
            owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owners").unwrap(),
            ),
            prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefix").unwrap(),
            ),
            start_after: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startAfter").unwrap(),
            ),
        }
    }
}
