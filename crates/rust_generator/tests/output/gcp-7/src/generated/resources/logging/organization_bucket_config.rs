/// Manages a organization-level logging bucket config. For more information see
/// [the official logging documentation](https://cloud.google.com/logging/docs/) and
/// [Storing Logs](https://cloud.google.com/logging/docs/storage).
///
/// > **Note:** Logging buckets are automatically created for a given folder, project, organization, billingAccount and cannot be deleted. Creating a resource of this type will acquire and update the resource that already exists at the desired location. These buckets cannot be removed so deleting this resource will remove the bucket config from your state but will leave the logging bucket unchanged. The buckets that are currently automatically created are "_Default" and "_Required".
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   basic:
///     type: gcp:logging:OrganizationBucketConfig
///     properties:
///       organization: ${default.organization}
///       location: global
///       retentionDays: 30
///       bucketId: _Default
///       indexConfigs:
///         - fieldPath: jsonPayload.request.status
///           type: INDEX_TYPE_STRING
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:organizations:getOrganization
///       arguments:
///         organization: '123456789'
/// ```
///
/// ## Import
///
/// This resource can be imported using the following format:
///
/// * `organizations/{{organization}}/locations/{{location}}/buckets/{{bucket_id}}`
///
/// When using the `pulumi import` command, this resource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/organizationBucketConfig:OrganizationBucketConfig default organizations/{{organization}}/locations/{{location}}/buckets/{{bucket_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_bucket_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationBucketConfigArgs {
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        #[builder(into)]
        pub bucket_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK
        /// key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by
        /// updating the log bucket. Changing the KMS key is allowed.
        #[builder(into, default)]
        pub cmek_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logging::OrganizationBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        #[builder(into, default)]
        pub index_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::logging::OrganizationBucketConfigIndexConfig>,
            >,
        >,
        /// The location of the bucket. The supported locations are: "global" "us-central1"
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The parent resource that contains the logging bucket.
        #[builder(into)]
        pub organization: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. Bucket retention can not be increased on buckets outside of projects.
        #[builder(into, default)]
        pub retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationBucketConfigResult {
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        pub bucket_id: pulumi_gestalt_rust::Output<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK
        /// key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by
        /// updating the log bucket. Changing the KMS key is allowed.
        pub cmek_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::logging::OrganizationBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        pub index_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::logging::OrganizationBucketConfigIndexConfig>,
        >,
        /// The bucket's lifecycle such as active or deleted. See [LifecycleState](https://cloud.google.com/logging/docs/reference/v2/rest/v2/billingAccounts.buckets#LogBucket.LifecycleState).
        pub lifecycle_state: pulumi_gestalt_rust::Output<String>,
        /// The location of the bucket. The supported locations are: "global" "us-central1"
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the bucket. For example: "organizations/my-organization-id/locations/my-location/buckets/my-bucket-id"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent resource that contains the logging bucket.
        pub organization: pulumi_gestalt_rust::Output<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. Bucket retention can not be increased on buckets outside of projects.
        pub retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationBucketConfigArgs,
    ) -> OrganizationBucketConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_id_binding = args.bucket_id.get_output(context);
        let cmek_settings_binding = args.cmek_settings.get_output(context);
        let description_binding = args.description.get_output(context);
        let index_configs_binding = args.index_configs.get_output(context);
        let location_binding = args.location.get_output(context);
        let organization_binding = args.organization.get_output(context);
        let retention_days_binding = args.retention_days.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:logging/organizationBucketConfig:OrganizationBucketConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketId".into(),
                    value: bucket_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cmekSettings".into(),
                    value: cmek_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexConfigs".into(),
                    value: index_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organization".into(),
                    value: organization_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionDays".into(),
                    value: retention_days_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationBucketConfigResult {
            bucket_id: o.get_field("bucketId"),
            cmek_settings: o.get_field("cmekSettings"),
            description: o.get_field("description"),
            index_configs: o.get_field("indexConfigs"),
            lifecycle_state: o.get_field("lifecycleState"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            organization: o.get_field("organization"),
            retention_days: o.get_field("retentionDays"),
        }
    }
}
