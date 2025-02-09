/// Provides a resource to manage an S3 Storage Lens configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3control:StorageLensConfiguration
///     properties:
///       configId: example-1
///       storageLensConfiguration:
///         enabled: true
///         accountLevel:
///           activityMetrics:
///             enabled: true
///           bucketLevel:
///             activityMetrics:
///               enabled: true
///         dataExport:
///           cloudWatchMetrics:
///             enabled: true
///           s3BucketDestination:
///             accountId: ${current.accountId}
///             arn: ${target.arn}
///             format: CSV
///             outputSchemaVersion: V_1
///             encryption:
///               sseS3s:
///                 - {}
///         exclude:
///           buckets:
///             - ${b1.arn}
///             - ${b2.arn}
///           regions:
///             - us-east-2
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Storage Lens configurations using the `account_id` and `config_id`, separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/storageLensConfiguration:StorageLensConfiguration example 123456789012:example-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod storage_lens_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StorageLensConfigurationArgs {
        /// The AWS account ID for the S3 Storage Lens configuration. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the S3 Storage Lens configuration.
        #[builder(into)]
        pub config_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The S3 Storage Lens configuration. See Storage Lens Configuration below for more details.
        #[builder(into)]
        pub storage_lens_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3control::StorageLensConfigurationStorageLensConfiguration,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StorageLensConfigurationResult {
        /// The AWS account ID for the S3 Storage Lens configuration. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the S3 Storage Lens configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the S3 Storage Lens configuration.
        pub config_id: pulumi_gestalt_rust::Output<String>,
        /// The S3 Storage Lens configuration. See Storage Lens Configuration below for more details.
        pub storage_lens_configuration: pulumi_gestalt_rust::Output<
            super::super::types::s3control::StorageLensConfigurationStorageLensConfiguration,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StorageLensConfigurationArgs,
    ) -> StorageLensConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let config_id_binding = args.config_id.get_output(context);
        let storage_lens_configuration_binding = args
            .storage_lens_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/storageLensConfiguration:StorageLensConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configId".into(),
                    value: config_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageLensConfiguration".into(),
                    value: storage_lens_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StorageLensConfigurationResult {
            account_id: o.get_field("accountId"),
            arn: o.get_field("arn"),
            config_id: o.get_field("configId"),
            storage_lens_configuration: o.get_field("storageLensConfiguration"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
