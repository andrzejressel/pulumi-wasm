/// Provides a lightsail resource access to a bucket.
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_bucket_resource_access` using the `id` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/bucketResourceAccess:BucketResourceAccess test example-bucket,example-instance
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_resource_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketResourceAccessArgs {
        /// The name of the bucket to grant access to.
        #[builder(into)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource to be granted bucket access.
        #[builder(into)]
        pub resource_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketResourceAccessResult {
        /// The name of the bucket to grant access to.
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource to be granted bucket access.
        pub resource_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketResourceAccessArgs,
    ) -> BucketResourceAccessResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_name_binding = args.bucket_name.get_output(context);
        let resource_name_binding = args.resource_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/bucketResourceAccess:BucketResourceAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceName".into(),
                    value: &resource_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketResourceAccessResult {
            bucket_name: o.get_field("bucketName"),
            resource_name: o.get_field("resourceName"),
        }
    }
}
