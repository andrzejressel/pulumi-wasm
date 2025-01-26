pub mod get_directory_buckets {
    #[allow(dead_code)]
    pub struct GetDirectoryBucketsResult {
        /// Bucket ARNs.
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Buckets names.
        pub buckets: pulumi_wasm_rust::Output<Vec<String>>,
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
    ) -> GetDirectoryBucketsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3/getDirectoryBuckets:getDirectoryBuckets".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDirectoryBucketsResult {
            arns: pulumi_wasm_rust::__private::into_domain(o.extract_field("arns")),
            buckets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("buckets"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
