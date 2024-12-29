/// Provides a resource to manage an S3 Storage Lens configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let example = storage_lens_configuration::create(
///         "example",
///         StorageLensConfigurationArgs::builder()
///             .config_id("example-1")
///             .storage_lens_configuration(
///                 StorageLensConfigurationStorageLensConfiguration::builder()
///                     .accountLevel(
///                         StorageLensConfigurationStorageLensConfigurationAccountLevel::builder()
///                             .activityMetrics(
///                                 StorageLensConfigurationStorageLensConfigurationAccountLevelActivityMetrics::builder()
///                                     .enabled(true)
///                                     .build_struct(),
///                             )
///                             .bucketLevel(
///                                 StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevel::builder()
///                                     .activityMetrics(
///                                         StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelActivityMetrics::builder()
///                                             .enabled(true)
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .dataExport(
///                         StorageLensConfigurationStorageLensConfigurationDataExport::builder()
///                             .cloudWatchMetrics(
///                                 StorageLensConfigurationStorageLensConfigurationDataExportCloudWatchMetrics::builder()
///                                     .enabled(true)
///                                     .build_struct(),
///                             )
///                             .s3BucketDestination(
///                                 StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestination::builder()
///                                     .accountId("${current.accountId}")
///                                     .arn("${target.arn}")
///                                     .encryption(
///                                         StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryption::builder()
///                                             .sseS3s(
///                                                 vec![
///                                                     StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryptionSseS3::builder()
///                                                     .build_struct(),
///                                                 ],
///                                             )
///                                             .build_struct(),
///                                     )
///                                     .format("CSV")
///                                     .outputSchemaVersion("V_1")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .enabled(true)
///                     .exclude(
///                         StorageLensConfigurationStorageLensConfigurationExclude::builder()
///                             .buckets(vec!["${b1.arn}", "${b2.arn}",])
///                             .regions(vec!["us-east-2",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Storage Lens configurations using the `account_id` and `config_id`, separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/storageLensConfiguration:StorageLensConfiguration example 123456789012:example-1
/// ```
pub mod storage_lens_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StorageLensConfigurationArgs {
        /// The AWS account ID for the S3 Storage Lens configuration. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the S3 Storage Lens configuration.
        #[builder(into)]
        pub config_id: pulumi_wasm_rust::Output<String>,
        /// The S3 Storage Lens configuration. See Storage Lens Configuration below for more details.
        #[builder(into)]
        pub storage_lens_configuration: pulumi_wasm_rust::Output<
            super::super::types::s3control::StorageLensConfigurationStorageLensConfiguration,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StorageLensConfigurationResult {
        /// The AWS account ID for the S3 Storage Lens configuration. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the S3 Storage Lens configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the S3 Storage Lens configuration.
        pub config_id: pulumi_wasm_rust::Output<String>,
        /// The S3 Storage Lens configuration. See Storage Lens Configuration below for more details.
        pub storage_lens_configuration: pulumi_wasm_rust::Output<
            super::super::types::s3control::StorageLensConfigurationStorageLensConfiguration,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: StorageLensConfigurationArgs,
    ) -> StorageLensConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let config_id_binding = args.config_id.get_inner();
        let storage_lens_configuration_binding = args
            .storage_lens_configuration
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/storageLensConfiguration:StorageLensConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "configId".into(),
                    value: &config_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageLensConfiguration".into(),
                    value: &storage_lens_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configId".into(),
                },
                register_interface::ResultField {
                    name: "storageLensConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StorageLensConfigurationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            config_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configId").unwrap(),
            ),
            storage_lens_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageLensConfiguration").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
