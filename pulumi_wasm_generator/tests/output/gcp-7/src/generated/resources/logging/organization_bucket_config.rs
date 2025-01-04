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
pub mod organization_bucket_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationBucketConfigArgs {
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        #[builder(into)]
        pub bucket_id: pulumi_wasm_rust::Output<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK
        /// key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by
        /// updating the log bucket. Changing the KMS key is allowed.
        #[builder(into, default)]
        pub cmek_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::logging::OrganizationBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        #[builder(into, default)]
        pub index_configs: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::logging::OrganizationBucketConfigIndexConfig>,
            >,
        >,
        /// The location of the bucket. The supported locations are: "global" "us-central1"
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The parent resource that contains the logging bucket.
        #[builder(into)]
        pub organization: pulumi_wasm_rust::Output<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. Bucket retention can not be increased on buckets outside of projects.
        #[builder(into, default)]
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationBucketConfigResult {
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        pub bucket_id: pulumi_wasm_rust::Output<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK
        /// key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by
        /// updating the log bucket. Changing the KMS key is allowed.
        pub cmek_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::logging::OrganizationBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        pub description: pulumi_wasm_rust::Output<String>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        pub index_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::logging::OrganizationBucketConfigIndexConfig>,
        >,
        /// The bucket's lifecycle such as active or deleted. See [LifecycleState](https://cloud.google.com/logging/docs/reference/v2/rest/v2/billingAccounts.buckets#LogBucket.LifecycleState).
        pub lifecycle_state: pulumi_wasm_rust::Output<String>,
        /// The location of the bucket. The supported locations are: "global" "us-central1"
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the bucket. For example: "organizations/my-organization-id/locations/my-location/buckets/my-bucket-id"
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent resource that contains the logging bucket.
        pub organization: pulumi_wasm_rust::Output<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. Bucket retention can not be increased on buckets outside of projects.
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationBucketConfigArgs,
    ) -> OrganizationBucketConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_id_binding = args.bucket_id.get_inner();
        let cmek_settings_binding = args.cmek_settings.get_inner();
        let description_binding = args.description.get_inner();
        let index_configs_binding = args.index_configs.get_inner();
        let location_binding = args.location.get_inner();
        let organization_binding = args.organization.get_inner();
        let retention_days_binding = args.retention_days.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/organizationBucketConfig:OrganizationBucketConfig"
                .into(),
            name: name.to_string(),
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
                    name: "indexConfigs".into(),
                    value: &index_configs_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
                register_interface::ObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucketId".into(),
                },
                register_interface::ResultField {
                    name: "cmekSettings".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "indexConfigs".into(),
                },
                register_interface::ResultField {
                    name: "lifecycleState".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "organization".into(),
                },
                register_interface::ResultField {
                    name: "retentionDays".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationBucketConfigResult {
            bucket_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketId").unwrap(),
            ),
            cmek_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cmekSettings").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            index_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexConfigs").unwrap(),
            ),
            lifecycle_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecycleState").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organization").unwrap(),
            ),
            retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionDays").unwrap(),
            ),
        }
    }
}
