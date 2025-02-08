/// Manages a project-level logging bucket config. For more information see
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
///     let basic = project_bucket_config::create(
///         "basic",
///         ProjectBucketConfigArgs::builder()
///             .bucket_id("_Default")
///             .location("global")
///             .project("${default.projectId}")
///             .retention_days(30)
///             .build_struct(),
///     );
///     let default = project::create(
///         "default",
///         ProjectArgs::builder()
///             .name("your-project-id")
///             .org_id("123456789")
///             .project_id("your-project-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Create logging bucket with customId
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = project_bucket_config::create(
///         "basic",
///         ProjectBucketConfigArgs::builder()
///             .bucket_id("custom-bucket")
///             .location("global")
///             .project("project_id")
///             .retention_days(30)
///             .build_struct(),
///     );
/// }
/// ```
///
/// Create logging bucket with Log Analytics enabled
///
/// ```yaml
/// resources:
///   analytics-enabled-bucket:
///     type: gcp:logging:ProjectBucketConfig
///     properties:
///       project: project_id
///       location: global
///       retentionDays: 30
///       enableAnalytics: true
///       bucketId: custom-bucket
/// ```
///
/// Create logging bucket with customId and cmekSettings
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: us-central1
///   key:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: crypto-key-example
///       keyRing: ${keyring.id}
///       rotationPeriod: 7776000s
///   cryptoKeyBinding:
///     type: gcp:kms:CryptoKeyIAMBinding
///     name: crypto_key_binding
///     properties:
///       cryptoKeyId: ${key.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       members:
///         - serviceAccount:${cmekSettings.serviceAccountId}
///   example-project-bucket-cmek-settings:
///     type: gcp:logging:ProjectBucketConfig
///     properties:
///       project: project_id
///       location: us-central1
///       retentionDays: 30
///       bucketId: custom-bucket
///       cmekSettings:
///         kmsKeyName: ${key.id}
///     options:
///       dependsOn:
///         - ${cryptoKeyBinding}
/// variables:
///   cmekSettings:
///     fn::invoke:
///       function: gcp:logging:getProjectCmekSettings
///       arguments:
///         project: project_id
/// ```
///
/// Create logging bucket with index configs
///
/// ```yaml
/// resources:
///   example-project-bucket-index-configs:
///     type: gcp:logging:ProjectBucketConfig
///     properties:
///       project: project_id
///       location: global
///       retentionDays: 30
///       bucketId: custom-bucket
///       indexConfigs:
///         - fieldPath: jsonPayload.request.status
///           type: INDEX_TYPE_STRING
/// ```
///
/// ## Import
///
/// This resource can be imported using the following format:
///
/// * `projects/{{project}}/locations/{{location}}/buckets/{{bucket_id}}`
///
/// When using the `pulumi import` command, this resource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/projectBucketConfig:ProjectBucketConfig default projects/{{project}}/locations/{{location}}/buckets/{{bucket_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod project_bucket_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectBucketConfigArgs {
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        #[builder(into)]
        pub bucket_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by updating the log bucket. Changing the KMS key is allowed. Structure is documented below.
        #[builder(into, default)]
        pub cmek_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logging::ProjectBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether or not Log Analytics is enabled. Logs for buckets with Log Analytics enabled can be queried in the **Log Analytics** page using SQL queries. Cannot be disabled once enabled.
        #[builder(into, default)]
        pub enable_analytics: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        #[builder(into, default)]
        pub index_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::logging::ProjectBucketConfigIndexConfig>>,
        >,
        /// The location of the bucket.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the bucket is locked. The retention period on a locked bucket cannot be changed. Locked buckets may only be deleted if they are empty.
        #[builder(into, default)]
        pub locked: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The parent resource that contains the logging bucket.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used.
        #[builder(into, default)]
        pub retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct ProjectBucketConfigResult {
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        pub bucket_id: pulumi_gestalt_rust::Output<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by updating the log bucket. Changing the KMS key is allowed. Structure is documented below.
        pub cmek_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::logging::ProjectBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether or not Log Analytics is enabled. Logs for buckets with Log Analytics enabled can be queried in the **Log Analytics** page using SQL queries. Cannot be disabled once enabled.
        pub enable_analytics: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        pub index_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::logging::ProjectBucketConfigIndexConfig>>,
        >,
        /// The bucket's lifecycle such as active or deleted. See [LifecycleState](https://cloud.google.com/logging/docs/reference/v2/rest/v2/billingAccounts.buckets#LogBucket.LifecycleState).
        pub lifecycle_state: pulumi_gestalt_rust::Output<String>,
        /// The location of the bucket.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Whether the bucket is locked. The retention period on a locked bucket cannot be changed. Locked buckets may only be deleted if they are empty.
        pub locked: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The resource name of the bucket. For example: "projects/my-project-id/locations/my-location/buckets/my-bucket-id"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent resource that contains the logging bucket.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used.
        pub retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProjectBucketConfigArgs,
    ) -> ProjectBucketConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_id_binding = args.bucket_id.get_output(context).get_inner();
        let cmek_settings_binding = args.cmek_settings.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enable_analytics_binding = args
            .enable_analytics
            .get_output(context)
            .get_inner();
        let index_configs_binding = args.index_configs.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let locked_binding = args.locked.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let retention_days_binding = args.retention_days.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/projectBucketConfig:ProjectBucketConfig".into(),
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
                    name: "enableAnalytics".into(),
                    value: &enable_analytics_binding,
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
                    name: "locked".into(),
                    value: &locked_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectBucketConfigResult {
            bucket_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucketId"),
            ),
            cmek_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cmekSettings"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enable_analytics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableAnalytics"),
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
            locked: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locked"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionDays"),
            ),
        }
    }
}
