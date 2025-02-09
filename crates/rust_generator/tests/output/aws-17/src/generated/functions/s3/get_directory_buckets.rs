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
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetDirectoryBucketsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:s3/getDirectoryBuckets:getDirectoryBuckets".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetDirectoryBucketsResult {
            arns: o.get_field("arns"),
            buckets: o.get_field("buckets"),
            id: o.get_field("id"),
        }
    }
}
