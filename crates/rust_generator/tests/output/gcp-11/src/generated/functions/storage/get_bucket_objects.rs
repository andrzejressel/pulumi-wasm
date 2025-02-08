#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBucketObjectsArgs,
    ) -> GetBucketObjectsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let match_glob_binding = args.match_glob.get_output(context).get_inner();
        let prefix_binding = args.prefix.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:storage/getBucketObjects:getBucketObjects".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "matchGlob".into(),
                    value: &match_glob_binding,
                },
                register_interface::ObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBucketObjectsResult {
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            bucket_objects: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucketObjects"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            match_glob: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("matchGlob"),
            ),
            prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prefix"),
            ),
        }
    }
}
