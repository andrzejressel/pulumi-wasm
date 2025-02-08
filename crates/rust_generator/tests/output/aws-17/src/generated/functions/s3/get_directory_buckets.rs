#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_directory_buckets {
    #[allow(dead_code)]
    pub struct GetDirectoryBucketsResult {
        /// Bucket ARNs.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Buckets names.
        pub buckets: pulumi_gestalt_rust::Output<Vec<String>>,
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
    ) -> GetDirectoryBucketsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3/getDirectoryBuckets:getDirectoryBuckets".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDirectoryBucketsResult {
            arns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arns")),
            buckets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("buckets"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
