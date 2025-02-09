/// Provides an AppFlow flow resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleSourceBucketV2:
///     type: aws:s3:BucketV2
///     name: example_source
///     properties:
///       bucket: example-source
///   exampleSourceBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: example_source
///     properties:
///       bucket: ${exampleSourceBucketV2.id}
///       policy: ${exampleSource.json}
///   example:
///     type: aws:s3:BucketObjectv2
///     properties:
///       bucket: ${exampleSourceBucketV2.id}
///       key: example_source.csv
///       source:
///         fn::FileAsset: example_source.csv
///   exampleDestinationBucketV2:
///     type: aws:s3:BucketV2
///     name: example_destination
///     properties:
///       bucket: example-destination
///   exampleDestinationBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: example_destination
///     properties:
///       bucket: ${exampleDestinationBucketV2.id}
///       policy: ${exampleDestination.json}
///   exampleFlow:
///     type: aws:appflow:Flow
///     name: example
///     properties:
///       name: example
///       sourceFlowConfig:
///         connectorType: S3
///         sourceConnectorProperties:
///           s3:
///             bucketName: ${exampleSourceBucketPolicy.bucket}
///             bucketPrefix: example
///       destinationFlowConfigs:
///         - connectorType: S3
///           destinationConnectorProperties:
///             s3:
///               bucketName: ${exampleDestinationBucketPolicy.bucket}
///               s3OutputFormatConfig:
///                 prefixConfig:
///                   prefixType: PATH
///       tasks:
///         - sourceFields:
///             - exampleField
///           destinationField: exampleField
///           taskType: Map
///           connectorOperators:
///             - s3: NO_OP
///       triggerConfig:
///         triggerType: OnDemand
/// variables:
///   exampleSource:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: AllowAppFlowSourceActions
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - appflow.amazonaws.com
///             actions:
///               - s3:ListBucket
///               - s3:GetObject
///             resources:
///               - arn:aws:s3:::example-source
///               - arn:aws:s3:::example-source/*
///   exampleDestination:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: AllowAppFlowDestinationActions
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - appflow.amazonaws.com
///             actions:
///               - s3:PutObject
///               - s3:AbortMultipartUpload
///               - s3:ListMultipartUploadParts
///               - s3:ListBucketMultipartUploads
///               - s3:GetBucketAcl
///               - s3:PutObjectAcl
///             resources:
///               - arn:aws:s3:::example-destination
///               - arn:aws:s3:::example-destination/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppFlow flows using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:appflow/flow:Flow example arn:aws:appflow:us-west-2:123456789012:flow/example-flow
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flow {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlowArgs {
        /// Description of the flow you want to create.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A Destination Flow Config that controls how Amazon AppFlow places data in the destination connector.
        #[builder(into)]
        pub destination_flow_configs: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::appflow::FlowDestinationFlowConfig>,
        >,
        /// ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
        #[builder(into, default)]
        pub kms_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A Catalog that determines the configuration that Amazon AppFlow uses when it catalogs the data that’s transferred by the associated flow. When Amazon AppFlow catalogs the data from a flow, it stores metadata in a data catalog.
        #[builder(into, default)]
        pub metadata_catalog_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appflow::FlowMetadataCatalogConfig>,
        >,
        /// Name of the flow.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Source Flow Config that controls how Amazon AppFlow retrieves data from the source connector.
        #[builder(into)]
        pub source_flow_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appflow::FlowSourceFlowConfig,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A Task that Amazon AppFlow performs while transferring the data in the flow run.
        #[builder(into)]
        pub tasks: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::appflow::FlowTask>,
        >,
        /// A Trigger that determine how and when the flow runs.
        #[builder(into)]
        pub trigger_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appflow::FlowTriggerConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct FlowResult {
        /// Flow's ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the flow you want to create.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A Destination Flow Config that controls how Amazon AppFlow places data in the destination connector.
        pub destination_flow_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appflow::FlowDestinationFlowConfig>,
        >,
        /// The current status of the flow.
        pub flow_status: pulumi_gestalt_rust::Output<String>,
        /// ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
        pub kms_arn: pulumi_gestalt_rust::Output<String>,
        /// A Catalog that determines the configuration that Amazon AppFlow uses when it catalogs the data that’s transferred by the associated flow. When Amazon AppFlow catalogs the data from a flow, it stores metadata in a data catalog.
        pub metadata_catalog_config: pulumi_gestalt_rust::Output<
            super::super::types::appflow::FlowMetadataCatalogConfig,
        >,
        /// Name of the flow.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Source Flow Config that controls how Amazon AppFlow retrieves data from the source connector.
        pub source_flow_config: pulumi_gestalt_rust::Output<
            super::super::types::appflow::FlowSourceFlowConfig,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A Task that Amazon AppFlow performs while transferring the data in the flow run.
        pub tasks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appflow::FlowTask>,
        >,
        /// A Trigger that determine how and when the flow runs.
        pub trigger_config: pulumi_gestalt_rust::Output<
            super::super::types::appflow::FlowTriggerConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlowArgs,
    ) -> FlowResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let destination_flow_configs_binding = args
            .destination_flow_configs
            .get_output(context);
        let kms_arn_binding = args.kms_arn.get_output(context);
        let metadata_catalog_config_binding = args
            .metadata_catalog_config
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let source_flow_config_binding = args.source_flow_config.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tasks_binding = args.tasks.get_output(context);
        let trigger_config_binding = args.trigger_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appflow/flow:Flow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationFlowConfigs".into(),
                    value: destination_flow_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsArn".into(),
                    value: kms_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadataCatalogConfig".into(),
                    value: metadata_catalog_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceFlowConfig".into(),
                    value: source_flow_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tasks".into(),
                    value: tasks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerConfig".into(),
                    value: trigger_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlowResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            destination_flow_configs: o.get_field("destinationFlowConfigs"),
            flow_status: o.get_field("flowStatus"),
            kms_arn: o.get_field("kmsArn"),
            metadata_catalog_config: o.get_field("metadataCatalogConfig"),
            name: o.get_field("name"),
            source_flow_config: o.get_field("sourceFlowConfig"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tasks: o.get_field("tasks"),
            trigger_config: o.get_field("triggerConfig"),
        }
    }
}
