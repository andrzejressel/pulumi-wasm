/// Manages a billing account level logging bucket config. For more information see
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
///     type: gcp:logging:BillingAccountBucketConfig
///     properties:
///       billingAccount: ${default.billingAccount}
///       location: global
///       retentionDays: 30
///       bucketId: _Default
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:organizations:getBillingAccount
///       arguments:
///         billingAccount: 00AA00-000AAA-00AA0A
/// ```
///
/// Create logging bucket with index configs
///
/// ```yaml
/// resources:
///   example-billing-account-bucket-index-configs:
///     type: gcp:logging:BillingAccountBucketConfig
///     properties:
///       folder: ${default.billingAccount}
///       location: global
///       retentionDays: 30
///       bucketId: _Default
///       indexConfigs:
///         - fieldPath: jsonPayload.request.status
///           type: INDEX_TYPE_STRING
/// ```
///
/// ## Import
///
/// This resource can be imported using the following format:
///
/// * `billingAccounts/{{billingAccount}}/locations/{{location}}/buckets/{{bucket_id}}`
///
/// When using the `pulumi import` command, this resource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/billingAccountBucketConfig:BillingAccountBucketConfig default billingAccounts/{{billingAccount}}/locations/{{location}}/buckets/{{bucket_id}}
/// ```
///
pub mod billing_account_bucket_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BillingAccountBucketConfigArgs {
        /// The parent resource that contains the logging bucket.
        #[builder(into)]
        pub billing_account: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        #[builder(into)]
        pub bucket_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK
        /// key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by
        /// updating the log bucket. Changing the KMS key is allowed.
        #[builder(into, default)]
        pub cmek_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::logging::BillingAccountBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        #[builder(into, default)]
        pub index_configs: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::logging::BillingAccountBucketConfigIndexConfig>,
            >,
        >,
        /// The location of the bucket.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. Bucket retention can not be increased on buckets outside of projects.
        #[builder(into, default)]
        pub retention_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct BillingAccountBucketConfigResult {
        /// The parent resource that contains the logging bucket.
        pub billing_account: pulumi_wasm_rust::Output<String>,
        /// The name of the logging bucket. Logging automatically creates two log buckets: `_Required` and `_Default`.
        pub bucket_id: pulumi_wasm_rust::Output<String>,
        /// The CMEK settings of the log bucket. If present, new log entries written to this log bucket are encrypted using the CMEK
        /// key provided in this configuration. If a log bucket has CMEK settings, the CMEK settings cannot be disabled later by
        /// updating the log bucket. Changing the KMS key is allowed.
        pub cmek_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::logging::BillingAccountBucketConfigCmekSettings>,
        >,
        /// Describes this bucket.
        pub description: pulumi_wasm_rust::Output<String>,
        /// A list of indexed fields and related configuration data. Structure is documented below.
        pub index_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::logging::BillingAccountBucketConfigIndexConfig>,
        >,
        /// The bucket's lifecycle such as active or deleted. See [LifecycleState](https://cloud.google.com/logging/docs/reference/v2/rest/v2/billingAccounts.buckets#LogBucket.LifecycleState).
        pub lifecycle_state: pulumi_wasm_rust::Output<String>,
        /// The location of the bucket.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the bucket. For example: "projects/my-project-id/locations/my-location/buckets/my-bucket-id"
        pub name: pulumi_wasm_rust::Output<String>,
        /// Logs will be retained by default for this amount of time, after which they will automatically be deleted. The minimum retention period is 1 day. If this value is set to zero at bucket creation time, the default time of 30 days will be used. Bucket retention can not be increased on buckets outside of projects.
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BillingAccountBucketConfigArgs,
    ) -> BillingAccountBucketConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let billing_account_binding = args
            .billing_account
            .get_output(context)
            .get_inner();
        let bucket_id_binding = args.bucket_id.get_output(context).get_inner();
        let cmek_settings_binding = args.cmek_settings.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let index_configs_binding = args.index_configs.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let retention_days_binding = args.retention_days.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/billingAccountBucketConfig:BillingAccountBucketConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding,
                },
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
                    name: "retentionDays".into(),
                    value: &retention_days_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "billingAccount".into(),
                },
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
                    name: "retentionDays".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BillingAccountBucketConfigResult {
            billing_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingAccount").unwrap(),
            ),
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
            retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionDays").unwrap(),
            ),
        }
    }
}
