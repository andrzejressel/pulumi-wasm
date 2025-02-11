/// A key for signing Cloud CDN signed URLs for BackendBuckets.
///
///
/// To get more information about BackendBucketSignedUrlKey, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/backendBuckets)
/// * How-to Guides
///     * [Using Signed URLs](https://cloud.google.com/cdn/docs/using-signed-urls/)
///
///
///
/// ## Example Usage
///
/// ### Backend Bucket Signed Url Key
///
///
/// ```yaml
/// resources:
///   urlSignature:
///     type: random:RandomId
///     name: url_signature
///     properties:
///       byteLength: 16
///   backendKey:
///     type: gcp:compute:BackendBucketSignedUrlKey
///     name: backend_key
///     properties:
///       name: test-key
///       keyValue: ${urlSignature.b64Url}
///       backendBucket: ${testBackend.name}
///   testBackend:
///     type: gcp:compute:BackendBucket
///     name: test_backend
///     properties:
///       name: test-signed-backend-bucket
///       description: Contains beautiful images
///       bucketName: ${bucket.name}
///       enableCdn: true
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: test-storage-bucket
///       location: EU
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backend_bucket_signed_url_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendBucketSignedUrlKeyArgs {
        /// The backend bucket this signed URL key belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backend_bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// 128-bit key value used for signing the URL. The key value must be a
        /// valid RFC 4648 Section 5 base64url encoded string.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub key_value: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the signed URL key.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackendBucketSignedUrlKeyResult {
        /// The backend bucket this signed URL key belongs.
        ///
        ///
        /// - - -
        pub backend_bucket: pulumi_gestalt_rust::Output<String>,
        /// 128-bit key value used for signing the URL. The key value must be a
        /// valid RFC 4648 Section 5 base64url encoded string.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub key_value: pulumi_gestalt_rust::Output<String>,
        /// Name of the signed URL key.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackendBucketSignedUrlKeyArgs,
    ) -> BackendBucketSignedUrlKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backend_bucket_binding = args.backend_bucket.get_output(context);
        let key_value_binding = args.key_value.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/backendBucketSignedUrlKey:BackendBucketSignedUrlKey"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backendBucket".into(),
                    value: &backend_bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyValue".into(),
                    value: &key_value_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackendBucketSignedUrlKeyResult {
            backend_bucket: o.get_field("backendBucket"),
            key_value: o.get_field("keyValue"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
