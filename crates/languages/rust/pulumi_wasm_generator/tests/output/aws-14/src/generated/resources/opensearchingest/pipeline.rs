/// Resource for managing an AWS OpenSearch Ingestion Pipeline.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: osis-pipelines.amazonaws.com
///   examplePipeline:
///     type: aws:opensearchingest:Pipeline
///     name: example
///     properties:
///       pipelineName: example
///       pipelineConfigurationBody: |
///         version: "2"
///         example-pipeline:
///           source:
///             http:
///               path: "/example"
///           sink:
///             - s3:
///                 aws:
///                   sts_role_arn: "${example.arn}"
///                   region: "${current.name}"
///                 bucket: "example"
///                 threshold:
///                   event_collect_timeout: "60s"
///                 codec:
///                   ndjson:
///       maxUnits: 1
///       minUnits: 1
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ### Using file function
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearchingest:Pipeline
///     properties:
///       pipelineName: example
///       pipelineConfigurationBody:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: example.yaml
///           return: result
///       maxUnits: 1
///       minUnits: 1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearch Ingestion Pipeline using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearchingest/pipeline:Pipeline example example
/// ```
pub mod pipeline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineArgs {
        /// Key-value pairs to configure persistent buffering for the pipeline. See `buffer_options` below.
        #[builder(into, default)]
        pub buffer_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::opensearchingest::PipelineBufferOptions>,
        >,
        /// Key-value pairs to configure encryption for data that is written to a persistent buffer. See `encryption_at_rest_options` below.
        #[builder(into, default)]
        pub encryption_at_rest_options: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::opensearchingest::PipelineEncryptionAtRestOptions,
            >,
        >,
        /// Key-value pairs to configure log publishing. See `log_publishing_options` below.
        #[builder(into, default)]
        pub log_publishing_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::opensearchingest::PipelineLogPublishingOptions>,
        >,
        /// The maximum pipeline capacity, in Ingestion Compute Units (ICUs).
        #[builder(into)]
        pub max_units: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The minimum pipeline capacity, in Ingestion Compute Units (ICUs).
        #[builder(into)]
        pub min_units: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The pipeline configuration in YAML format. This argument accepts the pipeline configuration as a string or within a .yaml file. If you provide the configuration as a string, each new line must be escaped with \n.
        #[builder(into)]
        pub pipeline_configuration_body: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the OpenSearch Ingestion pipeline to create. Pipeline names are unique across the pipelines owned by an account within an AWS Region.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub pipeline_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the pipeline. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::opensearchingest::PipelineTimeouts>,
        >,
        /// Container for the values required to configure VPC access for the pipeline. If you don't specify these values, OpenSearch Ingestion creates the pipeline with a public endpoint. See `vpc_options` below.
        #[builder(into, default)]
        pub vpc_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::opensearchingest::PipelineVpcOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineResult {
        /// Key-value pairs to configure persistent buffering for the pipeline. See `buffer_options` below.
        pub buffer_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearchingest::PipelineBufferOptions>,
        >,
        /// Key-value pairs to configure encryption for data that is written to a persistent buffer. See `encryption_at_rest_options` below.
        pub encryption_at_rest_options: pulumi_wasm_rust::Output<
            Option<
                super::super::types::opensearchingest::PipelineEncryptionAtRestOptions,
            >,
        >,
        /// The list of ingestion endpoints for the pipeline, which you can send data to.
        pub ingest_endpoint_urls: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value pairs to configure log publishing. See `log_publishing_options` below.
        pub log_publishing_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearchingest::PipelineLogPublishingOptions>,
        >,
        /// The maximum pipeline capacity, in Ingestion Compute Units (ICUs).
        pub max_units: pulumi_wasm_rust::Output<i32>,
        /// The minimum pipeline capacity, in Ingestion Compute Units (ICUs).
        pub min_units: pulumi_wasm_rust::Output<i32>,
        /// Amazon Resource Name (ARN) of the pipeline.
        pub pipeline_arn: pulumi_wasm_rust::Output<String>,
        /// The pipeline configuration in YAML format. This argument accepts the pipeline configuration as a string or within a .yaml file. If you provide the configuration as a string, each new line must be escaped with \n.
        pub pipeline_configuration_body: pulumi_wasm_rust::Output<String>,
        /// The name of the OpenSearch Ingestion pipeline to create. Pipeline names are unique across the pipelines owned by an account within an AWS Region.
        ///
        /// The following arguments are optional:
        pub pipeline_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the pipeline. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearchingest::PipelineTimeouts>,
        >,
        /// Container for the values required to configure VPC access for the pipeline. If you don't specify these values, OpenSearch Ingestion creates the pipeline with a public endpoint. See `vpc_options` below.
        pub vpc_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearchingest::PipelineVpcOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PipelineArgs,
    ) -> PipelineResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let buffer_options_binding = args.buffer_options.get_output(context).get_inner();
        let encryption_at_rest_options_binding = args
            .encryption_at_rest_options
            .get_output(context)
            .get_inner();
        let log_publishing_options_binding = args
            .log_publishing_options
            .get_output(context)
            .get_inner();
        let max_units_binding = args.max_units.get_output(context).get_inner();
        let min_units_binding = args.min_units.get_output(context).get_inner();
        let pipeline_configuration_body_binding = args
            .pipeline_configuration_body
            .get_output(context)
            .get_inner();
        let pipeline_name_binding = args.pipeline_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let vpc_options_binding = args.vpc_options.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearchingest/pipeline:Pipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bufferOptions".into(),
                    value: &buffer_options_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionAtRestOptions".into(),
                    value: &encryption_at_rest_options_binding,
                },
                register_interface::ObjectField {
                    name: "logPublishingOptions".into(),
                    value: &log_publishing_options_binding,
                },
                register_interface::ObjectField {
                    name: "maxUnits".into(),
                    value: &max_units_binding,
                },
                register_interface::ObjectField {
                    name: "minUnits".into(),
                    value: &min_units_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineConfigurationBody".into(),
                    value: &pipeline_configuration_body_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineName".into(),
                    value: &pipeline_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "vpcOptions".into(),
                    value: &vpc_options_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PipelineResult {
            buffer_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bufferOptions"),
            ),
            encryption_at_rest_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionAtRestOptions"),
            ),
            ingest_endpoint_urls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ingestEndpointUrls"),
            ),
            log_publishing_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logPublishingOptions"),
            ),
            max_units: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxUnits"),
            ),
            min_units: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minUnits"),
            ),
            pipeline_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pipelineArn"),
            ),
            pipeline_configuration_body: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pipelineConfigurationBody"),
            ),
            pipeline_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pipelineName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            vpc_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcOptions"),
            ),
        }
    }
}
