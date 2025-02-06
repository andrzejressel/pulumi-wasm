pub mod get_objects {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetObjectsArgs {
        /// Lists object keys in this S3 bucket. Alternatively, an [S3 access point](https://docs.aws.amazon.com/AmazonS3/latest/dev/using-access-points.html) ARN can be specified
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Character used to group keys (Default: none)
        #[builder(into, default)]
        pub delimiter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Encodes keys using this method (Default: none; besides none, only "url" can be used)
        #[builder(into, default)]
        pub encoding_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean specifying whether to populate the owner list (Default: false)
        #[builder(into, default)]
        pub fetch_owner: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Maximum object keys to return (Default: 1000)
        #[builder(into, default)]
        pub max_keys: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Limits results to object keys with this prefix (Default: none)
        #[builder(into, default)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If included, the only valid value is `requester`.
        #[builder(into, default)]
        pub request_payer: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Returns key names lexicographically after a specific object key in your bucket (Default: none; S3 lists object keys in UTF-8 character encoding in lexicographical order)
        #[builder(into, default)]
        pub start_after: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetObjectsResult {
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// List of any keys between `prefix` and the next occurrence of `delimiter` (i.e., similar to subdirectories of the `prefix` "directory"); the list is only returned when you specify `delimiter`
        pub common_prefixes: pulumi_gestalt_rust::Output<Vec<String>>,
        pub delimiter: pulumi_gestalt_rust::Output<Option<String>>,
        pub encoding_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub fetch_owner: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of strings representing object keys
        pub keys: pulumi_gestalt_rust::Output<Vec<String>>,
        pub max_keys: pulumi_gestalt_rust::Output<Option<i32>>,
        /// List of strings representing object owner IDs (see `fetch_owner` above)
        pub owners: pulumi_gestalt_rust::Output<Vec<String>>,
        pub prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// If present, indicates that the requester was successfully charged for the request.
        pub request_charged: pulumi_gestalt_rust::Output<String>,
        pub request_payer: pulumi_gestalt_rust::Output<Option<String>>,
        pub start_after: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetObjectsArgs,
    ) -> GetObjectsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let delimiter_binding = args.delimiter.get_output(context).get_inner();
        let encoding_type_binding = args.encoding_type.get_output(context).get_inner();
        let fetch_owner_binding = args.fetch_owner.get_output(context).get_inner();
        let max_keys_binding = args.max_keys.get_output(context).get_inner();
        let prefix_binding = args.prefix.get_output(context).get_inner();
        let request_payer_binding = args.request_payer.get_output(context).get_inner();
        let start_after_binding = args.start_after.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3/getObjects:getObjects".into(),
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
                    name: "requestPayer".into(),
                    value: &request_payer_binding,
                },
                register_interface::ObjectField {
                    name: "startAfter".into(),
                    value: &start_after_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetObjectsResult {
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            common_prefixes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("commonPrefixes"),
            ),
            delimiter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("delimiter"),
            ),
            encoding_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encodingType"),
            ),
            fetch_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fetchOwner"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            keys: pulumi_gestalt_rust::__private::into_domain(o.extract_field("keys")),
            max_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxKeys"),
            ),
            owners: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("owners"),
            ),
            prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prefix"),
            ),
            request_charged: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requestCharged"),
            ),
            request_payer: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requestPayer"),
            ),
            start_after: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startAfter"),
            ),
        }
    }
}
