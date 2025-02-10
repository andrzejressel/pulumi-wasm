#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bucket_objects {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketObjectsArgs {
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
        /// Returns key names lexicographically after a specific object key in your bucket (Default: none; S3 lists object keys in UTF-8 character encoding in lexicographical order)
        #[builder(into, default)]
        pub start_after: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBucketObjectsResult {
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
        pub start_after: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBucketObjectsArgs,
    ) -> GetBucketObjectsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let delimiter_binding = args.delimiter.get_output(context);
        let encoding_type_binding = args.encoding_type.get_output(context);
        let fetch_owner_binding = args.fetch_owner.get_output(context);
        let max_keys_binding = args.max_keys.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let start_after_binding = args.start_after.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:s3/getBucketObjects:getBucketObjects".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delimiter".into(),
                    value: delimiter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encodingType".into(),
                    value: encoding_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fetchOwner".into(),
                    value: fetch_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxKeys".into(),
                    value: max_keys_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startAfter".into(),
                    value: start_after_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBucketObjectsResult {
            bucket: o.get_field("bucket"),
            common_prefixes: o.get_field("commonPrefixes"),
            delimiter: o.get_field("delimiter"),
            encoding_type: o.get_field("encodingType"),
            fetch_owner: o.get_field("fetchOwner"),
            id: o.get_field("id"),
            keys: o.get_field("keys"),
            max_keys: o.get_field("maxKeys"),
            owners: o.get_field("owners"),
            prefix: o.get_field("prefix"),
            start_after: o.get_field("startAfter"),
        }
    }
}
