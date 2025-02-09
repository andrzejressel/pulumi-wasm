/// ## Example Usage
///
/// ### Firebasestorage Bucket Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = bucket::create(
///         "default",
///         BucketArgs::builder()
///             .location("US")
///             .name("test_bucket")
///             .uniform_bucket_level_access(true)
///             .build_struct(),
///     );
///     let defaultStorageBucket = storage_bucket::create(
///         "defaultStorageBucket",
///         StorageBucketArgs::builder()
///             .bucket_id("${default.id}")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Bucket can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/buckets/{{bucket_id}}`
///
/// * `{{project}}/{{bucket_id}}`
///
/// * `{{bucket_id}}`
///
/// When using the `pulumi import` command, Bucket can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/storageBucket:StorageBucket default projects/{{project}}/buckets/{{bucket_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/storageBucket:StorageBucket default {{project}}/{{bucket_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/storageBucket:StorageBucket default {{bucket_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod storage_bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StorageBucketArgs {
        /// Required. Immutable. The ID of the underlying Google Cloud Storage bucket
        #[builder(into, default)]
        pub bucket_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StorageBucketResult {
        /// Required. Immutable. The ID of the underlying Google Cloud Storage bucket
        pub bucket_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource name of the bucket in the format projects/PROJECT_IDENTIFIER/buckets/BUCKET_ID
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
        args: StorageBucketArgs,
    ) -> StorageBucketResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_id_binding = args.bucket_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/storageBucket:StorageBucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketId".into(),
                    value: bucket_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StorageBucketResult {
            bucket_id: o.get_field("bucketId"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
