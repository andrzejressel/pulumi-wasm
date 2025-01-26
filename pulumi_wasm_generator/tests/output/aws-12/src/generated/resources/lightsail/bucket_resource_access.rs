/// Provides a lightsail resource access to a bucket.
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_bucket_resource_access` using the `id` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/bucketResourceAccess:BucketResourceAccess test example-bucket,example-instance
/// ```
pub mod bucket_resource_access {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketResourceAccessArgs {
        /// The name of the bucket to grant access to.
        #[builder(into)]
        pub bucket_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource to be granted bucket access.
        #[builder(into)]
        pub resource_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketResourceAccessResult {
        /// The name of the bucket to grant access to.
        pub bucket_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource to be granted bucket access.
        pub resource_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BucketResourceAccessArgs,
    ) -> BucketResourceAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_name_binding = args.bucket_name.get_output(context).get_inner();
        let resource_name_binding = args.resource_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/bucketResourceAccess:BucketResourceAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceName".into(),
                    value: &resource_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucketName".into(),
                },
                register_interface::ResultField {
                    name: "resourceName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketResourceAccessResult {
            bucket_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketName").unwrap(),
            ),
            resource_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceName").unwrap(),
            ),
        }
    }
}
