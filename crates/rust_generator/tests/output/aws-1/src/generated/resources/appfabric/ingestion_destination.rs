/// Resource for managing an AWS AppFabric Ingestion Destination.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ingestion_destination::create(
///         "example",
///         IngestionDestinationArgs::builder()
///             .app_bundle_arn("${exampleAwsAppfabricAppBundle.arn}")
///             .destination_configuration(
///                 IngestionDestinationDestinationConfiguration::builder()
///                     .auditLog(
///                         IngestionDestinationDestinationConfigurationAuditLog::builder()
///                             .destination(
///                                 IngestionDestinationDestinationConfigurationAuditLogDestination::builder()
///                                     .s3Bucket(
///                                         IngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket::builder()
///                                             .bucketName("${exampleAwsS3Bucket.bucket}")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .ingestion_arn("${exampleAwsAppfabricIngestion.arn}")
///             .processing_configuration(
///                 IngestionDestinationProcessingConfiguration::builder()
///                     .auditLog(
///                         IngestionDestinationProcessingConfigurationAuditLog::builder()
///                             .format("json")
///                             .schema("raw")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ingestion_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IngestionDestinationArgs {
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        #[builder(into)]
        pub app_bundle_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Contains information about the destination of ingested data.
        #[builder(into, default)]
        pub destination_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appfabric::IngestionDestinationDestinationConfiguration,
            >,
        >,
        /// The Amazon Resource Name (ARN) of the ingestion to use for the request.
        #[builder(into)]
        pub ingestion_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Contains information about how ingested data is processed.
        #[builder(into, default)]
        pub processing_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appfabric::IngestionDestinationProcessingConfiguration,
            >,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appfabric::IngestionDestinationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct IngestionDestinationResult {
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        pub app_bundle_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Ingestion Destination.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Contains information about the destination of ingested data.
        pub destination_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appfabric::IngestionDestinationDestinationConfiguration,
            >,
        >,
        /// The Amazon Resource Name (ARN) of the ingestion to use for the request.
        pub ingestion_arn: pulumi_gestalt_rust::Output<String>,
        /// Contains information about how ingested data is processed.
        pub processing_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appfabric::IngestionDestinationProcessingConfiguration,
            >,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::appfabric::IngestionDestinationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IngestionDestinationArgs,
    ) -> IngestionDestinationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_bundle_arn_binding_1 = args.app_bundle_arn.get_output(context);
        let app_bundle_arn_binding = app_bundle_arn_binding_1.get_inner();
        let destination_configuration_binding_1 = args
            .destination_configuration
            .get_output(context);
        let destination_configuration_binding = destination_configuration_binding_1
            .get_inner();
        let ingestion_arn_binding_1 = args.ingestion_arn.get_output(context);
        let ingestion_arn_binding = ingestion_arn_binding_1.get_inner();
        let processing_configuration_binding_1 = args
            .processing_configuration
            .get_output(context);
        let processing_configuration_binding = processing_configuration_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appfabric/ingestionDestination:IngestionDestination".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appBundleArn".into(),
                    value: &app_bundle_arn_binding,
                },
                register_interface::ObjectField {
                    name: "destinationConfiguration".into(),
                    value: &destination_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "ingestionArn".into(),
                    value: &ingestion_arn_binding,
                },
                register_interface::ObjectField {
                    name: "processingConfiguration".into(),
                    value: &processing_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IngestionDestinationResult {
            app_bundle_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appBundleArn"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            destination_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationConfiguration"),
            ),
            ingestion_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingestionArn"),
            ),
            processing_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("processingConfiguration"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
