#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bucket_objects {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketObjectsArgs {
        /// The name of the containing bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A glob pattern used to filter results (for example, `foo*bar`).
        #[builder(into, default)]
        pub match_glob: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Filter results to include only objects whose names begin with this prefix.
        #[builder(into, default)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBucketObjectsResult {
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// A list of retrieved objects contained in the provided GCS bucket. Structure is defined below.
        pub bucket_objects: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketObjectsBucketObject>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub match_glob: pulumi_gestalt_rust::Output<Option<String>>,
        pub prefix: pulumi_gestalt_rust::Output<Option<String>>,
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
        let match_glob_binding = args.match_glob.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:storage/getBucketObjects:getBucketObjects".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "matchGlob".into(),
                    value: &match_glob_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBucketObjectsResult {
            bucket: o.get_field("bucket"),
            bucket_objects: o.get_field("bucketObjects"),
            id: o.get_field("id"),
            match_glob: o.get_field("matchGlob"),
            prefix: o.get_field("prefix"),
        }
    }
}
