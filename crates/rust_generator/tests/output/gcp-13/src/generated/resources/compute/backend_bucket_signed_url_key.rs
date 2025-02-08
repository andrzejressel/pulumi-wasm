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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackendBucketSignedUrlKeyArgs,
    ) -> BackendBucketSignedUrlKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backend_bucket_binding = args.backend_bucket.get_output(context).get_inner();
        let key_value_binding = args.key_value.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/backendBucketSignedUrlKey:BackendBucketSignedUrlKey"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendBucket".into(),
                    value: &backend_bucket_binding,
                },
                register_interface::ObjectField {
                    name: "keyValue".into(),
                    value: &key_value_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackendBucketSignedUrlKeyResult {
            backend_bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendBucket"),
            ),
            key_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyValue"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
