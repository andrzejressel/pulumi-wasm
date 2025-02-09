/// Manages a folder-level logging bucket config. For more information see
/// [the official logging documentation](https://cloud.google.com/logging/docs/) and
/// [Storing Logs](https://cloud.google.com/logging/docs/storage).
///
/// > **Note:** Logging buckets are automatically created for a given folder, project, organization, billingAccount and cannot be deleted. Creating a resource of this type will acquire and update the resource that already exists at the desired location. These buckets cannot be removed so deleting this resource will remove the bucket config from your state but will leave the logging bucket unchanged. The buckets that are currently automatically created are "_Default" and "_Required".
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = folder_bucket_config::create(
///         "basic",
///         FolderBucketConfigArgs::builder()
///             .bucket_id("_Default")
///             .folder("${default.name}")
///             .index_configs(
///                 vec![
///                     FolderBucketConfigIndexConfig::builder()
///                     .fieldPath("jsonPayload.request.status"). type ("INDEX_TYPE_STRING")
///                     .build_struct(),
///                 ],
///             )
///             .location("global")
///             .retention_days(30)
///             .build_struct(),
///     );
///     let default = folder::create(
///         "default",
///         FolderArgs::builder()
///             .display_name("some-folder-name")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// This resource can be imported using the following format:
///
/// * `folders/{{folder}}/locations/{{location}}/buckets/{{bucket_id}}`
///
/// When using the `pulumi import` command, this resource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/folderBucketConfig:FolderBucketConfig default folders/{{folder}}/locations/{{location}}/buckets/{{bucket_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod folder_bucket_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderBucketConfigArgs {
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        #[builder(into)]
        pub bucket_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK
        /// key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by
        /// updating the log bucket. Changing the KMS key is allowed.
        #[builder(into, default)]
        pub cmek_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logging::FolderBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent resource that contains the logging bucket.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        #[builder(into, default)]
        pub index_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::logging::FolderBucketConfigIndexConfig>>,
        >,
        /// The location of the bucket.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. Bucket retention can not be increased on buckets outside of projects.
        #[builder(into, default)]
        pub retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct FolderBucketConfigResult {
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        pub bucket_id: pulumi_gestalt_rust::Output<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK
        /// key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by
        /// updating the log bucket. Changing the KMS key is allowed.
        pub cmek_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::logging::FolderBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The parent resource that contains the logging bucket.
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        pub index_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::logging::FolderBucketConfigIndexConfig>,
        >,
        /// The bucket's lifecycle such as active or deleted. See [LifecycleState](https://cloud.google.com/logging/docs/reference/v2/rest/v2/billingAccounts.buckets#LogBucket.LifecycleState).
        pub lifecycle_state: pulumi_gestalt_rust::Output<String>,
        /// The location of the bucket.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the bucket. For example: "folders/my-folder-id/locations/my-location/buckets/my-bucket-id"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. Bucket retention can not be increased on buckets outside of projects.
        pub retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FolderBucketConfigArgs,
    ) -> FolderBucketConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_id_binding_1 = args.bucket_id.get_output(context);
        let bucket_id_binding = bucket_id_binding_1.get_inner();
        let cmek_settings_binding_1 = args.cmek_settings.get_output(context);
        let cmek_settings_binding = cmek_settings_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let folder_binding_1 = args.folder.get_output(context);
        let folder_binding = folder_binding_1.get_inner();
        let index_configs_binding_1 = args.index_configs.get_output(context);
        let index_configs_binding = index_configs_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let retention_days_binding_1 = args.retention_days.get_output(context);
        let retention_days_binding = retention_days_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/folderBucketConfig:FolderBucketConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucketId".into(),
                    value: &bucket_id_binding,
                },
                register_interface::ObjectField {
                    name: "cmekSettings".into(),
                    value: &cmek_settings_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "indexConfigs".into(),
                    value: &index_configs_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FolderBucketConfigResult {
            bucket_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucketId"),
            ),
            cmek_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cmekSettings"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folder"),
            ),
            index_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("indexConfigs"),
            ),
            lifecycle_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lifecycleState"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionDays"),
            ),
        }
    }
}
